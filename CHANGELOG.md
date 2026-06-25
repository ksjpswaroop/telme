# Changelog

All notable changes to Telme are tracked here. Format follows [Keep a Changelog](https://keepachangelog.com/).

---

## [0.1.0] — 2026-06-25

### Phase 0 — Scaffolding ✅

The Tauri 2 + React 19 + Rust scaffold is in place. The app boots on macOS, registers `⌘⇧Space` globally, and renders the title bar UI. No indexing or search yet — Phase 1.

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

**Phase 0: 14/14 points ✅**

---

## [Unreleased]

### Phase 1 — Indexing (Sprints 2–3, planned)

- US-101 — Add a folder to index
- US-102 — Walk indexed folders and extract text
- US-103 — Chunk text into 512-token overlapping segments
- US-104 — SQLite schema migration on startup
- US-105 — Watch filesystem for changes
- US-106 — First-run onboarding flow
- US-107 — Folder list in Settings
- US-108 — Index stats surface
