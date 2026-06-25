# Telme for Business — Strategy Hub

This directory holds the research and plans for evolving **Telme** from a single-user macOS app into a B2B SaaS that connects to the document systems teams already use.

## Documents

| # | File | What's in it |
|---|---|---|
| 01 | [market-research/01-problem-landscape.md](./market-research/01-problem-landscape.md) | The "I can't find the file" pain across roles and industries. Why existing tools fail. Telme's wedge. |
| 02 | [market-research/02-competitive-analysis.md](./market-research/02-competitive-analysis.md) | Detailed teardown of: Spotlight, Raycast, Alfred, DEVONthink, LucidLink, Glean, Hebbia, Guru, Notion AI, Microsoft Copilot, Dropbox Dash, Mem. Pricing, gaps, what to copy, what to avoid. |
| 03 | [market-research/03-target-segments.md](./market-research/03-target-segments.md) | Five specific buyer segments with company size, budget, pain intensity, champion, blocker. Tiers ranked. |
| 04 | [services/01-service-landscape.md](./services/01-service-landscape.md) | 20+ data sources we could connect to, ranked by effort, value, and strategic fit. |
| 05 | [services/02-connector-design.md](./services/02-connector-design.md) | Universal connector architecture: auth, sync, ACL, embeddings, rate limits. Same shape works for every source. |
| 06 | [services/03-source-tier-plan.md](./services/03-source-tier-plan.md) | Roadmap: which 3 connectors to ship in v2, v3, v4. Effort vs revenue. |
| 07 | [architecture/01-saas-architecture.md](./architecture/01-saas-architecture.md) | What changes when we go from local-first single-user to multi-tenant SaaS. New components (tenant service, sync workers, search API). |
| 08 | [architecture/02-tenant-isolation.md](./architecture/02-tenant-isolation.md) | Security model: tenant boundaries, encryption, audit, compliance posture. |
| 09 | [commercial/01-pricing-strategy.md](./commercial/01-pricing-strategy.md) | Five SKUs: Personal Free, Pro $20, Team $8/u/mo, Business $15/u/mo, Enterprise. Why this ladder. |
| 10 | [commercial/02-gtm-strategy.md](./commercial/02-gtm-strategy.md) | How we reach buyers: content, community, partnerships, outbound. Phased over 12 months. |
| 11 | [commercial/03-funding-strategy.md](./commercial/03-funding-strategy.md) | Bootstrapping vs pre-seed vs seed. When to take money. Diligence materials. |
| 12 | [commercial/04-pitch-deck-outline.md](./commercial/04-pitch-deck-outline.md) | 10-slide investor deck. The story, the wedge, the plan. |

## The strategy in 60 seconds

**Today:** Telme is a beautiful, local-first semantic search for macOS files.
Market: individuals & power users. Revenue: $0–50k/yr (lifestyle).

**12-month plan:** Keep shipping Telme for individuals (revenue + brand proof),
then launch **Telme for Teams** — same beautiful UX, but connects to the
document systems companies already use (SharePoint, Google Drive, Notion, Slack).
Market: 50–500 person companies. Revenue: $50–500k ARR.

**36-month plan:** If Teams hits, raise a seed round, hire 3-4 people,
build out enterprise connectors (Confluence, Box, Salesforce, ServiceNow) +
a proper admin dashboard + SOC 2. Target: $500k-$2M ARR, real SaaS business.

**The wedge:** the only semantic file search that's *both* beautiful for
individuals *and* serious about enterprise data sources. Most competitors
pick one or the other.

## Read in order

If you have 5 minutes: read this file.
If you have 30 minutes: read `01-problem-landscape.md` + `03-target-segments.md`.
If you have 2 hours: read everything in `market-research/` and `services/`.
If you're an investor: jump to `commercial/04-pitch-deck-outline.md`.
