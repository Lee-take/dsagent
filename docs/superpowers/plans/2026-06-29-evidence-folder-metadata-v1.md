# EvidenceFolder Metadata v1

## Goal

Make evidence-folder ingestion results more inspectable by exposing UTF-8 text encoding and byte-count metadata for each accepted local evidence file.

## Scope

- Add encoding metadata to `EvidenceFolderFile` with a serde default for legacy records.
- Populate local evidence-folder reads with real file size and `utf-8` encoding.
- Include compact encoding and byte-count metadata in successful evidence-folder excerpts shown by the inspector.

## Non-Goals

- Do not recurse into nested folders.
- Do not parse binary, PDF, or Office files.
- Do not change evidence-folder permission policy.
- Do not change file count or file size limits.

## Verification

Run the focused evidence-folder metadata test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
