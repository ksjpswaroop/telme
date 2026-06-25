# Telme — Design System v1

**Status:** Draft
**Last updated:** 2026-06-25

A minimal, native-feeling design system built on Tailwind + shadcn/ui primitives, tuned for an always-accessible search surface.

---

## 1. Principles

1. **Invisible by default** — the UI should feel like a part of the OS, not an app.
2. **One thing on screen** — search input is the hero; everything else supports it.
3. **Fast is a feature** — every animation ≤150ms; no decorative motion.
4. **Respect the system** — light/dark via `prefers-color-scheme`; native fonts; native spacing scale.
5. **Quiet until needed** — empty states and onboarding only show once.

---

## 2. Color tokens

### 2.1 Light theme

| Token | Hex | Usage |
|---|---|---|
| `--bg-base` | `#FAFAFA` | Page background, surface |
| `--bg-elevated` | `#FFFFFF` | Title bar, result cards |
| `--bg-overlay` | `rgba(250, 250, 250, 0.85)` | Backdrop blur layer |
| `--border-subtle` | `#EAEAEA` | Dividers, card borders |
| `--border-strong` | `#D4D4D4` | Focused input border |
| `--text-primary` | `#171717` | Main text |
| `--text-secondary` | `#525252` | File paths, metadata |
| `--text-tertiary` | `#A3A3A3` | Placeholder, hint text |
| `--accent` | `#171717` | Selected item, focus ring (mono) |
| `--accent-fg` | `#FAFAFA` | Text on accent |
| `--success` | `#16A34A` | "Indexed", success toasts |
| `--warning` | `#D97706` | "Ollama not running" |
| `--error` | `#DC2626` | "Index failed", errors |
| `--match-highlight` | `#FEF08A` | BM25 term highlights |

### 2.2 Dark theme

| Token | Hex | Usage |
|---|---|---|
| `--bg-base` | `#0A0A0A` | Page background |
| `--bg-elevated` | `#171717` | Title bar, result cards |
| `--bg-overlay` | `rgba(10, 10, 10, 0.85)` | Backdrop blur |
| `--border-subtle` | `#262626` | Dividers |
| `--border-strong` | `#404040` | Focused input border |
| `--text-primary` | `#FAFAFA` | Main text |
| `--text-secondary` | `#A3A3A3` | File paths, metadata |
| `--text-tertiary` | `#525252` | Placeholder |
| `--accent` | `#FAFAFA` | Selected item |
| `--accent-fg` | `#0A0A0A` | Text on accent |
| `--success` | `#22C55E` | |
| `--warning` | `#F59E0B` | |
| `--error` | `#EF4444` | |
| `--match-highlight` | `#713F12` | BM25 term highlights |

### 2.3 Implementation

```css
/* tailwind.config.ts */
{
  theme: {
    extend: {
      colors: {
        bg: { base: 'var(--bg-base)', elevated: 'var(--bg-elevated)' },
        border: { subtle: 'var(--border-subtle)', strong: 'var(--border-strong)' },
        fg: { primary: 'var(--text-primary)', secondary: 'var(--text-secondary)', tertiary: 'var(--text-tertiary)' },
        accent: { DEFAULT: 'var(--accent)', fg: 'var(--accent-fg)' },
        match: 'var(--match-highlight)',
      }
    }
  }
}
```

---

## 3. Typography

| Use | Family | Size | Weight | Line height |
|---|---|---|---|---|
| Search input | System UI | 20px | 400 | 28px |
| Result title | System UI | 14px | 500 | 20px |
| Result snippet | System UI (mono for paths) | 13px | 400 | 18px |
| File path | `ui-monospace, SF Mono, Menlo` | 12px | 400 | 16px |
| Metadata (score, date) | System UI | 11px | 400 | 14px |
| Settings header | System UI | 16px | 600 | 24px |
| Settings body | System UI | 13px | 400 | 18px |
| Onboarding heading | System UI | 28px | 600 | 36px |
| Onboarding body | System UI | 15px | 400 | 22px |

**System font stack:**
```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif;
font-feature-settings: "ss01", "cv11"; /* SF Pro refinements on macOS */
```

---

## 4. Spacing & sizing

Base unit: **4px**. Scale follows Tailwind defaults.

| Token | Value | Usage |
|---|---|---|
| `space-1` | 4px | Icon-to-text gap |
| `space-2` | 8px | Inline gaps, list item padding |
| `space-3` | 12px | Result item vertical padding |
| `space-4` | 16px | Card padding |
| `space-6` | 24px | Settings form field gap |
| `space-8` | 32px | Onboarding section gap |

**Dimensions:**

