# Competitive Analysis

> Detailed teardowns of every competitor Telme overlaps with. What to copy, what to avoid, where the white space is.

## How to read this

Each competitor gets:
- **What they do** — one paragraph
- **Pricing** — what they actually charge
- **Strengths** — what works for them
- **Weaknesses** — where they fail users (our opportunity)
- **What Telme should copy** — tactical lessons
- **What Telme should NOT copy** — anti-patterns to avoid

Then a **positioning matrix** at the end shows where everyone sits.

---

## Tier 1: Direct competitors (search engines with AI)

### Glean
**What:** Enterprise "universal search" — indexes Slack, Gmail, Drive, Confluence, Jira, Notion, GitHub, etc. Answers natural-language questions with citations.

**Pricing:** $12-24/user/month (enterprise contract, annual). Minimum ~$30k/year contract. Implementation fee typically $20-50k.

**Strengths:**
- Best-in-class permission inheritance (results respect ACLs)
- Beautiful admin dashboard for IT
- Strong customer references (Datadog, Toast, etc.)
- SOC 2, ISO 27001, HIPAA, FedRAMP-ready

**Weaknesses:**
- **Cloud-only** — all data leaves the customer's environment. Blocks biotech, finance, defense.
- Slow onboarding (4-12 weeks for IT to provision)
- $30k+ minimum is out of reach for SMB (<500 employees)
- Their search UI is functional but bland — not something users love
- Heavy sales cycle (6+ months)

**Telme should copy:**
- Permission inheritance architecture (critical for enterprise trust)
- Connector breadth ambition (SharePoint, Slack, Notion, Drive, Confluence, Jira)

