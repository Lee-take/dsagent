# Computer Use Backend Status v1

## Goal

Expose a runtime status contract for the confirmed Computer Use backend direction:

- `ComputerScreenshot` uses the Codex-style screen pixel capture backend.
- `ComputerControl` uses the Codex-style mouse and keyboard input backend.
- The current local app should say whether each backend is actually available.
- `ComputerControl` must remain visibly gated by explicit approval.

This slice does not implement real screen capture or mouse/keyboard execution.

## TDD Plan

1. Add a focused Rust test for the default Computer Use backend status.
2. Run the focused test and confirm it fails before implementation.
3. Implement a small kernel status model and Tauri command.
4. Add TypeScript types, fallback state, invoke loading, and backend inspector rows.
5. Update README and session handoff notes.
6. Verify with focused Rust tests, full Rust tests, frontend build, and diff check.

## Expected Runtime Status

- Screenshot backend: `codex_style_screen_capture`
- Screenshot available: `false`
- Control backend: `codex_style_input_control`
- Control available: `false`
- Control requires approval: `true`

## Non-goals

- Capturing real screen pixels.
- Moving the mouse.
- Typing keys.
- Storing any credential or screen content.
