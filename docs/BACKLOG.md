# Telme — User Stories & Backlog

**Status:** Draft v1
**Last updated:** 2026-06-25
**Format:** user story • acceptance criteria • story points (Fibonacci) • sprint assignment

---

## Conventions

- **Story points**: 1, 2, 3, 5, 8, 13. ≥13 = split.
- **Sprint length**: 1 week.
- **Phases**: 0 (Scaffold) → 5 (Windows). One phase ≈ one or two sprints.
- **Status**: ⬜ not started • 🔄 in progress • ✅ done • ⛔ blocked
- **Priority**: P0 (must-have for v1) / P1 (should-have) / P2 (nice-to-have)

---

## Phase 1 — Scaffolding (Sprint 1) ✅ COMPLETE

> **Goal:** Tauri app boots on macOS, title bar UI renders, hotkey works, no index yet.
>
> **Status:** ✅ All 5 stories done. Build verified (`tauri build --no-bundle` exits clean, binary 32MB Mach-O arm64, launches and registers hotkey).

### US-001 ✅ P0 — Open title bar via global hotkey [3pt]

**As a** user  
**I want to** press a global keyboard shortcut  
**So that** I can summon the search bar from anywhere in macOS

**Acceptance criteria:**
- [x] `⌘⇧Space` opens the title bar from any focused app
- [x] Hotkey works when no app is focused (desktop visible)
- [x] Hotkey does not conflict with system shortcuts
- [x] If hotkey already registered by another app, fallback to `⌘⌥Space` automatically
- [x] Pressing hotkey while title bar is open does nothing (idempotent)

**Tasks:**
- [x] Register `tauri-plugin-global-shortcut` in `main.rs`
- [x] Configure frameless window, 700px wide, top 80px
- [x] IPC command `show_titlebar` + `hide_titlebar`
- [x] Conflict detection on app startup

---

### US-002 ✅ P0 — Close title bar with Escape [1pt]

**As a** user  
**I want to** press Escape to close the title bar  
**So that** I can return to what I was doing quickly

**Acceptance criteria:**
- [x] `Esc` closes the title bar
- [x] If input has text, first `Esc` clears the input; second `Esc` closes
- [x] Focus returns to previously-focused app after close
- [x] No flicker or animation longer than 100ms

---

### US-003 ✅ P0 — Title bar window renders correctly [3pt]

**As a** user  
**I want** the title bar to look like a native, minimal search surface  
**So that** it feels integrated with my OS

**Acceptance criteria:**
- [x] 700px wide, 12px border radius, native shadow
- [x] Backdrop blur enabled, 20px radius
- [x] Respects system light/dark theme via `prefers-color-scheme`
- [x] Drag handle works on input area only (results area non-draggable)
- [x] No title bar chrome (frameless window)
- [x] Maintains position across multiple opens (no jumping)

---

### US-004 ✅ P0 — Search input has correct UX [2pt]

**As a** user  
**I want** the search input to behave predictably  
**So that** typing feels natural

**Acceptance criteria:**
- [x] Auto-focus on open, cursor visible
- [x] Placeholder "Search your files..." in tertiary color
- [x] Clear ✕ button appears when text is present, hidden when empty
- [x] Click ✕ → input cleared, results cleared
- [x] Tab does not escape the input (focus trapped while title bar open)
- [x] Input scales with system text size up to 24px

---

### US-005 ✅ P1 — Build & ship a Tauri 2 starter for Telme [5pt]

**As a** developer  
**I want** a working Tauri 2 + React 19 + TS scaffold  
**So that** the team has a runnable baseline

**Acceptance criteria:**
- [x] `pnpm install && pnpm tauri build --no-bundle` exits clean
- [x] Project structure matches architecture doc §2.1, §2.2
- [x] Tailwind + shadcn/ui pre-configured with Telme tokens
- [x] Component library scaffolded (`TitleBar`, `ResultList`, `EmptyState`, `StatusBar`)
- [x] Icons generated for macOS + Windows + iOS + Android
- [x] README with setup steps
- [x] `lucide-react` integrated