| Element | Size |
|---|---|
| Title bar width | 700px |
| Title bar max height | 600px |
| Title bar border radius | 12px |
| Title bar shadow | `0 8px 32px rgba(0,0,0,0.12), 0 2px 8px rgba(0,0,0,0.06)` |
| Search input height | 56px |
| Result item height | 64px |
| Result item gap | 4px |
| Settings window | 520×680px |
| Tray icon | 18×18px (template image) |

---

## 5. Iconography

**Library:** Lucide (matches shadcn default).

| Icon | Usage |
|---|---|
| `Search` | Default state of search input |
| `X` | Clear input button |
| `ArrowUp` / `ArrowDown` | Keyboard nav between results |
| `CornerDownLeft` | "Open" hint on focused result |
| `Folder` | Folder add button |
| `FolderOpen` | Indexed folder indicator |
| `FileText` | Generic file result |
| `FileCode` | Code file result |
| `Settings` | Tray menu entry |
| `Sparkles` | Semantic indicator (vs. keyword) |
| `CircleDot` | Indexing in progress |
| `TriangleAlert` | Error / warning state |
| `Download` | "Download embedding model" |
| `Loader2` (animated) | Indexing spinner |

Icon size: **16px** in result items, **20px** in search bar, **14px** in metadata.

---

## 6. Motion

| Interaction | Duration | Easing | Notes |
|---|---|---|---|
| Title bar open | 120ms | `cubic-bezier(0.16, 1, 0.3, 1)` | Scale 0.98 → 1, opacity 0 → 1 |
| Title bar close | 80ms | `cubic-bezier(0.4, 0, 1, 1)` | Reverse of open |
| Result item appear | 100ms stagger | linear | First 5 results, no later |
| Result item hover | 80ms | ease-out | Background color |
| Result item select | 60ms | ease-out | Accent background |
| Result item press | 40ms | ease-in | Scale 0.99 |
| Settings open | 150ms | ease-out | Fade + slide up 8px |
| Toast appear | 200ms | `cubic-bezier(0.16, 1, 0.3, 1)` | Slide in from top |

**Rule:** no animation exceeds 200ms. No bouncy springs. No decorative motion.

---

## 7. Components

### 7.1 Title bar window

- Floating, centered horizontally, top 80px from screen top
- 700px wide, auto height (max 600px)
- Background: `--bg-elevated` with `--bg-overlay` + 20px backdrop blur
- Border: 1px `--border-subtle`
- Shadow: defined in §4
- Rounded 12px
- **No title bar chrome** — frameless Tauri window
- Keyboard focus trapped inside when open

### 7.2 Search input

```
┌─────────────────────────────────────────────────────┐
│  🔍  Type to search...                          ✕   │
└─────────────────────────────────────────────────────┘
```

- Height: 56px, full width minus 32px padding
- Placeholder: "Search your files..." in `--text-tertiary`
- Cursor auto-focused on open
- Trailing `✕` button (16px icon, appears only when input has text)
- No border, no outline (inherits title bar container border)
- Focus state: 1px `--border-strong` on container only

### 7.3 Result item

```
┌─────────────────────────────────────────────────────┐
│  📄  getting-started.md                            ⌘⏎
│      ~/Documents/notes/getting-started.md            │
│      This guide covers **information retrieval**... │
└─────────────────────────────────────────────────────┘
```

- Height: 64px, 12px vertical padding, 16px horizontal
- Hover/select: background `--bg-base`, 6px radius
- **Selected state**: background `--accent`, text `--accent-fg`, keyboard hint on right
- Layout:
  - **Row 1**: file icon + filename (truncate with ellipsis, max-width 70%)
  - **Row 2**: file path (mono, `--text-secondary`) + similarity badge right-aligned
  - **Row 3**: snippet with `--match-highlight` on matched terms
- Selected keyboard hint: monospace badge "↵ to open" — `--text-tertiary`

### 7.4 Similarity badge

- Right-aligned, inline with file path row
- Pill shape: `padding: 2px 8px`, radius `999px`
- Background: `--bg-base`
- Text: `--text-secondary`, 11px
- Color cue: ≥0.8 green text, ≥0.5 default, <0.5 grey
- Format: `92% match` (semantic) or `↗ keyword` (BM25-only)

### 7.5 Onboarding (first run only)

**Window:** 560×480px, centered.

```
┌─────────────────────────────────────────┐
│                                         │
│              ✨ Welcome to Telme        │
│                                         │
│      Search your files by meaning.      │
│      Everything stays on your Mac.      │
│                                         │
│   ┌─────────────────────────────────┐   │
│   │  +  Add your first folder       │   │
│   └─────────────────────────────────┘   │
│                                         │
│   Or skip and add folders later.        │
│                                         │
│   ✓ Powered by local AI                │
│   ✓ No account needed                  │
│   ✓ Works offline                      │
│                                         │
│              [ Get started → ]          │
└─────────────────────────────────────────┘
```