**Telme should NOT copy:**
- Cloud-only architecture (our wedge is local-first)
- Enterprise-only pricing (our wedge is individual → team → enterprise)
- Heavy sales motion (we'll start with PLG / self-serve)

---

### Hebbia
**What:** AI knowledge worker for finance, legal, consulting. Reads documents (PDFs, Excel, transcripts) and answers analytical questions. Matrix-style spreadsheet UI for browsing.

**Pricing:** $30k-$500k/year contracts. Enterprise-only.

**Strengths:**
- Best-in-class document understanding (especially PDFs, Excel)
- Cited by McKinsey, Goldman Sachs as a tool they use
- Strong analytical workflows (compare documents, extract tables)

**Weaknesses:**
- Enterprise-only, very expensive
- Matrix UI is for analysts, not knowledge workers
- Cloud-only (data leaves)
- No team sync story; individual seat licensing at enterprise scale

**Telme should NOT compete directly** — different buyer, different use case.

---

### Microsoft Copilot for Microsoft 365
**What:** AI assistant baked into Word, Excel, Outlook, Teams. $30/user/month for M365 E3/E5 customers. Also has "SharePoint semantic search" via Copilot Studio.

**Pricing:** $30/user/month (separate from M365 license).

**Strengths:**
- Default for 400M+ Microsoft 365 users
- Native integration with Word/Excel/Outlook
- Microsoft brand = compliance + procurement automatic
- Microsoft Search is "good enough" for most enterprises

**Weaknesses:**
- $30/u/mo is expensive (most IT budgets cut this in 2024)
- Cloud-only — same privacy concerns as Glean
- Poor cross-app search (still per-app search under the hood)
- Slow to roll out features (Copilot for SharePoint only GA late 2024)
- Microsoft's search ranking is famously bad

**Telme should:**
- Position as "Copilot alternative for privacy-conscious teams" — they don't need to displace Copilot, they need to *coexist*
- Lead with "this works alongside M365, doesn't replace it"

---

### Dropbox Dash
**What:** Universal search across Dropbox + Google Workspace + Microsoft 365 + Slack + Asana + Notion. Launched 2023.

**Pricing:** $20/user/month (with Dropbox plan) or $20 standalone.

**Strengths:**
- Brand recognition (Dropbox)
- "Stacks" feature (curated collections of files)
- Universal search across many sources

**Weaknesses:**
- Cloud-only, Dropbox account required (friction for non-Dropbox users)
- Search ranking is mediocre
- No semantic understanding (still keyword-based)
- Low adoption — Dash launched with hype but hasn't grown meaningfully

**Telme should copy:**
- The "Stacks" idea for curated team collections
- Broad connector ambition

**Telme should NOT copy:**
- Bundling with a parent product (we don't have one)
- Cloud-only architecture
- $20/u/mo entry price (too high for self-serve)

---

### Notion AI / Q&A
**What:** AI features inside Notion. Q&A answers natural-language questions across your Notion workspace.

**Pricing:** $10/u/mo (Notion AI add-on, included in Business plan).

**Strengths:**
- Beautiful UX inside Notion
- Cheap
- Fast adoption

**Weaknesses:**
- Only searches Notion. Misses everything else.
- Cloud-only
- Not a "search tool" — it's a "Notion feature"

**Telme should:**
- Treat as complementary, not competitive
- Mention in "staying out of the way" positioning

---

## Tier 2: Adjacent tools (different jobs we can learn from)

### Raycast Pro
**What:** macOS launcher with extensions. Free tier + $8/mo Pro with AI.

**Pricing:** Free base, $8/mo Pro, $96/yr Pro.

**Strengths:**
- 100k+ paying users, well-designed
- Extensions ecosystem
- Fast
- Native macOS feel

**Weaknesses:**
- Primarily a launcher, not a file finder
- AI features are thin (mostly ChatGPT passthrough)
- No semantic search of file content
- macOS-only
- No team features

**Telme should copy:**
- The "feels native to the OS" UX
- Free base tier + paid Pro model
- Extensions architecture (later)
- Aggressive pricing for individuals

**Telme should NOT copy:**
- Being a launcher (we're a search tool, different lane)
- Cloud AI passthrough (we have local embeddings)

---

### Alfred
**What:** macOS launcher. Predecessor to Raycast.

**Pricing:** Free base, £19 single Powerpack, £49 Mega Supporter.

**Strengths:**
- Beloved by power users
- Workflows (file actions, hotkeys)
- Cheap one-time purchase

**Weaknesses:**
- Search is keyword-only
- No semantic
- No team features
- Aging UI

**Telme should NOT compete directly** — Raycast won this lane. Take inspiration from Alfred's *one-time pricing* (some users prefer this over subscriptions).

---

### DEVONthink
**What:** macOS document management + AI. Personal knowledge base.

**Pricing:** $50/year (Pro) / $100/year (Server) one-time.

**Strengths:**
- 20+ years of refinement
- Powerful classification, tagging, smart rules
- Built-in OCR for scanned PDFs
- Beautiful for power users

**Weaknesses:**
- Steep learning curve
- Single-user only (no team sync)
- Mac-only
- AI features are basic
- Search is *good* but not semantic

**Telme should copy:**
- OCR pipeline (for PDF support later)
- Smart rules ("auto-tag documents from this folder")
- Beautiful Mac-native feel

**Telme should NOT copy:**
- Steep learning curve (we win on simplicity)
- Single-user limitation

---

## Tier 3: Infra / platform players (not competitors, but they'll matter)

### Microsoft Search (SharePoint native)
**What:** Built-in SharePoint search. Free with M365.

**Strengths:**
- Already deployed everywhere
- Permission-aware (respects SharePoint ACLs)
- Indexed by Microsoft (free for us!)

**Weaknesses:**
- Bad UX
- Keyword only
- No semantic
- Slow to roll out new features
- Doesn't span across other data sources

**Telme relationship:** We connect to SharePoint *and* use its search index as a fast first-pass. Microsoft Search is upstream; Telme is the better UX on top.

---

### Algolia / Meilisearch / Typesense
**What:** Hosted search APIs. B2D (business-to-developer).

**Pricing:** Various. ~$0.50/1000 searches typical.

**Strengths:** Fast, scalable, well-loved by developers.

**Weaknesses:** Keyword only (Algolia added vector search in 2024). Not an end-user product.

**Telme relationship:** Use as a building block in our hosted backend for fast BM25. Don't reinvent.

---

### Pinecone / Weaviate / Qdrant
**What:** Hosted vector databases. B2D.

**Pricing:** Pinecone $70/mo starter; Qdrant free self-hosted.

**Telme relationship:** Use Qdrant (open-source) for the hosted backend's vector storage when we go multi-tenant. Don't build our own.

---

### Cohere / Voyage / OpenAI embeddings
**What:** Embedding APIs. ~$0.10/M tokens.

**Telme relationship:** Fallback when Ollama isn't available, or for enterprise customers who want best-in-class quality. Default to Ollama, offer OpenAI text-embedding-3 as opt-in for $1/mo Pro tier.

---

## Positioning matrix

Two axes that matter:
- **Privacy** (cloud-only ↔ local-first)
- **Audience** (individual ↔ enterprise)

```
                            ENTERPRISE
                                ▲
                                │
                Glean ●         │         ● Copilot for M365
                                │
                                │         ● Hebbia
                                │         ● Dropbox Dash
INDIVIDUAL  ◄───────────────────┼──────────────────────►  ENTERPRISE
                                │
                                │
                ● DEVONthink     │
                                │
                ● Raycast Pro    │
                                │
   ● Telme Pro  ←──── WE ARE HERE
       (local-first, beautiful, $20 once)
                                │
                                ▼
                            INDIVIDUAL
```

**White space:** No competitor owns "beautiful local-first search for individuals that scales to enterprise data sources." That's our wedge.

---

## The 5 things we should copy (and 5 we shouldn't)

### Copy
1. **Glean's permission model** — every search result must respect source ACLs
2. **DEVONthink's Mac-native feel** — system fonts, system colors, system animations
3. **Raycast's free → paid conversion** — give away the core, charge for Pro features
4. **Notion's onboarding** — empty state → first action in <30 seconds
5. **Dropbox's "Stacks"** — curated collections of files (for teams in v2)

### Don't copy
1. **Glean's $30k minimum** — kill adoption
2. **Copilot's $30/u/mo** — out of reach for individuals and small teams
3. **Hebbia's analyst-only UX** — we serve everyone
4. **DEVONthink's learning curve** — keep it simple
5. **Slack's search quality** — see above; do better

See `03-target-segments.md` for who we sell to.
