# Codex Bridge HTTP Runtime v1

## Goal

Implement the approved HTTP transport for ChatGPT/Codex Computer Use routes while keeping the bridge contract transport-neutral and local-first.

## Decisions

- Require `DEEPSEEK_AGENT_OS_CODEX_BRIDGE_TRANSPORT=http` before HTTP bridge execution.
- Read the loopback bridge URL from `DEEPSEEK_AGENT_OS_CODEX_BRIDGE_URL`.
- Reject non-loopback HTTP endpoints to avoid sending screenshots or control requests to remote services.
- Use the shared `deepseek-agent-os.codex-bridge.v1` contract for `/health`, `/screenshot`, and `/control`.
- Keep screenshot response bodies path-free; the desktop client writes returned PNG bytes into the local evidence folder.
- Include the user-approved control target in bridge control requests.
- Keep `stdio` visible as a future transport option but not executable in this slice.

## Implementation Plan

1. Add failing tests for HTTP screenshot execution, HTTP control execution, and health-driven connected status.
2. Add a shared loopback-only HTTP bridge client.
3. Wire Codex bridge screenshot/control clients to the HTTP transport.
4. Update Computer Use backend status to call `/health` and expose connected availability.
5. Update README and session handoff.
6. Run Rust tests, frontend build, Tauri debug build, and diff hygiene checks.

## Out Of Scope

- Starting or managing a Codex sidecar process.
- stdio JSON-RPC transport.
- Bridge authentication tokens.
- Real Codex app integration beyond the local HTTP contract.
