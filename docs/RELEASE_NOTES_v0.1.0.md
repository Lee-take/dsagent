# DeepSeek Agent OS v0.1.0 Candidate Notes

Status: Windows-first source-tree test candidate. The code is ready to upload
for maintainer testing, but no new GitHub tag or release should be created
until the maintainer finishes manual testing and explicitly resumes publication.

Packaging: source-first release candidate. No public installer binaries are
attached for this preview unless the maintainer later explicitly approves
unsigned binary distribution for a specific release.

Maintainer handoff notes, decision logs, and internal planning files are kept as
local-only continuation material and are intentionally excluded from public
source snapshots.

## Positioning

DeepSeek Agent OS is packaged locally as DS Agent. It is an independent open-source desktop project
written to help colleagues use DeepSeek large language models more conveniently
in daily work.

The preview uses a harness architecture with loop engineering implemented in
code: permissioned tool boundaries, append-only audit events, selective context
assembly, source-linked evidence, bounded workflow runs, and token-efficient
DeepSeek routing.

In this preview, context receipts show loop mode, workflow policy, selected
evidence, memory, route, token/cache state, validation results, and intentional
omissions; bounded repair loops keep failed-step retries small so longer work
can stay reviewable without loading the full conversation into every model
request.

Markdown and HTML report exports carry the same context receipt summary.

This project is not an official DeepSeek product, is not affiliated with
DeepSeek, and does not claim any DeepSeek ownership, authorization, or
endorsement. The DeepSeek name is used only to describe compatibility and
support for DeepSeek models.

## Why 0.1.0

The project is not complete. `0.1.0` is intentionally defined as a
Windows-first test candidate so the source tree can move forward without
overstating product maturity. The Windows build/install/launch/run path has
been locally validated through the repeatable release gate and installed UI
workflow smoke.

After the Windows preview continues to pass local release gates, the next
platform target is macOS. The repository already contains a macOS Tauri
packaging config, but macOS validation and release work will follow after the
Windows preview continues to pass local release gates.

## Basic Functions In This Preview

- Tauri + React + TypeScript + Rust desktop shell.
- Local-first workspace setup for a workspace folder, evidence folder, and
  export folder.
- DeepSeek route readiness through a local `DEEPSEEK_API_KEY` environment
  variable without storing or showing the key value.
- Optional local DeepSeek smoke tests for Chat Completions and Operations
  Briefing synthesis.
- Permissioned tool surfaces for file, network, browser, terminal,
  local-folder read/export, email read/draft/send approval records, and
  Computer Use operations.
- Append-only local audit records for access requests, approvals, tool
  attempts, workflow runs, memory records, and work packages.
- Computer Use remains experimental and high-risk: screen capture follows the
  selected access-mode policy, computer control also needs a one-shot approval
  plus a local unlock code, and desktop automation is still subject to
  foreground desktop, secure desktop, Screen Recording, and Accessibility
  limitations.
- Memory Studio for reviewable memories, edits, deletion, expiration, linked
  memory title/body search, linked memory search match source, linked memory
  relation notes, manual existing-memory links, and explicit conflict handling.
- Operations Briefing workflow:
  - Reads local evidence and drafts a management brief.
  - Can use DeepSeek synthesis when configured.
  - Exports Markdown, HTML, lightweight PDF, and work-package JSON to local
    paths.
  - During work-package imports, previews new/skipped workflow templates,
    new/skipped pending memory candidates, and new/skipped archived briefing
    runs.
  - Import preview: package-internal duplicate ids are counted as skipped.
  - Export safety: resolved memory candidates stay out of exported work
    packages.
  - Export safety: exported work packages redact source-machine evidence
    handles.
  - Import safety: imported memory candidates drop source-machine source links
    before local review.
  - Keeps imported archived runs as read-only replay details while redacted
    source-machine evidence handles stay visible as a safety boundary.
  - Uses blank operator templates under
    `docs/templates/operations-briefing-evidence` for the desktop
    seed-template button and a separate smoke sample folder for live smoke
    tests.
- Local report and package export paths for Markdown, HTML, lightweight PDF,
  and work-package JSON.
- Windows NSIS debug installer build path for local validation.

## Current Limits

- Real mailbox connectors are not complete.
- Real cloud-drive connectors are not complete.
- Browser form submission and terminal write keep approval and audit records;
  they are not broad automation executors.
- DS Agent does not install or manage local bridge services in this preview.
- Hosted sync, account systems, marketplaces, and arbitrary third-party
  executable plugins are not included.
- Public binary distribution is conservative until signing, packaging, and
  provenance are ready.
- PDF export is lightweight and ASCII-safe. Use Markdown or HTML for Chinese or
  other Unicode report content.

## Open Source Acknowledgement

This project benefits from the GitHub open-source ecosystem. Its architecture
and engineering direction were informed by public open-source work in desktop
apps, agent tooling, workflow systems, permission design, auditing, local-first
software, and the Rust/React/Tauri ecosystem.

We sincerely thank the founders, maintainers, and contributors of those
open-source projects. Their work makes projects like this possible.

No private, leaked, or non-authorized source code should be copied into this
repository. Public open-source references are used as learning material and
engineering inspiration, with respect for their licenses and maintainers.

## Local Verification

```powershell
npx pnpm@9.15.9 install
npx pnpm@9.15.9 test
npx pnpm@9.15.9 test:release-local
npx pnpm@9.15.9 test:release-local -- --require-live-deepseek --include-installed-workflow
npx pnpm@9.15.9 test:release-source
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop tauri build --debug
git diff --check
git diff --cached --check
```

The local release gate also runs deterministic helper checks, including the
Windows local helper self-test, installed UI helper self-test, release-local
helper self-test, and working-tree and staged diff whitespace checks. The
Windows local helper self-test does not call DeepSeek or read local secrets.

Optional live DeepSeek smoke tests:

```powershell
$env:DEEPSEEK_API_KEY = Read-Host "DeepSeek API key"
npx pnpm@9.15.9 test:deepseek
npx pnpm@9.15.9 test:deepseek:briefing
```

The briefing smoke uses the non-sensitive sample evidence under
`docs/templates/operations-briefing-smoke-evidence` by default. Use
`DEEPSEEK_BRIEFING_EVIDENCE_DIR` for another local evidence folder. The bundled
smoke files are marked as `SMOKE SAMPLE evidence for local verification only`
and `Replace before operational use`. Replace them before pointing the workflow
at real business evidence.

Do not commit API keys, `.env` files, local app data, local evidence folders, or
generated installer artifacts.
