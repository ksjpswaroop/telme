# Go-to-Market Strategy

> How we reach the right buyers, in the right order, on the right channels. Phased over 12 months. Honest about what works and what doesn't.

## TL;DR

**Phase 1 (months 1-3): product-led growth from the Mac app.**
Individual users install, evangelize inside their companies.

**Phase 2 (months 4-6): content-led SEO + community.**
"Privacy-first semantic search for teams" — blog posts, conference talks, Twitter.

**Phase 3 (months 7-12): light outbound + partnerships.**
After we have product-market fit signals, hire part-time AE, do targeted ABM.

## The fundamental assumption

**Individual users → team adoption → company revenue.**

This is the same motion Slack, Notion, Linear, Figma all used. It works because:

1. Individual users experience the product daily (PMF signal at user level)
2. They evangelize internally ("hey, you guys should try this")
3. The first team signup is the company's decision-maker being convinced by their own employee
4. Switching costs grow with team size (data, integrations, workflow)

This is **opposite** to traditional enterprise sales ("call CIO, demo, sign contract"). We don't have the budget for that, and our product is too new to win on cold credibility alone.

## Phase 1: Product-led growth (months 1-3)

### What we do

**Free tier is the funnel.** 90% of our growth comes from individual installs.

- Mac App Store listing (free download, in-app upgrade to Pro)
- Gumroad / Paddle for direct purchases ($20 one-time)
- Self-serve team signup at app.telme.com/teams

**Viral loops built into the product:**

1. **Shared results** — search result has a "share this" link that creates a teammate account
2. **Team discovery** — "3 people at your company use Telme" notification (privacy-preserving: only triggers after threshold + explicit consent)
3. **Status bar** — shows team plan savings vs. individual plans
4. **OSS maintainer program** — free Pro for maintainers of >1k-star GitHub projects. Public list. Each maintainer = evangelist.

### Conversion targets

| Stage | Target |
|---|---|
| App Store impressions → installs | 5% |
| Installs → Free tier active (1+ search) | 60% |
| Free → Pro conversion | 5-10% (within 90 days) |
| Pro → Team plan (advocate to team) | 2-5% |
| Team → Business upgrade | 10-20% |

If we get **10,000 Free users**, we should hit:
- 600 active free
- 30-60 Pro upgrades = $600-1,200
- 1-3 Team customers × avg 10 seats × $8 × 12 mo = $960-2,880 ARR

Modest. Real product-market fit signal: Pro upgrades and Team signups in proportion.

### Budget

- **Apple Developer Program**: $99/yr (required for App Store + signing)
- **Gumroad/Paddle fees**: 5% of revenue
- **No marketing budget** — word of mouth + content only

## Phase 2: Content + SEO + community (months 4-6)

### Why content works for us

Search for "best local file search" or "semantic search privacy" — the results are dominated by Raycast reviews and DEVONthink tutorials. **Nobody owns the "privacy-first AI search" niche.**

### What we publish

**Blog posts (1/week, SEO-targeted):**

- "How to do semantic search across SharePoint without sending data to the cloud"
- "Local-first AI: why it matters for biotech R&D"
- "How to build a privacy-first knowledge base for your team"
- "DEVONthink vs Raycast vs Telme: which Mac search tool is right for you?"
- "Semantic search explained: a 5-minute primer for non-engineers"
- "How we kept Telme fully local-first (architecture deep-dive)"
- "Why knowledge workers lose 2 hours a day to search (and what to do about it)"

**Target keywords** (medium difficulty, high intent):

- "private semantic search"
- "local semantic search mac"
- "sharepoint semantic search"
- "ai file finder"
- "find files on mac faster"
- "semantic search for teams"
- "alternative to glean"
- "alternative to devonthink"

### Where we publish

- **Telme blog** (telme.com/blog) — primary home
- **Hacker News** — Show HN at launch; thoughtful comments on AI/privacy threads
- **r/rust, r/macapps, r/selfhosted** — share technical posts
- **Twitter/X** — short threads; build in public
- **Dev.to + Hashnode** — cross-post for SEO backlinks
- **YouTube** — 5-10 min screencasts (build in public, demos)
- **GitHub Discussions** — for technical audience

### Conference / community speaking

- **RustConf / EuroRust** — technical audience, "we built this in Rust"
- **MacDevOps** — operations audience
- **KubeCon / SRECon** — for the cloud backend story (later)
- **Gartner IAM Summit** — when we go enterprise (year 2)
- **BIO International** — when we go biotech (year 2)

