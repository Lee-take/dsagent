# ComputerControl Local Unlock Token v1

## Goal

Add a short-lived local unlock gate around approved `ComputerControl` execution so one-shot approval does not immediately become live mouse/keyboard control without a present local operator.

## Decisions

- Keep one-shot approval as the policy source of truth.
- Generate a short local challenge code when the desktop app starts.
- Store unlock state only in memory; do not persist it into events, settings, or exported work packages.
- Require the local unlock only after a `ComputerControl` approval is available.
- Check the unlock before executor calls and before appending a `CapabilityInvocation`, so failed unlock attempts do not consume one-shot approval.
- Use a five-minute unlock window for the first implementation.

## Implementation Plan

1. Add red tests for initial locked state, wrong-token rejection, five-minute expiry, and approval-only unlock requirement.
2. Implement `ComputerControlUnlockState`, `ComputerControlUnlockStatus`, token normalization, and expiry checks.
3. Add `get_computer_control_unlock_status` and `unlock_computer_control` Tauri commands.
4. Gate `control_computer_boundary` after approval lookup and before execution/event append.
5. Add React state, local unlock UI, Chinese/English copy, and responsive styling.
6. Update README and session handoff notes.
7. Run Rust tests, frontend build, Tauri build, and diff hygiene checks.

## Out Of Scope

- Persistent OS credential prompts.
- Biometric unlock.
- A real Codex bridge runtime.
- Visual target selection or multi-step desktop planning.
