# Operations Briefing Report Export v1

## Goal

Export an Operations Briefing run as a readable Markdown report in the user's local export folder, while preserving the existing DriveWrite approval loop and avoiding hardcoded machine paths.

## Tests First

- Add a workflow renderer test proving a run renders to Markdown with summary, anomaly, action, warning, and evidence trace sections.
- Add a DriveWrite test proving a named Markdown export file can be written after policy allows.
- Add command helper tests proving report export uses the configured export directory and falls back to app data before first-run setup.

## Implementation

- Add `render_operations_briefing_report` and `operations_briefing_report_file_name`.
- Extend DriveWrite with an `export_file` payload in addition to the existing work-package JSON payload.
- Add `export_operations_briefing_report` as a Tauri command that finds a stored run, renders Markdown, and writes through DriveWrite.
- Add a React button next to the existing work-package export action.
- Add sample evidence templates under `docs/templates/operations-briefing-evidence/`.

## Verification

Run focused report/export tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
