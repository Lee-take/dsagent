# Local Directory Readiness Export v1

## Goal

Include local-directory setup readiness in exported work packages without leaking user machine paths.

## Decisions

- Add `local_directories` to `tool_readiness`.
- Report only setup booleans for workspace, evidence, and export directories.
- Always mark `paths_redacted=true`.
- Do not serialize workspace, evidence, export, app-data, or settings-file paths.
- Keep legacy work packages compatible with serde defaults.

## Implementation Plan

1. Add failing tests that require local-directory readiness in work-package JSON and reject path leakage.
2. Add `LocalDirectoryReadinessStatus` and `local_directory_readiness_from_state`.
3. Add `local_directories` to `WorkPackageToolReadiness`.
4. Wire export commands to derive readiness from the current app-data directory state.
5. Mirror TypeScript types and update docs.
6. Run Rust tests, frontend build, Tauri build, and diff hygiene checks.

## Out Of Scope

- Exporting actual local paths.
- Importing or applying another machine's local directory settings.
- Syncing directories across machines.
