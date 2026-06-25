# Universal Connector Architecture

> One shape that fits every data source. Whether it's SharePoint, Notion, Slack, or a future connector, the same components do the work.

## Goals

1. **Reuse** — adding a new connector should be 80% configuration, 20% code
2. **Safe** — never accidentally exfiltrate data; fail closed
3. **Observable** — admin dashboard shows per-connector status, errors, last sync
4. **Resumable** — if a sync crashes, it picks up where it left off
5. **Permission-aware** — every result respects source ACLs

## High-level architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       Telme Tenant                          │
│  ┌──────────────────────────────────────────────────────┐ │
│  │              Source Connector (per type)              │ │
│  │                                                      │ │
│  │  ┌─────────┐   ┌─────────┐   ┌─────────┐           │ │
│  │  │  Auth   │ → │ Crawler │ → │Embedder │ → SQLite  │ │
│  │  └─────────┘   └─────────┘   └─────────┘           │ │
│  │       ↓             ↓             ↓                 │ │
│  │   Refresh      List files    Batch embed           │ │
│  │   tokens       (delta API)  (Ollama/OpenAI)         │ │
│  └──────────────────────────────────────────────────────┘ │
│                          ↓                                   │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                   Tenant Index                       │ │
│  │   files + chunks + vectors + ACL tokens             │ │
│  └──────────────────────────────────────────────────────┘ │
│                          ↓                                   │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                  Search API                          │ │
│  │   /v1/search?q=...   (auth, rate-limit, hybrid)     │ │
│  └──────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                           ↓
            ┌──────────────────────────────┐
            │   Telme Mac / Web / Mobile   │
            │   client (renders results)    │
            └──────────────────────────────┘
```

## The Source trait

Every connector implements this Rust trait:

```rust
#[async_trait]
pub trait Source: Send + Sync {
    /// Display name for this source ("Acme SharePoint")
    fn name(&self) -> &str;

    /// Stable type identifier ("sharepoint", "gdrive")
    fn kind(&self) -> &'static str;

    /// OAuth flow (user clicks "Connect SharePoint" → MSAL pop-up → token)
    async fn authorize(&self, ctx: AuthCtx) -> Result<TokenSet>;

    /// Refresh expired tokens (called by background scheduler)
    async fn refresh_token(&self, current: &TokenSet) -> Result<TokenSet>;

    /// Initial scan + ongoing delta. Returns when complete or on error.
    /// Resumable via `Checkpoint`.
    async fn crawl(
        &self,
        conn: &mut Conn,
        since: Option<Checkpoint>,
    ) -> Result<CrawlResult>;

    /// Fetch text content for a single file. Returns bytes + mime type.
    async fn fetch_content(&self, item_id: &str) -> Result<(Bytes, String)>;

    /// Return the user's permission scope for this source.
    /// Used to filter search results by ACL.
    async fn permissions_for(&self, user: &UserId) -> Result<PermissionSet>;
}
```

A `Connector` struct implements `Source` for one specific service. We register each in a `ConnectorRegistry`.

## Auth flow (OAuth 2.0 standard)

```
User clicks "Connect SharePoint" in Telme admin UI
   ↓
Telme redirects to: https://login.microsoftonline.com/.../authorize
   ?client_id=...
   &redirect_uri=https://app.telme.com/oauth/sharepoint/callback
   &scope=Files.Read.All,Sites.Read.All
   &state=<signed_random>
   ↓
User signs in to Microsoft + grants consent (or admin grants tenant-wide)
   ↓
Microsoft redirects back to: https://app.telme.com/oauth/sharepoint/callback?code=...&state=...
   ↓
Telme exchanges code for tokens (POST /token endpoint)
   ↓
Tokens stored encrypted in tenant DB
   ↓
