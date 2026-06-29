# Open Source Release Plan

This document freezes the v0.1-alpha release goal so the project can become a
credible GitHub open-source baseline for DeepSeek-first desktop agent support.

## Release Goal

Ship a buildable local-first desktop Agent OS alpha that demonstrates:

- DeepSeek-first model routing, credential readiness, cache telemetry, and manual
  pricing configuration;
- permissioned local tools with audit records;
- Memory Studio with explicit review and conflict actions;
- an Operations Briefing workflow pack using local evidence and DeepSeek
  synthesis when configured;
- Windows debug installer packaging and source build instructions.

## Non-Goals Before v0.1-alpha

- No new workflow packs.
- No new model providers beyond the existing abstraction.
- No real email connector.
- No real cloud-drive connector.
- No managed Codex bridge sidecar.
- No PDF v2 CJK font work.
- No broader Computer Use automation.
- No arbitrary third-party executable plugin system.

## Required Before Public GitHub Release

- Confirm Apache-2.0 license metadata is present in the repository.
- Confirm repository visibility and project owner name.
- Publish v0.1-alpha as source-only unless the maintainer later approves
  unsigned installer artifacts explicitly.
- Run final verification on the release branch.
- Prepare release notes that call out alpha limits plainly.

## Release Hygiene Artifacts

- `CONTRIBUTING.md` explains the v0.1-alpha feature freeze.
- `SECURITY.md` documents current security boundaries and private reporting
  expectations.
- `.github/pull_request_template.md` keeps PRs scoped to existing alpha work.
- `.github/ISSUE_TEMPLATE/bug_report.yml` and
  `.github/ISSUE_TEMPLATE/deepseek_compatibility.yml` collect useful reports
  without encouraging new feature requests during the freeze.
- `.github/workflows/ci.yml` verifies the desktop frontend build and Rust tests
  on Windows without requiring secrets.
- `.env.example` documents local DeepSeek and external bridge environment
  variables without storing secret values.
- `docs/RELEASE_NOTES_v0.1-alpha.md` is the draft release note source.

## Recommended License Discussion

The project uses Apache-2.0. This was chosen for infrastructure-style open
source and its explicit patent grant.

## Alpha Honesty Rules

- Do not imply official DeepSeek affiliation.
- Do not claim live web evidence from plain chat-completion text.
- Do not claim cloud connectors where the implementation is local-folder or
  approval-boundary only.
- Do not hide high-risk Computer Use limitations.
- Do not add feature work during release cleanup.
