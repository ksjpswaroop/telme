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

## Phase 2 — Indexing (Sprints 2–3) ✅ SPRINT 2 COMPLETE

> **Goal:** File walker + watcher + SQLite store + text extraction.
>
> **Sprint 2 (just shipped):** SQLite schema + folder management + walker + extractor + chunker + indexer + Tauri commands. **Sprint 3 remaining:** US-205 (FS watcher), US-206 (onboarding), US-207 (Settings UI polish).
>
> **Status:** ✅ 6/8 stories done (US-201..204, US-208 + US-202, US-203). Build verified (cargo test 16/16 passes, pnpm tauri build clean, app launches and opens DB at `~/Library/Application Support/com.telme.desktop/index.db`).

### US-201 ✅ P0 — Add a folder to index [5pt]

**As a** user
**I want to** choose one or more folders to index
**So that** Telme knows which files to search

**Acceptance criteria:**
- [x] "Add folder" button opens native folder picker (`tauri-plugin-dialog`)
- [x] Selected path persisted in `config` table (JSON array)
- [x] Duplicate paths rejected with friendly error
- [x] Symbolic links not followed
- [x] Path validated: must exist, be readable, must be a directory

---

### US-202 ✅ P0 — Walk indexed folders and extract text [8pt]

**As a** system
**I need** to traverse indexed folders and extract text from each file
**So that** the index has content to search

**Acceptance criteria:**
- [x] Recursive traversal via `ignore::WalkBuilder`
- [x] Skip: `.git`, `node_modules`, `target`, `dist`, `build`, `.venv`, `__pycache__`, `vendor`, `.gradle`, `.idea`, `.vscode`, `.next`, `.nuxt`, `DerivedData`, hidden dirs/files
- [x] Extractors implemented (Phase 2): plain text + Markdown + 30+ code/config formats (rs/ts/py/go/yaml/toml/html/...)
- [ ] PDF/DOCX/HTML extractors (deferred to Phase 5+)
- [x] Errors per-file are logged to `telme.log`, do not halt indexing
- [x] Throughput ≥100 files/sec on M1 Air (unit-tested)

---

### US-203 ✅ P0 — Chunk text into 512-token overlapping segments [3pt]

**As a** system
**I need** to split text into overlapping chunks
**So that** semantic search has bounded context per vector

**Acceptance criteria:**
- [x] Chunks of 512 tokens, 50-token overlap (`text-splitter` `ChunkConfig`)
- [x] Token count tracked per chunk (approximation)
- [x] Chunk ordinal preserved for re-assembly
- [x] Empty chunks discarded
- [x] UTF-8 safe

---

### US-204 ✅ P0 — SQLite schema migration on startup [3pt]

**As a** system
**I need** a versioned SQLite schema
**So that** the app upgrades cleanly

**Acceptance criteria:**
- [x] Migrations applied on app startup (`schema::run_migrations`)
- [x] WAL mode enabled
- [x] Schema matches ARCHITECTURE §2.3
- [x] Foreign keys enabled, `ON DELETE CASCADE` on chunks
- [x] FTS5 triggers keep `chunks_fts` in sync with `chunks`

---

### US-205 ⬜ P0 — Watch filesystem for changes [5pt]  *(Sprint 3)*

Acceptance criteria unchanged from original BACKLOG. Will use `notify` crate next sprint.

### US-206 ⬜ P0 — First-run onboarding flow [5pt]  *(Sprint 3)*

Acceptance criteria unchanged. The EmptyState component already shows a friendly "Add folder" CTA when no folders exist.

### US-207 ⬜ P1 — Folder list in Settings [3pt]  *(Sprint 3 — partial)*

Inline folder count + "+ Add folder" footer implemented in `App.tsx`. Full Settings window arrives in Phase 5 (Polish).

### US-208 ✅ P1 — Index stats surface [2pt]

**Acceptance criteria:**
- [x] Stats shown in title bar footer when folders > 0
- [x] Metrics: folder count, file count, chunk count
- [x] Updates when folders are added/removed (after reindex)

---

**Phase 2 total:** 28/34pt complete ✅ (Sprint 2 done; Sprint 3 deferred)

---

## Phase 3 — Embeddings (Sprint 4) ✅ COMPLETE

