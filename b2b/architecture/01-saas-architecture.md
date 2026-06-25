# SaaS Architecture

> What changes when Telme moves from a local-first single-user app to a multi-tenant SaaS. New components, their responsibilities, and how they compose.

## The big picture

Today, Telme is a **local-first Mac app** with a local SQLite index. Going multi-tenant SaaS means:

- The **local Mac app stays** — it becomes the **Personal** tier (free + Pro) for local files
- A **cloud backend** appears — it indexes all connected sources per tenant, serves search, handles auth
- A **web admin** appears — for tenant onboarding, source connection, user management
- A **mobile / web search UI** may appear — so users can search from phone or non-Mac

## Three architectural decisions

Before drawing boxes, three choices:

### 1. Where does the index live?

Three options:

| Option | Pros | Cons |
|---|---|---|
| **A. On each Mac client** | Pure local-first; cheapest infra | No shared indexes; no team search |
| **B. On a cloud backend (per tenant)** | Shared indexes; team search; cloud-friendly | Infra cost; multi-tenant security |
| **C. Hybrid (client + cloud)** | Local fast path; cloud for shared; resilient | Complex sync; data duplication |

**We pick C.** It's the only way to honor the "local-first when possible, cloud when needed" promise. The Mac client caches its user's accessible subset; queries hit local first, fall back to cloud.

### 2. Who runs the embedding model?

Options:

| Option | Pros | Cons |
|---|---|---|
| **A. Mac client (Ollama)** | True local-first; privacy story | Slow at scale; inconsistent quality |
| **B. Cloud (OpenAI text-embedding-3)** | Best quality; consistent | Cloud cost; data leaves |
| **C. Cloud (self-hosted GPU)** | Consistent; data stays in our infra | Capex; ops burden |

**We pick A by default, B as opt-in for Pro/Business, C for Enterprise.** Free users get local embeddings (matches v1). Paid tiers can choose cloud embeddings for better recall.

### 3. What's the trust boundary?

**We don't see user data.** The Telme cloud stores embeddings + metadata + ACLs. It can fetch file content for indexing, but doesn't retain it after the chunk is vectorized. For org-wide search, we use app-only tokens and filter results per user at query time.

This is the key differentiator from Glean / Copilot: **we never store your documents in cleartext.**

## Components

```
┌──────────────────────────────────────────────────────────────┐
│                  Telme Cloud (managed by us)                 │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│  │  API Gateway │  │ Tenant       │  │ Source Sync  │       │
│  │  (REST+gRPC)│  │ Service      │  │ Workers      │       │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       │
│         │                 │                 │                │
│         ▼                 ▼                 ▼                │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  Tenant Postgres                      │  │
│  │  per-tenant schema: users, files, chunks, sources  │  │
│  └──────────────────────────────────────────────────────┘  │
│                          │                                  │
│                          ▼                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │            Shared Qdrant vector cluster               │  │
│  │  per-tenant collections; ACL-filtered at query time │  │
│  └──────────────────────────────────────────────────────┘  │
│                          │                                  │
│                          ▼                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  Web Admin (Next.js)                 │  │
│  │  tenant onboarding, source connection, billing       │  │
│  └──────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                           ▲ ▲
                           │ │
            ┌──────────────┘ └──────────────┐
            │                                │
┌───────────┴──────────┐         ┌────────────┴───────────┐
│  Mac Client         │         │  iOS / Web Client      │
│  (Tauri + Rust)     │         │  (later, optional)    │
│  Local-first tier   │         │  Cloud-only tier       │
└─────────────────────┘         └────────────────────────┘
```

## Component details

### API Gateway
- **Tech:** Rust + axum (we already use Tauri/Rust; stay in one language)
- **Endpoints:**
  - `POST /v1/auth/login` — email + SSO
  - `GET /v1/sources` — list user's connected sources
  - `POST /v1/sources/{kind}/connect` — start OAuth flow, return URL
  - `POST /v1/sources/{kind}/callback` — complete OAuth
  - `POST /v1/search` — query, return ranked hits
  - `GET /v1/admin/metrics` — for tenant admins
  - `POST /v1/billing/webhook` — Stripe events
- **Auth:** JWT (short-lived, 15 min) + refresh token (7 days, stored in httpOnly cookie)
- **Rate limit:** 100 req/min/user (search), 10 req/min/user (admin)
- **Region:** US-east-1 primary; eu-west-1 mirror (data residency for EU customers)

