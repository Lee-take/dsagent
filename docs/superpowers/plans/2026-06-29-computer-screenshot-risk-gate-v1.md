# Computer Screenshot Risk Gate v1

## Goal

Align `ComputerScreenshot` risk policy with future real screen-pixel capture.

This slice should:

- Reclassify `ComputerScreenshot` from low risk to medium risk.
- Make `ask_on_risk` require approval before screen inspection.
- Keep `limited_auto` able to run medium-risk screenshot reads after policy evaluation.
- Preserve `full_access` behavior for non-critical capabilities.

## TDD Plan

1. Add a failing policy test showing `ask_on_risk` must ask for `ComputerScreenshot`.
2. Raise `ComputerScreenshot` risk to medium.
3. Update screenshot boundary tests to use `limited_auto` for the successful auto-run path and add/keep pending approval coverage.
4. Update docs and run focused/full verification.

## Non-goals

- Do not implement real OS pixel capture in this slice.
- Do not change `ComputerControl` policy.
- Do not alter NetworkSearch route behavior.
