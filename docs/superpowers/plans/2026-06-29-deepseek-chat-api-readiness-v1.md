# DeepSeek Chat API Readiness v1

## Goal

Prepare the DeepSeek API execution layer without deciding the unresolved NetworkSearch product route.

This slice should:

- Track the official Chat Completions endpoint derived from the configured DeepSeek API base URL.
- Report local readiness from `DEEPSEEK_API_KEY` presence without exposing the key.
- Build a non-streaming Chat Completions request body using the selected model route and thinking level.
- Surface endpoint/model/readiness status in the runtime inspector.

## TDD Plan

1. Add failing Rust tests for the new status fields and request builder.
2. Implement the status fields and pure request builder.
3. Update TypeScript types and backend strategy UI labels.
4. Update README and session handoff.
5. Verify with focused Rust tests, full Rust tests, frontend build, and diff check.

## Explicit Non-goals

- Do not execute `NetworkSearch` through DeepSeek yet.
- Do not claim DeepSeek chat output is verified web-search evidence.
- Do not store or serialize the API key.
- Do not run a live API call in tests.
