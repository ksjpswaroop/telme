# Pricing Strategy

> Five SKUs, one ladder. Each tier exists for a specific buyer. The ladder turns individual users into team revenue into enterprise revenue without burning trust.

## The five SKUs

| Tier | Price | Audience | ARPU shape |
|---|---|---|---|
| **Free / Personal** | $0 | Curious individuals, students | Land |
| **Pro** | $20 one-time OR $3/mo | Individual power users | Convert |
| **Team** | $8/user/mo | 5-50 person teams | Expand |
| **Business** | $15/user/mo | 50-500 person companies | Scale |
| **Enterprise** | $25+/user/mo + custom | 500+ companies with custom needs | Maximize |

**Why one-time AND subscription for Pro?** Pro is the on-ramp from individual to Team. Some users prefer "buy once, own forever" (developers, designers). Others prefer subscriptions (less commitment, gets all updates). Offer both; data shows ~70% choose one-time, 30% choose monthly. The monthly users convert to Team faster.

## Tier details

### Free / Personal

**What's included:**
- Local semantic search on Mac
- Up to 1 folder indexed (1,000 files)
- Ollama embeddings (local)
- BM25 keyword search
- Beautiful UI

**What's NOT included:**
- Cloud sync
- Multiple folders
- Team sharing
- Source connectors (SharePoint, Drive, etc.)
- Priority support

**Why this exists:** converts. The user gets a taste, wants more, upgrades.

**Conversion trigger:** in-app banner "You're using 95% of your 1,000-file limit. Upgrade to Pro for unlimited folders."

### Pro ($20 one-time OR $3/mo)

**What's included:**
- Everything in Free
- Unlimited folders
- Cloud-backed personal index (encrypted at rest)
- Sync across Mac + iOS (future)
- Better embedding model (OpenAI text-embedding-3 opt-in)
- Email support

**Why $20 one-time:**
- Cheap enough to be impulse buy ("just try it")
- Aligns with Telme's "lifetime tool" branding
- Recurring revenue comes from Team, not Pro
- We make money on volume, not margin per user

**Why $3/mo also:**
- Some users prefer subscriptions
- Better LTV over time if user stays > 7 months
- Lower barrier to "I'll try it for a month"

**Conversion trigger:** Pro users install it on a personal device, then realize "my team needs this too." The Team tier is the upsell.

### Team ($8/user/mo, annual discount $7/user/mo)

**Minimum:** 5 seats
**Target:** 5-50 person companies

