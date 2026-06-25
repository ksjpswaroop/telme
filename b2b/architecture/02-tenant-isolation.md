# Tenant Isolation & Security Architecture

> How we keep tenant A from ever seeing tenant B's data. The single most important design decision in B2B SaaS.

## The threat model

We assume:

1. **Hostile network adversary** — TLS 1.3, certificate pinning for client → API.
2. **Curious insider** — engineers with DB access should not be able to read customer data.
3. **Compromised API token** — if a customer's SharePoint token leaks, scope blast radius must be small.
4. **Multi-tenant bugs** — the classic "off-by-one in WHERE clause" that exposes tenant B's rows to tenant A.

We do NOT assume:

1. Customer endpoint is fully secure (employee device may be compromised)
2. Tenant admin is fully trusted (a malicious admin shouldn't be able to read another tenant's data — but they CAN read their own)

## Defense in depth (5 layers)

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Network (TLS, HSTS, certificate pinning)         │
│  ┌──────────────────────────────────────────────────────┐ │
│  │  Layer 2: Auth (OAuth 2.0 + tenant claim in JWT)     │ │
│  │  ┌──────────────────────────────────────────────────┐ │ │
│  │  │  Layer 3: Per-tenant DB schema (row-level sec)   │ │ │
│  │  │  ┌──────────────────────────────────────────────┐ │ │ │
│  │  │  │  Layer 4: Per-tenant encryption keys (BYOK)   │ │ │ │
│  │  │  │  ┌──────────────────────────────────────────┐ │ │ │ │
│  │  │  │  │  Layer 5: Audit log + anomaly detection  │ │ │ │ │
│  │  │  │  └──────────────────────────────────────────┘ │ │ │ │
│  │  │  └──────────────────────────────────────────────┘ │ │ │
│  │  └──────────────────────────────────────────────────┘ │ │
│  └──────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Layer 1: Network

- TLS 1.3 everywhere, HSTS preloaded, HTTP/3
- Certificate pinning in Mac client (built into Tauri WebView)
- mTLS between internal services (Istio linkerd or Linkerd service mesh)
- WAF: Cloudflare in front of public APIs; rate limit per IP + per tenant

### Layer 2: Auth + Tenant Claim

Every request includes a JWT with these claims:

```json
{
  "sub": "user_abc123",            // Telme user ID
  "tenant_id": "acme-corp",          // CRITICAL — every query MUST filter by this
  "email": "alice@acme.com",
  "email_verified": true,
  "scopes": ["search", "admin"],
  "iat": 1735000000,
  "exp": 1735003600,                // 1 hour expiry
  "tenant_roles": ["member"]        // ["owner"] | ["admin"] | ["member"]
}
```

**Enforcement:** every API endpoint uses a shared extractor that:
1. Validates JWT signature + expiry
2. Pulls `tenant_id` claim
3. Sets a per-request "tenant scope" context (Postgres `SET LOCAL app.tenant_id = '...'` for RLS)

There is no way to write a query without a tenant scope. The DB literally refuses.

### Layer 3: Database-level tenant isolation

We use **PostgreSQL Row-Level Security (RLS)**:

```sql
-- Every tenant-scoped table gets RLS enabled
ALTER TABLE files ENABLE ROW LEVEL SECURITY;

-- Policy: only see rows where tenant_id matches the session variable
CREATE POLICY tenant_isolation ON files
  USING (tenant_id = current_setting('app.tenant_id')::uuid);

-- Every connection sets this on connect (via PgBouncer + app logic)
SET LOCAL app.tenant_id = '...';
```

**The application code never writes `WHERE tenant_id = ...` in queries** — the database enforces it. This is the strongest possible defense against "off-by-one" bugs.

**Defense in depth: app code also filters** by `tenant_id`. Two checks. If someone disables RLS, the app code still protects.

**Separate database per tenant tier:** Personal + Pro share a database. Business tier gets its own DB. Enterprise tier gets its own DB cluster (for data residency and contractual isolation). This is "noisy neighbor" protection.

### Layer 4: Per-tenant encryption keys (BYOK)

**At rest:**

- Database disk: encrypted (Postgres TDE or AWS RDS encryption)
- Object storage (raw file chunks): per-tenant encryption key
- Backups: encrypted with same key

**Bring Your Own Key (BYOK)** — Enterprise tier only:

