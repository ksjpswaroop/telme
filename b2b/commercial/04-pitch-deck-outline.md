# Pitch Deck Outline

> 10 slides. ~12 minutes. The story an investor needs to hear to write a check.

## Design principles

- **One idea per slide.** If you can't say the slide's point in one sentence, it's too busy.
- **Numbers on every slide.** Vague claims don't get funded.
- **Show, don't tell.** Screenshots > adjectives.
- **Acknowledge the elephant in the room.** Every investor will ask "How is this different from Glean?" Address it head-on.
- **End with a clear ask.** "$X for Y" is unambiguous.

## The deck

### Slide 1 — Title

```
[Telme logo]

The local-first semantic search platform for teams
that can't send their data to the cloud.

[Name, Title]        [Logo, YC, etc.]
```

**Speaker notes (30 sec):** "Hi, I'm [Name]. I built Telme, a desktop app that does semantic search over your files. We've sold X copies and now we're launching the team version that connects to SharePoint, Drive, and Notion — without ever sending files to a third party."

### Slide 2 — The problem

```
$8.25M/yr
wasted by a 200-person company
on file search

[3 bullet points:
 - 2.5 hours/day per knowledge worker (IDC)
 - Spans 10+ SaaS apps per employee
 - 130+ SaaS apps in average enterprise (Productiv)
 - "I know the file exists, I just can't find it"]
```

**Speaker notes (1 min):** "Knowledge workers spend 2.5 hours a day on information fragmentation. The pain is universal — every search vendor we've talked to confirms it. The reason it persists: existing search is keyword-only (Spotlight, Slack, Notion, SharePoint native), or cloud-only (Copilot, Glean, Dropbox Dash). Companies that can't send data to the cloud — biotech, healthcare, finance, defense — have nothing."

### Slide 3 — Why now

```
3 tailwinds

[Big numbers, one each]
1. LLMs make semantic search cheap
   $0.0001 per embedding
2. Local LLMs are good enough
   Ollama + nomic-embed-text runs on a MacBook in 50ms
3. Privacy regulation is tightening
   EU AI Act, US state laws, HIPAA enforcement
```

**Speaker notes (1 min):** "Three things changed in 2024. One, embedding costs cratered — less than a tenth of a cent per query. Two, open-source local LLMs hit the quality bar for embeddings. Three, regulators are catching up to AI: EU AI Act, state-level US laws, hospitals can't use cloud AI for patient data. Local-first has regulatory gravity it didn't have 2 years ago."

### Slide 4 — Solution

```
[3-panel mockup:]

[Mac app]   [Team index]   [Mobile future]

Local index       Shared index across      Same UX
on user's Mac      SharePoint + Drive      on iPhone
                   + Notion + Slack
                   + Confluence

"Beautiful for individuals. Serious about enterprise."
```

**Speaker notes (1 min):** "Telme is a tiny floating search bar. Press a hotkey, type what you remember, get the right file. Works on local files for individuals. Connects to SharePoint, Drive, Notion, Slack, Confluence for teams. Embeddings happen on-device or in your cloud — never ours."

### Slide 5 — Product demo

```
[Screen recording, 90 seconds]

0:00  Press ⌘⇧Space
0:03  Type "tauri global shortcut"
0:08  See tauri.md rank #1 (semantic match)
0:15  Open in editor
0:20  Switch to team index
0:25  Search across SharePoint + Slack
0:40  Show permission inheritance
      (Alice can't see Bob's HR docs)
0:55  Show mobile preview
```

**Speaker notes (1 min):** "Let me show you. This is the Mac app — I press cmd-shift-space, type what I remember about a file, and Telme surfaces it. This one is about Tauri global shortcuts — it found the right file even though the words don't match exactly. Now the team version: same UX, but I'm searching across SharePoint, Drive, Notion, and Slack. Results respect permissions — Alice can't see Bob's HR docs even though they're in the same index."

