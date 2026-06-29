# Security Policy

DeepSeek Agent OS is an alpha local-first desktop project. Security reports are
welcome, especially around local credentials, permission gates, audit records,
Computer Use boundaries, and package import/export behavior.

## Supported Version

| Version | Supported |
| --- | --- |
| 0.1-alpha | Security reports accepted; no stability guarantee |

## Reporting A Vulnerability

Before the public repository has a dedicated security advisory channel, report
security issues privately to the project maintainer. Do not open a public issue
with exploit details, secrets, screenshots, tokens, or private local paths.

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
- Manual DeepSeek pricing settings are local configuration, not live price
  claims.
- ComputerControl requires explicit approval plus a short local unlock window.
- ChatGPT/Codex Computer Use routes require an external loopback HTTP bridge in
  MVP; managed sidecar spawning is deferred.
- NetworkSearch evidence must preserve source URLs.
- Import writes memories as reviewable candidates, not automatic long-term
  memory.

## Out Of Scope For Alpha

- Hosted cloud sync.
- Real email sending or cloud-drive modification.
- Managed Codex bridge sidecar installation or supervision.
- Arbitrary third-party executable plugins.
