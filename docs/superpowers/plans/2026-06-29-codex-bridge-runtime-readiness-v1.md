# Codex Bridge Runtime Readiness v1

## Goal

Make ChatGPT/Codex Computer Use bridge requirements visible before a user tries screenshot or input execution.

## Decisions

- Keep real bridge execution out of this slice.
- Add a side-effect-free readiness status to `ComputerUseBackendStatus`.
- Use `DEEPSEEK_AGENT_OS_CODEX_BRIDGE_TRANSPORT` to record an explicit `http` vs `stdio` bridge transport decision.
- Use `DEEPSEEK_AGENT_OS_CODEX_BRIDGE_URL` as the local HTTP bridge endpoint configuration variable.
- Treat endpoint configuration as readiness information, not proof of connection.
- Keep `connected=false` until a bridge health check and execution transport are implemented.
- Include readiness in the runtime inspector and exported tool-readiness snapshot.

## Implementation Plan

1. Add failing tests for required transport decision, HTTP endpoint requirement, configured endpoint without connected status, stdio selection without endpoint, local route not requiring bridge, and legacy JSON defaults.
2. Add `CodexBridgeRuntimeStatus`, transport options, and a pure status builder for tests.
3. Read the transport and endpoint environment variables in normal status generation.
4. Mirror the status in TypeScript and render it in the inspector.
5. Update README and session handoff.
6. Run focused Rust tests, full Rust tests, frontend build, Tauri build, and diff hygiene checks.

## Out Of Scope

- HTTP/WebSocket bridge health checks.
- Real bridge screenshot capture.
- Real bridge mouse/keyboard input execution.
- Bridge authentication tokens.
