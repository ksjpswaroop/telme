# Data Source Service Landscape

> Every system we could connect to, ranked. Each entry: what it is, who uses it, our fit, auth model, real-time sync model, embedding strategy, and rough effort.

## How to read this

Each service gets:
- **What it is** — one paragraph
- **Used by** — who actually uses this
- **Why Telme should connect** — our specific value
- **Auth model** — OAuth? API key? Service account?
- **Sync model** — webhook? polling? delta query?
- **Permissions** — ACL complexity
- **Embedding strategy** — text extraction challenges
- **Effort** — T-shirt size (S/M/L/XL)
- **Priority** — P0/P1/P2

We rank by **effort × strategic value**, not just popularity.

---

## File storage & docs (Tier 1: ship first)

### Microsoft SharePoint / OneDrive
- **What:** Microsoft's enterprise file storage. 80%+ of Fortune 500. Documents, lists, libraries. Often combined with Teams.
- **Used by:** Every Microsoft 365 customer. The default for enterprise.
- **Why Telme should connect:** Largest B2B market. Compliance teams will block Copilot but allow Telme. SharePoint has built-in semantic search but bad UX — we improve UX without sending data to Microsoft AI.
- **Auth model:** Microsoft Graph API, OAuth 2.0 + MSAL. Tenant + user auth. On-behalf-of flow for enterprise.
- **Sync model:** Delta queries (`/drive/root/delta`) track changes. Initial scan via `/sites/{id}/drives/{id}/items`. Webhooks available but not needed if delta polling is fast enough.
- **Permissions:** Graph returns results scoped to the calling user's permissions (clean ACL inheritance). For org-wide search, need app-only token + manual ACL filtering per result.
- **Embedding strategy:** Extract text via Graph API (`.content` download property). PDFs, Office docs, .txt, .md. Pre-extracted `searchableText` available for SharePoint Online (we could even use Microsoft's pre-computed vectors for free).
- **Effort:** M (4-6 weeks)
- **Priority:** **P0** — must have for B2B

### Google Drive / Google Workspace
- **What:** Google's cloud file storage. Used by everyone who uses Google Docs.
- **Used by:** 3B+ users. Strong in education, startups, marketing teams.
- **Why Telme should connect:** Second-largest B2B market. Strong with non-Microsoft shops.
- **Auth model:** Google OAuth 2.0. User OR service account (with domain-wide delegation).
- **Sync model:** Changes API (`/changes`) returns deltas. Initial scan via `files.list` with pagination. Drive Activity API for audit.
- **Permissions:** Drive API returns user-scoped results. For org-wide, use service account + check ACL per file.
- **Embedding strategy:** Google Docs export to plain text via `?alt=media`. PDFs via Drive export API. Slides via export to text.
- **Effort:** M (3-5 weeks)
- **Priority:** **P0** — must have for B2B

### Dropbox
- **What:** Consumer + SMB file storage.
- **Used by:** Marketing teams, agencies, small businesses. ~700M users.
- **Why Telme should connect:** Smaller market but high willingness to pay. Many creative agencies use Dropbox.
- **Auth model:** OAuth 2.0. PKCE recommended for public clients.
- **Sync model:** Webhooks (`/users/{id}/files/delta` or `/webhooks`). Initial list via `/files/list_folder`.
- **Permissions:** Returned with each file. For org-wide, use team token + permission check.
- **Embedding strategy:** `/files/get_metadata` + `/files/get_temporary_link` + download content.
- **Effort:** M (3-4 weeks)
- **Priority:** **P1**

### Box
- **What:** Enterprise content management. Strong in regulated industries.
- **Used by:** Fortune 500, healthcare, life sciences, legal.
- **Why Telme should connect:** High ACV; compliance-ready API; pre-computed metadata.
- **Auth model:** OAuth 2.0 + JWT (server auth).
- **Sync model:** Events API + Box Skill (webhook) for real-time. Long poll `/events`.
- **Permissions:** Strong ACL system. Returned with each item.
- **Embedding strategy:** Box AI API offers pre-extracted text + metadata for many file types.
- **Effort:** M (4-5 weeks)
- **Priority:** **P1** (when we add enterprise tier)

### iManage / NetDocuments (legal-specific DMS)
- **What:** Document management for law firms.
- **Used by:** 80%+ of AmLaw 200 firms.
- **Why Telme should connect:** Massive pain + massive budget. $50-200/user/month spend.
- **Auth model:** OAuth 2.0 (both vendors added this in 2023-24).
- **Sync model:** Vendor-specific REST APIs.
- **Effort:** L each (6-10 weeks due to vendor quirks)
- **Priority:** **P2** (high value but niche)

---

## Knowledge bases & wikis (Tier 1)

### Notion
- **What:** All-in-one workspace. Docs, databases, wikis.
- **Used by:** Startups, product teams, design teams. ~30M users.
- **Why Telme should connect:** Telme users are likely Notion users. Complement, not compete.
- **Auth model:** OAuth 2.0.
- **Sync model:** Polling (no real-time webhooks for content). 3-5 min poll interval is acceptable.
- **Embedding strategy:** Notion API returns plain text content. Easy.
- **Effort:** S-M (2-3 weeks)
- **Priority:** **P0** (most-loved connector in our user base)

### Confluence
- **What:** Atlassian's wiki/knowledge base. The standard for engineering teams.
- **Used by:** Engineering orgs, especially in mid-tech.
- **Why Telme should connect:** Atlassian ecosystem = engineering teams = our ICP.
- **Auth model:** OAuth 2.0 (3LO). For server-side: API token + basic auth.
- **Sync model:** Webhooks for page updates. Initial scan via `/wiki/rest/api/content/search`.
- **Embedding strategy:** Confluence stores pages as `storage` (HTML) + `view` (rendered). Use storage for embedding (smaller, faster).
- **Effort:** M (3-4 weeks)
- **Priority:** **P0**

### Slab (knowledge base)
- **Used by:** Mid-tech. Growing.
- **Priority:** P2 (wait for demand)

### Guru (cards)
- **Used by:** Sales, support, mid-tech.
- **Priority:** P2

### Slite / Tettra / Notion AI
- Sub-niches. P2.

---

## Communication & chat (Tier 2)

### Slack
- **What:** Workplace chat.
- **Used by:** Almost every tech company.
- **Why Telme should connect:** "Where's that doc posted in Slack last week?" is asked 10×/day.
- **Auth model:** OAuth 2.0 (with workspace install for team-wide).
- **Sync model:** Events API (webhooks). Initial scan via `conversations.history` paginated.
- **Permissions:** Workspace tokens get all channels the user can see. Bot tokens need explicit channel invites.
- **Embedding strategy:** Concatenate `text` + `files` + `user` metadata. Slack files use the same Office formats; extract same way.
- **Effort:** M (3-4 weeks)
- **Priority:** **P0**

### Microsoft Teams
- **What:** Microsoft's chat + video.
- **Used by:** Every M365 customer.
- **Why Telme should connect:** Same reason as Slack, but in Microsoft shops.
- **Auth model:** Microsoft Graph (same as SharePoint).
- **Sync model:** Graph change notifications (webhooks).
- **Effort:** M (4-5 weeks) — Graph is complex.
- **Priority:** P1

### Discord
- **Used by:** Communities, crypto, gaming.
- **Priority:** P2 (niche for our ICP)

---

## Dev tools (Tier 2)

### GitHub
- **What:** Source code + issues + PRs + discussions + wikis.
- **Used by:** Every dev team.
- **Why Telme should connect:** "Where did we discuss this?" — search across code + PR comments + issues.
- **Auth model:** GitHub App (preferred) or OAuth 2.0 user token.
- **Sync model:** Webhooks (push, PR, issue). Initial scan via REST or GraphQL.
- **Embedding strategy:** For code: chunk by function/class. For issues: title + body + comments.
- **Effort:** M-L (4-6 weeks)
- **Priority:** P1

### GitLab
- **What:** Git + CI + issues. Self-hostable.
- **Auth:** OAuth or PAT.
- **Priority:** P1 (ship after GitHub)

### Linear
- **What:** Issue tracking for product teams.
- **Auth:** OAuth 2.0.
- **Embedding:** Issue title + description + comments.
- **Effort:** S (1-2 weeks)
- **Priority:** P1

### Jira
- **Used by:** Larger orgs (often Atlassian shop).
- **Priority:** P2 (ship after Confluence)

### Sentry / Datadog / observability tools
- **Used by:** Engineering.
- **Priority:** P2 (specialty search needs different UX)

---

## Communication async (Tier 2)

### Gmail
- **What:** Email. Still #1 "where is that message" pain.
- **Auth:** OAuth 2.0.
- **Embedding:** Subject + body, skipping quoted replies.
- **Effort:** M (3-4 weeks)
- **Priority:** P1

### Outlook 365
- Same as Teams (Graph API).
- **Priority:** P1

### Front
- **What:** Shared inbox for support/sales.
- **Used by:** Mid-tech support teams.
- **Priority:** P2

---

## Sales / CRM (Tier 3)

### Salesforce
- **Used by:** Sales orgs. High willingness to pay ($150+/user/month).
- **Auth:** OAuth 2.0.
- **Priority:** P2 (different buyer; sales-driven search is a feature, not core)

### HubSpot
- **Priority:** P2

### Pipedrive
- **Priority:** P3

---

## Customer support (Tier 3)

### Zendesk
- **Used by:** Customer support orgs.
- **Priority:** P2

### Intercom
- **Used by:** Tech-forward support.
- **Priority:** P2

### Front
- Already listed under email.

---

## Spreadsheets & data (Tier 3)

### Airtable
- **Used by:** Ops, marketing.
- **Priority:** P2

### Google Sheets
- Bundled with Google Workspace connector.
- **Priority:** P0 (with Drive)

### Excel Online
- Bundled with SharePoint/OneDrive connector.
- **Priority:** P0

---

## Design & media (Tier 3)

### Figma
- **Used by:** Design teams.
- **Auth:** OAuth 2.0.
- **Embedding:** Design metadata + comments. Files are binary, not full-text searchable.
- **Priority:** P2 (visual content doesn't embed well; mostly comment search)

### Canva
- **Priority:** P3

---

## HR / People (Tier 3)

### BambooHR / Rippling / Workday
- **Why:** "Find that PTO policy" — real but infrequent.
- **Priority:** P3

---

## Finance / Legal (Tier 3)

### NetSuite / QuickBooks / Xero
- **Priority:** P3 (specialty tools)

### Ironclad / DocuSign / ContractWorks
- **Used by:** Legal ops.
- **Priority:** P2 (high value, niche)

---

## Project / PM (Tier 2)

### Asana
- **Used by:** Cross-functional teams.
- **Auth:** OAuth.
- **Priority:** P2

### Monday.com
- **Priority:** P2

### Trello
- **Priority:** P3

---

## Notes / personal (Tier 2)

### Apple Notes
- **Auth:** AppleScript / CloudKit (private API).
- **Priority:** P2 (Mac users love this; tricky to access)

### Obsidian vaults (local)
- Already covered (local file system).
- **Priority:** P0 (free)

### Evernote
- **Used by:** Older demographic.
- **Priority:** P3

### Bear / Craft / Ulysses
- **Mac-only notes apps.**
- **Priority:** P2

---

## Bookmarks & read-later (Tier 2)

### Pocket / Raindrop / Pinboard
- **Why:** "Where did I save that article?"
- **Priority:** P2

### Browser history (Chrome / Safari / Firefox)
- **Privacy-sensitive** — many won't grant access.
- **Priority:** P3

---

## Communication (other)

### Discord (already covered)
### WhatsApp Business
- **Priority:** P3

### Telegram
- **Priority:** P3

---

## Calendar / scheduling

### Google Calendar
- **Priority:** P3 (low embedding value)

### Outlook Calendar
- **Priority:** P3

---

## Source priority summary

### P0 — must-have for v2 launch
1. **SharePoint / OneDrive** (4-6 weeks)
2. **Google Drive / Workspace** (3-5 weeks)
3. **Notion** (2-3 weeks)
4. **Confluence** (3-4 weeks)
5. **Slack** (3-4 weeks)
6. **Local file system** (already shipped!)

Total P0 effort: ~20 weeks (1 engineer, 5 months). Achievable as a series of releases.

### P1 — high-value, ship in v3
- GitHub
- Linear
- Gmail / Outlook
- Microsoft Teams
- Dropbox
- Box
- GitLab
- Jira

Total P1: ~25 weeks. After P0 lands + Series Seed.

### P2 — long tail
Everything else. Build based on user votes.

---

## Universal connector design

To make all these connectable without rewriting for each, see `02-connector-design.md`.
