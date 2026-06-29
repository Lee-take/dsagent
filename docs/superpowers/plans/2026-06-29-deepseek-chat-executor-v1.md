# DeepSeek Chat Executor v1

## Goal

Add a DeepSeek Chat Completions executor layer that can be reused once the NetworkSearch route is confirmed.

This slice should:

- Send a prepared chat completion request through an injectable transport.
- Provide a real reqwest-backed transport for the official DeepSeek endpoint.
- Keep tests fully offline with a fake transport.
- Redact the API key from all returned errors.
- Parse a minimal non-streaming response shape.

## TDD Plan

1. Add failing tests for executor success, missing API key, and secret redaction.
2. Implement the transport trait and executor function.
3. Implement the reqwest-backed transport.
4. Update documentation and handoff notes.
5. Run focused DeepSeek tests, full Rust tests, frontend build, and diff check.

## Non-goals

- Do not call the live DeepSeek API in tests.
- Do not expose a UI button for arbitrary chat calls.
- Do not connect NetworkSearch to DeepSeek until the user confirms the search route.
