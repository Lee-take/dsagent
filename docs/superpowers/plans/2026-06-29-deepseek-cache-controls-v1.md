# DeepSeek Cache Controls v1

## Goal

Make the in-session DeepSeek Chat request cache visible and manually clearable
from the desktop UI.

## Tests First

- Add a focused cache test proving `clear()` returns the number of removed
  entries and leaves the cache empty.
- Verify the test fails before `clear()` exists.

## Implementation

- Add `DeepSeekChatCacheState` with the current cache entry count.
- Add `DeepSeekMemoryChatCompletionCache::state()` and `clear()`.
- Expose `get_deepseek_chat_cache_state` and `clear_deepseek_chat_cache` Tauri
  commands.
- Load cache state at startup, refresh it after Operations Briefing runs, and
  show a clear-cache button in the Tool Backend Strategy inspector.

## Non-goals

- Do not persist the cache across app restarts.
- Do not add TTL or eviction policy in this slice.
- Do not clear telemetry events; cache clearing only removes in-memory cached
  responses.

## Verification

Run the focused cache test, full Rust tests, desktop TypeScript/Vite build,
Tauri debug build, and `git diff --check`.