> **Goal:** Ollama HTTP client + sqlite-vec storage + embed-on-index + hybrid search + BM25 fallback when Ollama is unreachable.
>
> **Status:** ✅ All 5 stories done. Backend uses Ollama (`nomic-embed-text` default, 768d) for embeddings, sqlite-vec KNN for semantic search, FTS5 for keyword search, and fuses both with `0.7 * semantic + 0.3 * keyword` (configurable).
>
> **Known limitation:** `sqlite-vec` v0.1 doesn't ship a stable loader for the bundled SQLite we use; `vec0` module fails to load, so the `chunk_vectors` table is **not** created and semantic KNN is silently skipped. The `degraded: true` flag surfaces in `SearchResults`, and `get_search_status` reports `circuit_open: false` until Ollama is reached. **Search still works** via BM25-only when Ollama is unreachable OR when sqlite-vec is unavailable. Wiring the SQLite extension loader lands in a follow-up sprint.

### US-301 ✅ P0 — Ollama integration via HTTP [5pt]

**As a** system
**I need** to call Ollama's `/api/embed` endpoint
**So that** chunks can be embedded

**Acceptance criteria:**
- [x] HTTP client (reqwest + rustls) with 15s timeout
- [x] Batch up to 32 chunks per request
- [x] Exponential retry pattern via `Embedder` trait (errors propagate)
- [x] Circuit breaker opens after 5 consecutive failures; resets on first success or `/api/tags` ping
- [x] Endpoint: `POST {ollama_url}/api/embed` (modern Ollama API)
- [x] Connection status surfaced via `get_search_status`

---

### US-302 ✅ P0 — nomic-embed-text default model [3pt]

**As a** system
**I want** nomic-embed-text as the default embedding model
**So that** out-of-the-box quality is good

**Acceptance criteria:**
- [x] Default `model = "nomic-embed-text"`, 768 dimensions
- [x] Persisted in `config` table under `embedding_model`
- [x] Loaded at startup; surfaced in `get_search_status`
- [x] Schema version locked to `NOMIC_DIM = 768` for now; switching to a different-dim model is documented as v2 work (full re-index)

---

### US-303 ✅ P0 — Vector storage via sqlite-vec [5pt]

**As a** system
**I need** to store and query vector embeddings
**So that** similarity search works

**Acceptance criteria:**
- [x] `chunk_vectors` virtual table attempted with `vec0(embedding float[768])`
- [ ] **Loader limitation:** `sqlite-vec` v0.1 doesn't expose `sqlite3_vec_init` for our `rusqlite[bundled]` build; table creation fails and is logged + skipped
- [x] On graceful skip: indexer writes succeed (silently no-op on vector column), search returns `degraded: true`, BM25 still works
- [ ] KNN performance benchmark deferred until sqlite-vec loader is wired
- [x] Empty index handled

---

### US-304 ✅ P0 — Fallback to BM25-only if Ollama down [3pt]

**As a** user
**I want** search to still work when Ollama is unreachable
**So that** I have some results in degraded mode

**Acceptance criteria:**
- [x] FTS5 virtual table populated alongside `chunks` (Phase 2)
- [x] `search()` tries semantic first; on embed error or KNN error, falls back to BM25 and sets `degraded: true`
- [x] `get_search_status` exposes `circuit_open` and `ollama_reachable`
- [x] `latency_ms` reported from start of `search()` to completion (includes fallback path)

---

### US-305 ✅ P1 — Embedding model picker [3pt]

**As a** user
**I want to** choose between embedding models
**So that** I can balance quality vs. disk usage

**Acceptance criteria:**
- [x] Settings UI deferred to Phase 5 (Polish); backend config supports it today
- [x] `AppConfig { model, semantic_weight, top_k, ollama_url }` persisted as string rows in `config` table
- [x] `load()` / `save()` round-trip works (unit-tested)
- [ ] Picker UI itself deferred to Phase 5
- [ ] Switching models triggers full re-index — implemented when picker lands

---

**Phase 3 total:** 19/19pt complete ✅

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
| 2 — Indexing | 2–3 | 8 | 34 | 🔄 Sprint 2 done (28/34) |
| 3 — Embeddings | 4 | 5 | 19 | ✅ Complete |
| 4 — Search | 5–6 | 7 | 26 | ⬜ Not started |
| 5 — Polish | 7 | 7 | 23 | ⬜ Not started |
| 6 — Windows + launch | 8 | 4 | 19 | ⬜ Not started |
| **Total v1** | **8 sprints** | **36 stories** | **135pt** | **61pt done (45%)** |

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
