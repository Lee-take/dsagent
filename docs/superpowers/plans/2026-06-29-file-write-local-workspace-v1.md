# FileWrite Local Workspace v1

## Goal

Turn FileWrite from an approval-only boundary into a real local file writer that stays inside the user's configured workspace.

## Decisions

- Keep FileWrite as a high-risk capability governed by the existing policy and approval queue.
- Require path, summary, and content before a write can run.
- Write only UTF-8 content, capped at 512 KiB.
- Resolve relative paths against the configured workspace directory.
- Allow absolute paths only when they stay inside the configured workspace.
- Reject parent-directory traversal such as `../`.
- Before first-run setup, use an app-data `workspace` folder rather than developer-machine paths.

## Implementation Plan

1. Add failing tests for approved workspace writes and outside-workspace rejection.
2. Add `FileWriteRequest.content`, `FileWriteClient`, `FileWriteResult`, and `LocalWorkspaceFileWriteClient`.
3. Update `run_file_write_boundary` to call the client after policy allows or user approval exists.
4. Wire the Tauri command to the configured local workspace.
5. Add a content field and success copy to the React inspector.
6. Update README and session handoff.
7. Run focused FileWrite tests, full Rust tests, frontend build, Tauri build, and diff hygiene checks.

## Out Of Scope

- Binary file writes.
- Patch/diff application.
- Conflict detection or backups.
- Writing outside the configured workspace.
