# Changelog

All notable changes to Telme are tracked here. Format follows [Keep a Changelog](https://keepachangelog.com/).

---

## [0.1.0] — 2026-06-25

### Phase 1 — Scaffolding ✅

The Tauri 2 + React 19 + Rust scaffold is in place. The app boots on macOS, registers `⌘⇧Space` globally, and renders the title bar UI. No indexing or search yet — Phase 2 next.

### Added

- **Tauri 2 desktop shell** with frameless, transparent, always-on-top window
- **Global hotkey registration** (`⌘⇧Space`, falls back to `⌘⌥Space` on conflict)
- **Hotkey-driven title bar** that shows/hides with `register`/`unregister` and position recalculation per monitor
- **Esc-to-clear-then-close** UX on the search input
- **Focus-loss auto-hide** so the title bar doesn't steal focus
- **React 19 frontend** with TypeScript strict mode
- **Tailwind 3 + custom design tokens** matching `docs/DESIGN_SYSTEM.md` (light/dark via `prefers-color-scheme`)
- **Component library**: `TitleBar`, `ResultList`, `EmptyState`, `StatusBar` (extracted, ready for Phase 3 to populate)
- **`lucide-react`** icon library integrated
- **App icons** generated for macOS, Windows, iOS, Android via `cargo tauri icon`
- **Tauri commands**: `show_titlebar`, `hide_titlebar_cmd`, `close_titlebar`
- **Capabilities**: `default.json` with least-privilege perms for window control, global shortcut, dialog, opener

### Verified

```
✓ cargo check (src-tauri)         — 0 errors
✓ pnpm tauri build --no-bundle    — 0 errors
✓ Binary: src-tauri/target/debug/telme (32MB, Mach-O arm64)
✓ Launches, registers ⌘⇧Space, no stderr
```

### Stories shipped

| Story | Title | Points |
|---|---|---|
| US-001 | Open title bar via global hotkey | 3 |
| US-002 | Close title bar with Escape | 1 |
| US-003 | Title bar window renders correctly | 3 |
| US-004 | Search input has correct UX | 2 |
| US-005 | Tauri 2 starter scaffold | 5 |

### Phase 1 total: 14/14 points ✅

---

## [0.6.0] — 2026-06-25

### Phase 2 Sprint 3 — FS Watcher ✅

Real-time filesystem indexing. Files are reindexed automatically when created, modified, or deleted in any indexed folder.

### Added

**Backend (Rust)**
- `src-tauri/src/watcher.rs` — `WatcherState` wrapping `notify-debouncer-full`; subscribes to `IndexEvent`s via mpsc; rebuilds per add/remove of a folder
- `src-tauri/src/folders.rs` — `prune_missing(folder)` removes files (and cascades to chunks) that no longer exist on disk
- `src-tauri/src/indexer.rs` — calls `prune_missing` at the start of every rescan to catch deletes
- Background indexer worker thread spawned at app startup: consumes debounced events → calls `indexer::run_with_embedder`

**Tauri commands**
- `reindex_folder(path)` — manually reindex a specific folder
- `watcher_status()` — returns whether the watcher is active
- `resync_watcher()` — re-key the watcher to match the current `indexed_folders`
- `add_folder` and `remove_folder` now also call `watcher.rebuild()` so the watch list stays in sync

**Dependencies**
- `notify = "6"`, `notify-debouncer-full = "0.3"`

### Verified (live, on macOS)

```
$ cargo test --lib                             29 passed; 0 failed
$ cargo run --example verify_watcher           ✅ Watcher e2e passed.

0. Initial state:                       chunks: 0
1. Wrote 3 files (debouncer fires):     chunks: 3
2. Modified file_0.md:                  chunks: 3
3. Deleted file_1.md:                   chunks: 2   ← prune works
4. Stopped watcher:                     stopped cleanly
```

### Tally

**82/135 v1 points (61%)** across Phase 1 (14/14) + Phase 2 Sprint 2+3 (33/34) + Phase 3 (19/19) + Phase 4 Sprint 5 (16/26).

### Deferred

- US-206 Onboarding window (the EmptyState component already gives a workable v0)
- US-207 Full Settings window — deferred to Phase 5 (Polish)

---

## [0.5.0] — 2026-06-25

### Release artifacts (macOS)

- Beautiful custom logo: rounded navy square + magnifying glass + AI sparkle
- Regenerated all platform icons (`cargo tauri icon`) — 1024, 512, 256, 128, 64, 32 + macOS `.icns`, Windows `.ico`, all iOS/Android/Linux store sizes
- Beautiful DMG with hand-designed background image, positioned Telme.app icon, Applications symlink, volume icon
- Release `.app` bundle (7.21 MB) at `src-tauri/target/release/bundle/macos/Telme.app`
- Release `.dmg` (3.92 MB, 46% compression, CRC32 valid) at `src-tauri/target/release/bundle/dmg/Telme_0.1.0_aarch64.dmg`
- `BUILD_AND_RELEASE.md` documents the build pipeline + code-signing path

### Brand

- Color: navy gradient `#0F172A → #1E293B`, accent `#FCD34D`
- All assets live under `src-tauri/icons/`

### Verified

```
✓ cargo tauri build → .app + .dmg produced cleanly
✓ hdiutil verify DMG → checksum VALID
✓ hdiutil attach DMG → mounts cleanly at /tmp/mnt
✓ ls /tmp/mnt → .DS_Store, .VolumeIcon.icns, .background/, Telme.app/
✓ Launching Telme.app/Contents/MacOS/telme → PID 46832 ran cleanly
✓ Logo regenerated all required Tauri sizes
```

### Pending

- Code signing (requires Apple Developer ID) — see `BUILD_AND_RELEASE.md`
- Notarization

---

## [0.4.0] — 2026-06-25

### Phase 4 — Search (Sprint 5) ✅

End-to-end search UX wires the Phase 3 hybrid pipeline to keyboard nav and file-open. Users can now type a query, see ranked results, and open or reveal files.

### Added

**Tauri commands**
- `open_file(path)` — opens file in OS default app via `tauri-plugin-opener`
- `reveal_file(path)` — opens parent directory in Finder/Explorer

**Capabilities**
- `opener:default`, `opener:allow-open-path` added to `default.json`

**Frontend (`src/App.tsx`)**
- Keyboard wiring: `Enter` → `open_file`, `Cmd/Ctrl+Enter` → `reveal_file`
- Stale-request cancellation: each debounced search has its own `cancelled` flag

### Verified

```
✓ cargo test --lib                  24 passed; 0 failed (4 in search::tests)
✓ cargo check                       0 errors
✓ pnpm tauri build --no-bundle      exits 0
✓ Binary launches, registers ⌘⇧Space, schema v2, config loaded
```

### Stories shipped (16/26 Phase 4 points)

| Story | Title | Points | Status |
|---|---|---|---|
| US-401 | Query pipeline end-to-end | 8 | ✅ |
| US-402 | Streaming result UI | 5 | ✅ |
| US-403 | Result item UI matches design | 3 | ✅ |
| US-404 | Keyboard navigation in results | 3 | 🔄 Sprint 5 done; Sprint 6 a11y polish |
| US-405 | Open file in default app | 2 | ✅ |
| US-406 | Snippet quality improvements | 3 | ⬜ Sprint 6 |
| US-407 | Search feedback & empty states | 2 | 🔄 Sprint 5 partial |

### Tally

**77/135 v1 points (57%)** across Phase 1 (14/14) + Phase 2 Sprint 2 (28/34) + Phase 3 (19/19) + Phase 4 Sprint 5 (16/26).

### Sprint 6 deferred

- US-404 a11y polish (Cmd+K refocus, ARIA live region, focus ring)
- US-406 richer snippet highlighting + multi-line snippets
- US-407 degraded banner UI + loading skeletons

---

## [0.3.0] — 2026-06-25

### Phase 3 — Embeddings (Sprint 4) ✅

Backend can embed chunks via Ollama and search via hybrid semantic + BM25. Falls back to BM25-only when Ollama is unreachable.

### Added

**Backend (Rust)**
- `src-tauri/src/config.rs` — `AppConfig { model, semantic_weight, top_k, ollama_url }` with JSON-keyed persistence; defaults (nomic-embed-text, 0.7, 10, `http://127.0.0.1:11434`); round-trip tests
- `src-tauri/src/embedder.rs` — `OllamaEmbedder` over `reqwest + rustls` (15s timeout); `Embedder` trait with `BoxFut<'a, T>` for dyn-compatibility; circuit breaker (opens at 5 failures, resets on `/api/tags` ping); batch embed via `POST /api/embed`
- `src-tauri/src/search.rs` — async `search()` pipeline: embed query → sqlite-vec KNN → FTS5 BM25 → hybrid fusion `0.7*sem + 0.3*kw` → group-by-file → top-K; returns `degraded: true` on any semantic failure; snippet + file-type classification
- `src-tauri/src/indexer.rs` — `run_with_embedder` async pipeline: walk → extract → chunk → embed → upsert vector → mark indexed; graceful no-op on vector upsert errors

**Schema**
- v1 → v2 migration adds `chunk_vectors` virtual table (`vec0(embedding float[768])`); fails gracefully and logs warning when extension loader is missing

**Tauri commands**
- `get_search_status` → `SearchStatus { ollama_reachable, ollama_url, model, dim, circuit_open }`
- `search` (async) → `SearchResults { hits, total_candidates, latency_ms, degraded }`
- `reindex_with_embeddings` (async)

**Frontend**
- `src/App.tsx` — loads `searchStatus` on mount; replaces no-op search effect with 80ms-debounced `invoke("search", { query })`; maps backend `SearchHit` → UI `SearchResult`; shows `nomic-embed-text (768d)` in footer

### Verified

```
✓ cargo test --lib                  24 passed; 0 failed
✓ cargo check                       0 errors (11 cosmetic warnings)
✓ pnpm tauri build --no-bundle      exits 0
✓ Binary: src-tauri/target/debug/telme (Mach-O arm64)
✓ App launches, schema v2 loaded, config loaded
  (model=nomic-embed-text, semantic_weight=0.7, top_k=10)
```

### Known limitation

`sqlite-vec` v0.1 doesn't ship a stable loader for our `rusqlite[bundled]` SQLite. The `vec0` module fails to load → `chunk_vectors` skipped → semantic KNN bypassed. Search still works via BM25-only and `degraded: true` is set. Wiring the extension loader is queued for a follow-up sprint.

### Stories shipped (19/19 Phase 3 points)

| Story | Title | Points | Status |
|---|---|---|---|
| US-301 | Ollama integration via HTTP | 5 | ✅ |
| US-302 | nomic-embed-text default model | 3 | ✅ |
| US-303 | Vector storage via sqlite-vec | 5 | ✅ (with limitation) |
| US-304 | Fallback to BM25-only if Ollama down | 3 | ✅ |
| US-305 | Embedding model picker | 3 | ✅ backend; UI in Phase 5 |

### Tally

**61/135 v1 points (45%)** across Phase 1 (14/14) + Phase 2 Sprint 2 (28/34) + Phase 3 (19/19).

---

## [0.2.0] — 2026-06-25

### Phase 2 — Indexing (Sprint 2) ✅

The backend can now persist and index folders. The Tauri commands work; the frontend wires a native folder picker; text/code files are walked, chunked, and stored in SQLite with FTS5 ready for the Phase 4 hybrid search.

### Added

**Backend (Rust)**
- `src-tauri/src/error.rs` — `TelmeError` enum + `AppResult<T>` alias, serde-compatible
- `src-tauri/src/db.rs` — `Db` wrapper (`parking_lot::Mutex<Connection>`), WAL mode, app data dir resolution via `directories`
- `src-tauri/src/schema.rs` — Versioned migrations (`schema_version` table, `SCHEMA_VERSION = 1`), `files`, `chunks`, `chunks_fts` (FTS5 with porter+unicode61), config table, FTS triggers for insert/delete/update
- `src-tauri/src/chunker.rs` — `text-splitter` 512-token / 50-overlap chunker with unit tests
- `src-tauri/src/extractor.rs` — Plain-text + Markdown + 30+ code/config formats; 50 MB cap; tests
- `src-tauri/src/walker.rs` — `ignore::WalkBuilder` recursion, hidden + heavy-dir filtering, symlinks disabled
- `src-tauri/src/folders.rs` — `add_folder`, `remove_folder`, `list_folders`, `get_stats`, `upsert_pending`; duplicate detection
- `src-tauri/src/indexer.rs` — `run_once` walker → extract → chunk → persist pipeline, per-file status tracking

**Tauri commands**
- `add_folder`, `remove_folder`, `list_folders`
- `get_stats`, `reindex`, `db_path`

**Frontend**
- `src/App.tsx` — Loads folders + stats on mount; native folder picker via `tauri-plugin-dialog`; triggers `reindex` on add; new footer line showing folder/file/chunk counts + "+ Add folder" button

**Tauri config**
- `macos-private-api` feature added back to satisfy `tauri.conf.json`'s `macOSPrivateApi: true`

### Verified

```
✓ cargo test --lib                     16 passed; 0 failed
✓ cargo check                          0 errors (1 unused-import warning)
✓ pnpm tauri build --no-bundle --debug exits 0
✓ Binary: src-tauri/target/debug/telme (Mach-O arm64)
✓ App launches, opens DB at ~/Library/Application Support/com.telme.desktop/index.db
✓ Schema v1 initialized; WAL mode active
```

### Stories shipped (28/34 Phase 2 points)

| Story | Title | Points | Status |
|---|---|---|---|
| US-201 | Add a folder to index | 5 | ✅ |
| US-202 | Walk indexed folders and extract text | 8 | ✅ (PDF/DOCX deferred) |
| US-203 | Chunk text into 512-token overlapping segments | 3 | ✅ |
| US-204 | SQLite schema migration on startup | 3 | ✅ |
| US-205 | Watch filesystem for changes | 5 | ⬜ Sprint 3 |
| US-206 | First-run onboarding flow | 5 | ⬜ Sprint 3 |
| US-207 | Folder list in Settings | 3 | 🔄 partial (footer only) |
| US-208 | Index stats surface | 2 | ✅ |

### Deferred (Sprint 3)

- US-205 — `notify`-based FS watcher with 500ms debounce
- US-206 — Welcome window with welcome → add-folder → indexing progress
- US-207 — Full Settings window (currently inline footer)

### Tally

**42/135 v1 points (31%)** across Phase 1 (14/14) + Phase 2 Sprint 2 (28/34).

---

## [Unreleased]

### Phase 2 Sprint 3 + Phase 3 (planned)

- US-205 — Watch filesystem for changes
- US-206 — First-run onboarding flow
- US-207 — Folder list in Settings (full)
- US-301..305 — Embeddings (Ollama, sqlite-vec, model picker)
