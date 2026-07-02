# DeepSeek Agent OS v0.1-alpha Release Notes

Historical note: this release label is superseded by the current `0.1.0`
Windows-first test candidate. Do not treat `v0.1-alpha` as the current project version.

Status: historical source-only alpha note, superseded by the current v0.1.0
Windows-first preview.

## Positioning

DeepSeek Agent OS is an independent local-first desktop Agent OS optimized for
DeepSeek. It is not an official DeepSeek product and is not affiliated with
DeepSeek.

The v0.1-alpha release is feature-frozen. The goal is a credible open-source
baseline for DeepSeek-first desktop agent support, not a broad feature launch.

## What Is Included

- Tauri + React + TypeScript + Rust desktop shell.
- Local SQLite append-only event store.
- DeepSeek-first model route defaults, credential readiness, in-session cache
  telemetry, and manual local pricing configuration.
- Permissioned local tools for file, network, browser, email, local folders,
  terminal, and Computer Use surfaces.
- One-shot approvals for high-consequence actions such as outbound email and
  computer control.
- Source-linked web search with preserved source URLs.
- Optional loopback bridge support for screen inspection, computer control, and
  source-linked web search.
- Local Windows/macOS screen inspection and computer control routes, with
  explicit approval and unlock boundaries.
- Memory Studio candidate review, edit/delete/expiration, and explicit
  conflict actions: link, merge, and replace.
- Operations Briefing workflow pack using local evidence folders and optional
  DeepSeek synthesis when `DEEPSEEK_API_KEY` is configured.
- Work package export/import preview, workflow template import, and read-only
  Operations Briefing archive replay.
- Windows debug NSIS build path and source build instructions.

## Important Alpha Limits

- Email read, draft, and send tools record approval and audit decisions only;
  they do not connect to a mailbox.
- Local folder read and work-package export use local folders and export
  packages; they do not connect to cloud-drive accounts.
- Browser form submission records approval and audit decisions only; it does
  not submit web forms.
- Mutating terminal commands record approval and audit decisions only; they do
  not execute mutating shell commands.
- PDF export is lightweight and ASCII-safe. Use Markdown or HTML for full
  Unicode and Chinese report content.
- Optional local bridge use is loopback-only. Local bridge service startup is
  not included in this historical alpha note.
- Web search must preserve source links. Plain model response text is
  not treated as verified web evidence.
- No hosted sync, account system, marketplace, or arbitrary third-party
  executable plugins are included.

## Verification Before Release

Run on the release branch:

```powershell
npx pnpm@9.15.9 install
npx pnpm@9.15.9 test:secrets
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_agent_os_cargo_target'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop tauri build --debug
git diff --check
```

Optional live DeepSeek smoke test:

```powershell
$env:DEEPSEEK_API_KEY = Read-Host "DeepSeek API key"
npx pnpm@9.15.9 test:deepseek
npx pnpm@9.15.9 test:deepseek:briefing
```

Do not commit API keys, `.env` files, local app data, local evidence folders, or
generated installer artifacts.
