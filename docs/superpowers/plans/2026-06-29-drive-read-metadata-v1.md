# DriveRead Metadata v1

## Goal

Make local DriveRead results more inspectable by exposing UTF-8 text encoding and byte-count metadata for each matching evidence file.

## Scope

- Add encoding metadata to `DriveFolderEntry` with a serde default for legacy records.
- Populate local DriveRead matches with real file size and `utf-8` encoding.
- Include compact encoding and byte-count metadata in successful DriveRead excerpts shown by the inspector.

## Non-Goals

- Do not add cloud drive connectors.
- Do not parse binary, PDF, or Office files.
- Do not expand DriveRead beyond the existing local-folder permission boundary.
- Do not change file count or file size limits.

## Verification

Run the focused DriveRead metadata test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
