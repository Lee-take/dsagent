# TerminalRead Exit Code Coverage v1

## Goal

Lock the TerminalRead audit behavior for commands that start successfully but exit with a nonzero code.

## Scope

- Add regression coverage for an allowlisted TerminalRead command returning a nonzero exit code.
- Confirm the capability invocation is recorded as failed.
- Confirm stderr remains visible in the recorded excerpt and the exit code appears in warnings.

## Non-Goals

- Do not expand the TerminalRead allowlist.
- Do not add arbitrary shell input.
- Do not enable TerminalWrite execution.

## Verification

Run the focused TerminalRead exit-code test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
