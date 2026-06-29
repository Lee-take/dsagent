# DeepSeek Pricing Config v1

## Goal

Populate DeepSeek telemetry cost estimates from a local user-configured pricing table instead of hardcoded public prices.

## Tests First

- Add pricing-module tests for missing settings, save/load round-trip, decimal validation, matching-model cost estimates, and missing/disabled rates.
- Add a command-layer test proving telemetry receives `estimated_cost_micro_usd` before it is appended when matching pricing is configured.

## Implementation

- Add `deepseek-pricing.json` under the OS app data directory, separate from local directory settings.
- Store manual USD / 1M token prices for Flash input, Flash output, Pro input, and Pro output.
- Parse prices as decimals into integer micro-USD values to avoid floating-point cost math.
- Estimate cost only when pricing is enabled, the model matches a configured rate, and DeepSeek usage includes prompt/completion token counts.
- Expose `get_deepseek_pricing_state` and `save_deepseek_pricing_settings` Tauri commands.
- Add a desktop pricing panel next to first-run local setup and show formatted cost in the latest DeepSeek telemetry line.

## Non-goals

- Do not fetch live DeepSeek prices.
- Do not hardcode default prices.
- Do not block Operations Briefing runs if pricing settings are absent; telemetry simply keeps cost empty.

## Verification

Run focused pricing tests, full Rust tests, desktop TypeScript/Vite build, Tauri debug build, and `git diff --check`.
