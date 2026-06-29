# NetworkSearch Route Status v1

## Goal

Make the unresolved NetworkSearch evidence route explicit in the runtime inspector.

This slice should:

- Report that NetworkSearch currently remains permission/audit only.
- Report that live network requests are disabled until the user confirms an evidence route.
- Show whether DeepSeek orchestration is locally ready, without treating it as verified web evidence.
- Keep `search_network_boundary` behavior unchanged.

## TDD Plan

1. Add failing Rust tests for the route status model and serialization.
2. Implement a small status model and read-only Tauri command.
3. Add TypeScript types, fallback state, invoke loading, and inspector rows.
4. Update README and session handoff notes.
5. Run focused Rust tests, full Rust tests, frontend build, and diff check.

## Non-goals

- Do not choose DeepSeek-only vs Search API + DeepSeek.
- Do not execute live search requests.
- Do not call DeepSeek from the NetworkSearch button.
