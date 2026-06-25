# Telme — Architecture Document

**Status:** Draft v1
**Last updated:** 2026-06-25
**Companion doc:** `PRD.md`

---

## 1. System overview

Telme is a local-first desktop search engine. The user invokes a global hotkey, types a query into a floating title bar, and sees semantically-ranked results from their local files. All work happens on-device.

```
┌──────────────────────────────────────────────────────────────┐
│                       USER'S MACHINE                         │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                   Telme (Tauri 2)                   │    │
│  │                                                     │    │
│  │   ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │    │
│  │   │ Frontend │  │ Backend  │  │  Local Storage   │  │    │
│  │   │ (React)  │◄─┤  (Rust)  │─►│  SQLite + vec    │  │    │
│  │   └──────────┘  └─────┬────┘  └──────────────────┘  │    │
│  │                       │                              │    │
│  │                       ▼                              │    │
│  │              ┌────────────────┐                      │    │
│  │              │ Ollama / GGUF  │  (embeddings)        │    │
│  │              └────────────────┘                      │    │
│  │                                                     │    │
│  └─────────────────────────────────────────────────────┘    │
│                          │                                   │
│                          ▼                                   │
│               ┌─────────────────────┐                        │
│               │  User's filesystem   │                       │
│               │  (PDFs, MDs, code)   │                       │
│               └─────────────────────┘                        │
│                                                              │
└──────────────────────────────────────────────────────────────┘
         NO network calls. NO cloud. NO LLM generation.
```

---

## 2. Component breakdown

### 2.1 Frontend (React 19 + TypeScript)

| File | Responsibility |
|---|---|
| `App.tsx` | Root component, routing between search bar / settings |
| `components/TitleBar.tsx` | Floating search input, hotkey receiver |
| `components/ResultList.tsx` | Streaming result list with snippets |
| `components/ResultItem.tsx` | Single result: filename, path, score, snippet |
| `components/Settings.tsx` | Folder management, model picker, index stats |
| `components/Onboarding.tsx` | First-run flow: add folders, install Ollama |
| `lib/tauri.ts` | Thin wrapper over `@tauri-apps/api` invoke commands |
| `lib/streamSearch.ts` | Debounced query + result stream consumer |
| `hooks/useHotkey.ts` | Global hotkey registration via Tauri plugin |
| `hooks/useIndexStatus.ts` | Polls backend for indexing progress |

**Why React?** Same stack as Foundry → faster iteration, shared component patterns.

### 2.2 Backend (Rust, Tauri commands)

| Module | Responsibility | Key crates |
|---|---|---|
| `main.rs` | Tauri app setup, command registration, plugin init | `tauri` |
| `commands/` | Tauri command handlers (frontend ↔ backend RPC) | — |
| ├─ `search.rs` | `search(query)` → ranked results | — |
| ├─ `index.rs` | `add_folder`, `remove_folder`, `rebuild_index` | — |
| ├─ `settings.rs` | `get_config`, `set_config` | — |
| └─ `status.rs` | `get_index_status`, `get_stats` | — |
| `indexer/walker.rs` | Recursive directory traversal | `walkdir`, `ignore` |
| `indexer/watcher.rs` | FS event watcher (create/modify/delete) | `notify` |
| `extractor/` | File-type → plain text | — |
| ├─ `mod.rs` | Dispatcher by extension | — |
| ├─ `pdf.rs` | PDF → text | `pdf-extract` |
| ├─ `docx.rs` | DOCX → text | `docx-rs` |
| ├─ `html.rs` | HTML → text | `html5ever`, `markup5ever` |
| └─ `text.rs` | Plain text + code files | — |
| `chunker.rs` | Text → overlapping chunks | `text-splitter` |
| `embedder/` | Chunk → vector | — |
| ├─ `mod.rs` | Trait + dispatcher | — |
| ├─ `ollama.rs` | HTTP client to Ollama `/api/embeddings` | `reqwest` |
| └─ `local.rs` | Bundled GGUF via llama.cpp bindings | `candle-core` |
| `store/` | SQLite + sqlite-vec | — |
| ├─ `db.rs` | Connection, migrations | `rusqlite` |
| ├─ `schema.rs` | Table definitions | — |
| ├─ `files.rs` | File metadata CRUD | — |
| └─ `vectors.rs` | Vector insert + KNN search | `sqlite-vec` |
| `search/` | Query pipeline | — |
| ├─ `pipeline.rs` | Embed query → vector search → BM25 merge → rerank | — |
| ├─ `hybrid.rs` | Score fusion (semantic + keyword) | — |
| └─ `snippet.rs` | Generate context snippet around match | — |
| `config.rs` | App config (folders, model, hotkey) | `serde`, `directories` |
| `state.rs` | Tauri-managed shared state | — |
| `error.rs` | Unified error type | `thiserror` |

