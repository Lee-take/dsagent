# Memory Studio Merge Preview v1

## Goal

Let a reviewer inspect a proposed merged-memory draft for a pending candidate and overlapping long-term memories without writing, replacing, deleting, or linking anything.

## Scope

- Add a pure-read merge preview model for a pending memory candidate.
- Build the draft from selected visible long-term memory bodies plus the candidate body, with duplicate bodies removed.
- Preserve the candidate metadata on the preview so reviewers can see the proposed type, scope, sensitivity, lifecycle, and expiration.
- Expose a Tauri command and a Memory Studio `Preview merge` action for candidates with overlaps.

## Non-Goals

- Do not save the merged draft.
- Do not mark the candidate accepted or rejected.
- Do not edit, delete, replace, or link existing long-term memories.
- Do not decide the final merge/replace UX.

## Verification

Run the focused merge-preview test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
