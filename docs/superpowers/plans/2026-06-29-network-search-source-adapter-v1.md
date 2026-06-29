# NetworkSearch Source Adapter V1

## Goal

Connect the selected free NetworkSearch source model to a real source-backed
search adapter while preserving the Phase 2 permission and audit loop.

## Implemented

- Added `NetworkSearchClient`, `NetworkSearchResult`, and source result item
  contracts in the Rust capability kernel.
- Changed `run_network_search_boundary` so pending approval returns before any
  network client is called.
- Added source-backed success behavior: the invocation records the search URL,
  first source result URL, source-backed excerpt, elapsed time, and policy
  decision.
- Added failure behavior for provider errors and empty result sets.
- Added `HttpNetworkSearchClient` for the free source-model path using a
  source-link-preserving HTTP search route.
- Updated `search_network_boundary` to require the selected source model from
  the UI and construct the source adapter.
- Required the free source-backed adapter for every provider in alpha, including
  large models that may support native search once a bridge contract exists.
- Updated route status so a selected free source model reports
  `source_backed_adapter`, `source_links_required`, and live network enabled.
- Updated UI copy so pending approval, successful search, and provider failure
  states are distinct.

## Verification

```powershell
$env:CARGO_TARGET_DIR='D:\codex-target\deepseek-ui'; cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml network_search
$env:CARGO_TARGET_DIR='D:\codex-target\deepseek-ui'; cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
$env:CARGO_TARGET_DIR='D:\codex-target\deepseek-ui-tauri'; npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop tauri build --debug
git diff --check
```

Focused NetworkSearch tests passed with 11 tests. Full Rust tests passed with
128 tests. Desktop TypeScript/Vite build passed. Tauri debug build produced the
NSIS installer. `git diff --check` passed with only LF-to-CRLF warnings.

## Remaining

- Add native large-model NetworkSearch bridge contracts where available.
- Decide whether all free source-model presets should share the alpha web-source
  adapter or split into separate providers.
- Add provider health/status UI once multiple adapters exist.
