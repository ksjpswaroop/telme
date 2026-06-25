# Telme — Hi-Fi Wireframes

**Status:** Draft v1
**Last updated:** 2026-06-25

ASCII representations of every screen in v1, with measurements, states, and micro-interactions. Use these as the source of truth during implementation; the design system doc defines the tokens, this doc defines the screens.

---

## 1. Title bar — default state

When user hits hotkey, this is what appears.

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │                                                           │   ║
║   │  🔍  Search your files...                              ✕   │   ║
║   │                                                           │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║                                                                   ║
║   (empty state shown below input when no query)                   ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
         700px wide • auto height • 80px from top of screen
```

**Specs:**
- Container: 700×auto (max 600px), 12px radius, `--bg-elevated`, 20px backdrop blur
- Shadow: `0 8px 32px rgba(0,0,0,0.12)`
- Input row: 56px tall, 16px horizontal padding, 24px from container top
- Auto-focus cursor on input
- ✕ button hidden until text present

---

## 2. Title bar — query typed, results streaming

```
╔═══════════════════════════════════════════════════════════════════╗
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  🔍  information retrieval                           ✕   │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  📄  introduction-to-ir.md                          ⌘⏎   │   ║
║   │      ~/Documents/notes/nlp/intro.md            92% match │   ║
║   │      …covers **information retrieval** systems including │   ║
║   │      inverted indexes and BM25 ranking…                  │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  📕  Manning-IR-Chapter01.pdf                     ⌘⏎   │   ║
║   │      ~/Books/IR/manning-ch01.pdf             88% match │   ║
║   │      …**Information Retrieval** is the science of search  │   ║
║   │      over text…                                          │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  📝  search-engines.md                            ⌘⏎   │   ║
║   │      ~/Notes/ideas/search.md                 81% match │   ║
║   │      …modern **retrieval** combines lexical and vector   │   ║
║   │      signals for better **information** finding…         │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  💻  search.py                                   ⌘⏎   │   ║
║   │      ~/code/playground/search.py               74% match │   ║
║   │      …def **retrieve**(query: str) -> List[Doc]:…         │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║   Showing 4 of 47 results                            80ms          ║
╚═══════════════════════════════════════════════════════════════════╝
```

**Specs:**
- Input: as above; clear ✕ visible because text present
- Results list: starts 8px below input, 16px horizontal padding, scrollable
- First result auto-selected (highlighted in `--accent`)
- Selected result: 6px radius, slightly inset
- Status row (bottom): 11px, `--text-tertiary`, 16px from container bottom
  - Left: "Showing N of M results"
  - Right: latency ("80ms") in mono font
- Keyboard hint `⌘⏎` only on selected row

---

## 3. Title bar — no folders indexed (empty)

```
╔═══════════════════════════════════════════════════════════════════╗
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  🔍  Search your files...                              ✕   │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║                                                                   ║
║                                                                   ║
║                         📂                                        ║
║                                                                   ║
║                 No folders indexed yet.                          ║
║          Add a folder from Settings to begin.                    ║
║                                                                   ║
║                    [ Open Settings ]                              ║
║                                                                   ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

**Specs:**
- Centered vertically in available space
- Icon: `Folder` 32px, `--text-tertiary`
- Heading: 14px / 500, `--text-secondary`
- Body: 13px / 400, `--text-tertiary`
- Button: `--accent` bg, `--accent-fg` text, 8px 16px padding

---

## 4. Title bar — no results

```
╔═══════════════════════════════════════════════════════════════════╗
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  🔍  quantum entanglement in plant biology           ✕   │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                       No matches.                                 ║
║        Try different words, or check Settings → Folders.          ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

---

## 5. Title bar — Ollama unavailable (warning banner)

```
╔═══════════════════════════════════════════════════════════════════╗
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  🔍  information retrieval                           ✕   │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  ⚠  Start Ollama to search        [ Start Ollama → ]      │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  📄  introduction-to-ir.md                          ⌘⏎   │   ║
║   │      ~/Documents/notes/nlp/intro.md            92% match │   ║
║   │      …covers **information retrieval** systems including │   ║
║   │      inverted indexes and BM25 ranking…                  │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║   ⚠ Showing keyword-only results (Ollama offline)                ║
╚═══════════════════════════════════════════════════════════════════╝
```

**Specs:**
- Banner: 36px tall, full width, `--warning` background at 10% alpha, top border `--border-subtle`
- Banner text: 13px / 500, `--warning`
- "Start Ollama →" button: text-only, `--warning` underline
- Status row text changes to warning copy when in degraded mode

---

## 6. Onboarding — step 1 (welcome)

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║                                                                   ║
║                              ✨                                   ║
║                                                                   ║
║                      Welcome to Telme                             ║
║                                                                   ║
║             Search your files by meaning.                         ║
║              Everything stays on your Mac.                        ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║   ✓  Powered by local AI                                          ║
║   ✓  No account needed                                            ║
║   ✓  Works offline                                                ║
║                                                                   ║
║                                                                   ║
║                      [ Get started → ]                            ║
║                                                                   ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
         560 × 480px • centered • frameless
```

