# PRD: Telme

**Search everything by meaning, fully local.**

---

## 1. Overview

**Telme** is a cross-platform desktop app (macOS + Windows) that lets users semantically search across their local files and notes via a minimal title-bar interface. All embedding, indexing, and search runs locally — no cloud, no LLM calls, no data leaving the device.

---

## 2. Goals

| Goal | Success metric |
|---|---|
| Single search bar feels native and instant | First result visible <300ms after query |
| "Search by meaning" works on user's real files | ≥80% relevance on test queries vs. keyword-only |
| Zero-config first-run experience | User searches within 60s of opening the app |
| Works fully offline | No network calls after install |

---

## 3. Non-goals (v1)

- Cloud sync / multi-device
- LLM-generated answers (chat-style Q&A)
- Indexing email, browser history, or system metadata
- Collaborative / shared indexes
- Mobile (iOS/Android)

---

## 4. Target users

- **Primary**: Knowledge workers with growing personal document collections (PDFs, markdown notes, code, text)
- **Secondary**: Researchers, writers, students with large local reference libraries
- **Tertiary**: Privacy-conscious users who refuse cloud AI tools

---

## 5. User experience

### 5.1 First run

1. App opens to a minimal title bar at the top of the screen
2. User clicks "+ Add folder" → selects one or more directories
3. Indexing starts in background; progress bar visible
4. User can search immediately (partial results)

### 5.2 Core flow (post-setup)

1. User presses global hotkey (`⌘⇧Space` / `Ctrl+Shift+Space`)
2. Title bar appears (floating, centered top)
3. User types a query — results stream in below as they type
4. Each result shows: **filename, snippet, file path, similarity %, open button**
5. Click result → opens file in default app
6. `Esc` closes the bar

### 5.3 Visual design

- **Minimal**: just a search input + result list
- **Native feel**: respects system theme (light/dark), fonts
- **No chrome**: no menus, no settings panels by default (settings via tray icon)
- **Width**: 700px, height: auto (max 600px), rounded corners

---

## 6. Functional requirements

### 6.1 Indexing

- **Sources**: PDF, Markdown, TXT, DOCX, HTML, code files (`.ts`, `.py`, `.rs`, `.js`, etc.)
- **Watcher**: file-system watcher detects adds/edits/deletes → re-embeds affected chunks
- **Chunking**: 512 tokens with 50-token overlap
- **Skip**: `.git`, `node_modules`, hidden dirs, files >50MB
- **Storage**: SQLite + sqlite-vec (vector index) in `~/Library/Application Support/telme/` (macOS) / `%APPDATA%/telme/` (Windows)

### 6.2 Embedding

- **Default model**: `nomic-embed-text` via Ollama (274MB, runs on CPU)
- **Fallback**: bundled GGUF model if Ollama not installed
- **Model choice**: user can swap in settings (mxbai-embed-large, all-MiniLM, etc.)
- **First-run**: prompts user to install Ollama or use bundled model

### 6.3 Search

- **Query**: user input → embed → cosine similarity search
- **Hybrid**: combine with BM25 keyword score (70% semantic, 30% keyword) — best of both
- **Ranking**: results sorted by combined score
- **Top-K**: return top 20 results, show first 10 in UI, rest on scroll
- **Re-ranking**: optional cross-encoder re-rank in v2

### 6.4 File handling

- **Extract text** from PDFs (pdf-extract crate), DOCX (docx-rs), HTML (html2text)
- **Cache**: extracted text cached in SQLite; re-extract only on file change (mtime)
- **Open**: click result → `open::that(path)` (cross-platform default app opener)

### 6.5 Settings (tray menu)

- Add/remove folders
- Change embedding model
- Hotkey configuration
- Exclude patterns (glob)
- Index stats (N files, N chunks, model name)
- Rebuild index / clear index

---

## 7. Technical architecture

```
┌─────────────────────────────────────────────┐
│  Tauri 2 (Rust backend + React 19 frontend) │
├─────────────────────────────────────────────┤
│                                             │
│  Frontend (React + TS)                      │
│   ├── TitleBar.tsx    ← search input        │
│   ├── ResultList.tsx  ← results             │
│   └── Settings.tsx    ← tray menu           │
│                                             │
│  Backend (Rust)                             │
│   ├── indexer/      ← file walker + watcher │
│   ├── extractor/    ← PDF/DOCX/HTML → text  │
│   ├── chunker/      ← text → chunks         │
│   ├── embedder/     ← chunks → vectors      │
│   │                  (Ollama HTTP or local) │
│   ├── store/        ← SQLite + sqlite-vec   │
│   └── search/       ← query → results       │
│                                             │
│  External                                   │
│   └── Ollama (optional, for embeddings)     │
└─────────────────────────────────────────────┘
```

---

## 8. Tech stack

| Layer | Choice | Rationale |
|---|---|---|
| Shell | Tauri 2 | Smaller bundle than Electron, native feel |
| Frontend | React 19 + TypeScript | Matches Foundry stack, fast iteration |
| UI | Tailwind + shadcn/ui | Minimal styling, native-feeling components |
| Backend | Rust | File watching, perf, single binary |
| Database | SQLite + sqlite-vec | Embedded, vector search built-in |
| Embeddings | Ollama (default) | Local, well-supported, swappable models |
| File watcher | `notify` crate | Cross-platform FSEvents/ReadDirectoryChangesW |
| Text extraction | `pdf-extract`, `docx-rs`, `html5ever` | Pure Rust, no system deps |
| Global hotkey | `tauri-plugin-global-shortcut` | Built-in Tauri plugin |

---

## 9. Out of scope (v2+)

- Image OCR + image embedding
- Audio/video transcription
- Re-ranking with cross-encoder
- Multi-user profiles
- Index sharing/export
- Plugin system for custom extractors

---

## 10. Open questions

1. **First-run friction**: bundled model (~50MB) vs. require Ollama install?
   → Recommend: bundled for ease, offer Ollama for power users.
2. **Background indexing**: throttle to N% CPU? Pause on battery?
   → Recommend: throttle to 50% CPU, pause on battery by default.
3. **Result snippets**: show semantic-aware highlight or just keyword context?
   → Recommend: keyword context for v1.
4. **Privacy messaging**: how prominently to advertise "100% local"?
   → Recommend: tagline in app + website, no scare copy.

---

## 11. Milestones (proposed)

| Phase | Deliverable |
|---|---|
| **0. Scaffolding** | Tauri app boots, title bar UI renders, hotkey works |
| **1. Indexing** | File walker + watcher + SQLite store + text extraction |
| **2. Embeddings** | Ollama integration, chunking, vector storage |
| **3. Search** | Query pipeline, hybrid ranking, result UI |
| **4. Polish** | Settings, tray menu, installer, first-run UX |
| **5. Windows** | Verify all features on Windows 11, fix path/FS quirks |
