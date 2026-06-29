# DriveWrite Output Name v1

## Goal

Make successful DriveWrite exports easier to audit by surfacing the actual local output file name in the recorded tool output.

## Scope

- Include the written file name in successful DriveWrite excerpts.
- Keep the durable evidence reference as the full local output path.
- Cover local work-package JSON export with a regression test.

## Non-Goals

- Do not change DriveWrite approval policy.
- Do not add cloud-drive uploads or account connectors.
- Do not change export file naming rules.
- Do not add PDF/font behavior.

## Verification

Run the focused DriveWrite output-name test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