- Hero icon: 48px `Sparkles` in `--text-primary`
- Headline: 28px / 600
- Subhead: 15px / 400 in `--text-secondary`
- Privacy bullets: 13px / 400, ✓ in `--success`
- "Get started" button: 12px 24px padding, `--accent` bg, `--accent-fg` text

### 7.6 Settings window

- 520×680px, opened from tray menu
- Sections (vertical list):
  1. **Folders** — list with add/remove + last-indexed time
  2. **Embedding model** — radio list (nomic-embed-text, mxbai, custom)
  3. **Search** — sliders for hybrid weights, top-K
  4. **Hotkey** — recorder input ("⌘⇧Space")
  5. **Privacy** — toggle: clear history on quit
  6. **Index** — stats + "Rebuild" / "Clear" buttons
  7. **About** — version, links

### 7.7 Tray menu

- Right-click tray icon → menu:
  - "Search..." (opens title bar)
  - "Indexing: 245 files queued" (status, disabled if done)
  - "Open Settings..."
  - "Pause indexing" (toggle)
  - "Reveal index folder"
  - ───
  - "Quit Telme"

### 7.8 Status banner (inside title bar, below input)

```
┌─────────────────────────────────────────────┐
│  ⚠  Start Ollama to search  →  Open Ollama  │
└─────────────────────────────────────────────┘
```

- Only shown when Ollama unreachable
- Height: 36px, full width, `--warning` background (10% alpha)
- Border-top: 1px `--border-subtle`
- Icon + message + action button on right

---

## 8. States

### 8.1 Empty (no folders indexed)

```
┌─────────────────────────────────────────────┐
│  🔍  Type to search...                      │
│                                             │
│     No folders indexed yet.                 │
│     Add a folder from Settings to begin.    │
└─────────────────────────────────────────────┘
```

### 8.2 Loading (during search)

- Skeleton rows (3) with shimmer, 100ms each
- Existing results stay visible (append-only streaming)

### 8.3 No results

```
     No matches.
     Try different words, or check Settings → Folders.
```

Centered, `--text-secondary`, 13px.

### 8.4 Error

```
     ⚠  Couldn't reach Ollama. Check that it's running.
```

### 8.5 Indexing progress (tray + settings)

- "Indexed 1,234 / 5,678 files" — when active
- Hidden when idle
- Tray icon shows `CircleDot` when indexing

---

## 9. Keyboard map

| Key | Action |
|---|---|
| `⌘⇧Space` / `Ctrl+Shift+Space` | Open title bar |
| `Esc` | Close title bar (clears input first if text present) |
| `↓` | Move selection down |
| `↑` | Move selection up |
| `↵` | Open selected file |
| `⌘↵` / `Ctrl+↵` | Reveal in Finder/Explorer |
| `⌘O` | Open focused result (alternative) |
| `⌘K` | Focus input from result list |
| `⌘,` | Open settings |
| `Tab` (in settings) | Cycle fields |

Selection wraps (last → first on `↓`).
First result auto-selected when results arrive.

---

## 10. Accessibility

- **Contrast**: all text ≥ WCAG AA (4.5:1 normal, 3:1 large)
- **Focus indicators**: 2px outline `--accent` offset 2px, never removed
- **Screen reader**: title bar announces "Search Telme", results as "List with N items"
- **Reduced motion**: respect `prefers-reduced-motion`, disable all animation
- **Keyboard-only**: every action reachable without mouse (see §9)
- **Dynamic type**: search input scales with system text size up to 24px
- **Color independence**: never use color alone — pair with icon/label

---

## 11. Implementation notes

- **Component lib**: shadcn/ui primitives (Dialog, Command, ScrollArea, Tooltip, Toast)
- **Tailwind**: v3, JIT, with `tailwindcss-animate` for micro-motion
- **Icons**: `lucide-react`
- **Fonts**: system stack only — zero font assets bundled
- **Theme**: `next-themes` style but manual (no Next.js); CSS variables switch on `data-theme`
- **Window manager**: Tauri 2 frameless window with custom drag region (just title bar drag handle, results area non-draggable)

```css
[data-theme="light"] { /* light tokens */ }
[data-theme="dark"]  { /* dark tokens */ }
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) { /* dark tokens */ }
}
```

---

## 12. Asset checklist

| Asset | Source | Size |
|---|---|---|
| Tray icon (template) | Hand-drawn or SF Symbol derivative | 18×18 + 36×36@2x |
| App icon (light/dark) | Designer or SF Symbol derivative | 512×512, 1024×1024 |
| Onboarding illustration | Optional: simple line-art folder | 240×240 |
| Empty-state illustration | None — use typography only | — |

**No raster assets bundled in v1** beyond app + tray icons. Everything else is type + color.
