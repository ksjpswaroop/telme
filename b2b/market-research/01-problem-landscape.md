# The "I Can't Find The File" Problem Landscape

> How big is the problem we're solving, who has it worst, and why do existing tools fail them?

## The macro problem

Knowledge workers spend **8.8% of their workday** (≈ 1 hour/day) searching for information,
per McKinsey's 2012 *Social Economy* study — widely cited but rarely updated. IDC's
2023 *Knowledge Worker Productivity* report pegs it at **2.5 hours/day** for "information
fragmentation" tasks (searching + re-finding).

For a 200-person company billing $150k average loaded cost, that's:
- 200 × 2.5h/day × 220 workdays × $75/hr ≈ **$8.25M/year of search labor**
- Even a 5% reduction = **$412k/year saved per mid-size firm**

This is real, measurable, and growing as data sprawls across more SaaS apps every year
(avg enterprise uses 130+ SaaS apps, per Productiv).

## Who feels it worst (ranked by pain intensity)

### 1. Senior engineers in mid-stage startups (50-500 ppl)
**Pain: extreme.** Their company grew from "everything in Slack + Notion" to "10 systems,
nobody knows where anything is." They regularly spend 20-30 min hunting for the
architecture decision record, the runbook, the postmortem. **Tools they try:**
- Slack search (misses attachments after 90 days, search ranking is poor)
- Notion search (slow, requires login, misses cross-app context)
- macOS Spotlight (no semantic, no content search beyond filename)
- grep / ripgrep (requires knowing the path)

**Why current tools fail:**
- Slack/Notion search is *intra-app*. They need *cross-app*.
- Spotlight/grep are *local*. They need *remote-aware*.
- AI tools (Copilot, Notion AI) require moving data to a third party — blocked by security review at most companies >100 people.

### 2. R&D scientists in biotech/pharma
**Pain: extreme + IP-sensitive.** Lab notebooks in SharePoint or specialized ELN
(Electronic Lab Notebook) systems. Compliance auditors require detailed search logs.
Years of research trapped in documents that no current search tool can find by
*concept*. "Show me all the experiments where we tried inhibiting protein X."
**Tools they try:**
- SharePoint search (key-phrase only; ranking is bad; no semantic)
- Specialized ELN search (per-vendor, $50k+/yr enterprise contract)
- EndNote / Reference Manager (academic citations only)

**Why current tools fail:**
- Semantic search across PDFs + lab notes is genuinely unsolved
- They can't send proprietary research to OpenAI / Copilot
- **Local-first semantic search with SharePoint sync is the perfect fit**

### 3. Management consultants
**Pain: high + multi-client.** Partners who bill $500/hr lose credibility if they
"can't find that slide deck from the Acme engagement last quarter." Documents
scattered across SharePoint, OneDrive, Dropbox, email. NDA constraints on which
data can leave which environment.
**Tools they try:**
- Windows Search (terrible)
- SharePoint search (slow, requires browser)
- Slack/Teams (most consultants underuse)
- Personal folder structures ("mine alone")

**Why current tools fail:**
- Search has to span *all* their silos
- Privacy + NDA rules out most AI tools
- They will pay $30/seat/month for a tool that genuinely solves this

### 4. Architects, lawyers, accountants
**Pain: high + regulated.** Decades of project files. Strict retention rules.
Need to find "all contracts signed with vendor X in the last 5 years" — a
*query*, not a *keyword*.
**Tools they try:**
- Windows Search + grep
- SharePoint advanced search (requires query syntax training)
- DMS vendors (iManage, NetDocuments, OpenText) — $100k+/yr enterprise contracts
- **They will pay for better search if it doesn't compromise compliance**

**Why current tools fail:**
- Existing DMS search is bad *and* expensive
- Cloud AI isn't compliant
- Telme's local-first + semantic story is exactly their fit

### 5. Journalists, academics, writers
**Pain: medium + intermittent.** They write a lot, save in many places, but
search need is episodic ("where's that interview I did last March?").
**Tools they try:**
- Spotlight / Windows Search
- DEVONthink ($50/yr) — great for personal archives, Mac-only, single user
- Notion / Obsidian — only if they're disciplined
- They have lower budgets but high word-of-mouth virality

