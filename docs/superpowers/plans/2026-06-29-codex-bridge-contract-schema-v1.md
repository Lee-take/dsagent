# Codex Bridge Contract Schema v1

## Goal

Define a transport-neutral JSON contract for future Codex bridge health, screenshot, and control messages without choosing HTTP or stdio yet.

## Decisions

- Use contract version `deepseek-agent-os.codex-bridge.v1`.
- Keep the schema transport-neutral; no endpoint, socket, process, or auth fields live in message bodies.
- Use capability names `computer_screenshot` and `computer_control`.
- Screenshot responses include display label, dimensions, capture timestamp, and PNG base64.
- Screenshot responses do not include local evidence paths.
- Control requests keep the existing structured action strings, such as `click:120,340`, and reject natural-language action text.

## Implementation Plan

1. Add failing tests for health request serialization, control request validation, screenshot response validation, and transport-neutral response bodies.
2. Add `codex_bridge_contract` module and register it in the kernel.
3. Implement serde structs and constructors for health, screenshot, and control messages.
4. Update README and session handoff.
5. Run focused Rust tests, full Rust tests, frontend build, Tauri build, and diff hygiene checks.

## Out Of Scope

- HTTP bridge routes.
- stdio sidecar process handling.
- Bridge health checks.
- Bridge authentication tokens.
- Runtime screenshot/control execution through the bridge.
