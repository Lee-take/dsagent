# Approval Traceability v1

## Goal

Link recorded critical invocations back to the concrete approval request that authorized them.

## Scope

- Add optional `approval_request_id` to `CapabilityInvocation`.
- Keep legacy invocation events readable with serde defaults.
- Prefer explicit approval IDs when deriving one-shot consumption state.
- Fall back to same-capability timestamp consumption for legacy invocations without an ID.
- Attach approval IDs for EmailSend and ComputerControl pending and approved-retry invocations.

## Test First

1. Add a failing test requiring `CapabilityInvocation.approval_request_id`.
2. Prove an invocation linked to the first approved request consumes only the first grant.
3. Prove a later second approved request remains one-shot available.

## Verification

- Run the focused traceability test.
- Run full Rust tests.
- Run desktop TypeScript build.
- Run `git diff --check`.
