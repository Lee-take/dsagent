# Memory Studio Conflict Surfacing v1

## Goal

Surface likely overlap between pending memory candidates and existing visible long-term memories so an operator can review before accepting.

## Tests First

- Add an event-store test proving a candidate with the same title as an existing memory includes that memory ID as a conflict.
- Add an event-store test proving deleted memories are ignored by conflict surfacing.

## Implementation

- Add `conflicting_memory_ids` to `MemoryCandidateRecord` with a serde default for legacy payload compatibility.
- Compute conflicts from visible long-term memories when listing candidate records.
- Treat same normalized title as a conflict.
- Treat same type/scope plus exact or clear body containment as a conflict.
- Ignore the memory created from the same accepted candidate.
- Show a compact candidate-row conflict warning in Memory Studio without blocking accept/reject actions.

## Verification

Run focused memory-candidate tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
