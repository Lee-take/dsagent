# Memory Studio Edit v1

## Goal

Allow an operator to revise an accepted long-term memory from Memory Studio while preserving the append-only memory audit trail.

## Tests First

- Add an event-store test proving an update event replaces the visible memory version in `list_memory_records` and `search_memory_records`.
- Add an event-store test proving a deleted memory cannot be updated.

## Implementation

- Add `MemoryRecordUpdate` and the `memory_record.updated` event type.
- Add `EventStore::update_memory_record` and `EventStore::list_memory_record_updates`.
- Merge the latest update event into each visible memory record while preserving original `id`, `source`, `source_id`, and `created_at`.
- Keep deletion tombstones authoritative: deleted memories remain hidden and cannot be updated.
- Add the `update_memory_record` Tauri command.
- Add Memory Studio edit/save/cancel actions for long-term memory rows.

## Verification

Run focused memory-record tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
