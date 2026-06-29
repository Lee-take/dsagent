# ComputerControl Structured Action v1

## Goal

Move `ComputerControl` from an approval/audit-only boundary to an approval-gated local input executor without allowing arbitrary natural-language desktop control.

## Decisions

- Keep the Tauri command shape compatible: the UI still sends `target` and `action` strings.
- Treat `action` as a strict mini protocol, not free text:
  - `click:x,y[,left|middle|right]`
  - `move:x,y`
  - `type:text`
  - `press:key`
  - `hotkey:key+key`
  - `scroll:delta[,vertical|horizontal]`
- Reject natural-language actions before policy execution.
- Preserve the existing critical-risk policy and one-shot approval behavior.
- Use `enigo` for local mouse/keyboard input simulation.
- Keep Codex bridge execution out of this slice; model strategy can select it, but no bridge call is wired yet.

## Implementation Plan

1. Add red tests for structured action parsing and natural-language rejection.
2. Add red tests proving pending approval does not call the executor and approved actions do.
3. Add red tests for local client translation: click move/click order and hotkey reverse release order.
4. Implement `ComputerControlAction`, `ComputerControlClient`, and `LocalComputerControlClient`.
5. Add `EnigoLocalComputerControlInputBackend` and map normalized keys/buttons/axes to `enigo`.
6. Update Tauri command wiring, UI copy, and documentation.
7. Run Rust, frontend, Tauri, and diff hygiene verification.

## Out Of Scope

- Codex bridge control execution.
- Local password/unlock and local agent token gates.
- Visual target recognition from screenshots.
- Multi-step planning over desktop state.