### Slide 6 — Market

```
TAM    $120B/year  every knowledge worker
SAM    $5B/year    privacy-conscious teams (50M ppl)
SOM    $3M/year    what we can capture in 36 mo
                 (5,000 teams × 25 seats × $15 × 12)

[Stack ranking showing why:
 - 80%+ of Fortune 500 use SharePoint
 - 100M+ knowledge workers in privacy-conscious verticals
 - $15/user/mo beats Glean ($24) and Copilot ($30)]
```

**Speaker notes (1 min):** "A billion knowledge workers use computers. Fifty million of them work at companies that can't send data to the cloud — finance, healthcare, biotech, defense, gov. We capture 5,000 teams in three years, average 25 seats, at $15 a seat a month — that's our $3 million SOM. Modest by VC standards, real by lifestyle standards."

### Slide 7 — Business model

```
5 tiers

Free / Personal    $0          Land
Pro               $20 once     Convert
Team              $8/u/mo      Expand
Business          $15/u/mo     Scale
Enterprise        $25+/u/mo    Maximize

Funnel math:
1,000 Free users
 → 60 active
 → 30-60 Pro    ($600-1,200)
 → 1-3 Teams    ($960-2,880 ARR)

LTV / CAC: 3.5x (target >3)
```

**Speaker notes (1 min):** "Five tiers, same shape Slack uses. Free users become Pro, Pro advocates bring in their teams, teams upgrade to Business when they need SSO or compliance. Our LTV-to-CAC is 3.5x after 12 months. The pricing ladder matches value to willingness-to-pay without burning trust — no enterprise-only feature gates."

### Slide 8 — Traction

```
[Numbers from current state, e.g.]

Today (bootstrap, month 6):
  • 12,000 Free downloads
  • 350 Pro upgrades    ($7,000 revenue)
  • 8 Team customers    (avg 12 seats, $1,152 MRR)
  • NPS: 52

Pipeline:
  • 3 enterprise pilots in progress
  • 1 design-partner LOI signed ($30k ACV)
```

**Speaker notes (1 min):** "We started six months ago, no funding. Today: 12,000 free downloads, 350 Pro upgrades at $20 each, eight teams on the $8 plan. NPS is 52, which is high for a search tool. We have three enterprise pilots and a signed LOI from [design-partner]. The conversion funnel works."

### Slide 9 — Competition

```
              Local-first   Semantic   <$15/u/mo   Privacy
Telme             ✓           ✓          ✓           ✓
Glean                          ✓                      (cloud)
Copilot (M365)                ✓                      (cloud)
DEVONthink         ✓                      ✓           ✓ (no team)
Raycast Pro        ✓           (basic)    ✓          ✓ (no sharepoint)
"Nothing owns this wedge"
```

**Speaker notes (1 min):** "We compete with Glean for the team-search market — but Glean is $24 a seat, cloud-only, and your data leaves. We compete with Copilot — but $30 a seat, Microsoft-locked, cloud-only. DEVONthink is local but single-user. Raycast Pro is local but no semantic and no SharePoint. Nobody owns our wedge: beautiful, local-first, semantic, team-aware, under $15 a seat."

### Slide 10 — Ask

```
We're raising $2M (seed)

Use of funds (18-month runway):
  • 2-3 engineers (Rust, frontend, integrations)
  • First sales hire (AE or DevRel)
  • SOC 2 Type 1 ($80k, 12 months)
  • AWS + observability ($60k)
  • Marketing: content + 2 conferences ($120k)
  • Legal + buffer ($480k)

Milestones we hit:
  ✓ 100+ paying teams
  ✓ SOC 2 Type 1 certified
  ✓ $1M+ ARR
  ✓ 3 enterprise reference customers

→ Series A: $10-20M at $1.5-3M ARR

[Contact info, GitHub, downloads]
```

