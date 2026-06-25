# Telme

**Search everything by meaning, fully local.**

Telme is a cross-platform desktop app (macOS + Windows) that lets you semantically search across your local files via a minimal title-bar interface. All embedding, indexing, and search runs on your machine — no cloud, no LLM calls, no data leaves your device.

## Status

✅ **Phase 2 Sprint 3 — FS Watcher complete.** Files auto-reindex on create/modify/delete. **82/135 v1 points (61%)** shipped. Onboarding window + full Settings deferred to Phase 5 (Polish).

See [CHANGELOG.md](./CHANGELOG.md) for delivery log.

## For users

- 👉 **[USER_GUIDE.md](./USER_GUIDE.md)** — install, first-run setup, daily use, troubleshooting on macOS
- 📦 [Download Telme](https://github.com/ksjpswaroop/telme/releases) (latest release DMG)

## For builders & investors

📈 **[b2b/](./b2b/00-README.md)** — strategic plan for evolving Telme into a multi-tenant SaaS

- Market research: problem landscape, competitive teardown, 5 ranked ICPs
- Services: 20+ connector candidates (SharePoint, Drive, Notion, Confluence, Slack, GitHub, etc.) with tier priorities and universal connector architecture
- Architecture: cloud backend, 5-layer tenant isolation, encryption + BYOK, compliance posture
- Commercial: 5-tier pricing ladder ($0 / $20 / $8/u/mo / $15/u/mo / $25+), GTM phases, funding scenarios, 10-slide pitch deck outline

**TL;DR:** keep shipping Telme v1 for individuals; if/when team demand emerges, build cloud v2 around SharePoint + Drive + Notion first ($2-5M seed to $1M+ ARR in 24 months).


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
