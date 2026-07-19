# DS Agent v1.2.0

`v1.2.0` adds the Step 2 GoalEnvelope contract to the stable Windows desktop
app. Package, desktop, Tauri, Cargo, updater, and installer metadata are
`1.2.0` / `v1.2.0`.

## Goal proposal and Kernel authority

- DeepSeek can propose the versioned, bounded
  `ds-agent.goal-envelope-proposal/v1` contract: user goal, assumptions,
  constraints, `done_when`, required artifacts, verifiers, proposed
  capabilities, external targets, and stop conditions.
- A proposal does not grant a capability, trust a path or account, approve an
  action, execute a tool, produce verifier authority, or declare completion.
- The local Kernel validates proposed capability names against its tool catalog,
  readiness, risk/effect policy, verifier availability, workspace state, and
  locally bound targets. It alone creates the normalized revision, validation
  receipt, frozen fingerprint, and durable lifecycle projection.
- Unknown fields, unsupported versions, unbounded or duplicate values,
  secret-like content, unsafe local references, unbound targets, missing
  verifier coverage, and unavailable local authority fail closed with stable
  codes.

## Read-only UI and verified completion

- Chat can show a read-only goal projection with the bounded goal summary,
  proposed/blocked/validated/frozen/verification-blocked/complete state, stable
  reason codes, revision/fingerprint, and verifier/artifact counts.
- The frontend cannot deserialize or write validation context, bound authority,
  frozen receipts, evidence, or completion authority. Model-supplied projection
  and completion fields are ignored.
- Completion requires locally authoritative passing evidence bound to the exact
  frozen goal ID, revision, fingerprint, verifier, `done_when`, evidence kind,
  and required artifact identity. Missing, failed, stale, wrong-goal,
  wrong-revision, wrong-fingerprint, unknown, duplicate, mismatched, or
  incomplete evidence remains verification-blocked.
- Model narrative, UI booleans, approval state, tool result prose, or the mere
  existence of an artifact cannot complete a goal.

## Compatibility and migration

- Existing v1.1.0 onboarding, DPAPI credential, workspace, conversation,
  connector-vault, and isolated-profile behavior is preserved.
- Existing SQLite databases gain compatible GoalEnvelope lifecycle and
  completion projection tables through additive creation. Legacy databases
  default to no goal and not complete; no historical event or artifact is
  promoted into goal authority.
- Duplicate proposal, freeze, evidence, and completion processing is
  deterministic and restart-safe. A changed frozen revision or fingerprint
  invalidates earlier completion evidence.
- Goal events, receipts, ordinary DTOs, UI projections, and work packages are
  guarded against secret, provider-body, absolute app-data/vault path, local
  authority, and internal claim-token leakage.

## Windows download and integrity

> **Unsigned release:** both `ds-agent.exe` and `DS.Agent_1.2.0_x64-setup.exe` are intentionally Authenticode `NotSigned`.
> Windows may display `Unknown publisher` or a Microsoft Defender SmartScreen
> warning. This is not a signed or SignPath-approved release.

Download only over HTTPS from the official
[`v1.2.0` GitHub Release](https://github.com/Lee-take/dsagent/releases/tag/v1.2.0).
Before running the installer, compare its filename, product version, exact byte
size, and SHA-256 with the values in that Release. The final Release body binds
the sole x64 installer asset to the exact source commit and immutable annotated
tag after exact-main build and fresh-download readback.

The SignPath Foundation application remains submitted and approval is pending.
If the project is accepted later, signing starts only with a subsequent new
version; the immutable v1.1.0 and v1.2.0 tags, Releases, and assets will not be
moved, overwritten, or replaced.

## Scope and limits

This release is limited to C2A/C2B/C2C GoalEnvelope proposal,
Kernel validation/freeze, compatible persistence, read-only UI projection, and
the verified-evidence completion gate. It does not add Step 3 grouped
authorization, an Office executor, a connector or Computer Use expansion, a
new verifier runtime, production external accounts, or external-write
authority. Production Microsoft/Google account registration and live
mail/calendar writes remain disabled. DS Agent remains an independent
open-source project, not an official DeepSeek product.

Release verification covers focused and full tests, production frontend build,
Rust formatting, migration/restart/idempotence and fail-closed evidence
regressions, release-source and secret scans, isolated candidate/installed UI
and workflow smoke, PR CI, exact-main CI, and final
version/filename/byte-size/SHA-256/Auth/source/tag/Release/Latest/fresh-download
readback. The cancelled formal 20-run Windows lab is not claimed.

## Deterministic briefing templates

`docs/templates/operations-briefing-smoke-evidence` remains deterministic local
test material. The bundled smoke files are marked as
`SMOKE SAMPLE evidence for local verification only` and
`Replace before operational use`; replace them before operational use. The
desktop seed action continues to use the blank operator templates under
`docs/templates/operations-briefing-evidence`.
