# Operations Briefing PDF Report Export v1

## Goal

Export an Operations Briefing run as a local PDF report in the user's configured export folder, using the existing DriveWrite approval and audit loop.

## Tests First

- Add a DriveWrite test proving binary export bytes can be written after policy allows.
- Add a workflow renderer test proving PDF report bytes start with a valid PDF header, include report content, end with `%%EOF`, and use the `.pdf` file extension.

## Implementation

- Extend `DriveWriteExportFile` with optional `content_base64` so binary exports can use the same permission boundary as text exports.
- Add `render_operations_briefing_pdf_report` and `operations_briefing_pdf_report_file_name`.
- Add `export_operations_briefing_pdf_report` as a Tauri command that finds a stored run, renders PDF bytes, and writes through DriveWrite.
- Register the command in Tauri and expose a React `Export PDF` action next to the Markdown and HTML report exports.
- Add Chinese and English success, pending-approval, and failure copy.

## Verification

Run focused PDF/binary export tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