**Speaker notes (1 min):** "We're raising $2 million on an $18 million cap, which gives us 18 months of runway. The plan: 100 paying teams, SOC 2 Type 1 certified, $1 million ARR, three enterprise references. That puts us in position for a $10-20 million Series A at $1.5-3 million ARR. If we hit those numbers, we're a real SaaS business. If we don't, we'll have learned a lot and either raise a smaller round or run as a profitable lifestyle business."

## Total: 10 slides, 12 minutes

Q&A: 8 minutes reserved.

## Common follow-up questions (have answers ready)

1. **"How is this different from Glean?"**
   - 60% cheaper ($15 vs $24)
   - Local-first (their data never leaves; Glean's always leaves)
   - Individual-first motion (their enterprise-first)
   - Native Mac UX (Glean is web-only)

2. **"What if Microsoft copies you?"**
   - They've had Copilot since 2023; it hasn't dented Glean
   - Microsoft is cloud-first; privacy is our wedge
   - Native Mac UX is hard for Microsoft (they don't ship good Mac apps)
   - Even if they did, our individual users still win

3. **"What's your moat?"**
   - Switching cost (data + integrations + habits)
   - Brand (privacy-first AI search)
   - Team network effects (shared indexes become more useful as team grows)
   - Technical (we have specific embedding + sync architecture)

4. **"How do you get to $10M ARR?"**
   - 1000 Business teams × 50 seats × $15 = $750k MRR = $9M ARR
   - Achievable in 30-36 months with the right sales motion + SOC 2

5. **"Why not just stay bootstrapped?"**
   - We could — $50-200k ARR is realistic
   - But the cloud backend + SOC 2 + multi-connector story requires 3+ engineers and $500k+ to execute
   - Capital accelerates the path to $1M+ ARR without changing the probability of success

6. **"What's the burn rate?"**
   - Currently $300/month (just me + AWS)
   - Post-funding: $80-120k/month burn (3 engineers + AE + AWS + SOC 2)
   - 18 months runway on $2M raise

7. **"Who are your competitors in [vertical X]?"**
   - Have a specific answer for biotech, legal, consulting, F500
   - See `../market-research/03-target-segments.md`

8. **"What's your retention?"**
   - Track NRR (Net Revenue Retention) — target >110% for SaaS
   - Personal: churn is a non-issue ($20 once)
   - Team: measure 12-month NRR; expect 80-90% logo retention, 110-130% NRR with seat expansion

## Anti-patterns to avoid in the deck

- ❌ "We're the Uber for X" — investors hate this
- ❌ Fake growth projections ("We'll hit $50M ARR in year 3") — do bottom-up math
- ❌ Hand-waving on competition — name names, acknowledge strengths
- ❌ Skipping the unit economics slide — investors will ask anyway
- ❌ Too much product, not enough business
- ❌ "We'll figure out monetization later" — investors hate this
- ❌ Huge TAM claim without SOM — "$120B TAM" with no $3M SOM is suspicious
- ❌ Slides full of text — investors skim

## What to send after the deck

1. **Data room** (Notion page with):
   - Current metrics dashboard (ChartMogul / Baremetrics)
   - Customer interviews (recorded calls, anonymized)
   - Codebase link (private GitHub)
   - Cohort retention chart
   - P&L / financial model
   - Cap table
   - Customer reference list

2. **1-pager** — single page summary of the deck for partners who don't have time to sit through 12 minutes

3. **2-week follow-up email** — "Hey, here's what we shipped this week" + updated metrics. Shows momentum.

## When to use this deck

- Raising pre-seed or seed ($300k-$5M)
- NOT for raising Series A (different deck — focus on traction, team, repeatable sales motion)
- NOT for accelerators (they want to see technical chops and ambition)
- For demo days, cut slides 6, 7, 10. Focus on problem, solution, traction.

See `../commercial/01-pricing-strategy.md`, `02-gtm-strategy.md`, and `03-funding-strategy.md` for the underlying details.
