# Memory Studio Delete v1

## Goal

Allow an operator to remove an accepted long-term memory from normal Memory Studio list/search results without rewriting or deleting the original append-only event.

## Tests First

- Add an event-store test proving deleting a memory appends a tombstone and hides the memory from `list_memory_records` and `search_memory_records`.
- Add an event-store test proving deleting a missing memory returns `NotFound`.

## Implementation

- Add `MemoryRecordDeletion` and the `memory_record.deleted` event type.
- Add `EventStore::delete_memory_record` and `EventStore::list_memory_record_deletions`.
- Filter deleted memory IDs from memory listing and search.
- Add the `delete_memory_record` Tauri command.
- Add a Memory Studio delete action for long-term memory rows.

## Verification

Run focused deletion tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
