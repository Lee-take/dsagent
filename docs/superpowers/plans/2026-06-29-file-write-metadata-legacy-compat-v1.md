# FileWrite Metadata Legacy Compatibility v1

## Goal

Lock legacy JSON compatibility for FileWrite metadata after adding UTF-8 encoding to successful local workspace writes.

## Scope

- Add regression coverage for legacy `FileWriteResult` JSON without an `encoding` field.
- Confirm old payloads deserialize with the `utf-8` default.

## Non-Goals

- Do not add new FileWrite behavior.
- Do not change workspace path validation.
- Do not change approval policy or write limits.

## Verification

Run the focused FileWrite legacy metadata test, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
