# Telme — User Guide for macOS

**Search everything by meaning. Fully local. No cloud. No LLM calls.**

Telme is a tiny floating search bar that finds files on your Mac by *meaning*, not just keywords. You press a hotkey, type what you remember about a file ("rust perf", "tauri shortcut"), and Telme surfaces the most relevant files — including ones that don't contain your exact words.

Everything happens on your machine. Nothing is uploaded.

---

## Table of contents

1. [What you need](#what-you-need)
2. [Install Telme](#install-telme)
3. [Install Ollama (the embedding engine)](#install-ollama)
4. [First-run setup](#first-run-setup)
5. [Using Telme](#using-telme)
6. [Keyboard shortcuts](#keyboard-shortcuts)
7. [Tips & tricks](#tips--tricks)
8. [Troubleshooting](#troubleshooting)
9. [Uninstall](#uninstall)

---

## What you need

- **macOS 10.15 (Catalina) or newer**, on Apple Silicon (M1/M2/M3/M4) or Intel
- **~50 MB** for the Telme app
- **~300 MB** for the embedding model (downloaded by Ollama on first use)
- **Ollama** (free, local) — Telme uses it to embed text. Telme itself never talks to the internet.

> **First time?** You need *both* the Telme app and Ollama. Skip to [Install Telme](#install-telme) and [Install Ollama](#install-ollama) below.

---

## Install Telme

1. Download the latest release from
   **[github.com/ksjpswaroop/telme/releases](https://github.com/ksjpswaroop/telme/releases)**.

2. Open the downloaded `.dmg` file. A Finder window opens showing the Telme
   icon and an **Applications** folder alias. Drag `Telme.app` into the
   Applications folder.

4. Eject the DMG (right-click the desktop icon → Eject).

5. Open **Applications** in Finder and double-click `Telme`.

   > **First launch on macOS Sonoma+:** Right-click the app → **Open** →
   > confirm the security prompt. After the first run, double-click works
   > normally. (Gatekeeper warning is expected because Telme isn't
   > notarized with an Apple Developer ID yet.)

6. macOS will ask **"Do you want to allow Telme to control your computer?"**
   → click **OK** (required for the global hotkey).

7. Look for the **Telme icon in your menu bar** (top-right of the screen).
   It confirms Telme is running in the background.

> To make Telme start automatically on login: **System Settings → General →
> Login Items → Open at Login → +** → pick **Telme**.

---

## Install Ollama

Ollama runs the small AI model that turns your text into numbers Telme can
compare. It's free, runs entirely on your Mac, and never sends data anywhere.

### Option A — Homebrew (recommended)

```bash
brew install ollama
ollama serve &           # starts the Ollama daemon
ollama pull nomic-embed-text
```

The `pull` downloads ~270 MB once.

### Option B — Direct download

1. Download from [ollama.com/download](https://ollama.com/download).
2. Drag `Ollama.app` to Applications, launch it.
3. In Terminal:
   ```bash
   ollama pull nomic-embed-text
   ```

### Verify Ollama works

```bash
curl http://127.0.0.1:11434/api/tags
```

You should see `nomic-embed-text` in the JSON response.

> **Telme ships with `nomic-embed-text` as the default embedding model.** It
> runs on CPU in under 50ms per query. You can swap models later in Settings
> (Phase 5 feature).

---

## First-run setup

When you first launch Telme, you'll see a friendly empty-state with a
single button:

```
   ✨  No folders indexed yet.
        Pick a folder to start indexing its text
        and code files.
                [ Add folder ]
```

1. Click **Add folder**.
2. Choose the top of whatever you want to index. Good starting points:
   - `~/Documents`
   - `~/Notes` (if you keep markdown notes)
   - A specific project: `~/code/my-app`
   - Your Obsidian vault
3. Wait. The first folder takes a while — Telme walks every file, extracts
   text, chunks it, and sends each chunk to Ollama for embedding. Plan for
   ~1 minute per 1,000 files.
4. While indexing, you can still search — partial results appear as
   they're ready.
5. Add more folders any time from Settings.

### What's excluded automatically

Telme ignores these to save time and disk:
- `.git/`, `node_modules/`, `target/`, `dist/`, `build/`, `__pycache__/`,
  `.venv/`, `vendor/`, `.gradle/`, `.idea/`, `.vscode/`, `.next/`,
  `.nuxt/`, `DerivedData/`
- Any hidden file or folder (starting with `.`)
- Files larger than 50 MB
- Binary types Telme can't read (PDF, DOCX, images, video) — v1 limitation

### What *is* indexed

- **Text**: `.txt`, `.md`, `.rst`, `.log`
- **Code**: `.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.py`, `.rb`, `.go`,
  `.java`, `.kt`, `.swift`, `.c`, `.cpp`, `.h`, `.cs`, `.sql`, `.sh`
- **Config / data**: `.json`, `.yaml`, `.toml`, `.xml`, `.html`, `.css`,
  `.scss`, `.proto`, `.graphql`

> **Note:** PDFs and Word docs are *not* parsed in v1. If you need that,
> convert to `.md` first with `pandoc file.pdf -o file.md`.

---

## Using Telme

### The global hotkey

Press **`⌘⇧Space`** (Cmd + Shift + Space) from anywhere on your Mac.

A floating bar slides down from the top of the screen. Start typing —
results appear within ~50–200 ms.
- Use **↑** / **↓** to move through results.
- Press **Enter** to open the selected file in its default app (PDFs open in
  Preview, code in your editor, etc.).
- Press **⌘⏎** (Cmd + Enter) to **reveal in Finder**.
- Press **Esc** to clear the query, or **Esc** again to close.

### What results look like

```
   rust-tips.md  ⌘⏎         ← selected (highlighted)
   ~/code/notes/rust-tips.md       74% match
   …cargo build --release for production bi-
   naries. Enable LTO and set codegen-units..

   tauri.md  ⌘⏎
   ~/code/notes/tauri.md           69% match
   …global shortcuts need the tauri-plugin-gl
   obal-shortcut plugin. File dialogs need..

   Showing 4 of 12 results          142ms
```

Each result shows the filename, full path with a match score badge
(`92% match` for semantic hits, `↗ keyword` for BM25-only), and a snippet
with relevant context.

### Match scoring

Two signals, fused 70/30 by default:

- **Semantic** (Ollama embeddings): finds files *about* your query even if
  the words don't match. "rust performance" → finds a file about LTO and
  codegen-units.
- **Keyword** (SQLite FTS5): exact-word matching, very fast.

If Ollama is unreachable, Telme automatically falls back to keyword-only and
shows a "⚠ Showing keyword-only results" indicator in the status bar.

---

## Keyboard shortcuts

| Key | Action |
|---|---|
| `⌘⇧Space` | Open / hide Telme from anywhere |
| `Esc` | Clear query (first press) → close Telme (second press) |
| `↑` / `↓` | Move selection (wraps top↔bottom) |
| `Enter` | Open selected file in default app |
| `⌘⏎` | Reveal selected file in Finder |
| `Tab` | (Future) Cycle between search input and results |

---

## Tips & tricks

### Indexing strategy

- **Be specific.** "Index `~/Documents/Projects`" rather than your entire home
  directory. Smaller indexes = faster searches + less disk.
- **Re-index when things change.** Telme's FS watcher (US-205) re-indexes
  files automatically when you edit, create, or delete them. No manual
  refresh needed.
- **Exclude build outputs.** The default walker already skips `node_modules`,
  `target/`, etc. To exclude a specific path, prefix its folder with `.` or
  add a `.gitignore`-style rule (coming in a future release).

### Search tips

- **Be conversational.** Telme understands meaning. "how does rust borrow
  checker work" works better than literal keywords.
- **Use partial words.** "inform retr" finds documents about "information
  retrieval".
- **Combine with context.** "tauri global shortcut" finds notes about
  Tauri's shortcut plugin even if the file never contains those exact words.

### Performance

| Corpus size | First index | Steady-state search |
|---|---|---|
| 1,000 files | ~1 min | <100 ms |
| 10,000 files | ~10 min | <300 ms |
| 100,000 files | ~1.5 hours | ~500 ms |

Steady-state: if files haven't changed, they're skipped on reindex (the
`files` table tracks mtime).

### Where is the index stored?

```
~/Library/Application Support/com.telme.desktop/
  index.db           # SQLite + vectors (your data)
  index.db-wal       # write-ahead log (transient)
  config.json        # model, hotkey, weights
  models/            # downloaded GGUF models (if you bring your own)
  logs/
    telme.log
```

To inspect the index by hand: `sqlite3 ~/Library/Application\ Support/com.telme.desktop/index.db`
then `.tables` to see what's there.

---

## Troubleshooting

### "⌘⇧Space doesn't open anything"

1. **Is Telme running?** Look for the magnifying-glass icon in your menu bar
   (top-right). If absent, open `Applications/Telme.app` again.
2. **macOS Accessibility permission missing.** Go to **System Settings →
   Privacy & Security → Accessibility** and make sure **Telme** is checked.
3. **Hotkey conflict.** Spotlight uses `⌘Space`. Alfred, Raycast, or other
   launchers may use `⌘⇧Space`. Quit them or change Telme's hotkey in
   Settings (Phase 5 feature; for now, edit
   `~/Library/Application Support/com.telme.desktop/config.json` and restart
   Telme).

### "⚠ Start Ollama to search" banner

Telme can't reach Ollama at `http://127.0.0.1:11434`. Check:

```bash
# Is Ollama running?
curl http://127.0.0.1:11434/api/tags

# If not, start it:
ollama serve &

# Is the model pulled?
ollama list
# If `nomic-embed-text` isn't there:
ollama pull nomic-embed-text
```

### Searches return 0 results

- **Indexing in progress.** Wait until the status bar shows your folder count
  is stable.
- **Wrong folder.** Open Settings, verify the folder you expected is
  indexed. Telme indexes what's *under* the folder you picked.
- **File type not supported.** v1 doesn't parse PDFs or Word docs.
- **Re-index.** In Settings, click **Rebuild index**.

### App crashes on launch

1. Check the log:
   ```bash
   tail -f ~/Library/Logs/telme/telme.log
   ```
2. Most common: a stale DB lock from a previous crash. Quit Telme fully
   (right-click the menu bar icon → Quit), then:
   ```bash
   rm ~/Library/Application\ Support/com.telme.desktop/index.db-wal
   rm ~/Library/Application\ Support/com.telme.desktop/index.db-shm
   ```
   Restart Telme.

### High CPU when idle

The first ~minute after launch is the FS watcher doing its initial sweep.
After that, CPU should drop to ~0%. If it stays high, check
`~/Library/Application Support/com.telme.desktop/logs/telme.log` — Ollama
embedding is the most likely heavy task; consider closing some folders.

### "Database is locked"

You probably have two Telme instances running. Find them:

```bash
pgrep -fl Telme.app
# kill all but the most recent:
pkill -f "Telme.app/Contents/MacOS/telme"
```

---

## Uninstall

1. Quit Telme (right-click menu bar icon → **Quit Telme**, or `⌘Q`).
2. Drag `Telme.app` from `/Applications` to the Trash.
3. Remove user data (optional — this deletes your index):
   ```bash
   rm -rf ~/Library/Application\ Support/com.telme.desktop/
   rm -rf ~/Library/Logs/telme/
   ```
4. Remove Ollama (optional, only if you don't use it elsewhere):
   ```bash
   brew uninstall ollama
   # or: drag Ollama.app to Trash
   rm -rf ~/.ollama/        # deletes all downloaded models
   ```

---

## What's *not* in v1 (yet)

These are tracked in our [roadmap](https://github.com/ksjpswaroop/telme/issues):

- **PDF / Word / Excel parsing** — convert with `pandoc` for now
- **Onboarding wizard** (welcome → pick folder → watch progress)
- **Full Settings window** (currently inline; we have the components,
  just need a window)
- **Custom hotkey** (currently `⌘⇧Space` only)
- **Windows version** (we're macOS-first; Windows ships after v1)

---

## Get help

- **Issues**: [github.com/ksjpswaroop/telme/issues](https://github.com/ksjpswaroop/telme/issues)
- **Discussions**: [github.com/ksjpswaroop/telme/discussions](https://github.com/ksjpswaroop/telme/discussions)
- **Source & contributing**: [github.com/ksjpswaroop/telme](https://github.com/ksjpswaroop/telme)

Telme is **MIT-licensed** and built in the open. PRs welcome.
