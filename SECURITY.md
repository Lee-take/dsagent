# Security Policy

DeepSeek Agent OS is an alpha local-first desktop project. The 0.1.0 preview is
a source-first public preview and is not an official DeepSeek product. Security
reports are welcome, especially around local credentials, permission gates,
audit records, Computer Use boundaries, and package import/export behavior.

## Supported Version

| Version | Supported |
| --- | --- |
| 0.1.0 | Security reports accepted; no stability guarantee |

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

- DeepSeek API keys are read from the local process environment and must not be
  stored in events, UI state, logs, or exported work packages.
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

## Out Of Scope For Alpha

- Hosted cloud sync.
- Real email sending or cloud-drive modification.
- DS Agent does not install, launch, or supervise local bridge services.
- Arbitrary third-party executable plugins.
