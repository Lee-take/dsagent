# Open Source Installation Guide v1

## Goal

Document how a GitHub user installs, builds, configures, and starts DeepSeek
Agent OS without relying on the maintainer's local paths.

## Scope

- Explain that installer-selected program files are separate from app data,
  workspace, evidence, and export folders.
- Document first-run local directory setup.
- Document source build commands and Windows `CARGO_TARGET_DIR` guidance.
- Document `DEEPSEEK_API_KEY` environment setup without storing secrets.
- Document local manual DeepSeek pricing configuration.
- Summarize NetworkSearch, Computer Use, deferred email connectors, and local
  Drive behavior.

## Non-goals

- Do not choose the unresolved PDF CJK font strategy.
- Do not choose whether Codex bridge runtime is bundled as a sidecar.
- Do not claim macOS packaging has been verified on Windows.

## Verification

Run `git diff --check` after the documentation changes.