### 2.3 Data layer

**SQLite database** at `~/Library/Application Support/telme/index.db` (macOS) / `%APPDATA%/telme/index.db` (Windows).

```sql
-- File registry
CREATE TABLE files (
    id INTEGER PRIMARY KEY,
    path TEXT UNIQUE NOT NULL,
    mtime INTEGER NOT NULL,        -- unix seconds
    size INTEGER NOT NULL,
    content_hash TEXT,             -- SHA256 of extracted text
    indexed_at INTEGER,
    status TEXT DEFAULT 'pending'  -- pending | indexed | error
);

CREATE INDEX idx_files_path ON files(path);
CREATE INDEX idx_files_status ON files(status);

-- Text chunks (for BM25 + snippets)
CREATE TABLE chunks (
    id INTEGER PRIMARY KEY,
    file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
    ordinal INTEGER NOT NULL,      -- chunk index within file
    text TEXT NOT NULL,
    token_count INTEGER
);

CREATE INDEX idx_chunks_file ON chunks(file_id);

-- BM25 virtual table (FTS5)
CREATE VIRTUAL TABLE chunks_fts USING fts5(
    text,
    content='chunks',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Vector index (sqlite-vec)
CREATE VIRTUAL TABLE chunk_vectors USING vec0(
    embedding float[768]            -- dim depends on model (nomic = 768)
);

-- App config
CREATE TABLE config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

**Why sqlite-vec?** Embedded, no server, fast KNN at small-to-medium scale (≤1M vectors). If we ever outgrow it, swap to Qdrant — same query interface.

### 2.4 Embedding service

```
┌────────────────────────────────────┐
│  Embedder (trait Embedder)         │
├────────────────────────────────────┤
│  + embed(text: &str) -> Vec<f32>   │
│  + embed_batch(texts: &[&str])     │
│  + dim() -> usize                  │
│  + name() -> &str                  │
└────────────────────────────────────┘
            ▲               ▲
            │               │
   ┌────────┴────┐   ┌──────┴──────┐
   │ OllamaClient│   │ LocalGguf   │
   │ (default)   │   │ (fallback)  │
   └─────────────┘   └─────────────┘
```

**Ollama path** (default):
- HTTP POST to `http://localhost:11434/api/embeddings`
- Model: `nomic-embed-text` (768-dim, 274MB)
- Batch up to 32 chunks per request
- Retry with backoff; circuit-break after 5 failures

**Local GGUF path** (fallback when Ollama not installed):
- Bundle a quantized embedding model (~50MB) in app resources
- Load via `candle-core` + custom GGUF loader
- Slower startup, zero external dependency

---

## 3. Data flow

### 3.1 Indexing flow

```
[File watcher event]  OR  [Initial folder scan]
            │
            ▼
   ┌────────────────┐
   │ PathFilter     │  ← skip .git, node_modules, >50MB
   └────────┬───────┘
            ▼
   ┌────────────────┐
   │ Extractor      │  ← PDF/DOCX/HTML/Text → string
   └────────┬───────┘
            ▼
   ┌────────────────┐
   │ Chunker        │  ← 512 tokens, 50 overlap
   └────────┬───────┘
            ▼
   ┌────────────────┐
   │ Embedder       │  ← batch of chunks → Vec<Vec<f32>>
   └────────┬───────┘
            ▼
   ┌────────────────┐
   │ Store          │  ← insert files/chunks/chunk_vectors/fts
   └────────────────┘
```

**Concurrency model:**
- Single dedicated thread owns the indexer (`indexer_thread`)
- FS watcher feeds it via `crossbeam::channel` (mpsc, bounded 10k)
- Batch up to 32 chunks per embedding call
- Backpressure: if queue full, drop oldest events (file scan catches up later)

### 3.2 Search flow

```
[User types in TitleBar]
            │
            ▼ debounce 80ms
   ┌────────────────┐
   │ Embed query    │  ← single string → Vec<f32>
   └────────┬───────┘
            ▼
   ┌────────────────────┐
   │ Vector KNN search  │  ← sqlite-vec, top-50
   └────────┬───────────┘
            ▼
   ┌────────────────────┐
   │ BM25 search        │  ← FTS5, top-50
   └────────┬───────────┘
            ▼
   ┌────────────────────┐
   │ Hybrid fusion      │  ← 0.7 * semantic + 0.3 * keyword
   └────────┬───────────┘
            ▼
   ┌────────────────────┐
   │ Dedupe + rerank    │  ← group by file, keep best chunk
   └────────┬───────────┘
            ▼
   ┌────────────────────┐
   │ Snippet generation │  ← 200 chars around match
   └────────┬───────────┘
            ▼
   [Frontend ResultList]
```

