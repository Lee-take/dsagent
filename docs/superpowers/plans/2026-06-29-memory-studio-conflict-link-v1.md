# Memory Studio Conflict Link v1

## Goal

Let a reviewer keep both a pending memory candidate and an overlapping long-term memory while recording an explicit relationship between them.

## Scope

- Add an append-only memory-link event instead of rewriting either memory.
- Add a backend path that accepts a pending candidate, writes it as a new long-term memory, and links it to selected existing memories.
- Project linked-memory summaries onto `list_memory_records` so the UI can show the relationship.
- Add a Memory Studio `Link and accept` action for candidates with detected overlaps.

## Non-Goals

- Do not merge memory bodies automatically.
- Do not replace existing long-term memories.
- Do not delete or archive linked memories.
- Do not add a full graph editor.

## Verification

Run the focused link test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
