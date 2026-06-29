# Computer Screenshot Local Capture v1

## Goal

Turn the existing `ComputerScreenshot` permission/audit boundary into a real local screen-pixel capture path for non-bridge providers, while keeping the storage path portable for open-source installs.

## Decisions

- Use `xcap` as the local capture dependency because it supports Windows and macOS and already exposes monitor capture as an RGBA image.
- Keep capture and persistence separated:
  - `LocalScreenshotCaptureBackend` captures pixels and returns PNG bytes plus display metadata.
  - `LocalComputerScreenshotClient` owns evidence-file persistence and `ComputerScreenshot` metadata.
- Save screenshot evidence under the user-selected evidence directory when first-run setup is complete.
- Fall back to the OS app data directory before setup, so the app never depends on a developer-machine path.
- Keep evidence refs relative, for example `computer-screenshots/<uuid>-primary-display.png`.

## Implementation Plan

1. Add tests for local screenshot evidence persistence and invalid empty captures.
2. Add tests for evidence-base path selection.
3. Implement `CapturedScreenshotImage`, `LocalScreenshotCaptureBackend`, and `LocalComputerScreenshotClient::capture_with_backend`.
4. Wire `xcap` into the production backend and encode captures as PNG through `xcap::image`.
5. Update the Tauri command to prefer `LocalDirectorySettings.evidence_dir` with app-data fallback.
6. Update UI copy and documentation.
7. Run Rust, frontend, Tauri, and diff hygiene verification.

## Out Of Scope

- Codex bridge screenshot execution.
- Real mouse/keyboard `ComputerControl` execution.
- Rendering captured screenshot thumbnails in the UI.
- Cloud drive upload or remote evidence storage.