**Latency budget** (target ≤300ms p95):
- Embed query: ~30ms (Ollama local)
- Vector KNN: ~50ms (10k chunks in sqlite-vec)
- BM25: ~20ms
- Hybrid + snippet: ~20ms
- IPC + render: ~50ms
- **Headroom: ~130ms**

---

## 4. Concurrency model

| Concern | Strategy |
|---|---|
| Indexer runs in background | Dedicated thread + channel-fed events |
| FS watcher events | `notify` crate, debounced 500ms per path |
| Search during indexing | Readers don't block writers (WAL mode on SQLite) |
| UI never blocks | All Tauri commands async; results stream via Tauri events |
| Embedder concurrency | Semaphore-limited (max 2 in-flight batch requests to Ollama) |

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│ FS Watcher   │───►│ Indexer      │───►│ Embedder     │
│ (notify)     │    │ (1 thread)   │    │ (sem=2)      │
└──────────────┘    └──────┬───────┘    └──────┬───────┘
                           │                   │
                           ▼                   ▼
                     ┌──────────────────────────────┐
                     │  SQLite (WAL mode)           │
                     └──────────────────────────────┘
                                      ▲
                                      │
                              ┌───────┴────────┐
                              │ Search command │
                              │ (async)        │
                              └────────────────┘
```

---

## 5. Tauri command surface

| Command | Args | Returns |
|---|---|---|
| `search` | `query: String, limit: usize` | `Vec<SearchResult>` (streamed via events) |
| `add_folder` | `path: String` | `()` |
| `remove_folder` | `path: String` | `()` |
| `list_folders` | — | `Vec<String>` |
| `get_stats` | — | `IndexStats { files, chunks, model, size_mb }` |
| `get_index_status` | — | `IndexStatus { pending, indexing, error }` |
| `set_model` | `name: String` | `()` |
| `list_models` | — | `Vec<ModelInfo>` |
| `rebuild_index` | — | `()` |
| `clear_index` | — | `()` |
| `open_file` | `path: String` | `()` |
| `get_config` / `set_config` | key/value | JSON value |

All commands are `#[tauri::command] async fn ...`. Long-running operations stream progress via `app.emit_all("index_progress", ...)`.

---

## 6. Security & privacy

| Concern | Mitigation |
|---|---|
| Reads user files | Explicit folder allowlist — never traverse whole disk |
| Path traversal | All paths normalized + validated against allowlist on every command |
| Embeddings leak text? | No — embeddings are lossy, computed locally, never transmitted |
| Ollama exfiltration | Default to `127.0.0.1`; configurable host but warn if non-loopback |
| Binary tampering | Code-sign on macOS (Developer ID) + Windows (Authenticode) |
| First-run telemetry | **None.** No analytics. No crash reporting in v1. |
| File content caching | Plain-text chunks cached in SQLite (user's disk, user's encryption) |
| Search history | Stored only in memory; cleared on app quit (configurable: persist? no, default off) |

---

## 7. Platform-specific concerns

### 7.1 macOS

- App bundle: `Telme.app`
- Code-signing: Developer ID + notarization for Gatekeeper
- File watcher: FSEvents via `notify` (kqueue fallback)
- Global hotkey: requires Accessibility permission for some apps (ours uses `NSEvent` addGlobalMonitorForEvents)
- Bundle ID: `com.telme.desktop`
- Data dir: `~/Library/Application Support/telme/`

### 7.2 Windows

