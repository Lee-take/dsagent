# FileRead Metadata v1

## Goal

Make successful FileRead results more inspectable by exposing text encoding and byte count in the recorded tool output.

## Scope

- Add byte count and encoding metadata to `FileContent`.
- Populate local UTF-8 text reads with real file size and `utf-8` encoding.
- Include a compact metadata warning on successful FileRead invocations so the inspector shows what kind of file was read.

## Non-Goals

- Do not parse binary, PDF, or Office files.
- Do not expand FileRead beyond the existing low-risk permission boundary.
- Do not change file size limits.

## Verification

Run the focused FileRead test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