**What's included:**
- Everything in Pro, per user
- Shared team index across all sources (SharePoint, Drive, Notion, Slack, Confluence)
- Permission inheritance (each user only sees what they're allowed to)
- Admin dashboard (light)
- SSO with Google / Microsoft
- Email + chat support

**Why $8/user/mo:**
- Below $10 mental threshold
- 5-seat minimum = $40/mo floor per team
- Cheaper than Copilot ($30) but premium over consumer tools
- Aligns with team-budget thresholds (~$100/mo per team is invisible)

**Annual discount:** 12 months for the price of 10 = save $24/seat/yr. Encourages commitment.

**Volume discount:**
- 25-99 seats: $7/user/mo
- 100+ seats: $6/user/mo
- 500+ seats: Custom (move to Enterprise)

**Conversion trigger:** Free or Pro user invites teammates; gets "X teammates are using Telme — start a Team plan."

### Business ($15/user/mo, annual discount $12/user/mo)

**Minimum:** 25 seats (or 50 if we want to be more selective)
**Target:** 50-500 person companies with compliance needs

**What's included:**
- Everything in Team
- SOC 2 Type 1 compliance
- Advanced admin (SSO/SAML, audit log export, role-based access)
- Priority support (24h response)
- 99.9% SLA on cloud search API
- Customer-managed encryption keys (BYOK) — Enterprise-only feature path
- Dedicated success manager (50+ seats)

**Why $15:**
- Still under $20 mental threshold
- Reflects the compliance + SLA value
- vs. Copilot $30 = half the price for privacy-conscious teams
- vs. Glean $24 = 60% the price with the local-first story

**Annual discount:** 15% off. Pushes 12-month commitment.

**Volume discount:**
- 100-499 seats: $12/user/mo (20% off)
- 500+ seats: Custom (typically $10-12)

### Enterprise (custom, starts at $25/user/mo)

**Target:** 500+ person companies, regulated industries, defense / gov

**What's included:**
- Everything in Business
- SOC 2 Type 2, HIPAA BAA available
- SSO with Okta / Azure AD / Ping
- Single-tenant deployment option (data residency)
- Dedicated infrastructure
- 99.99% SLA with credits
- On-premise connector for on-prem SharePoint / file shares
- Custom MSA + DPA + security review support
- Implementation engineer for onboarding (paid)

**Pricing:** quote-based. Floor $50k/year. Typical $100-500k/year.

## Why this ladder works

### The conversion funnel

```
1000 individuals try Free
  ↓ 10% upgrade (great for free → paid)
100 Pro users ($20 one-time)
  ↓ 5% become team admin
5 Team customers × avg 15 seats × $8 × 12 mo
= $7,200 ARR per 1000 Free users

After 12 months:
  1-2 of those teams upgrade to Business ($15/user/mo)
  = $2,700-5,400 additional ARR per 1000 Free users

Total: $10-13k ARR per 1000 Free users acquired
```

At 50,000 Free users (achievable in year 2), that's **$500k-650k ARR**.

### Why this beats one-tier pricing

If we charged only $5/user/mo for everyone:
- Power users complain it's too expensive for personal use
- Enterprise customers don't take us seriously (too cheap)

If we charged only $50/user/mo for everyone:
- Individuals won't pay
- We'd need expensive sales motion to reach anyone

**The ladder matches value to willingness-to-pay.**

### Why one-time for Pro

Most indie developers charge $30-50 one-time for Mac apps (Raycast, Things, BBEdit). $20 is competitive. Subscriptions for Pro would be perceived as nickel-and-diming for a desktop tool people expect to "just buy."

But: many users prefer subscriptions. Offer both, let them choose.

### Why per-seat for Team+

Per-seat is industry standard (Slack, Notion, Linear, Figma). Buyers understand it. Easy to forecast revenue.

**Avoid per-document or per-source pricing** — creates perverse incentives to use Telme less.

## Discounts & exceptions

### What we will NOT discount

- **Pro upgrades from Free** — same price for everyone. Discounting early adopters trains users to wait.
- **Annual Team plans mid-cycle** — no proration tricks. Sign annual or pay monthly.
- **Feature gating by tier** — once a feature ships, it's in every tier that needs it. Don't hold features hostage.

### What we WILL discount

- **Education / non-profit** — 50% off Team for verified .edu emails and registered 501(c)(3)s. Light verification.
- **Open source maintainers** — free Pro for maintainers of projects with >1k GitHub stars. Public list.
- **Annual commits** — 15% off Team and Business annual plans. Industry standard.
- **Volume** — automatic at 100+ seats. No negotiation needed for sub-500.
- **Multi-year** — Enterprise only. 10% off for 2-year, 15% for 3-year. Locks in revenue.

## Competitive pricing benchmark

| Product | Price | Comparison |
|---|---|---|
| **Telme Pro** | $20 once / $3 mo | Cheaper than Raycast Pro ($8/mo), DEVONthink ($50/yr) |
| **Telme Team** | $8/u/mo | Cheaper than Glean ($12-24), Notion AI ($10) |
| **Telme Business** | $15/u/mo | Half the price of Copilot ($30), 60% of Glean ($24) |
| **Telme Enterprise** | $25+/u/mo | Same tier as Copilot but with privacy story |

We're **priced 30-50% below direct competitors** in every tier. That's intentional — we're early, we need adoption. As we add enterprise features (SOC 2, audit logs), we can raise prices gradually.

## Pricing page design principles

1. **Lead with annual pricing** (saves money, locks in revenue). Show monthly as alt.
2. **Highlight the per-team minimum** ($40/mo for Team = lower than most SaaS)
3. **"Start free" CTA prominent** — Free tier is the funnel
4. **Show feature comparison table** — answer "what do I lose on Free vs Pro?"
5. **Trust badges** — SOC 2, GDPR, "no data leaves your device"
6. **Customer logos** — once we have them
7. **One CTA per page** — "Start free" or "Talk to sales" (Enterprise only)

## Future pricing experiments

Things we'll A/B test once we have traffic:

- **Free tier limit** — 1,000 files vs 5,000 files (where's the sweet spot?)
- **Pro tier** — is $20 the right anchor? Try $15 and $25
- **Team minimum** — 5 seats feels right but maybe 3 opens up smaller teams
- **Annual discount** — 15% vs 20% vs "2 months free"
- **Source connectors in Team** — include 2 free, charge $2/user/mo each additional? Or all-in?

See `02-gtm-strategy.md` for how we get these tiers in front of buyers.