- Customer provides encryption key in their cloud KMS (AWS KMS, GCP KMS, Azure Key Vault)
- Telme never sees the plaintext key
- Customer can revoke access → all their data becomes unreadable (compliance + exit strategy)
- Implemented with envelope encryption: KMS encrypts a DEK (data encryption key), DEK encrypts data, DEK stored alongside data but unusable without KMS unwrap

**Key hierarchy:**

```
Customer KMS (BYOK, optional)
    └── Tenant Master Key (TMK)
            └── Per-source DEK (SharePoint, Drive, Notion, ...)
                    └── Per-file content encryption key (CEK)
                            └── File content (AES-256-GCM)
```

This means:
- Revoking customer KMS access kills all their data (compliance gold)
- Compromise of one DEK only affects one source's content for that tenant
- We never see plaintext customer files unless we explicitly need to (we don't)

### Layer 5: Audit log + anomaly detection

Every action that touches customer data is logged:

```
audit_log:
  ts: 2026-06-25T15:30:00Z
  tenant_id: acme-corp
  user_id: user_abc123
  action: search.execute
  details: { query_hash: "sha256:..." }  // never log query text — PII risk
  ip: 1.2.3.4
  ua_hash: sha256(user_agent)
```

**Why query_hash, not query text?** Search queries often contain confidential info ("salary review Alice", "M&A target XYZ"). We hash queries for analytics without storing PII.

**Anomaly detection:**
- Volume spikes per user / per tenant → page on-call
- Searches for known sensitive patterns (e.g., regex for SSN) → flag for review
- Bulk file downloads → require step-up auth
- Off-hours access from new IP → email admin

**Retention:** 90 days hot, 2 years cold (compliance with SOC 2 + GDPR)

## Failure modes

| Failure | Blast radius | Mitigation |
|---|---|---|
| Tenant token leaked | One tenant's data | Force token rotation; auto-revoke after 90 days |
| Engineer DB access | All tenants' metadata, NOT content | RLS + encryption at rest + audit log |
| Customer KMS compromised | All of one tenant's data | Customer can revoke instantly; we can't decrypt without their key |
| Telme infra compromised | All tenants' metadata | Customer data encrypted at rest with per-tenant keys; rotate + customer can revoke |
| App bug exposes wrong tenant's data | 2 tenants | RLS makes this impossible at DB layer; audit log catches it anyway |
| Phishing of tenant admin | One tenant | SSO + MFA mandatory for Business tier; audit log flags unusual admin actions |

## Compliance posture (target by tier)

| Compliance | Free/Pro | Business | Enterprise |
|---|---|---|---|
| GDPR (EU) | ✅ | ✅ | ✅ |
| CCPA (California) | ✅ | ✅ | ✅ |
| SOC 2 Type 1 | ❌ | ❌ | ✅ (12 months in) |
| SOC 2 Type 2 | ❌ | ❌ | ✅ (24 months in) |
| ISO 27001 | ❌ | ❌ | 🔜 |
| HIPAA BAA | ❌ | ❌ | ✅ (paid add-on) |
| FedRAMP | ❌ | ❌ | 🔜 (12-month roadmap) |

**SOC 2 timeline:** start prep at $1M ARR. Audit + remediation = $50-100k + 6 months. Don't start before there's revenue to justify it.

## What we tell customers in the security whitepaper

> **"Telme is the only semantic search platform built for teams that can't send data to the cloud. By default, all file content is processed locally — on the user's device for individual tier, or in your private cloud region for team tier. We never read your files, train models on your data, or share with third parties. Enterprise customers can also bring their own encryption keys for an additional layer of control. The architectural promise: if our infrastructure is fully compromised, your files remain encrypted."**

## Open questions to resolve before launch

1. **Tenant data residency** — start US-only, add EU region when first EU customer appears (~$100/mo for EU AWS region)
2. **Customer-managed encryption keys** — BYOK is required for healthcare / gov; defer to v3
3. **Subprocessors list** — required for GDPR. Will include: AWS (hosting), Cloudflare (WAF), Stripe (billing), Ollama Cloud (optional opt-in embeddings), OpenAI (optional opt-in embeddings)
4. **Audit log export** — Enterprise customers want this for their own SOC 2 audits. Build as a CSV export endpoint.

See `02-tenant-isolation.md` for the encryption details and `../commercial/01-pricing-strategy.md` for how the tiers map to compliance.