### Content ROI tracking

Every blog post:
- Targets 2-3 keywords
- Has a CTA: "Try Telme Free"
- UTM-tracked so we know which posts drive installs
- Goal: 1,000 organic visits/month by month 12

## Phase 3: Light outbound + partnerships (months 7-12)

### When to start outbound

**Trigger:** 5+ Team customers without us doing any outbound. If teams are finding us through content + word of mouth, we have PMF.

**Don't start before PMF.** Outbound to non-PMF product wastes money and burns reputation.

### Who does the outbound

Hire a **part-time AE** at month 6 (~$50k/year, 20 hours/week).

Profile:
- Has sold to mid-tech CTOs before (e.g., ex-AE at Linear, Notion, Retool)
- Believes in product-led growth (doesn't want to "do demos all day")
- Comfortable with async-first culture

### Who they target

Tier 1 ICP list from `../market-research/03-target-segments.md`:
- 50-500 person tech companies
- Uses SharePoint or Google Workspace
- Has a "blocked AI tools" list
- Mid-tech, ideally Series B-D

### How they reach out

**LinkedIn ABM** ($500-1000/mo for Sales Navigator):
1. Build a target list (200 companies matching ICP)
2. Personalized connection requests referencing a relevant blog post
3. After connect: share a 2-min demo video + offer 30-day pilot
4. **Never** do cold email blasts — they don't work for our ICP

**Warm intros** (much higher conversion):
- Ask existing customers for intros to peers at similar companies
- Offer $500 referral credit for each successful introduction

**Pilot structure** (offered to qualified leads):
- 30-day free Business tier
- Weekly check-in with the founder for first month
- Migration assistance (connect SharePoint, etc.)
- No procurement needed (sign in 5 minutes)

### Target conversion

Outbound → Pilot: 10-15%
Pilot → Paid: 50-70%

If we send 200 LinkedIn messages/month at 10% conversion = 20 pilots, 50% close = 10 customers/month × 25 seats × $15 × 12 mo = **$45k ARR/mo added.**

## Partnerships (lightweight)

### Who to partner with

- **Managed Service Providers (MSPs)** — they serve 50-500 person companies and bundle tools. Offer them 20% revenue share for bringing Telme to their clients.
- **Compliance consultancies** — SOC 2 / ISO 27001 shops. They hear "we need better search" from clients daily. We become their recommended tool.
- **Atlassian / Microsoft solution partners** — already selling to Atlassian / Microsoft customers. They add Telme to their stack.
- **Newsletter writers** (Ben Thompson, Cloudflare blog, Pragmatic Engineer) — sponsor one post each quarter.

### What we DON'T do (year 1)

- **Cold email blasts** — too low conversion for our ICP
- **Google / Facebook ads** — wrong audience, too expensive
- **Reseller programs with big SIs** — Deloitte, Accenture — they won't touch a startup without SOC 2
- **Trade shows** (until SOC 2) — cost too high, wrong buyers

## Year 2 GTM (preview)

After $500k ARR + SOC 2 + 3 reference customers:

- **Hire a VP Sales** — first sales exec
- **Hire a content marketer** — dedicated to SEO + case studies
- **Apply to AppSource** — Microsoft marketplace listing (free, requires cert)
- **Sponsor 2 conferences** — Gartner IAM + one vertical (BIO for biotech, ABA Tech for legal)
- **Publish "State of Enterprise Search 2027"** annual report — generates inbound leads

## What we'd measure weekly

| Metric | Target month 3 | Target month 6 | Target month 12 |
|---|---|---|---|
| Free tier installs / week | 100 | 500 | 1,500 |
| Pro upgrades / month | 5 | 30 | 100 |
| Team customers (cumulative) | 2 | 20 | 100 |
| Business customers | 0 | 3 | 15 |
| MRR | $1k | $10k | $50k |
| ARR | $12k | $120k | $600k |
| Free → Pro conversion | 5% | 7% | 8% |

If we're below these by month 6, **stop and reassess the PMF**, not the marketing.

## The single most important growth lever

**A truly great free tier experience.** If someone installs Telme and has an "aha!" moment within 5 minutes, they'll tell 5 people. If they don't, no amount of marketing will save us.

Every other lever (content, SEO, partnerships, ABM) is multiplier on top of that core experience.

See `03-funding-strategy.md` for the capital plan.
