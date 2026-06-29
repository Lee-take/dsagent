# Memory Studio Expiration v1

## Goal

Let accepted long-term memories carry an expiration date and keep expired memories out of normal Memory Studio list/search results without rewriting history.

## Tests First

- Add an event-store test proving expired memories are hidden from `list_memory_records` and `search_memory_records`.
- Add an event-store test proving a future-expiring accepted candidate remains visible and preserves `expires_at`.

## Implementation

- Add `expires_at` to memory candidates, records, and update events.
- Add `MemoryCandidate::new_with_metadata_and_expiration`.
- Require an expiration date when new candidate/update writes use the `expires` lifecycle.
- Preserve legacy events without `expires_at` as readable non-expiring records.
- Add `EventStore::list_memory_records_at` and `EventStore::search_memory_records_at` for deterministic expiration tests.
- Hide only records whose lifecycle is `expires` and whose `expires_at` is at or before the evaluation time.
- Add Memory Studio date inputs for expiring candidates and edited long-term memories.

## Verification

Run focused memory tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
