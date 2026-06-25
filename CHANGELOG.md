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