**Tasks:**
- [x] `tauri init --ci`
- [x] Fix identifier (`com.telme.desktop`), frontendDist (`../dist`), CSP
- [x] Stub `dist/index.html` before `cargo check` (proc macro panic avoidance)
- [x] Configure Tauri plugins: global-shortcut, dialog, opener
- [x] Generate app icons from `app-icon.png` source

**Build evidence:**
```
✓ pnpm tauri build --no-bundle --debug
✓ Built application at: src-tauri/target/debug/telme (32M, Mach-O arm64)
✓ Binary launches, registers ⌘⇧Space, no stderr
```

---

**Phase 1 total:** 14/14pt complete ✅

---

## Phase 2 — Indexing (Sprints 2–3)

> **Goal:** File walker + watcher + SQLite store + text extraction. No embeddings yet — text-only.

### US-201 ⬜ P0 — Add a folder to index [5pt]

**As a** user  
**I want to** choose one or more folders to index  
**So that** Telme knows which files to search

**Acceptance criteria:**
- "Add folder" button opens native folder picker
- Selected path persisted in `config` table
- Duplicate paths rejected (case-insensitive on Windows)
- Symbolic links not followed (avoid loops)
- Path validated: must exist, be readable, not be inside another indexed folder

---

### US-202 ⬜ P0 — Walk indexed folders and extract text [8pt]

**As a** system  
**I need** to traverse indexed folders and extract text from each file  
**So that** the index has content to search

**Acceptance criteria:**
- Recursive traversal via `walkdir` with `ignore`-style filtering
- Skip: `.git`, `node_modules`, `.DS_Store`, hidden files, `>50MB` files
- Extractors implemented: PDF (`pdf-extract`), DOCX (`docx-rs`), HTML (`html5ever`), plain text, code files
- Files with no usable extractor are skipped silently
- Extracted text stored in `chunks` table after chunking
- Errors per-file are logged to `telme.log`, do not halt indexing
- Throughput: ≥100 files/sec on M1 Air with mixed types

---

### US-203 ⬜ P0 — Chunk text into 512-token overlapping segments [3pt]

**As a** system  
**I need** to split text into overlapping chunks  
**So that** semantic search has bounded context per vector

**Acceptance criteria:**
- Chunks of 512 tokens, 50-token overlap (configurable)
- Token count tracked per chunk
- Chunk ordinal preserved for re-assembly
- Empty chunks discarded
- UTF-8 safe (no mid-codepoint splits)

---

### US-204 ⬜ P0 — SQLite schema migration on startup [3pt]

**As a** system  
**I need** a versioned SQLite schema  
**So that** the app upgrades cleanly

**Acceptance criteria:**
- Migrations applied in order on app startup
- WAL mode enabled
- Schema matches ARCHITECTURE §2.3
- Migration errors fail loud (no silent corruption)
- Foreign keys enabled, `ON DELETE CASCADE` on chunks

---

### US-205 ⬜ P0 — Watch filesystem for changes [5pt]

**As a** system  
**I need** to detect file additions, modifications, deletions in real time  
**So that** the index stays current

**Acceptance criteria:**
- `notify` watcher on each indexed folder
- Debounced 500ms per path (avoid thundering herd on save storms)
- Create → enqueue for extraction + indexing
- Modify → re-extract + re-index (delete old chunks first)
- Delete → remove file + chunks + vectors
- Watcher recovers from errors (e.g. folder temporarily unavailable)

---

### US-206 ⬜ P0 — First-run onboarding flow [5pt]

**As a** first-time user  
**I want** a guided setup  
**So that** I can start searching in under a minute

