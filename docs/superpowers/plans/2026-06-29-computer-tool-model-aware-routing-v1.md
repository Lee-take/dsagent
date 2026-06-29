# Computer Tool Model-Aware Routing v1

## Goal

Make screenshot and control commands honor the selected large-model provider instead of always using local desktop backends.

## Decisions

- Pass `large_model_provider` and optional `network_search_source_model` into `capture_computer_screenshot` and `control_computer_boundary`.
- Reuse `model_driven_tool_strategy_for_current_platform` as the command source of truth.
- For DeepSeek/custom providers, use local Windows/macOS screenshot and input clients.
- For ChatGPT/Codex providers, use Codex bridge contract clients.
- In this desktop build, Codex bridge clients return a clear unconnected-bridge failure rather than falling back to local input.

## Implementation Plan

1. Add tests proving ChatGPT routes to Codex bridge and DeepSeek routes to local computer backends.
2. Add bridge client tests proving unconnected bridge failures are explicit and auditable.
3. Update backend status availability so unconnected Codex bridge routes are not shown as executable.
4. Update Tauri commands and React invoke calls to pass selected provider state.
5. Update documentation and verification.

## Out Of Scope

- Implementing the real Codex bridge runtime.
- Retrying bridge failures through local fallback.
- Local unlock/token gates for high-risk ComputerControl execution.
