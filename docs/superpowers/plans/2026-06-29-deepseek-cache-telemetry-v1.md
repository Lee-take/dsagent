# DeepSeek Cache And Telemetry v1

## Goal

Add a secret-safe local request cache and telemetry trail for DeepSeek Chat usage in Operations Briefing synthesis.

## Tests First

- Add a DeepSeek executor test proving a first request records a cache miss and the same request later records a cache hit without calling the transport again.
- Add a DeepSeek telemetry test proving telemetry stores a request hash, model, cache status, elapsed time, and token counts without serializing prompt text or API keys.
- Add an event-store test proving DeepSeek chat telemetry is appended and listed through the append-only event log.

## Implementation

- Add `DeepSeekChatCompletionUsage`, cache-status, telemetry, and execution envelope types to the DeepSeek kernel module.
- Add a `DeepSeekChatCompletionCache` trait plus an in-memory cache owned by `AppState`.
- Hash serialized chat requests with SHA-256 for cache keys and telemetry IDs without storing prompt text in the telemetry event.
- Route Operations Briefing DeepSeek synthesis through the cached executor when a cache is available.
- Append `deepseek_chat.telemetry_recorded` events after workflow runs and expose `list_deepseek_chat_telemetry` to the desktop UI.
- Show the latest DeepSeek telemetry in the Tool Backend Strategy inspector with cache status, elapsed milliseconds, and total token count when the provider returns usage.

## Non-goals

- Do not persist the response cache across app restarts in v1.
- Do not hardcode DeepSeek pricing. `estimated_cost_micro_usd` stays empty until a pricing source or user-configured cost table is confirmed.
- Do not treat DeepSeek Chat output as verified NetworkSearch evidence.

## Verification

Run focused DeepSeek/event-store tests, full Rust tests, desktop TypeScript/Vite build, Tauri debug build, and `git diff --check`.
