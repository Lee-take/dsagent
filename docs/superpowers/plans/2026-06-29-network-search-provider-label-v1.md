# NetworkSearch Provider Label v1

## Goal

Make successful NetworkSearch audit output identify the provider or source adapter that produced the source links.

## Scope

- Include the NetworkSearch result provider in successful invocation titles.
- Preserve existing evidence URLs and source-link excerpts.
- Keep a fallback label for legacy or blank provider values.

## Non-Goals

- Do not change live search providers.
- Do not add new source adapters.
- Do not alter approval policy or source-link requirements.

## Verification

Run the focused NetworkSearch provider-label test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
