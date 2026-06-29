# Memory Studio Conflict Details v1

## Goal

Make Memory Studio candidate conflicts inspectable before the user accepts or rejects a candidate, without deciding the later merge/replace UX.

## Tests First

- Extend the event-store conflict surfacing test to require conflict summaries, not only conflicting memory IDs.
- Verify the focused test fails before the summary field exists.
- Implement the smallest model/event-store change needed for the test to pass.

## Implementation

- Add `MemoryConflictSummary` as a compact serializable snapshot of a visible conflicting long-term memory.
- Keep `conflicting_memory_ids` for compatibility and add `conflicting_memories` with serde defaults for older payloads.
- Populate conflict summaries from the same visible-memory set used by the existing conflict detector.
- Extend TypeScript types and render overlapping memory title, body, metadata, and updated timestamp under each candidate conflict warning.

## Non-goals

- Do not automatically merge memories.
- Do not replace accepted memory records from candidate review.
- Do not add a new conflict-resolution event type in this slice.

## Verification

Run the focused Memory Studio conflict test, full Rust tests, desktop TypeScript/Vite build, Tauri debug build, and `git diff --check`.
