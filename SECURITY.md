# Security Policy

DS Agent is a local-first Windows desktop agent. DS Agent v1.0.2 is the current
published stable release and is not an official DeepSeek product. Security
reports are welcome, especially around local credentials, permission gates,
audit records, Computer Use boundaries, update integrity, code signing, and
package import/export behavior.

## Supported Version

| Version | Supported |
| --- | --- |
| 1.0.2 | Supported |

## Reporting A Vulnerability

Use GitHub Private Vulnerability Reporting on this repository for sensitive
security reports. Do not open a public issue with exploit details, secrets,
screenshots, tokens, or private local paths.

Include:

- affected commit or release;
- operating system;
- steps to reproduce;
- expected and actual behavior;
- whether the issue can expose secrets, local files, screen contents, input
  control, mailbox data, or exported work packages.

## Security Boundaries

- Current stable v1.0.2 reads a user-supplied DeepSeek API key from the desktop
  process environment. It must not be stored in events, UI state, logs, or
  exported work packages.
- Unpublished Step 1 onboarding work proposes a dedicated Windows DPAPI vault
  and secret-free readiness receipt. It is not part of the current stable
  release and must not be described as published until its release gates pass.
- `pnpm test:secrets` scans tracked and unignored repository files for live
  `sk-` style keys and non-empty `DEEPSEEK_API_KEY` assignments without printing
  candidate values.
- Manual DeepSeek pricing settings are local configuration, not live price
  claims.
- Computer Use remains experimental and high-risk. Computer control requires
  explicit approval plus a short local unlock window.
- Optional local desktop bridge use requires a user-started local loopback bridge
  in this preview.
- Web search evidence must preserve source URLs.
- Import writes memories as reviewable candidates, not automatic long-term
  memory.
- Release identity must follow the [code signing policy](CODE_SIGNING_POLICY.md).
  An unsigned or invalidly signed artifact must not be represented as a signed
  release. See also the [privacy policy](PRIVACY.md).

## Out Of Scope For Alpha

- Hosted cloud sync.
- Real email sending or cloud-drive modification.
- DS Agent does not install, launch, or supervise local bridge services.
- Arbitrary third-party executable plugins.
