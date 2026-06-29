# NetworkSearch Source Option Transparency v1

## Goal

Make the free NetworkSearch source-model options honest about the alpha backend implementation.

## Scope

- Clarify that reserved free source options currently share the source-backed HTTP adapter.
- Keep the option values stable so saved settings and exported work packages remain compatible.
- Add focused coverage so future option copy does not overstate separate implementations before they exist.

## Non-Goals

- Do not split `free_local_browser` or `free_source_aggregator` into separate providers.
- Do not change NetworkSearch routing, approval policy, or source-link requirements.

## Verification

Run the focused option-disclosure test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