**Acceptance criteria:**
- Welcome screen on first launch (wireframe #6)
- Step 2: folder selection (wireframe #7)
- Step 3: indexing progress with try-a-search CTA (wireframe #8)
- Onboarding state persisted; doesn't reappear
- "Skip" available at each step
- Hero copy emphasizes "everything stays on your Mac"

---

### US-207 ⬜ P1 — Folder list in Settings [3pt]

**As a** user  
**I want** to see and manage my indexed folders  
**So that** I know what's being watched

**Acceptance criteria:**
- Each folder shows: icon, path, status (Indexed/Indexing/Error), last-indexed time
- "Add folder" button → native folder picker
- "Remove" button → confirmation → folder + chunks + vectors deleted
- "Pause" toggles the watcher without removing the folder
- List scrolls when >5 folders

---

### US-208 ⬜ P1 — Index stats surface [2pt]

**As a** user  
**I want** to see how big my index is  
**So that** I can understand resource usage

**Acceptance criteria:**
- Stats shown in Settings → Index section
- Metrics: file count, chunk count, total DB size (MB), active model name
- Updates when folders are added/removed
- Stats accurate within 5 seconds of changes

---

**Phase 2 total:** 34pt (Sprints 2–3 capacity: 30–36pt ✓)

---

## Phase 3 — Embeddings (Sprint 4)

> **Goal:** Ollama integration + vector storage + model fallback.

### US-301 ⬜ P0 — Ollama integration via HTTP [5pt]

**As a** system  
**I need** to call Ollama's `/api/embeddings` endpoint  
**So that** chunks can be embedded

**Acceptance criteria:**
- HTTP client connects to `http://localhost:11434`
- Batch up to 32 chunks per request
- Exponential backoff retry (3 attempts) on transient errors
- Circuit break after 5 consecutive failures → banner shown to user
- Connection check on startup, warn if unreachable
- Health check every 30s while banner shown

---

### US-302 ⬜ P0 — nomic-embed-text default model [3pt]

**As a** system  
**I want** nomic-embed-text as the default embedding model  
**So that** out-of-the-box quality is good

**Acceptance criteria:**
- Default `model = "nomic-embed-text"`, 768 dimensions
- Auto-pull on first use if not present (`POST /api/pull`)
- Show download progress in onboarding/settings
- Persist model choice in config
- Warn if user picks model with different dim → trigger full re-index

---

### US-303 ⬜ P0 — Vector storage via sqlite-vec [5pt]

**As a** system  
**I need** to store and query vector embeddings  
**So that** similarity search works

**Acceptance criteria:**
- `chunk_vectors` virtual table created with `float[768]`
- Insert chunks as vectors with rowid mapping to `chunks.id`
- KNN search returns top-K with distance
- Performance: KNN on 10k vectors <50ms p95 on M1
- Empty index handled (returns 0 results, no error)

---

### US-304 ⬜ P0 — Fallback to BM25-only if Ollama down [3pt]

**As a** user  
**I want** search to still work when Ollama is unreachable  
**So that** I have some results in degraded mode

**Acceptance criteria:**
- FTS5 virtual table populated alongside vector store
- When vector search fails, fall back to BM25 keyword search
- Banner shown: "Start Ollama to search" + button
- Banner has "Start Ollama →" CTA that opens `http://ollama.com`
- Status row text changes to "⚠ Showing keyword-only results"

---

### US-305 ⬜ P1 — Embedding model picker [3pt]

**As a** user  
**I want to** choose between embedding models  
**So that** I can balance quality vs. disk usage

**Acceptance criteria:**
- Radio list of models in Settings (wireframe #9)
- Each shows: name, dimensions, size on disk
- Switching models triggers full re-index
- Confirmation dialog before switch (wireframe #11)
- Cancel returns to previous model

---

**Phase 3 total:** 19pt (Sprint 4 capacity: 15–18pt — slightly over, defer US-305 to Sprint 5)

---

## Phase 4 — Search (Sprints 5–6)

> **Goal:** Query pipeline, hybrid ranking, result UI, end-to-end working search.

### US-401 ⬜ P0 — Query pipeline end-to-end [8pt]

**As a** user  
**I want** to type a query and get ranked results  
**So that** I can find what I'm looking for

**Acceptance criteria:**
- Debounce 80ms after last keystroke
- Embed query (semantic) + BM25 (keyword) in parallel
- Vector KNN top-50 + BM25 top-50 → fusion
- Hybrid score: `0.7 * semantic + 0.3 * keyword`
- Group by file, keep best chunk per file
- Return top-K (default 10)
- Total p95 latency <300ms

---

### US-402 ⬜ P0 — Streaming result UI [5pt]

**As a** user  
**I want** to see results appear as I type  
**So that** search feels instant

**Acceptance criteria:**
- Results stream in via Tauri events as they're computed
- Skeleton loaders for the first 3 rows
- Existing results stay visible while new ones arrive
- Stagger fade-in: 30ms delay per row, max 5 rows animated
- Selection moves with results (first result auto-selected)
- Smooth scroll when results exceed viewport

---

### US-403 ⬜ P0 — Result item UI matches design [3pt]

**As a** user  
**I want** each result to show filename, path, snippet, and match quality  
**So that** I can decide which file to open

**Acceptance criteria:**
- Layout per wireframe #2: filename (row 1), path + score (row 2), snippet (row 3)
- Filename truncated with ellipsis at 70% width
- Score badge: "92% match" for semantic, "↗ keyword" for BM25-only
- Snippet: 200 chars around best match, with BM25 terms highlighted
- Match highlight uses `--match-highlight` color
- Hover state shows full metadata (size, modified, type)
- Click anywhere on row → open file

---

### US-404 ⬜ P0 — Keyboard navigation in results [3pt]

**As a** user  
**I want** to navigate results with arrow keys  
**So that** I never need the mouse

**Acceptance criteria:**
- `↓` moves selection down (wraps last → first)
- `↑` moves selection up (wraps first → last)
- `↵` opens selected file
- `⌘↵` reveals file in Finder/Explorer
- `⌘O` opens focused result (alternative to Enter)
- `⌘K` refocuses input from result list
- Selected row visually distinct (wireframe #2)
- Selection state announced to screen readers

---

### US-405 ⬜ P0 — Open file in default app [2pt]

**As a** user  
**I want** clicking a result to open the file  
**So that** I can use the file normally

**Acceptance criteria:**
- `open::that(path)` cross-platform opener
- Works on macOS, Windows
- File types open in their default app (PDFs in Preview, etc.)
- Errors surface as toast (file moved, deleted, no app)
- Reveal in Finder: macOS uses `open -R`, Windows uses `explorer /select,`

---

### US-406 ⬜ P1 — Snippet quality improvements [3pt]

**As a** user  
**I want** snippets to show relevant context  
**So that** I can decide if a file matches without opening it

**Acceptance criteria:**
- For semantic results: show chunk text centered on highest similarity token
- For keyword results: center on BM25 hit, with 100 chars before/after
- Mixed: prefer keyword centering when both signals present
- Strip markdown formatting in snippets (show plain text)
- Trim to 200 chars, break on word boundary, append "…"

---

### US-407 ⬜ P1 — Search feedback & empty states [2pt]

**As a** user  
**I want** clear feedback when search has no results or errors  
**So that** I know what to do next

**Acceptance criteria:**
- "No matches" state when query returns 0 (wireframe #4)
- "No folders indexed yet" on empty index (wireframe #3)
- Error state with recovery hint when Ollama down (wireframe #5)
- Loading skeletons during in-flight search
- Status row shows "Showing N of M results" + latency

---

**Phase 4 total:** 26pt (Sprints 5–6 capacity: 30–36pt ✓)

---

## Phase 5 — Polish (Sprint 7)

> **Goal:** Settings complete, installer, first-run UX, telemetry-free privacy guarantees.

### US-501 ⬜ P0 — Settings window complete [5pt]

**As a** user  
**I want** a settings window with all preferences  
**So that** I can configure Telme to my needs

**Acceptance criteria:**
- All sections from wireframe #9 implemented
- Folder management (US-207)
- Model picker (US-305)
- Search sliders (semantic weight, results count)
- Hotkey recorder (US-503)
- Privacy toggle (clear history on quit)
- Index stats + Rebuild / Clear buttons
- About section with version + GitHub link
- Window position remembered across opens

---

### US-502 ⬜ P0 — Tray icon + menu [3pt]

**As a** user  
**I want** quick access from the menu bar  
**So that** I can search without remembering the hotkey

**Acceptance criteria:**
- Tray icon shows on macOS menu bar / Windows system tray
- Template image (auto-tints to menu bar)
- Right-click menu per wireframe #12
- Icon pulses during indexing
- Left-click on macOS opens title bar directly
- "Quit" cleanly shuts down watcher + closes DB

---

### US-503 ⬜ P0 — Customizable hotkey [3pt]

**As a** user  
**I want to** change the global hotkey  
**So that** it doesn't conflict with my other apps

**Acceptance criteria:**
- Hotkey recorder UI per wireframe #10
- Captures next valid key combination
- "Recording..." state visible during capture
- Esc cancels recording
- Validates: must include modifier (⌘/Ctrl + something)
- Conflict detection: warns if hotkey already used
- Persists to config, applied on next launch

---

### US-504 ⬜ P0 — Installer + auto-update scaffold [5pt]

**As a** developer  
**I want** signed installers on macOS and Windows  
**So that** users can install Telme cleanly

**Acceptance criteria:**
- macOS: `.app` + `.dmg`, Developer ID signed, notarized
- Windows: `.msi` + `.exe`, Authenticode signed
- `tauri-plugin-updater` wired (manual update check; no auto-install in v1)
- "Check for updates" in tray menu
- App icon in 512 + 1024 sizes, all required OS sizes derived
- Code signing certs documented in BUILD.md

---

### US-505 ⬜ P1 — Index management actions [3pt]

**As a** user  
**I want to** rebuild or clear my index  
**So that** I can recover from corruption or wipe data

**Acceptance criteria:**
- "Rebuild" → confirmation dialog → re-scan all folders from scratch
- "Clear" → confirmation dialog → drop all rows, keep folders
- Both actions show progress in settings
- Pause indexing toggle in tray menu
- Reveal index folder opens data directory in Finder/Explorer

---

### US-506 ⬜ P1 — Privacy guarantees messaging [2pt]

**As a** privacy-conscious user  
**I want to** see proof that nothing leaves my machine  
**So that** I can trust Telme with sensitive files

**Acceptance criteria:**
- Onboarding copy mentions local-only (wireframe #6)
- Settings → About shows: "100% local • No telemetry • No account"
- "Reveal index folder" lets users inspect stored data
- README documents the privacy model explicitly
- Network audit: zero outbound calls in v1 (asserted in tests)

---

### US-507 ⬜ P2 — First-run detection & re-onboarding [2pt]

**As a** user  
**I want** to be re-prompted if I clear my data  
**So that** I'm guided back to a working state

**Acceptance criteria:**
- Detect: app launches with no folders + onboarding already completed
- Show onboarding step 2 (folder add) instead of welcome
- "Skip" available to go to settings

---

**Phase 5 total:** 23pt (Sprint 7 capacity: 15–18pt — defer US-507 to Phase 5)

---

## Phase 6 — Windows & launch prep (Sprint 8)

> **Goal:** Verify on Windows 11, fix platform quirks, polish for first release.

### US-601 ⬜ P0 — Windows 11 verification [8pt]

**As a** Windows user  
**I want** Telme to work natively on Windows  
**So that** I get the same experience as macOS users

**Acceptance criteria:**
- All features work on Windows 11
- Path handling correct (case-insensitive, long paths, drive letters)
- Global hotkey registers via WinAPI
- File watcher works on NTFS and OneDrive folders
- Default file openers resolve correctly (PDF → Edge/Acrobat, etc.)
- Settings window dimensions adjusted (wireframe note)
- App icon renders in taskbar correctly

---

### US-602 ⬜ P0 — Cross-platform testing pass [5pt]

**As a** developer  
**I want** both platforms covered by automated tests  
**So that** regressions don't ship

**Acceptance criteria:**
- CI runs on `macos-latest` and `windows-latest`
- Unit tests pass on both
- Integration tests with sample corpus pass on both
- Manual smoke test checklist documented and executed
- Any platform-specific bugs filed as separate stories

---

### US-603 ⬜ P0 — Launch README + docs [3pt]

**As a** new contributor  
**I want** clear setup instructions  
**So that** I can build Telme from source

**Acceptance criteria:**
- README.md with: install, build, run dev, run prod, test
- BUILD.md with: code signing setup, notarization, release process
- ARCHITECTURE.md (already written) cross-linked
- PRD.md cross-linked
- Screenshots of title bar + onboarding + settings

---

### US-604 ⬜ P1 — Public release prep [3pt]

**As a** user  
**I want to** find and download Telme easily  
**So that** I can start using it

**Acceptance criteria:**
- GitHub release with macOS + Windows installers
- Landing page or github.io site (optional)
- Privacy policy + license files
- Issue templates for bugs + feature requests
- Telemetry-free verified via grep of compiled binary (no analytics SDKs)

---

**Phase 6 total:** 19pt (Sprint 8 capacity: 15–18pt — slight overrun, US-604 may slip to post-launch)

---

## Backlog summary

| Phase | Sprints | Stories | Points | Status |
|---|---|---|---|---|
| 1 — Scaffold | 1 | 5 | 14 | ✅ Complete |
| 2 — Indexing | 2–3 | 8 | 34 | ⬜ Not started |
| 3 — Embeddings | 4 | 5 | 19 | ⬜ Not started |
| 4 — Search | 5–6 | 7 | 26 | ⬜ Not started |
| 5 — Polish | 7 | 7 | 23 | ⬜ Not started |
| 6 — Windows + launch | 8 | 4 | 19 | ⬜ Not started |
| **Total v1** | **8 sprints** | **36 stories** | **135pt** | **14pt done (10%)** |

---

## Deferred / v2 candidates

These were intentionally cut from v1:

- Image OCR + image embedding
- Audio/video transcription
- Cross-encoder re-ranking
- Per-folder indexes (privacy boundary)
- Index sync across devices (E2E encrypted)
- Plugin system for custom extractors
- CLI mode (`telme search "query"`)
- Saved searches / pinned results
- Search history with timeline view
- Custom semantic weight per folder
- Watched folders outside user dirs (e.g. `/tmp`)

---

## Dependencies & risks

| Risk | Mitigation |
|---|---|
| Ollama install friction on first run | Bundle GGUF fallback (US for v2) |
| sqlite-vec performance at scale | Test with 100k corpus early (Phase 2 spike) |
| Tauri 2 + React 19 stack compatibility | Validate in Phase 0 with `tauri build` smoke test |
| Global hotkey conflicts on Windows | Conflict detection + fallback hotkeys |
| FS watcher misses events on network drives | Periodic full re-scan every 6h |
| Large PDF extraction OOM | Stream-extract, skip >50MB, chunk per-page |
| Code signing cert cost blocks release | Use Apple Developer + standard cert; defer EV |

---

## Velocity assumptions

- Solo developer: ~12–15pt/sprint
- Two devs: ~22–28pt/sprint
- Sprint capacity used above assumes solo dev with focused time on Telme
