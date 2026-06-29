# Computer Use OS Permission Readiness v1

## Goal

Make local Windows/macOS Computer Use prerequisites visible before a user tries screen capture or real input control, and keep macOS packaging separate from Windows installer choices.

## Decisions

- Extend `ComputerUseBackendStatus` instead of hardcoding OS notes in React.
- Keep status checks side-effect free: no screen capture and no mouse/keyboard control.
- Report macOS Screen Recording for local screenshot capture.
- Report macOS Accessibility for local mouse/keyboard control.
- Report Windows foreground-desktop and secure-desktop limitations without claiming every window can be controlled.
- Add serde defaults for new status fields so older exported tool-readiness JSON remains readable.
- Add a separate `tauri.macos.conf.json` for `.app` and `.dmg` packaging.

## Implementation Plan

1. Add failing tests for macOS permission notes, Windows foreground-desktop notes, and legacy status JSON compatibility.
2. Add structured permission fields to `ComputerUseBackendStatus`.
3. Surface the fields in TypeScript and the runtime inspector.
4. Add macOS platform bundling config.
5. Update README and session handoff notes.
6. Run Rust tests, frontend build, Tauri Windows debug build, and diff hygiene checks.

## Out Of Scope

- Verifying a macOS build on a Windows host.
- Requesting macOS TCC permissions programmatically.
- OS credential or biometric prompts.
- Real Codex bridge runtime connection.
