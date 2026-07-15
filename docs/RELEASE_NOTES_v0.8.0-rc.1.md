# DS Agent v0.8.0-rc.1 Release Notes

Status: Windows-first release candidate for update testing. The public stable
release remains `v0.5.0` until this candidate has passed real installed-app
update verification.

Repository: https://github.com/Lee-take/dsagent

## Versioning Note

The v0.6 Automation connector and v0.7 Artifact Engine names were internal
roadmap milestones. Their completed capabilities were already consolidated into
the public `v0.5.0` release, so this project does not create retroactive v0.6 or
v0.7 tags. `v0.8.0-rc.1` is the next public prerelease.

## Durable Verified Computer Use

This candidate adds the first complete Windows-first Durable Verified Computer
Use step. It implements one evidence-driven vertical loop:

`observe -> approve -> revalidate -> record ActionStarted -> act once -> observe -> verify`

- Sessions and steps are persisted in SQLite with revision compare-and-swap,
  bounded recovery and malformed-row quarantine.
- The exact action, one-shot approval, stable window identity, title hash,
  accessibility target, semantic state and checkpoint are bound together.
- DS Agent persists `ActionStarted` before the external input effect. If the app
  restarts across that boundary, the step becomes `EffectUnknown` and is never
  replayed automatically.
- The foreground window and target are revalidated immediately before acting.
  A changed or stale binding blocks the input with zero control calls.
- Post-action screenshot and UI Automation evidence are captured automatically.
  Screenshot-only evidence cannot claim semantic verification; the declared
  deterministic postcondition must pass.
- User takeover stops subsequent control, records durable state and releases the
  shared desktop resource. Re-observation is required before continuing.
- The right rail exposes safe controls for create, bind, approve and run,
  takeover, re-observe and cancel without exposing raw private evidence.
- Local loopback bridge requests bypass system proxies so local capability calls
  cannot be diverted through an unrelated proxy configuration.

## Safety And Privacy Boundaries

- The supported RC scenario is an isolated, low-risk Notepad-like Windows app.
- Secure/UAC desktops, privileged targets, managed browser login state,
  cross-application coverage and general undo are not claimed.
- Raw screenshots, typed text, window titles, accessibility text and sensitive
  local paths stay local. Public UI data contains bounded summaries and
  fingerprints only.
- DeepSeek may propose content and one exact action. DS Agent owns schema and
  policy validation, authorization, execution, evidence, verification, recovery
  and replay prevention.

## Update Test Notes

- The intended test path is an installed public `v0.5.0` client updating to
  `v0.8.0-rc.1` through DS Agent's built-in updater.
- The Windows asset is `DS.Agent_0.8.0_x64-setup.exe`.
- Existing workspace configuration and durable local state should remain in the
  OS app-data and workspace locations rather than the install directory.
- The installer is unsigned, so Windows may display an unknown-publisher
  warning.
- Testers should confirm update detection, download, silent install, restart,
  displayed version and preservation of existing settings/state before this RC
  is promoted to `v0.8.0`.

## Validation

The implementation completed the production frontend build, the full desktop
test suite, secret scan, source-only release guard, Rust formatting and diff
checks. The pre-release implementation baseline recorded 778 passing Rust
tests, zero failures and seven environment-only ignored tests. Focused Computer
Use tests, repeated Windows screenshot capture and an isolated Notepad-like
act-once/verify smoke also passed. The release candidate is published only after
the versioned release gate, remote CI and installer build complete successfully.

Operations Briefing live smoke tests use
`docs/templates/operations-briefing-smoke-evidence` by default. The bundled
smoke files are marked as `SMOKE SAMPLE evidence for local verification only`
and `Replace before operational use`. The desktop seed action continues to use
blank operator templates under `docs/templates/operations-briefing-evidence`,
not smoke or business data.

Bumps the package, desktop, Tauri and Cargo metadata to `0.8.0`, while the
updater identity is `v0.8.0-rc.1`, so installed Windows clients can detect and
test this release candidate as newer than `v0.5.0`.

## Installer Integrity

SHA-256: `FBC08E49CCACEBFF725AFC4B2994C64B8AEAD051099B7EF1AE7A6DE132659139`
