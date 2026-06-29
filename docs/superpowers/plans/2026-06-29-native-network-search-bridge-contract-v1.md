# Native NetworkSearch Bridge Contract v1

## Goal

Let a large model that supports NetworkSearch use a native bridge contract when a local bridge runtime is configured, without treating plain chat completions as verified web-search evidence.

## Tests First

- Add strategy tests proving ChatGPT can switch to `NativeLargeModel` NetworkSearch when a native bridge is available.
- Add route-status tests proving the UI reports native bridge execution, source-link evidence, and no source-model confirmation gate in that mode.
- Add Codex bridge contract tests proving NetworkSearch requests serialize provider/query/scope and responses require source links.
- Add HTTP/client tests proving native search posts to `/network-search` and maps source links into the existing `NetworkSearch` result boundary.

## Implementation

- Extend the Codex bridge contract with `network_search`, `CodexBridgeNetworkSearchRequest`, `CodexBridgeNetworkSearchItem`, and `CodexBridgeNetworkSearchResponse`.
- Add `CodexBridgeHttpClient::network_search` for the local loopback `/network-search` route.
- Add `CodexBridgeNetworkSearchClient` implementing the existing `NetworkSearchClient` trait.
- Teach model-driven strategy to use `NativeLargeModel` only when the selected provider supports NetworkSearch and the local Codex bridge HTTP runtime is configured.
- Route the Tauri `search_network_boundary` command through either native bridge search or the selected free source-backed adapter.
- Pass the selected large-model provider from React and display `native_bridge_contract` in the inspector.

## Verification

Run focused NetworkSearch tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