**Specs:**
- Sparkles icon: 48px, `--text-primary`, top of content
- Headline: 28px / 600
- Subhead: 15px / 400, `--text-secondary`
- Bullets: 13px / 400, ✓ in `--success`, 8px gap between
- CTA: 12px 24px padding, radius 8px

---

## 7. Onboarding — step 2 (add folder)

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ✨  Let's index your first folder                               ║
║                                                                   ║
║   Telme will watch this folder and keep its search                ║
║   index up to date as files change.                               ║
║                                                                   ║
║                                                                   ║
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  📁  ~/Documents                                          │   ║
║   │      Estimated: ~1,200 files • ~80 MB                     │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║                                                                   ║
║              [ + Choose another folder ]                          ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                  [ Continue → ]                  ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

**Specs:**
- Folder card: `--bg-base` background, 12px radius, 16px padding
- Icon: `Folder` 20px
- Estimate text: 12px, `--text-tertiary`
- "+ Choose another folder" ghost button
- Continue button bottom-right

---

## 8. Onboarding — step 3 (indexing in progress)

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ⏳  Indexing in progress                                        ║
║                                                                   ║
║   You can start searching now — results will improve              ║
║   as more files are indexed.                                     ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║              ┌─────────────────────────────────┐                  ║
║              │████████████░░░░░░░░░░░░░░░░░░░░░│  45%              ║
║              └─────────────────────────────────┘                  ║
║                                                                   ║
║                   542 / 1,200 files indexed                       ║
║                   Estimated time remaining: 3 min                 ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                                                   ║
║                                  [ Try a search → ]               ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

**Specs:**
- Progress bar: 8px tall, full width minus 64px padding
- Background: `--bg-base`
- Fill: `--accent`
- Numbers: 13px mono, `--text-secondary`
- Estimated time: 12px, `--text-tertiary`

---

## 9. Settings window — main

```
╔═══════════════════════════════════════════════════════════════════╗
║  Settings                                                  ✕     ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║   FOLDERS                                                         ║
║   ┌───────────────────────────────────────────────────────────┐   ║
║   │  📁  ~/Documents                          Indexed • 5 min  │   ║
║   │                                              [ Remove ]   │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  📁  ~/Notes                              Indexed • 2 min  │   ║
║   │                                              [ Remove ]   │   ║
║   ├───────────────────────────────────────────────────────────┤   ║
║   │  📁  ~/Projects                            Indexing 45%    │   ║
║   │                                              [ Pause ]    │   ║
║   └───────────────────────────────────────────────────────────┘   ║
║              [ + Add folder ]                                     ║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   EMBEDDING MODEL                                                 ║
║   ◉  nomic-embed-text  (768d, 274MB)                             ║
║   ○  mxbai-embed-large (1024d, 670MB)                            ║
║   ○  all-MiniLM-L6-v2 (384d, 23MB)                               ║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   SEARCH                                                          ║
║   Semantic weight           [████████░░] 70%                       ║
║   Results to show           [10]                                   ║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   HOTKEY                                                          ║
║   Open Telme                [ ⌘⇧Space ]   [ Record ]              ║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   PRIVACY                                                         ║
║   ◻  Clear search history on quit                                 ║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   INDEX                                                           ║
║   Files: 1,847  •  Chunks: 14,201  •  Size: 124 MB                ║
║   Model: nomic-embed-text                                         ║
║                                              [ Rebuild ] [ Clear ]║
║                                                                   ║
║   ──────────────────────────────────────────────                  ║
║                                                                   ║
║   ABOUT                                                           ║
║   Telme v0.1.0  •  github.com/ksjpswaroop/telme                  ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
         520 × 680px • scrollable content area
```

**Specs:**
- Window: 520×680, scrollable
- Header: 56px, "Settings" 16px / 600, ✕ button right-aligned
- Section spacing: 32px between sections, dividers 1px `--border-subtle`
- Section labels: 11px / 600, uppercase, `--text-tertiary`, letter-spacing 0.05em
- Form rows: 48px tall, 16px horizontal padding

