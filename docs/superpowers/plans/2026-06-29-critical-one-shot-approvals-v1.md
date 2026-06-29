# Critical One-Shot Approvals v1

## Goal

Prevent critical capabilities from becoming permanent reusable grants while keeping the existing approve-and-retry workflow usable.

## Scope

- Apply to `EmailSend` and `ComputerControl`.
- Keep non-critical explicit approvals reusable where current behavior already supports it.
- Treat an approved critical request as available only until the next invocation for the same capability is recorded.
- Preserve append-only audit history.
- Do not enable real email sending or real desktop control.

## Test First

1. Add an event-store test proving a critical approval is available immediately after approval.
2. Record a same-capability invocation after the approval timestamp.
3. Assert the same approval is no longer treated as available.
4. Keep the existing browser approval reuse test passing.

## Verification

- Run the new critical approval test.
- Run the existing reusable browser approval test.
- Run full Rust tests.
- Run the desktop TypeScript build.
