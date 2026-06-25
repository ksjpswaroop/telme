# Telme

**Search everything by meaning, fully local.**

Telme is a cross-platform desktop app (macOS + Windows) that lets you semantically search across your local files via a minimal title-bar interface. All embedding, indexing, and search runs on your machine — no cloud, no LLM calls, no data leaves your device.

## Status

🚧 **Phase 0 — Scaffolding.** Design + planning complete; code not yet written.

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

(Will be filled in during Phase 0.)

```bash
# Phase 0 setup (forthcoming)
pnpm install
pnpm tauri dev
```

## License

TBD
