# Text Evidence Metadata Legacy Compatibility v1

## Goal

Lock legacy JSON compatibility for text evidence metadata added to DriveRead and evidence-folder ingestion.

## Scope

- Add regression coverage for legacy `EvidenceFolderFile` JSON without an `encoding` field.
- Add regression coverage for legacy `DriveFolderEntry` JSON without an `encoding` field.
- Confirm both deserialize with `utf-8` defaults so old events and work packages remain readable.

## Non-Goals

- Do not add new metadata fields.
- Do not change export/import behavior.
- Do not alter permission policy or local folder limits.

## Verification

Run focused legacy metadata tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
