# Operations Briefing HTML Report Export v1

## Goal

Export an Operations Briefing run as a standalone local HTML report in the user's configured export folder, using the existing DriveWrite approval and audit loop.

## Tests First

- Add a workflow renderer test proving HTML reports escape user/evidence content and render summary, anomaly, action, warning, and evidence trace sections.
- Add a file-name test proving HTML report exports use the `.html` extension.

## Implementation

- Add `render_operations_briefing_html_report` and `operations_briefing_html_report_file_name`.
- Add `export_operations_briefing_html_report` as a Tauri command that finds a stored run, renders static HTML, and writes through DriveWrite.
- Register the command in Tauri and expose a React `Export HTML` action next to the Markdown report export.
- Add Chinese and English success, pending-approval, and failure copy.

## Verification

Run focused HTML report tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
