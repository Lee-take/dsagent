# Capability Grant State v1

## Goal

Make approval availability visible instead of only showing whether an approval record was resolved.

## Scope

- Add a derived `grant_state` to capability access records.
- Keep persisted event payloads unchanged.
- Show reusable grants for non-critical explicit approvals.
- Show one-shot available and consumed states for critical approvals.
- Hide `not_granted` in the capability card UI to keep the inspector compact.

## Test First

1. Extend the existing browser approval reuse test to expect `grant_state=reusable`.
2. Extend the critical approval test to expect `one_shot_available` before invocation and `one_shot_consumed` after invocation.
3. Keep `has_user_approved_capability` driven by the same derived grant state.

## Verification

- Run focused grant-state tests.
- Run full Rust tests.
- Run desktop TypeScript build.
- Run `git diff --check`.