### Tenant Service
- **Responsibilities:** create tenant, invite users, manage SSO, billing, audit logs
- **Storage:** Postgres `tenants` table, `users` table, `memberships` table
- **Multi-tenancy model:** shared schema with `tenant_id` on every row + Postgres Row-Level Security policies

### Source Sync Workers
- **Responsibilities:** for each tenant, for each connected source, run crawl → extract → chunk → embed → store
- **Topology:** per-tenant worker pool, 1 worker per source (per source can be slow, parallelize per source)
- **Concurrency:** up to 8 workers per tenant, global cap 1000 workers
- **State machine:** `pending → crawling → extracting → embedding → ready` or `failed`
- **Retry:** exponential backoff on transient failures, manual retry from admin UI for permanent failures
- **Resumable:** via checkpoint token (see `02-connector-design.md`)

### Tenant Postgres
- **Schema per tenant:**
  ```sql
  CREATE SCHEMA tenant_<id>;
  CREATE TABLE tenant_<id>.files (
    id BIGSERIAL PRIMARY KEY,
    source_kind TEXT NOT NULL,
    source_id TEXT NOT NULL,
    path TEXT NOT NULL,
    mime TEXT,
    size_bytes BIGINT,
    modified_at TIMESTAMPTZ,
    embedding_status TEXT,
    acl JSONB,  -- simplified ACL token for filtering
    UNIQUE(source_kind, source_id)
  );
  CREATE TABLE tenant_<id>.chunks (
    id BIGSERIAL PRIMARY KEY,
    file_id BIGINT REFERENCES files(id) ON DELETE CASCADE,
    ordinal INT,
    text TEXT,
    token_count INT,
    vector_id TEXT,  -- Qdrant point ID
    acl JSONB
  );
  ```
- **Postgres Row-Level Security** enforces tenant isolation at the DB layer
- **Backups:** daily snapshot to S3, 30-day retention

### Shared Qdrant cluster
- **Topology:** Qdrant cluster, 3 nodes, replication factor 2
- **Per-tenant collections:** `tenant_<id>` (separate Qdrant collection per tenant)
- **Vector dimension:** 768 (nomic-embed-text) or 1536 (OpenAI text-embedding-3-small), chosen at tenant signup
- **HNSW index:** M=16, efConstruction=100 for fast ANN at scale
- **Filtering:** ACL filter applied at search time using Qdrant's payload filters

### Web Admin (Next.js + React)
- **Pages:**
  - `/login` — email + SSO
  - `/onboarding` — pick your plan, connect first source
  - `/sources` — list of connected sources, add new, view sync status
  - `/users` — team members, invitations, role management
  - `/billing` — subscription, invoices, payment method
  - `/settings` — embedding model, retention, SSO config
  - `/audit` — admin: every connector sync, every search, every permission decision
- **Hosted on:** Vercel (Next.js native) or Cloudflare Pages
- **Auth:** same JWT flow as API; web uses session cookies