**Why current tools fail:**
- DEVONthink is the closest competitor — but doesn't sync to SharePoint
- Personal tools can't help with team research

## Why existing solutions fail (cross-cutting analysis)

| Solution | What it does well | Where it falls down |
|---|---|---|
| **macOS Spotlight** | Fast, system-wide | Keyword only, no semantic, no snippet context |
| **Windows Search** | Indexes file content | Index constantly breaks; ranking is poor |
| **DEVONthink** | Power-user document management | Single-user, no team sync, no SharePoint |
| **Slack / Teams search** | Recent messages | Misses older files; poor ranking; no semantic |
| **SharePoint native search** | Already deployed, free | Bad UX; slow; keyword only; no AI |
| **Microsoft Copilot for M365** | Real AI, integrated | $30/u/mo; requires cloud trust; per-query cost |
| **Glean** | Best enterprise search | $12-24/u/mo; cloud-only; 6-month sales cycle |
| **Hebbia** | Best for finance/legal | $$$ enterprise; analyst-driven, not self-serve |
| **Dropbox Dash** | Universal search | $20/u/mo; cloud-only; weak on semantic |
| **Notion AI / Q&A** | Within Notion | Only searches Notion; misses external docs |
| **Raycast AI** | Fast launcher + AI | Local files only; no team sync |

## The "Telme wedge"

What nobody else does:
1. **Beautiful for individuals** (not just an enterprise dashboard)
2. **Connects to real work systems** (SharePoint, Google Drive, Notion)
3. **Semantic by default** (Ollama locally + cloud option)
4. **Local-first** (data stays on device unless user opts in)
5. **$20 one-time for individuals**, $8-15/user/mo for teams — accessible pricing

The combination is genuinely rare. Competitors pick one side or the other:
- **Personal-feeling + team sync**: nobody (Raycast Pro is closest but $8/mo for AI features, no semantic, no SharePoint)
- **Semantic + local-first**: nobody in B2B
- **Enterprise + semantic**: Glean, Hebbia, Microsoft Copilot — $12-30/u/mo, cloud-only

## The trend tailwinds (why now)

1. **LLMs make semantic search cheap.** A 768-dim embedding costs $0.0001 to compute.
2. **Local LLMs are good enough.** Ollama + `nomic-embed-text` runs on a MacBook in <50ms.
3. **Privacy regulation is tightening.** EU AI Act (2024), US state AI laws, HIPAA enforcement. **Local-first has regulatory gravity.**
4. **Hybrid work made file-finding worse.** Documents spread across personal laptop + company SharePoint + personal cloud. A single search surface is more valuable than ever.
5. **Microsoft Teams + SharePoint penetration.** 80%+ of Fortune 500 use SharePoint. Any B2B search that doesn't connect to SharePoint is leaving 80% of the market on the table.

## Sizing the wedge (TAM / SAM / SOM)

**TAM** (Total Addressable Market): every knowledge worker who uses a computer.
- 1B+ knowledge workers globally
- Avg $10/mo ARPU = $120B/year

**SAM** (Serviceable Addressable Market): teams that use at least one of our connectors + care about privacy.
- 50M knowledge workers in privacy-conscious companies (finance, legal, healthcare, gov, defense, biotech)
- $100M – $5B depending on ARPU

**SOM** (Serviceable Obtainable Market): what we can realistically capture in 3 years.
- 5,000 paying teams × avg 25 seats × $15/u/mo = **$2.25M ARR**
- 50,000 individual Pro licenses × $20 one-time = **$1M one-time**
- **SOM ≈ $3M/year within 36 months**

That's a real business, not a unicorn — but it's a comfortable lifestyle-to-small-SaaS outcome.

## What this means for the product

1. **The Mac app is the wedge.** Sell beautiful local-first search to individuals first. They become evangelists inside their companies.
2. **Add team features when there's pull.** Don't build enterprise connectors speculatively — wait until individuals ask "can my whole team use this?"
3. **Lead with SharePoint.** 80% of our eventual buyers use it. Build that connector first when we go B2B.
4. **Keep "no data leaves your device" as the brand.** Privacy is the differentiator that enterprise Glean / Copilot can't match.

See `02-competitive-analysis.md` for the detailed teardown of each competitor and what we should copy vs. avoid.
