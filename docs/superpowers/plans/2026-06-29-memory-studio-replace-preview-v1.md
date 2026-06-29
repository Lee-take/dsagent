# Memory Studio Replace Preview v1

## Goal

Let a reviewer inspect what a candidate would replace before any destructive or state-changing memory action exists.

## Scope

- Add a pure-read replace preview model for a pending memory candidate.
- Return the candidate as the replacement draft and selected visible long-term memories as the target list.
- Expose a Tauri command and a Memory Studio `Preview replace` action for candidates with overlaps.
- Keep the candidate pending and leave memory records, deletions, and links unchanged.

## Non-Goals

- Do not accept the candidate.
- Do not delete, tombstone, archive, or replace existing long-term memories.
- Do not save the replacement draft.
- Do not decide the final replace confirmation UX.

## Verification

Run the focused replace-preview test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
