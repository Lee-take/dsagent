# Approval ID Tool Output v1

## Goal

Surface linked approval request IDs in recent tool output so operators can trace an invocation back to the exact approval record without opening raw event JSON.

## Scope

- Keep `approval_request_id` optional and nullable.
- Show the approval ID only when present.
- Reuse the existing recent tool output footer.
- Add compatibility tests for invocation JSON with and without `approval_request_id`.

## Verification

- Run focused capability invocation serialization tests.
- Run full Rust tests.
- Run desktop TypeScript build.
- Run `git diff --check`.
