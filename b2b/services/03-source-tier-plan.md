# Source Tier Plan — What to Ship in v2, v3, v4

> Phased connector roadmap. Effort estimates assume 1 experienced Rust engineer. All tiers preserve the Telme brand promise: local-first, beautiful, semantic.

## The constraint

We have limited engineering capacity. We must pick the **minimum connector set** that:

1. Validates the B2B motion (users will pay for shared indexes)
2. Covers the largest installed base (SharePoint + Drive = 80%+ of files)
3. Hits the "everyone uses these" baseline (Notion + Slack + Confluence)
4. Doesn't kill us with maintenance burden (10 enterprise connectors = 10 OAuth integrations)

## Tier mapping

### Tier 1 (v2, ships 4-6 months after seed): "The Microsoft + Google + Notes" stack

These cover ~80% of the addressable market:

| Connector | Effort | Why |
|---|---|---|
| **Local file system** | DONE | Telme v1 already does this |
| **SharePoint / OneDrive** | 5 weeks | Largest B2B market; SharePoint + OneDrive cover Microsoft shops |
| **Google Drive / Workspace** | 4 weeks | Second-largest; covers Google-native shops |
| **Notion** | 2 weeks | Loved by our early adopters; complement not compete |
| **Confluence** | 3 weeks | Atlassian shops are a clear ICP |
| **Slack** | 3 weeks | "Where did Alice post that doc?" is asked 10x/day |

**Total v2 effort: ~19 weeks** (1 engineer, 5 months including cloud infrastructure)

### Tier 2 (v3, ships 4-6 months after v2): "The Dev Tools" stack

| Connector | Effort | Why |
|---|---|---|
| **GitHub** | 4 weeks | Code search across PRs/issues/comments |
| **Gmail** | 3 weeks | "Where did they say X?" — universal pain |
| **Linear** | 2 weeks | Issue tracking; quick win |
| **Microsoft Teams** | 4 weeks | Chat + files; shares Graph API with SharePoint |
| **Dropbox** | 3 weeks | SMB + creative agencies |
| **Outlook 365** | 3 weeks | Email for Microsoft shops |
| **GitLab** | 3 weeks | Self-hosted alternative |

**Total v3 effort: ~22 weeks**

### Tier 3 (v4+): Vertical & niche

Build based on user votes (in-app poll), weighted by ACV:

- **Biotech**: Benchling, Dotmatics, ELN systems (high ACV, low volume)
- **Legal**: iManage, NetDocuments, Ironclad (high ACV, niche)
- **Sales/CRM**: Salesforce, HubSpot
- **Design**: Figma (metadata + comments only)
- **Customer support**: Zendesk, Intercom
- **Other**: Asana, Monday, Trello, Box, Airtable, Calendar, etc.

**Per connector: 1-3 weeks.**

## The build-vs-buy decision

Two ways to add connectors:

### Option A: Build each connector ourselves
**Pros:** Full control; matches our exact security model; no per-connector licensing cost.
**Cons:** ~3 weeks per connector *forever*; we own OAuth refresh logic, rate limits, API changes.

### Option B: Use an aggregator (e.g., Unipile, Nango, Merge.dev)
**Pros:** Faster to ship (~1 week per connector after aggregator supports it); aggregators handle rate limits, OAuth, schema drift.
**Cons:** Monthly fee per connector ($100-500/mo); adds dependency; some aggregators don't support all the sources we want.

### Recommendation: Hybrid

- **Tier 1 (v2):** Build SharePoint + Google Drive ourselves (these are the largest, most-revenue-critical, and have complex enough auth that aggregators cost similar effort).
- **Tier 1 (v2):** Use aggregator for Notion, Confluence, Slack (commodity, fast to ship).
- **Tier 2 onwards:** Use aggregator for everything; build custom only when we have unique requirements.

Cost comparison at 1,000 paying teams (avg 50 seats):

- Self-built: $0/mo per connector, ~$5k/mo infra
- Aggregator: ~$500/mo per connector × 10 connectors = $5k/mo
- Hybrid: ~$3k/mo total

Same cost, much faster shipping.

## Concrete v2 roadmap (months 1-6 after seed)

### Month 1-2: Cloud backend MVP

- Auth: SSO + tenant model
- Database: per-tenant Postgres + shared Qdrant cluster
- API: REST search endpoint with rate limiting
- Web admin: tenant onboarding, source connection, user management
- Status: alpha with 5 design-partner customers

### Month 2-3: First two connectors (SharePoint + Drive)

- Build SharePoint connector (5 weeks)
- Build Google Drive connector in parallel (4 weeks)
- Migrate existing local file system code into the new architecture
- Status: beta with 20 customers, shareable indexes

### Month 3-4: Three more connectors via aggregator

- Notion, Confluence, Slack
- Use Nango or Unipile (3 weeks to integrate, 1 week per connector config)
- Status: 50 customers, 10 paying teams

### Month 4-5: Polish & paid launch

- Pricing page, billing (Stripe), self-serve team signup
- SOC 2 Type 1 prep (engage auditor)
- Marketing site: telme.com with deep SEO content
- Status: 100 paying teams, $200k ARR run-rate

### Month 6: Series Seed raise

- With traction, raise $2-5M seed
- Hire 3-4 engineers to build Tier 2 + enterprise tier

## Effort summary

| Phase | Effort | Calendar |
|---|---|---|
| v1 (Local-first Mac) | DONE | shipped |
| v2 cloud backend + 5 connectors | ~25 weeks | 6 months post-seed |
| v3 dev tools + email (7 connectors) | ~22 weeks | 5 months |
| v4 vertical (5-10 connectors) | ~15-30 weeks | 4-8 months |
| **Total to "mature SaaS"** | **~80-100 weeks** | **18-24 months** |

## "Should we build this?" — the honest answer

**If you've raised $2-5M seed and have 2-3 engineers:** yes, do v2 exactly as above.

**If you're bootstrapping with one engineer:** v2 is 6 months of full-time work before any revenue. Consider alternatives:

- Stay on v1 (individual Telme), grow to $50k ARR lifestyle business
- Build only **1-2 connectors** as paid add-ons ($3/mo per source) instead of building the whole cloud
- Partner with an existing aggregator and resell their integration

## What "minimum viable B2B" actually looks like

If we strip v2 down to bare bones:

- One connector (SharePoint only) — 5 weeks
- One-page web admin for onboarding — 1 week
- Per-seat billing via Stripe — 1 week
- Email-only auth (no SSO yet) — 1 week
- Total: **8 weeks to first paying B2B customer**

This is the version I'd ship if we're pre-seed and want to validate the B2B motion with minimal investment. Add more connectors after we have $20k MRR from this minimal version.

See `../architecture/01-saas-architecture.md` for the cloud backend design.