Background sync worker uses refresh_token to keep access_token alive
```

**Each connector implements `authorize()` differently**, but the surrounding flow is identical. The framework provides:
- State CSRF token (signed, single-use)
- Token storage with encryption-at-rest
- Refresh scheduler

### Multi-tenant auth for SharePoint / Graph
Use the **on-behalf-of flow** (OBO): the user signs in once, the Telme backend can act on their behalf with their permissions. For app-wide org search, we additionally use **SharePoint App-Only tokens** (client credentials) and filter results by user ACLs at query time.

### Refresh token storage
- Encrypted at rest with `AES-256-GCM`
- Key stored in cloud KMS (AWS KMS / GCP KMS) for hosted backend, OS Keychain for desktop-only
- Tokens never logged
- Rotated on every refresh

## Crawler architecture

Every connector implements a unified `CrawlResult`:

```rust
pub struct CrawlResult {
    /// Files discovered/updated/deleted since last crawl
    pub upserts: Vec<FileMetadata>,
    pub deletes: Vec<String>,  // file IDs
    /// Resumption token for next call (opaque to caller)
    pub checkpoint: Option<Checkpoint>,
    /// Whether the crawl completed (false = paginate more)
    pub done: bool,
}

pub struct FileMetadata {
    pub id: String,                  // connector-specific
    pub name: String,
    pub mime: String,
    pub size_bytes: u64,
    pub modified: DateTime<Utc>,
    pub url: Option<String>,         // clickable in results
    pub owner: Option<String>,
    pub acl: AclRef,                 // for permission filtering
    pub extra: serde_json::Value,    // connector-specific metadata
}
```

The crawler:
1. Calls the source's API with the checkpoint (or from-scratch on first sync)
2. Paginates until `done: true`
3. Persists upserts to `files` table, deletes to `files` table (tombstones)
4. Emits embedding jobs to a queue
5. Returns the new checkpoint

## Embedding pipeline

```
Crawler writes file metadata to `files` table
   ↓
Embedding worker picks up files where `embedding_status = 'pending'`
   ↓
Worker fetches content (Connector::fetch_content)
   ↓
Text extraction pipeline (PDF, Office, Markdown, code, etc.)
   ↓
Chunking (Phase 2 chunker, 512 tokens, 50 overlap)
   ↓
Embedding (Ollama locally; OpenAI text-embedding-3 opt-in for Pro tier)
   ↓
Vector stored in shared Qdrant (or sqlite-vec for local-only)
   ↓
`files.embedding_status = 'ready'`
```

Each chunk also gets metadata:
- `source_kind` ("sharepoint", "gdrive")
- `source_path` (URL within the source)
- `acl_token` (for filtering)
- `last_modified`

This means a search result can render as: "📄 found in SharePoint > Engineering > Specs > v3-architecture.docx" or "💬 from Slack #eng-general, posted 3 days ago by Alice."

## Permission / ACL handling

This is the most important part of B2B. **Every search result must respect the source's ACLs.**

### Per-user search (default)
Each user's Mac client (or web session) authenticates with the source. The search query is executed server-side with that user's token, and the source's API returns only what they can see. **We never see the data we shouldn't.**

### Org-wide search (Business+ tier)
For team-wide search, we use a service account with broad read access. For each result, before returning to a user, we check whether the user has read permission. Two strategies:

**Strategy A: Check on-demand (slow but safe)**
For each result, call the source's permission API: "can user X see file Y?" If yes, return; if no, filter out.

**Strategy B: Pre-computed ACL graph (fast)**
Build a `users → files` ACL graph at sync time. Periodically refresh. Search joins against this graph. **Privacy trade-off**: we have a copy of who-can-see-what, which is itself sensitive.

We default to Strategy A. Customers who want Strategy B for performance can opt-in (with appropriate compliance review).

## Sync scheduling

```rust
// Per-tenant scheduler
pub struct SyncScheduler {
    tenant_id: TenantId,
    connectors: Vec<Box<dyn Source>>,
    interval: Duration,  // default 15 minutes
    max_concurrent: usize,
}

impl SyncScheduler {
    /// Run forever: every interval, sync each source
    pub async fn run_forever(self) {
        loop {
            for connector in &self.connectors {
                let result = connector.crawl(...).await;
                log_and_handle(result);
            }
            tokio::time::sleep(self.interval).await;
        }
    }
}
```

**Adaptive intervals:**
- New source → 1-minute polling for the first hour (catch initial sync quickly)
- Active source → 15-minute polling
- Quiet source (no changes) → 1-hour polling
- Always use delta queries (not full re-scans)

## Rate limit handling

Every external API has rate limits. We respect them with a token bucket:

```rust
pub struct RateLimiter {
    tokens: AtomicU64,
    capacity: u64,
    refill_rate: f64,  // tokens per second
}

