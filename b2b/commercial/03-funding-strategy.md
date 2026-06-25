# Funding Strategy

> When to bootstrap vs. raise. How much. What investors look for. The honest math on what capital does and doesn't buy.

## TL;DR

**Bootstrap through $300k ARR.** Raise a $2-5M seed round at $300-500k ARR with 50+ paying teams. Raise Series A at $1.5-3M ARR with SOC 2 + 500+ teams.

## Why bootstrap first

**Bootstrapping means:**
- You keep 100% of equity (worth more than you'd think)
- You learn what customers actually pay for before investors pressure you
- You build only what makes money, not what looks good in a deck
- You can raise later from a position of strength

**Bootstrapping doesn't mean:**
- "Do everything yourself forever"
- "Refuse all investment on principle"
- "Don't spend money on growth"

**Bootstrap until traction is undeniable** (>$300k ARR, 50+ paying teams, NPS >40). Then raise to accelerate.

## Capital scenarios — what each tier buys

### Scenario A: Pure bootstrap (no outside capital)

**Resources:**
- Apple Developer Program ($99/yr)
- One Mac, one server ($50/mo)
- Domain + email ($50/yr)
- Stripe fees + Gumroad fees (5% of revenue)
- Ollama Cloud for Pro opt-in embeddings ($100-500/mo at scale)

**Total burn:** $100-500/mo

**What you can build:**
- Current Telme v1 (already shipped)
- Polish + Windows version
- Maybe 1-2 source connectors (SharePoint if you're brave)
- Sell via App Store + Gumroad

**What you CAN'T build:**
- A proper cloud backend with multi-tenant SaaS
- 5+ source connectors (each is 2-5 weeks of work)
- A web admin
- SOC 2 compliance
- Enterprise sales motion

**Realistic revenue:** $50-200k ARR after 18 months of solo work.

**Probability of success:** 60%. You'll have a profitable lifestyle business but probably not venture-scale.

### Scenario B: Pre-seed round ($300-500k, 6-12 months)

**When:** After Pro tier has traction (>200 paying users OR 5+ Team customers)

**Use of funds:**
- One full-time engineer for 6-12 months ($120-180k loaded)
- Apple Developer + AWS + Stripe ($30k)
- Part-time designer for marketing site ($20k)
- Part-time DevRel for content ($30k)
- Marketing (conferences, sponsorships): $30k
- Legal (incorporation, terms, basic MSA): $20k
- Buffer: $50k

**Valuation:** $3-5M post-money (uncapped SAFE or $5M cap on $500k)

**What you can build:**
- Cloud backend with multi-tenant SaaS
- 5 source connectors (SharePoint + Drive + Notion + Slack + Confluence)
- Web admin for onboarding
- Stripe billing
- First 10-20 paying teams

**Realistic revenue:** $300-700k ARR after 18 months.

**Probability of success:** 75%. You'll have real PMF signal or you'll know what doesn't work.

### Scenario C: Seed round ($2-5M, 18-24 months)

**When:** $300-500k ARR, 50+ paying teams, NPS >40, 1+ enterprise customer reference

**Use of funds:**
- 2-3 engineers (Rust backend + frontend + integrations) ($600-900k loaded)
- Founding engineer #2 (CTO if you're not technical)
- Part-time designer for marketing site + admin UI ($80k)
- First sales hire (AE or DevRel) ($150k loaded)
- AWS + observability stack ($60k)
- SOC 2 Type 1 prep + audit ($80k)
- Marketing (conferences, content, sponsorships): $120k
- Legal (incorporation, IP, SOC 2, MSA templates): $60k
- Buffer: $400k (12 months runway minimum)

**Valuation:** $15-25M post-money (priced round)

**Dilution:** 15-25% (founders keep 75-85%)

**What you can build:**
- All v2 connectors (SharePoint, Drive, Notion, Confluence, Slack, GitHub)
- DevOps tier connectors (Linear, Jira, GitLab)
- Web admin + SSO + audit logs
- SOC 2 Type 1 (12 months from start)
- Self-serve onboarding + Stripe billing
- First 100+ paying teams, 10+ paying enterprise

**Realistic revenue:** $1.5-3M ARR after 24 months.

**Probability of success:** 70%. Big upside if it works, real downside if it doesn't.

### Scenario D: Series A ($10-25M)

**When:** $1.5-3M ARR, SOC 2 Type 2, 500+ teams, clear enterprise pipeline, recognizable brand

**Use of funds:**
- 10-15 employees: more engineers, 2-3 AEs, customer success, design, marketing
- SOC 2 Type 2 + ISO 27001 + HIPAA
- Expand to EU (data residency)
- AppSource + Google Workspace Marketplace listings
- Brand campaigns, major conferences
- Acquisition channel development

**Valuation:** $50-150M post-money

**Dilution:** 20-30% in this round

**Realistic outcome:** $5-15M ARR after 24 months. Real SaaS business.

## What capital does NOT buy

**Important to internalize:**

1. **Capital doesn't create PMF.** If 50 customers don't love the product, 500 won't either.
2. **Capital doesn't fix product bugs.** More engineers = more bugs without good testing.
3. **Capital doesn't substitute for sales.** You can't ABM your way into a market that doesn't want you.
4. **Capital doesn't accelerate "product-market fit."** It can accelerate "growth once fit is found."

The **worst** outcome is raising too early, spending on growth before PMF, and running out of money with no learnings.

## What investors look for (at each stage)

### Pre-seed ($300-500k)

- Solo founder or small team
- Working prototype + paying customers (any number > 0)
- Clear problem + clear ICP
- Founder background / domain expertise
- Clear next milestone the money unlocks

**Sources:** angels, syndicates (e.g., AngelList), early-stage funds that write $250-500k checks.

### Seed ($2-5M)

- $300k+ ARR with strong growth (>20% MoM)
- Strong founder-market fit
- Reference customers who'll give quotes
- Initial unit economics working (LTV/CAC > 3)
- Plan that scales with capital

**Sources:** seed funds (Precursor, Afore, Lerer Hippeau, South Park Commons, etc.), strategic angels, prior investors pro-rata.

### Series A ($10-25M)

- $1.5M+ ARR
- 100%+ YoY growth
- NPS >40
- Clear repeatable sales motion
- Multiple reference customers at the target ICP
- Defensible product (technical moat, network effects, or switching costs)

**Sources:** Series A funds (Andreessen Horowitz, Accel, Sequoia, Lightspeed), growth funds.

## The pitch deck essentials

See `04-pitch-deck-outline.md` for the full structure. Summary:

1. **The problem**: "I know the file exists but can't remember the name" — quantified in $/year
2. **The wedge**: Local-first semantic search, beautiful for individuals, scales to enterprise data sources
3. **Why now**: LLMs make semantic search cheap; local LLMs are good enough; privacy regulation tightening; Microsoft 365 penetration
4. **Traction**: Whatever numbers you have (Free installs, Pro upgrades, Team customers)
5. **Market**: $500k-$3M ARR SOM, $120B TAM
6. **Business model**: 5-tier ladder ($0 / $20 / $8 / $15 / $25+)
7. **Competition**: Glean ($24/u/mo cloud), Copilot ($30/u/mo cloud), DEVONthink (single-user local); nobody owns our wedge
8. **Why us**: Privacy-first + beautiful UX + reasonable price
9. **Traction + Plan**: Where we are, where we're going, what the money unlocks
10. **Ask**: $X for Y months runway to $Z ARR

## Cap table considerations

**What you'll have after pre-seed (~$3-5M post):**

- Founders: 90-95%
- Option pool: 5-10% (set aside for future hires)
- Investors: 5-10%

**What you'll have after seed (~$15-25M post):**

- Founders: 70-80%
- Employees (option pool): 10-15%
- Investors: 15-25%
- Advisors: 0.5-1% each (with vesting)

**Don't give up board control pre-Series A.** Take observation rights. Take pro-rata. Don't take preferred without a fight.

## When NOT to raise

- **"I'm bored and want to work on something harder."** — Find a cofounder or take a sabbatical. Don't raise money to make yourself busy.
- **"I want to feel legit."** — Building real revenue is more legit than a term sheet.
- **"A VC I know will give me money."** — Term sheets are easy. Spending the money well is hard.
- **"Everyone else is raising."** — Survivorship bias. You don't see the dead startups.

## When to DEFINITELY raise

- **You have PMF** (people pay, use weekly, refer others) and can't grow without more engineers
- **A real enterprise customer** is waiting for a feature only money can build (SOC 2, SAML, etc.)
- **A competitor** is raising and you're in the same space — speed matters
- **You're burned out** and need a cofounder or a few months of runway to keep going

## The honest timeline

If everything goes well:
- **Months 0-6**: Bootstrap, ship v1 polish, get 100 Pro users, $2-5k MRR
- **Months 6-12**: Raise pre-seed $300-500k, hire 1 engineer, build v2 connectors, 30+ teams, $20-30k MRR
- **Months 12-24**: Raise seed $2-5M, hire 3-4 engineers + first AE, ship v3 connectors + SOC 2, 100+ teams, $100-300k MRR
- **Months 24-36**: Raise Series A, scale to 500+ teams, $1.5M+ ARR

If everything goes poorly:
- **Months 0-12**: Bootstrap, build slowly, never find PMF
- **Months 12**: Either pivot, shut down, or keep lifestyle business

Both outcomes are realistic. The capital doesn't change the probability much — it just changes the magnitude if it works.

See `04-pitch-deck-outline.md` for the actual deck.
