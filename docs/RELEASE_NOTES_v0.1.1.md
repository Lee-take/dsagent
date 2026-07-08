# DeepSeek Agent OS v0.1.1 Release Notes

Status: Windows-first formal release hotfix. The `v0.1.1` release supersedes
`v0.1.0` for ordinary downloads.

Search aliases: DS Agent, DSAgent, dsagent, DeepSeek Agent OS.

Repository: https://github.com/Lee-take/dsagent

## v0.1.1 Hotfix

- Fixes the desktop updater current-release tag so the formal Windows build
  identifies itself as `v0.1.1` instead of the last release candidate.
- Bumps the Windows app package version to `0.1.1`, producing a distinct NSIS
  installer for GitHub downloads.
- Keeps the release discipline from `v0.1.0`: the published `v0.1.0` tag is not
  moved, and this patch is released as a new public tag.

## Release Focus

DS Agent remains a Windows-first local desktop agent for DeepSeek users who need
practical office help: summarize evidence, draft management briefings, create
local artifacts, continue projects with auditable memory, and keep local actions
behind visible permission and audit boundaries.

## For Everyday Users

DS Agent is intended to help ordinary DeepSeek users move from chat to finished
local office output. In the current Windows release, users can:

- Choose a local workspace and keep reports, logs, work packages, and artifacts
  in visible local folders.
- Turn evidence folders into management briefings, HTML/PDF reports, and
  reusable work packages.
- Convert meeting notes into action items, owners, deadlines, risks, and
  follow-up drafts.
- Continue prior projects while seeing which memories and evidence were used.
- Keep local actions behind permission, validation, and audit records.

The necessary setup after installation is simple: set `DEEPSEEK_API_KEY` and
choose one local workspace folder. Network search is optional; when live web
information is needed, configure one additional source-linked search route,
such as a search-capable model/provider key or an available free web-search
option.

## For Practitioners

DS Agent is built as a local DeepSeek-first agent harness. Its technical focus
is on making agent work bounded, inspectable, and recoverable:

- Harness architecture: structured envelopes separate user-facing replies,
  proposed actions, missing prerequisites, confirmations, artifact targets, and
  memory candidates.
- Loop engineering: preflight, context assembly, model routing, permission
  gates, local execution, validation, retries, and resumable work packages are
  handled as a controlled task loop.
- Auditable memory: selected memories carry reasons and receipts; feedback,
  quality scoring, expiration, deletion, maintenance history, and conflict
  handling remain reviewable.
- Context receipts: workflows can show evidence, memory, route decisions,
  validation, omissions, costs, and output paths.
- Local-first Windows runtime: workspace files, reports, logs, and artifacts
  stay inside user-controlled local folders.

## Open-Source Thanks

DS Agent also benefits from the broader GitHub open-source ecosystem. Public
open-source projects have provided valuable reference points in desktop apps,
agent harnesses, workflow systems, permission design, auditing, local-first
software, and release engineering. We sincerely thank the founders,
maintainers, and contributors of those projects for their generous sharing and
long-term dedication.

## Source Use Note

DS Agent is published under the Apache-2.0 license for learning, research,
evaluation, internal pilots, and adaptation within the license terms. The
published source is not presented as a turnkey codebase to repackage directly
as a commercial product or hosted service. Any commercial evaluation should
preserve required notices, avoid implying DS Agent or DeepSeek endorsement, and
include independent security review, compliance review, testing, signing, and
user-support responsibilities.

## 中文说明

`v0.1.1` 是当前面向普通下载的 Windows 正式版本。项目本地安装名为 DS Agent，也可以用
DSAgent、dsagent 或 DeepSeek Agent OS 搜索。

DS Agent 的重点是帮助 DeepSeek 用户处理真实办公任务：读取本地证据，生成经营简报，
创建本地 Office 产物，使用可审计记忆延续项目，并在本地动作前保持可见的权限、校验
和记录。

面向普通用户：DS Agent 可以选择本地工作目录，读取证据文件夹，生成经营简报、HTML/
PDF 报告和工作包，把会议纪要整理成行动项、责任人、截止时间和风险提示，并在延续项
目时说明使用了哪些记忆和证据。

安装后的必要设置很简单：设置本地 `DEEPSEEK_API_KEY`，再选择一个本地工作目录。网络
搜索是可选增强；如果任务需要实时联网信息，再额外配置一条能够返回来源链接的搜索路
线，例如具备搜索能力的大模型 key/服务，或在可用时选择免费的来源型搜索选项。

面向专业人员：DS Agent 的重点是本地 Agent harness、loop engineering、可审计记忆系
统和 Context Receipt。模型提出结构化动作，DS Agent 负责上下文组装、权限门、本地执
行、验证、重试、工作包恢复、证据回执和本地产物落地，让 Agent 执行过程保持有边界、
可检查、可恢复。

开源致谢：DS Agent 也受益于 GitHub 上广泛的开源生态。公开开源项目在桌面应用、
Agent harness、工作流系统、权限设计、审计记录、本地优先软件和发布工程等方面提供
了大量参考价值。我们真诚感谢这些项目的创建者、维护者和贡献者，感谢他们长期开放、
无私分享和持续奉献。

源代码使用说明：DS Agent 按照 Apache-2.0 许可证对外开源，欢迎用于学习、研究、评
估、内部试用和合规二次开发参考。但当前公开源代码不是作为可以直接包装成商业产品或
托管服务的成品代码库来发布；我们也不希望下载用户未经独立评审、合规处理和工程加
固，就直接拿当前源代码做商业化交付。如果评估任何商业场景，请遵守 Apache-2.0 许可
证，保留必要声明，不暗示 DS Agent 或 DeepSeek 官方背书，并自行完成安全审查、合规
审查、测试、签名和用户支持责任。

For full feature notes, see `docs/RELEASE_NOTES_v0.1.0.md`.