### Mac Client (Tauri + React, mostly unchanged)
- New in v2:
  - Sign in to your Telme account (syncs your source list)
  - "Local-only mode" toggle (don't sync anything to cloud)
  - "Sync this folder to my team" toggle (requires Business plan)
- **Architecture:**
  - Local SQLite index for offline + personal tier
  - Cloud search API for team tier
  - Transparent: typing in the bar queries both, merges results

## Multi-tenancy isolation — the security model

Tenants must not be able to see each other's data, even by accident. Defenses:

| Layer | Defense |
|---|---|
| **Network** | Per-tenant VPC in production; strict security groups |
| **Compute** | Per-tenant worker queues (no shared thread pools across tenants) |
| **Database** | Postgres Row-Level Security + per-tenant schemas; Qdrant per-tenant collections |
| **Auth** | Every API call carries tenant_id; backend validates on every query |
| **Embeddings** | Per-tenant vector namespace in Qdrant |
| **Logs** | Logs are tenant-tagged; log access is gated to tenant admins |
| **Backups** | Encrypted per-tenant with tenant-scoped keys |
| **Crypto** | AES-256-GCM at rest, TLS 1.3 in transit, KMS-managed keys |

Penetration testing before any enterprise sale. SOC 2 Type 1 within 12 months of starting Business tier.

## Search API design

```http
POST /v1/search
Authorization: Bearer <jwt>
Content-Type: application/json

{
  "query": "rust performance optimization",
  "limit": 10,
  "sources": ["sharepoint", "gdrive", "notion"],
  "recency_days": 365
}

→ 200 OK
{
  "hits": [
    {
      "id": "sp:abc123:0",
      "source_kind": "sharepoint",
      "title": "Rust Performance Notes",
      "url": "https://acme.sharepoint.com/...",
      "snippet": "Cargo build --release for production binaries...",
      "score": 0.87,
      "kind": "semantic",
      "modified_at": "2024-03-15T...",
      "owner": "alice@acme.com",
      "acl_verified": true
    }
  ],
  "latency_ms": 142,
  "degraded": false
}
```

`acl_verified: true` means Telme checked the user has read access on this file before returning. **Default for all B2B queries.**

## The flow: "Add SharePoint" in the Mac client

```
1. User clicks "Add SharePoint" in Mac client
   ↓
2. Mac client opens browser → telme.com/connect/sharepoint
   ↓
3. User signs in to Telme (or already signed in)
   ↓
4. Web admin shows MSAL popup → user grants Files.Read.All, Sites.Read.All
   ↓
5. MSAL redirects back to telme.com/callback?code=...
   ↓
6. Telme backend exchanges code for tokens
   ↓
7. Backend stores encrypted refresh_token in tenant_<id>.oauth_tokens
   ↓
8. Backend kicks off initial sync (background)
   ↓
9. Mac client polls /v1/sources until SharePoint shows "indexing" → "ready"
   ↓
10. Mac client's local search now includes SharePoint results
```

## Cost model (1,000 paying teams × 50 seats)

| Component | Cost/mo |
|---|---|
| EC2 instances (4 × m6i.xlarge) | $500 |
| RDS Postgres (db.r6g.2xlarge) | $700 |
| Qdrant cluster (3 × r6g.large + 100GB gp3) | $300 |
| S3 (embeddings cache, backups) | $100 |
| CloudFront (API + web) | $200 |
| Embedding compute (Ollama on dedicated hosts) | $1,500 |
| Aggregator fees (Tier 2+) | $1,000 |
| Misc (monitoring, email, Stripe) | $300 |
| **Total infra** | **~$4,600/mo** |
| Per-team | **$4.60/mo** |
| At $15/user × 50 seats = **$9,000/team/yr ARR** | **95% gross margin** |

This is very healthy SaaS economics. We can sustain ~$4-5M ARR with $500k-$1M of infra.

## Failure modes & disaster recovery

| Scenario | Detection | Recovery |
|---|---|---|
| Postgres primary dies | RDS failover to replica (1 min) | Automatic; no data loss |
| Qdrant node dies | Cluster self-heals (replication factor 2) | No data loss; query latency briefly elevated |
| Sync worker crashes | Heartbeat timeout | Restart; resume from checkpoint |
| Token leak | Audit log alert | Revoke; force re-auth |
| Tenant data corruption | Daily integrity check | Restore from backup; replay connector |
| Region down | Health check | DNS failover to backup region (within 30 min) |

RPO (Recovery Point Objective): 1 hour. RTO (Recovery Time Objective): 4 hours.

## What this means for the codebase

To support SaaS mode, we need to refactor Telme v1 into:

1. **Library `telme-core`** — the indexer, search, embedder — usable from server and client
2. **Binary `telme-server`** — the cloud backend (API gateway + tenant service + sync workers)
3. **Binary `telme`** — the existing Mac client (unchanged UX; gains auth + source list)
4. **App `telme-admin`** — the Next.js web admin
5. **Shared schema** — Postgres + Qdrant migrations live in `telme-core/migrations/`

The current `src-tauri/src/` becomes the foundation for `telme-core`. Most modules move unchanged. New modules: `tenant.rs`, `source/{sharepoint,...}.rs`, `api/`, `sync_worker.rs`, `billing/`.

## Hiring implications (when we go SaaS)

| Role | When | Why |
|---|---|---|
| Backend engineer (Rust) | Month 1 | Build connector framework + SharePoint + Drive |
| Backend engineer (Rust) | Month 3 | Build Tier 2 connectors + sync reliability |
| Frontend engineer (React/Next.js) | Month 1 | Build web admin |
| Designer | Month 2 | Polish Mac client + web admin to "this is premium" level |
| DevOps / SRE | Month 4 | SOC 2, monitoring, infra scaling |
| Sales / AE | Month 6 | Close first 10 Business tier customers |
| Total by month 6 | 6 people | At $200-300k fully-loaded each, ~$1.5M annual burn |

This is achievable on a $3-5M seed round with 24 months of runway.

See `02-tenant-isolation.md` for the detailed security architecture.
