# Work Package Tool Readiness v1

## Goal

Export a secret-safe tool readiness snapshot inside each work package so a handoff package carries not only chosen backend settings, but also the current execution gates and local readiness flags.

## Scope

- Add a `tool_readiness` section to exported work packages.
- Include `DeepSeekCredentialStatus`, `NetworkSearchRouteStatus`, and `ComputerUseBackendStatus`.
- Preserve legacy package parsing with serde defaults.
- Never serialize `DEEPSEEK_API_KEY` or any key value.
- Do not call the live DeepSeek API.
- Do not capture screen pixels or control mouse/keyboard.

## Test First

1. Add a focused work-package test that builds a package with a fake configured `DEEPSEEK_API_KEY`.
2. Assert the package JSON contains readiness metadata and backend/gate names.
3. Assert the package JSON does not contain the fake key value.
4. Keep older package JSON parsing compatible by defaulting `tool_readiness`.

## Implementation Notes

- Introduce `WorkPackageToolReadiness`.
- Keep the existing `export_work_package` function as a compatibility wrapper.
- Add `export_work_package_with_tool_readiness` for command-level dynamic readiness.
- Have the Tauri `export_work_package` command derive readiness from local env and existing status builders.
- Mirror the new shape in `apps/desktop/src/types.ts`.

## Verification

- Run the focused Rust tests for work-package readiness.
- Run full Rust tests.
- Run the desktop TypeScript build.
- Run `git diff --check`.
