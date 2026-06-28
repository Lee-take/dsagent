# DeepSeek Agent OS

Local-first open-source desktop Agent OS optimized for DeepSeek.

Read first:

- `PROJECT_CONTEXT.md`
- `DECISIONS.md`
- `SESSION_HANDOFF.md`
- `docs/superpowers/specs/2026-06-28-deepseek-agent-os-architecture-design.md`

## Development

```powershell
pnpm install
pnpm dev
```

## Architecture

The app uses a stable Agent OS Kernel with Workflow Packs. The first implementation slice builds the desktop shell, local event store, policy model, and DeepSeek route model.
