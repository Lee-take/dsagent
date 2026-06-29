# Computer Screenshot Evidence Ref v1

## Goal

Make successful `ComputerScreenshot` invocations point at durable screenshot evidence instead of using the display label as the evidence reference.

This slice should:

- Add an explicit `evidence_ref` field to `ComputerScreenshot`.
- Preserve display labels for human-readable titles.
- Use `evidence_ref` for successful `CapabilityInvocation.evidence_ref`.
- Keep pending/failure behavior unchanged.

## TDD Plan

1. Add a failing assertion that successful screenshot invocations return a saved evidence reference.
2. Add `ComputerScreenshot.evidence_ref` and update fake screenshot client.
3. Update `run_computer_screenshot` to use the evidence ref.
4. Update docs and run focused/full verification.

## Non-goals

- Do not capture real OS pixels in this slice.
- Do not add a screenshot crate yet.
- Do not show screenshot images in the UI yet.
