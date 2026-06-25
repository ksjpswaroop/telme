# Telme

**Search everything by meaning, fully local.**

Telme is a cross-platform desktop app (macOS + Windows) that lets you semantically search across your local files via a minimal title-bar interface. All embedding, indexing, and search runs on your machine — no cloud, no LLM calls, no data leaves your device.

## Status

🔄 **Phase 2 — Indexing (Sprint 2 of 2 complete).** The Rust backend persists folders, walks directories, extracts text/code, chunks, and stores in SQLite with FTS5. Frontend can add folders via native picker. Next: FS watcher + onboarding (Sprint 3), then embeddings (Phase 3).

See [CHANGELOG.md](./CHANGELOG.md) for delivery log.

## Docs

- [PRD.md](./PRD.md) — Product requirements
- [ARCHITECTURE.md](./ARCHITECTURE.md) — Technical architecture
- [docs/DESIGN_SYSTEM.md](./docs/DESIGN_SYSTEM.md) — UI tokens, components, motion
- [docs/WIREFRAMES.md](./docs/WIREFRAMES.md) — Hi-fi ASCII screens
- [docs/BACKLOG.md](./docs/BACKLOG.md) — User stories by phase & sprint

## Stack

- **Shell**: Tauri 2
- **Frontend**: React 19 + TypeScript + Tailwind + shadcn/ui
- **Backend**: Rust
- **Storage**: SQLite + sqlite-vec
- **Embeddings**: Ollama (nomic-embed-text default)
- **File watcher**: `notify` crate

## Development

```bash
# Install deps
pnpm install

# Run dev (HMR + Tauri shell)
pnpm tauri:dev

# Build debug binary (no installer)
pnpm tauri build --no-bundle --debug

# Build signed .app + .dmg
pnpm tauri:build
```

The Rust backend lives in `src-tauri/`. Run `cargo check` from there.

## License

TBD