---

## 10. Settings — hotkey recorder

```
║   HOTKEY                                                          ║
║   Open Telme                [ ⌘⇧Space ]   [ Recording... ]        ║
║                                          Press any key combo     ║
║                                          Esc to cancel            ║
```

**Specs:**
- Button transitions to "Recording..." state in `--warning` color
- Helper text below: 12px, `--text-tertiary`
- Next valid key combo replaces the hotkey
- Conflict detection: if hotkey already taken by another app, show red border + "Hotkey conflicts with [App]"

---

## 11. Settings — model change confirmation

```
   EMBEDDING MODEL
   ◉  nomic-embed-text  (768d, 274MB)                            
   ○  mxbai-embed-large (1024d, 670MB)                           
       ↳ Switching requires re-indexing all 1,847 files (~25 min)
       [ Confirm switch ]  [ Cancel ]
```

---

## 12. Tray menu

```
┌─────────────────────────────────┐
│  🔍  Search...              ⌘⇧S│
├─────────────────────────────────┤
│  ⏳  Indexing: 245 files queued │
├─────────────────────────────────┤
│  ⚙  Open Settings...       ⌘, │
│  ⏸  Pause indexing            │
│  📂  Reveal index folder       │
├─────────────────────────────────┤
│  🚀  Check for updates         │
├─────────────────────────────────┤
│  ⏻  Quit Telme            ⌘Q │
└─────────────────────────────────┘
```

**Specs:**
- Standard macOS menu, 280px wide
- Items: 28px tall, 12px 16px padding
- Disabled items (e.g. when not indexing) shown in `--text-tertiary`

---

## 13. Toast notifications

Bottom-right of screen, stack upward, 8px gap.

```
                                                       ┌────────────────────────────┐
                                                       │ ✓  Indexed 1,847 files    │
                                                       │    now                     │
                                                       └────────────────────────────┘
```

**Specs:**
- Width: 360px, height: auto (min 56px)
- Background: `--bg-elevated`, 8px radius, shadow
- Icon left, text 13px, ✕ right
- Auto-dismiss after 4s (success), 8s (error)

---

## 14. Result item — focused & expanded detail

When user holds `⌘` on a result, show full path tooltip.

```
┌─────────────────────────────────────────────────────────────┐
│  📄  introduction-to-ir.md                            ⌘⏎   │
│      ~/Documents/notes/nlp/intro.md            92% match    │ ← focused
│      …covers **information retrieval** systems including    │
│      inverted indexes and BM25 ranking…                     │
│      ──────────────────────────────────────────────         │
│      Last modified: 2 days ago • 4.2 KB • Markdown           │
└─────────────────────────────────────────────────────────────┘
```

**Specs:**
- Expanded detail appears below snippet when item is focused (keyboard) or hovered (mouse)
- 200ms fade-in
- 11px metadata, `--text-tertiary`

---

## 15. First-run empty state inside title bar (vs onboarding)

If user skips onboarding or has zero folders after using the app:

```
                          📂
                 No folders indexed yet.
            Add a folder from Settings to begin.

                   [ Open Settings ]
```

(Same as wireframe #3.)

---

## Micro-interactions reference

| Trigger | Animation | Duration |
|---|---|---|
| Hotkey pressed | Title bar fades + scales 0.98 → 1 | 120ms |
| Hotkey pressed (no folders) | Title bar opens with empty state | 120ms |
| First keystroke | ✕ button slides in from right | 80ms |
| Result arrives | Slide down 4px + fade in | 100ms (staggered 30ms, max 5) |
| Selection moves | Background crossfade | 60ms |
| Selection activates | Brief scale 0.99 → 1 | 40ms |
| Banner appears | Slide down 8px + fade | 150ms |
| Banner dismisses | Slide up + fade | 100ms |
| Settings window opens | Fade + slide up 8px | 150ms |
| Settings window closes | Fade | 100ms |
| Toast appears | Slide in from right + fade | 200ms |
| Tray icon during indexing | Subtle pulse (scale 1 → 1.1 → 1) | 1.2s loop |

---

## Responsive considerations

The app has only one "responsive" surface — the title bar — and it's a fixed 700px width by design. Settings window is fixed 520×680. The only platform-specific sizing:

| Window | macOS | Windows |
|---|---|---|
| Title bar | 700×auto, top 80px | 700×auto, top 80px |
| Settings | 520×680 | 520×720 (slight extra for title bar) |

**High-DPI:** all assets must be 2x. Tray icon needs 18px + 36px@2x. App icon needs 512 + 1024.