impl RateLimiter {
    pub async fn acquire(&self) -> Acquired {
        loop {
            if self.try_consume(1) { return Acquired; }
            let wait = self.time_to_next_token();
            tokio::time::sleep(wait).await;
        }
    }
}
```

Per-connector rate limits (Microsoft Graph: 10k req / 10 min / app / tenant; Google: 1k req / 100 sec / user; Notion: 3 req / sec; Slack: tier-based):

| Service | Limit | Our budget |
|---|---|---|
| Microsoft Graph | 10k/10min | 6k/10min (60% for safety) |
| Google Workspace | 1k/100s/user | 800/100s |
| Notion | 3/sec | 2/sec |
| Slack | tier 3: ~50/min | 40/min |
| Confluence | 5k/hr | 4k/hr |

The crawler uses the rate limiter to stay under budgets.

## Observability

Every connector emits structured logs and metrics:

```
log:
  level=info
  source=sharepoint
  tenant_id=acme-corp
  files_processed=247
  files_failed=3
  bytes_downloaded=48291093
  embeddings_computed=1247
  duration_ms=18394
  checkpoint="abc123..."
```

Per-tenant metrics exposed via `/v1/admin/metrics`:
- Per-source: last sync time, error count, files indexed, bytes
- Per-user: query latency, most-searched sources
- Per-system: queue depth, worker saturation

## Failure modes & recovery

| Failure | Detection | Recovery |
|---|---|---|
| Token expired | 401 from API | Auto-refresh; retry once |
| Rate limited | 429 from API | Backoff exponentially; resume after Retry-After |
| Network blip | TCP timeout | Retry with jittered backoff |
| Source changed schema | Parser error | Mark source as degraded; alert admin; continue with cached data |
| File content too large | Skip with warning | Don't fail the whole sync |
| Disk full | Write fails | Pause sync; alert admin |
| Worker crashes | Heartbeat timeout | Restart; resume from checkpoint |

**The system is designed to fail closed** — a connector in error state doesn't take down the rest.

## Configuration per source type

```yaml
# config/sources/sharepoint.yaml
name: "Acme SharePoint"
kind: sharepoint
tenant_id: acme-corp
auth:
  type: oauth2
  client_id_env: SP_CLIENT_ID
  client_secret_env: SP_CLIENT_SECRET
  scopes: [Files.Read.All, Sites.Read.All]
sync:
  interval_seconds: 900  # 15 min
  initial_scan_concurrency: 8
  delta_scan_concurrency: 4
indexing:
  embedding_model: nomic-embed-text  # or text-embedding-3-small
  max_chunk_tokens: 512
  include_mime_types: ["*"]
  exclude_paths: ["*/archive/*", "*/.git/*"]
permissions:
  strategy: on_demand
  cache_ttl_seconds: 300
```

Same shape for every connector. Adding a new source = writing one YAML + implementing the `Source` trait.

## What the Mac client sees

When we go multi-tenant, the Mac client:

1. User logs into Telme with their email (SSO)
2. Telme cloud pushes the user's authorized sources ("Acme SharePoint", "Personal Drive")
3. Searches happen against the cloud index (which has all sources combined)
4. Results respect the user's permissions automatically
5. Files can be opened locally (if cached) or in the source app (browser for SharePoint)

The Mac client becomes a **thin viewer** for a cloud-hosted search index. The local-first Mac app (today's Telme) becomes the **Personal** tier — local files only, no cloud sync.

## Open questions

1. **Do we ever send files to OpenAI for embedding?** If user opted in to "better embeddings" yes, otherwise no. Need clear UX.
2. **How do we handle deleted files?** Tombstones for 30 days, then hard delete.
3. **Multi-region?** Start US-only, expand to EU when EU customers appear (GDPR data residency).
4. **Offline mode?** Cloud index is the source of truth; client queues queries when offline; syncs when reconnected.

See `03-source-tier-plan.md` for the prioritized connector roadmap.
