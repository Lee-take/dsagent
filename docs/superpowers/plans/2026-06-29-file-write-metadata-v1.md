# FileWrite Metadata v1

## Goal

Make successful FileWrite results more inspectable by exposing UTF-8 text encoding and byte-count metadata in the recorded tool output.

## Scope

- Add encoding metadata to `FileWriteResult` with a serde default for legacy records.
- Populate local workspace FileWrite results with real byte count and `utf-8` encoding.
- Include compact encoding and byte-count metadata in successful FileWrite excerpts shown by the inspector.

## Non-Goals

- Do not add binary file writes.
- Do not expand FileWrite beyond the configured local workspace.
- Do not change FileWrite approval policy.
- Do not change file size limits.

## Verification

Run the focused FileWrite metadata test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