- Installer: MSI or NSIS (Tauri default)
- Code-signing: Authenticode (EV cert recommended)
- File watcher: ReadDirectoryChangesW via `notify`
- Global hotkey: RegisterHotKey WinAPI
- Data dir: `%APPDATA%/telme/`
- Path quirks: handle `\\?\` long paths, case-insensitive FS

### 7.3 Shared code

- `core/` — pure Rust, no platform deps
- `platform/macos.rs`, `platform/windows.rs` — thin shims
- Tauri handles cross-platform window/menu/tray

---

## 8. Build & distribution

| Step | Tool |
|---|---|
| Frontend bundling | Vite |
| Backend compilation | `cargo` (release profile: opt-level=3, lto=thin, codegen-units=1) |
| App bundling | `tauri build` |
| macOS bundle | `.app` + `.dmg` |
| Windows bundle | `.msi` + `.exe` |
| Auto-update (v2) | `tauri-plugin-updater` |
| CI | GitHub Actions: build on macOS-latest + windows-latest |

---

## 9. Performance targets

| Metric | Target | Measurement |
|---|---|---|
| Cold start → first paint | <500ms | Tauri window-ready event |
| First search after open | <300ms | Perf trace from keystroke to render |
| Steady-state search p95 | <300ms | Same |
| Indexing throughput | ≥100 chunks/sec on M1 / Ryzen 5 | Background benchmark |
| Idle CPU | <1% | After indexing complete |
| Idle RAM | <150MB | Empty index |
| Index size | ~30% of corpus text size | `du -sh` on db file |

---

## 10. Failure modes & recovery

| Failure | Behavior |
|---|---|
| Ollama not running | Show banner "Start Ollama to enable search" + button to launch |
| Embedding model missing | Offer to download (one-time, ~50MB) |
| File extraction fails | Mark file `status=error`, skip, log to local error log |
| SQLite corruption | Detect on startup, offer rebuild from scratch |
| Watcher misses events | Periodic full re-scan every 6h (configurable) |
| Index out of sync with FS | Manual "Rebuild" button in settings |
| Disk full | Surface error in status bar; pause indexing |

---

## 11. Testing strategy

| Layer | Tool | Coverage target |
|---|---|---|
| Unit (Rust) | `cargo test` | Core logic: chunker, hybrid fusion, snippet gen, extractor dispatch |
| Integration (Rust) | `cargo test` + temp DB | End-to-end index → search with sample corpus |
| Unit (TS/React) | Vitest + React Testing Library | Components, hooks, debounce logic |
| E2E | Tauri WebDriver | Hotkey → search → open file flow |
| Manual | — | Real corpus (≥10k files), both platforms |

Test corpus lives in `tests/fixtures/`:
- 100 PDFs (synthesized text)
- 200 markdown notes
- 50 DOCX
- 30 HTML
- 100 code files

---

## 12. Open architectural questions

1. **Re-ranking**: v2 cross-encoder add-on, or skip and rely on hybrid scoring?
2. **Multi-index**: support per-folder indexes (privacy boundary)? Defer to v2.
3. **Streaming embed**: stream embeddings to DB as they're computed vs. batch insert? Recommend: batch insert (sqlite-vec faster in bulk).
4. **Embedding cache invalidation**: re-embed on model change? Yes — full rebuild on model swap (rare op).
5. **PDFs with images**: OCR in v2 or skip silently? Skip silently in v1; surface "N image-only PDFs skipped" in stats.

---

## 13. Future extensions (architecture hooks)

- **Plugin system** for custom extractors (trait `Extractor`, loaded as `cdylib`)
- **Remote index sync** via E2E-encrypted sync (libsignal)
- **Cross-encoder rerank** as optional `Reranker` trait, swappable impl
- **Multi-modal** (image embedding) via additional `Embedder` impl
- **CLI mode** (`telme search "query"`) reusing `core/` crate

---

## Appendix A: Crate dependency tree (key items)

```
telme
├── tauri = "2"
├── tauri-plugin-global-shortcut = "2"
├── tauri-plugin-dialog = "2"
├── rusqlite = { version = "0.31", features = ["bundled"] }
├── sqlite-vec = "0.1"
├── tokio = { version = "1", features = ["full"] }
├── reqwest = { version = "0.12", features = ["json"] }
├── notify = "6"
├── walkdir = "2"
├── ignore = "0.4"
├── serde = { version = "1", features = ["derive"] }
├── serde_json = "1"
├── thiserror = "1"
├── anyhow = "1"
├── text-splitter = "0.15"
├── pdf-extract = "0.7"
├── docx-rs = "0.4"
├── html5ever = "0.27"
├── directories = "5"
├── crossbeam-channel = "0.5"
├── tracing = "0.1"
└── tracing-subscriber = "0.3"
```

## Appendix B: Data directory layout

```
~/Library/Application Support/telme/      # macOS
%APPDATA%/telme/                            # Windows
├── index.db                # SQLite + vec + FTS5
├── index.db-wal            # WAL file
├── index.db-shm            # WAL shared memory
├── config.json             # App config
├── models/                 # Downloaded/bundled GGUF models
│   └── nomic-embed-text-v1.5-Q8_0.gguf
├── logs/
│   └── telme.log           # tracing output, rotated daily
└── tmp/                    # In-flight extraction temp files
```
