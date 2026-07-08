# DeepSeek Agent OS (DS Agent / DSAgent)

Independent local-first desktop agent for DeepSeek office workflows.

Latest release: [DS Agent v0.1.1](https://github.com/Lee-take/dsagent/releases/tag/v0.1.1)

Search aliases: DS Agent, DSAgent, dsagent, DeepSeek Agent OS.

中文搜索别名：DS Agent、DSAgent、dsagent、DeepSeek Agent OS。

## Two Audiences / 两类读者入口

### For Everyday Users / 面向普通用户

DS Agent helps DeepSeek users finish practical office work on a local Windows
desktop. You can choose a workspace, point DS Agent at local evidence, and ask
in chat for work that normally takes repeated copying, checking, formatting,
and rewriting.

Typical tasks include:

- Turn a folder of evidence into a management briefing, HTML/PDF report, or
  reusable work package.
- Summarize meeting notes into action items, owners, deadlines, risks, and
  follow-up drafts.
- Create local Office-style artifacts from local files while keeping output
  paths visible.
- Continue a project from prior context and show which memories were used.
- Keep local file actions behind visible permission, validation, and audit
  records.

After installing DS Agent, the necessary setup is intentionally simple: set a
local `DEEPSEEK_API_KEY` and choose one local workspace folder. If you want DS
Agent to use live network search, add one more search-capable route: either a
large-model key/provider that can return source-linked web results, or a free
source-linked search option when available.

DS Agent 面向普通 DeepSeek 用户，目标不是让用户学习复杂开发工具，而是帮助他们把日
常办公任务做完：选择一个本地工作目录，把证据文件放进去，然后直接用聊天提出任务。

常见场景包括：

- 把一组本地证据整理成经营简报、HTML/PDF 报告或可复用工作包。
- 把会议纪要整理成行动项、责任人、截止时间、风险提示和后续草稿。
- 从本地文件生成可检查的 Office 类办公产物，并清楚显示输出路径。
- 延续上次项目，并说明本次用了哪些记忆。
- 对本地文件动作保持可见授权、校验和审计记录。

安装 DS Agent 后，必要设置刻意保持简单：设置本地 `DEEPSEEK_API_KEY`，再选择一个本
地工作目录即可。如果要使用实时网络搜索功能，再额外配置一条具备搜索能力的路线：可
以是支持返回来源链接的大模型 key/服务，也可以是在可用时选择免费的来源型搜索选项。

### For Practitioners / 面向专业人员

DS Agent is a local agent harness around DeepSeek. It brings model replies,
structured actions, local execution, evidence receipts, artifacts, and recovery
into one inspectable workflow. The model proposes structured work; DS Agent
owns context assembly, policy checks, local execution, receipts, artifacts, and
recovery.

Technical highlights:

- Agent harness: structured envelopes separate user reply, proposed actions,
  missing prerequisites, confirmations, artifact targets, and memory
  candidates.
- Loop engineering: bounded loops for preflight, context assembly, model
  routing, permission gates, local tool execution, validation, retries, and
  resumable work packages.
- Auditable memory: selected memories have reasons and receipts; feedback,
  quality scoring, maintenance history, expiration, deletion, and conflict
  review stay inspectable.
- Context receipts: each workflow can show selected evidence, selected memory,
  route decisions, validations, omissions, costs, and output paths.
- Local-first execution: Windows/Tauri desktop runtime keeps workspaces,
  reports, logs, and artifacts inside a user-controlled local boundary.
- DeepSeek-first routing: context is compacted and selected to fit practical
  DeepSeek usage while preserving reviewable evidence.

DS Agent 对专业人员来说，是围绕 DeepSeek 构建的本地 Agent harness。它把模型回复、
结构化动作、本地执行、证据回执、产物落地和失败恢复纳入同一个可检查工作流。模型负
责提出结构化工作建议；DS Agent 负责上下文组装、策略校验、本地执行、证据回执、产物
落地和失败恢复。

技术亮点包括：

- Agent harness：把用户回复、待执行动作、缺失前置条件、确认项、产物目标和记忆候
  选拆成结构化 envelope。
- Loop engineering：把 preflight、上下文组装、模型路由、权限门、本地工具执行、校
  验、重试和可恢复工作包纳入有边界的任务循环。
- 可审计记忆系统：已选记忆有理由和回执，反馈、质量评分、维护历史、过期、删除和冲
  突处理都可检查。
- Context Receipt：工作流可以展示证据、记忆、路线选择、校验、省略项、成本和输出
  路径。
- 本地优先执行：Windows/Tauri 桌面运行时让工作目录、报告、日志和产物留在用户可控
  的本地边界内。
- DeepSeek-first 路由：面向 DeepSeek 的实际上下文预算做选择和压缩，同时保留可复核
  的证据链。

## Project Introduction / 项目介绍

### English

DeepSeek Agent OS is packaged locally as DS Agent and can also be searched as
DSAgent or dsagent. It is an independent open-source desktop project written to
help colleagues use DeepSeek large language models more conveniently in daily
work. It focuses on local desktop agent workflows, permissioned tools,
auditable memory, source traceability, local files, and operations workflow
packs.

Its practical strength is turning local evidence into reviewable office
outputs. DS Agent can read an evidence folder, draft an operations briefing,
export Markdown/HTML/PDF and work-package JSON, create local Office artifacts,
and then show what evidence, memory, route, validation, omissions, and output
paths were used. The goal is to make everyday office work easier to finish,
inspect, continue, and correct.

The first-run setup is deliberately lightweight. For normal DeepSeek-backed
office work, install DS Agent, set `DEEPSEEK_API_KEY`, and choose one local
workspace folder. Network search is optional; when current web information is
needed, configure one additional source-linked search route, such as a
search-capable model/provider key or an available free web-search option.

This project is not an official DeepSeek product, is not affiliated with
DeepSeek, and does not claim any DeepSeek ownership, authorization, or
endorsement. The DeepSeek name is used only to describe compatibility and
support for DeepSeek models.

The project is shared publicly under the Apache-2.0 license. Friends,
colleagues, and anyone who finds it useful are welcome to use it, study it,
fork it, and adapt it within the license terms. When convenient, please open an
issue, discussion, or pull request with suggestions, corrections, criticism, or
other feedback. All advice is welcome.

This project also benefits from the broader GitHub open-source ecosystem.
Public open-source work has provided valuable reference points for DS Agent in
desktop apps, agent harnesses, workflow systems, permission design, auditing,
local-first software, and release engineering. We sincerely thank the founders,
maintainers, and contributors of those projects for their generous sharing and
long-term dedication. Their open work gives projects like DS Agent a stronger
foundation to learn from and build on.

### 中文

DeepSeek Agent OS 本地安装名为 DS Agent，也可以用 DSAgent 或 dsagent 搜索到。本项
目是一个独立开源的桌面端项目。写这个项目的初衷，是为了让同事们在日常工作中更方便
地使用 DeepSeek 大模型。项目重点放在本地桌面 Agent 工作流、可授权工具、可审计记
忆、来源追溯、本地文件和经营管理工作流包。

DS Agent 的强项，是把本地证据变成可检查、可继续、可交付的办公产物。它可以读取证
据文件夹，生成经营简报，导出 Markdown/HTML/PDF 和工作包 JSON，创建本地 Office
文档，并说明本次任务用了哪些证据、记忆、模型路线、校验、省略项和输出路径。我们的
定位是让日常办公任务真的能被完成、复核、延续和纠偏。

首次使用的设置也尽量简单。普通 DeepSeek 办公任务只需要安装 DS Agent，设置本地
`DEEPSEEK_API_KEY`，再选择一个本地工作目录。网络搜索是可选增强；如果任务需要实时
联网信息，再额外配置一条能够返回来源链接的搜索路线，例如具备搜索能力的大模型
key/服务，或在可用时选择免费的来源型搜索选项。

本项目不是 DeepSeek 官方产品，也不隶属于 DeepSeek；项目不主张任何 DeepSeek 的所有
权、授权或官方背书。这里使用 DeepSeek 名称，只是为了说明项目面向 DeepSeek 模型做
兼容和支持。

本项目按照 Apache-2.0 许可证对外开源。有需要的同事、朋友和开发者，都可以在许可证
范围内自由使用、学习、fork 和改造。大家方便或有空的时候，也欢迎通过 issue、讨论或
pull request 提出意见建议；批评指正都非常欢迎。

这个项目也受益于 GitHub 上广泛的开源生态。公开开源项目在桌面应用、Agent harness、
工作流系统、权限设计、审计记录、本地优先软件和发布工程等方面，为 DS Agent 提供了
大量参考价值、工程经验和架构启发。我们真诚感谢这些开源项目的创建者、维护者和贡献
者，感谢他们长期开放、无私分享和持续奉献。正是这些开源工作，让 DS Agent 这样的项
目能够站在更扎实的基础上继续学习和建设。

No private, leaked, or non-authorized source code should be copied into this
repository. Public open-source references are used as learning material and
engineering inspiration, with respect for their licenses and maintainers.

本仓库不应复制任何私有、泄露或未授权代码。公开开源项目仅作为学习材料和工程参考，
并尊重原项目许可证和维护者权益。

## 0.1.1 Status / 0.1.1 状态

Version `0.1.1` is the current Windows-first formal release. The codebase
is still a practical preview rather than a finished agent product, but the
Windows build/install/launch/run path is verified through the repeatable
release gate and installed UI workflow smoke before publication. The `v0.1.1`
release includes a Windows NSIS installer for ordinary colleagues to download
and test.

After the Windows preview continues to pass local release gates, the next
platform target is macOS. A macOS Tauri packaging config already exists in the
repository, but macOS validation and release work will follow after the Windows
preview continues to pass local release gates.

`0.1.1` 是当前 Windows 优先正式发布版本，仍然是实用预览版，还不是完整成熟的
Agent 产品。正式发布前，当前 Windows 构建、安装、启动和运行路径都要通过本地
release gate 与 installed UI workflow smoke 验证。`v0.1.1` release 附带 Windows
NSIS 安装包，方便普通同事直接下载测试。

Windows 预览版持续通过本地 release gates 后，下一步会推进 macOS 版本。仓库里已经保
留了 macOS 的 Tauri 打包配置，macOS 的验证和发布会在 Windows 预览版持续通过本地
release gates 后推进。

License: Apache-2.0.

The public `v0.0.1` and `v0.1.0` releases remain unchanged. The `v0.1.1`
release is the current Windows installer line for colleague testing. The
installer is unsigned, so Windows may show an unknown-publisher warning, but
the NSIS package is built with the Microsoft WebView2 bootstrapper embedded and
run silently so ordinary Windows users do not need a developer toolchain.

`v0.1.1` focuses on Windows office work: install DS Agent, connect DeepSeek
through a local environment variable, choose a workspace, ask in chat, and
create auditable local reports or office artifacts from local evidence.

`v0.1.1` 的重点是 Windows 办公场景：安装 DS Agent，通过本地环境变量连接
DeepSeek，选择一个本地工作目录，在聊天中提出任务，并从本地证据生成可审计的报告
或办公产物。

## Why DS Agent / DS Agent 亮点

DS Agent is built for local office work: it packages context for DeepSeek,
validates proposed actions before touching local files, records audit events,
shows receipts for evidence and memory, and exports artifacts under a local
workspace.

DS Agent 是一个 Windows-first 的 DeepSeek 本地桌面 Agent，面向真实办公任务：
读取本地证据、整理会议和行动项、生成经营简报、创建可检查的本地文件，并在项目延
续时使用可审计的记忆。

Highlights:

- DeepSeek-first local desktop experience for Windows users.
- Chat-first office workflows for evidence summaries, management briefings, and
  local artifacts.
- Permissioned local execution: the model proposes, DS Agent validates and
  records.
- Context Receipts that show selected evidence, memory, route, validation,
  omissions, and output paths.
- Auditable memory with feedback, quality scoring, maintenance history, and no
  silent model-owned writes.
- Local-first workspace for files, reports, work packages, logs, and replayable
  artifacts.

亮点：

- 面向 DeepSeek 用户的 Windows 本地桌面体验。
- 从聊天直接进入办公任务：证据整理、经营简报、本地办公产物。
- 权限化本地执行：模型提出动作，DS Agent 校验、执行并记录。
- Context Receipt 展示证据、记忆、路线、验证、省略项和输出路径。
- 可审计记忆：反馈、质量评分、维护历史清楚可查，不做模型黑箱式静默写入。
- 本地优先工作目录：文件、报告、工作包、日志和可回放产物都落在本地边界内。

Try these tasks:

- `根据我的证据文件夹，生成一份经营简报，并导出 HTML 和 PDF。`
- `把这段会议纪要整理成行动项、责任部门、截止时间和风险提示。`
- `继续上次的项目，先说明你用了哪些记忆，再给我下一步建议。`

## Basic Functions / 基本功能

### English

The current codebase is intended to provide these basic functions:

- Windows desktop shell built with Tauri, React, TypeScript, and Rust.
- Local-first workspace setup for a workspace folder, evidence folder, and
  export folder.
- DeepSeek route readiness through a local `DEEPSEEK_API_KEY` environment
  variable without storing or showing the key value.
- Optional local DeepSeek smoke tests for Chat Completions and Operations
  Briefing synthesis.
- Permissioned tool surfaces for file, network, browser, terminal,
  local-folder read/export, email read/draft/send approval records, and
  Computer Use operations.
- Audited Windows local filesystem mutations from chat for explicit file and
  directory create, update, delete, and rename requests.
- Append-only local audit records for access requests, approvals, tool
  attempts, workflow runs, memory records, and work packages.
- Memory Studio for reviewable memories, edits, deletion, expiration, linked
  memory title/body search, linked memory search match source, linked memory
  relation notes, manual existing-memory links, explicit conflict handling, and
  feedback-informed retrieval scoring. Selected-memory feedback stays
  append-only, while DS Agent automatically records retrieval tuning, applies
  audited updates, and archives repeated stale memories in the background.
- Operations Briefing workflow:
  - Reads local evidence and drafts a management brief.
  - Can use DeepSeek synthesis when configured.
  - Exports Markdown, HTML, lightweight PDF, and work-package JSON to local paths.
  - Shows context receipts with loop mode, workflow policy, selected evidence,
    validation, and intentional omissions.
  - During work-package imports, previews new/skipped workflow templates,
    new/skipped pending memory candidates, and new/skipped archived briefing runs.
  - Keeps imported archived runs as read-only replay details while redacted
    source-machine evidence handles stay visible as a safety boundary.
  - Uses blank operator templates under
    `docs/templates/operations-briefing-evidence/` for local evidence-folder
    seeding.
- Windows NSIS installer build path for local validation and RC distribution,
  including an embedded Microsoft WebView2 bootstrapper.

Current limits are intentional: real mailbox connectors, real cloud-drive
connectors, automatic local bridge-service management, hosted sync, broad
plugin execution, and polished signed installers are not complete in `0.1.0`.

### 中文

当前代码库的基础功能目标如下：

- 基于 Tauri、React、TypeScript 和 Rust 的 Windows 桌面应用外壳。
- 本地优先的工作目录设置：首次只选择一个工作目录，证据、导出、报告、运行记录和工作包等子目录由 DS Agent 自动维护。
- 通过本地 `DEEPSEEK_API_KEY` 环境变量检测 DeepSeek 可用性，但不保存、不展示密钥
  明文。
- 可选的本地 DeepSeek 联调脚本，用于 Chat Completions 和经营简报合成验证。
- 面向文件、网络、浏览器、终端、本地文件夹读取/导出、邮件读取/草稿/发送审批记录和
  Computer Use 的权限化工具入口。
- 从聊天中执行经过审计的 Windows 本地文件系统变更，包括明确提出的文件/目录创建、
  修改、删除和重命名。
- 本地追加式审计记录，用于记录授权请求、审批、工具调用、工作流运行、记忆记录和工
  作包。
- Memory Studio，用于记忆审计、编辑、删除、过期、关联记忆标题/正文搜索、关联记忆搜索命中来源、
  关联说明、手动关联已有长期记忆、冲突审计和反馈驱动检索。Context Receipt 里的已选记忆反馈会参与后续检索评分，
  stale/conflicting/should_update 反馈会压缩成后台维护线索；DS Agent 会自动记录检索调优、应用审计更新，
  并在重复 stale 反馈下自动归档过时记忆。
- Operations Briefing 经营简报工作流：
  - 读取本地证据并生成管理简报。
  - 配置 DeepSeek 后可调用模型合成。
  - 将 Markdown、HTML、轻量 PDF 和 work-package JSON 导出到本地路径。
  - 展示上下文回执，包括循环模式、工作流策略、选入证据、验证结果和有意省略内容。
  - 导入工作包时预览新增/跳过的工作流模板、待审核记忆候选和归档简报运行。
  - 导入归档运行保持只读回放详情，同时保留已清理的源机器证据句柄作为安全边界。
  - 使用 `docs/templates/operations-briefing-evidence/` 下的空白运营模板进行本地证据目录初始化。
- Windows NSIS 安装包构建路径，便于本地验证和 RC 分发，并内置 Microsoft WebView2
  bootstrapper。

当前限制也要说清楚：`0.1.0` 还没有完成真实邮箱连接器、真实云盘连接器、自动安装或管理本地桥接服务、
云同步、广泛插件执行和正式签名安装包。这一版先把 Windows 本地可运行和 DeepSeek 基础支持打牢。

Read first:

- `docs/INSTALLATION.md`
- `docs/OPEN_SOURCE_RELEASE.md`
- `docs/RELEASE_NOTES_v0.1.0.md`
- `CONTRIBUTING.md`
- `SECURITY.md`
- `.env.example`

Maintainer handoff notes, decision logs, and internal planning files are kept as
local-only continuation material and are intentionally excluded from public
source snapshots.

## Development

Desktop source commands:

```powershell
npx pnpm@9.15.9 install
npx pnpm@9.15.9 test
npx pnpm@9.15.9 tauri:dev
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop tauri build --config src-tauri/tauri.windows.conf.json
npx pnpm@9.15.9 dev
```

Use `tauri:dev` for the real DS Agent desktop window. The root `dev` command
starts only the Vite web preview and is useful for frontend layout work, but it
does not provide the Tauri command bridge that the chat workflow uses. On
Windows, `tauri:dev` automatically keeps Rust build output under the system
temporary directory when `CARGO_TARGET_DIR` is not set, avoiding MinGW path
parsing failures when the source checkout path contains spaces.

`pnpm test` runs the repository secret scan, desktop frontend build, and Rust
tests. The scan covers tracked files plus unignored new files. On Windows, the
test helper automatically keeps Rust build output out of the repository path to
avoid local MinGW path parsing issues when the checkout path contains spaces.
Set `CARGO_TARGET_DIR` yourself only when you need a specific build cache
location.

To run only the repository secret scan before committing or pushing:

```powershell
npx pnpm@9.15.9 test:secrets
```

Before any publication decision for a new release, run the local release gate:

```powershell
npx pnpm@9.15.9 test:release-local
```

This runs the full project test, working-tree and staged diff whitespace checks
(`git diff --check` and `git diff --cached --check`), and the source-only
release guard. The source-only guard checks version/name consistency, required
release docs, generated WebView2 loader ignore coverage, Windows WebView2
bootstrapper packaging config, and currently tracked or unignored files for
accidental installer/binary release artifacts, local runtime artifacts,
generated workflow exports, unexpected binary files, oversized source files,
and stale smoke-test release labels.
The local gate also runs deterministic helper checks for the Windows local
Operations Briefing smoke helper, the installed UI helper, and the release-local
helper itself; the Windows local helper self-test does not call DeepSeek or read
local secrets.
When `DEEPSEEK_API_KEY` is configured locally, the gate also runs the Windows
local Operations Briefing smoke test plus both DeepSeek live smoke tests. Use
`npx pnpm@9.15.9 test:release-local -- --skip-live-deepseek` for an offline
source-only pass. Use
`npx pnpm@9.15.9 test:release-local -- --include-installed-ui` when a Windows
DS Agent install already exists and you also want to smoke-test the installed
WebView2 UI. Use
`npx pnpm@9.15.9 test:release-local -- --include-installed-workflow` for the
stronger installed-app workflow smoke, which exercises the installed app command layer,
Operations Briefing run, and Markdown/HTML/PDF exports through the installed
app.
When a local DeepSeek test key and installed Windows app are available, use the
strongest local gate before that publication decision:

```powershell
npx pnpm@9.15.9 test:release-local -- --require-live-deepseek --include-installed-workflow
```

To run only the source-only release guard:

```powershell
npx pnpm@9.15.9 test:release-source
```

`test:deepseek` is an optional local smoke test. It reads `DEEPSEEK_API_KEY`
from the local environment, calls DeepSeek Chat Completions, and prints only
secret-safe metadata such as model, finish reason, and token usage. It is not
used by GitHub CI:

```powershell
npx pnpm@9.15.9 test:deepseek
```

`test:deepseek:briefing` is the optional local workflow smoke test. It sends the
Operations Briefing smoke sample evidence manifest from
`docs/templates/operations-briefing-smoke-evidence` to DeepSeek, validates that
the response matches the expected workflow result format, and prints only counts and
token metadata by default. The bundled smoke files are marked as
`SMOKE SAMPLE evidence for local verification only` and
`Replace before operational use`. Replace them before pointing the workflow at
real business evidence. When a custom evidence directory is an absolute local
path, the script redacts that path from both the prompt and the output:

```powershell
npx pnpm@9.15.9 test:deepseek:briefing
```

`test:windows-installed-ui` is an optional local Windows smoke test for the
installed app. It launches `ds-agent.exe` with a temporary WebView2 DevTools
port, checks that the installed UI loads at `tauri.localhost`, confirms the
desktop command layer is available, and writes a screenshot under the OS temp directory:

```powershell
npx pnpm@9.15.9 test:windows-installed-ui
```

Add `-- --workflow` to run a stronger installed-app workflow smoke. It saves
temporary local directory settings, seeds Operations Briefing evidence templates,
runs the briefing, exports Markdown/HTML/PDF reports, and restores the original
local directory settings file and app-data event store. When
`DEEPSEEK_API_KEY` is configured, it also requires a newly recorded DeepSeek
telemetry event from the installed app process:

```powershell
npx pnpm@9.15.9 test:windows-installed-ui -- --workflow
```

Windows builds automatically merge `apps/desktop/src-tauri/tauri.windows.conf.json`
and produce an NSIS installer under the configured Cargo target directory, for
example `release/bundle/nsis/DS Agent_0.1.1_x64-setup.exe`. The Windows config
embeds the Microsoft WebView2 bootstrapper and runs it silently during install
when the target machine needs the WebView2 runtime.

macOS builds have a separate platform config at
`apps/desktop/src-tauri/tauri.macos.conf.json` for `.app` and `.dmg` packaging.
Run the same Tauri build command on a macOS host to produce those bundles.

## Installation And Local Directories

The app is installed like a normal desktop app, but the installation directory is
not used as the workspace or data directory.

- Program files live in the installer-selected application location.
- App state, SQLite events, logs, and local settings live under the OS-provided
  app data directory.
- First run asks for one workspace. Evidence, exports, reports, runs, work
  packages, and related artifact folders are managed automatically under that
  workspace.
- File, folder, Drive-local, evidence, and export-package paths remain runtime
  user inputs on the user's own machine.
- Local directory settings are stored as `local-directories.json` under the app
  data directory.
- Windows alpha packaging uses a Tauri NSIS installer. macOS packaging uses a
  separate `.app`/`.dmg` config, so Windows installer choices do not block macOS
  distribution.

## Architecture

Harness architecture v1 runs through a stable Agent OS Kernel plus Workflow
Packs. Loop engineering lives in the product surface: it uses permissioned tool
boundaries, source-linked evidence, bounded workflow runs, selective context
assembly, and token-efficient DeepSeek routing. The runtime keeps context
focused instead of loading every available source into each request, so users
get faster feedback while deeper workflows can still verify their evidence. The
first public preview brings the desktop shell, local event history, policy
model, and DeepSeek route model into one buildable Windows app.

The model boundary is explicit: DeepSeek handles open-ended reasoning,
understanding, planning, and generation, while DS Agent handles deterministic
local preflight, protocol context, permission checks, workspace structure, tool
execution, audit records, and artifacts. Model-returned actions are proposals
until DS Agent validates them against local policy. See
[`docs/AGENT_MODEL_BOUNDARY.md`](docs/AGENT_MODEL_BOUNDARY.md).

Context receipts show loop mode, workflow policy, selected evidence, memory,
model route, token/cache state, validation results, and intentional omissions.
Markdown and HTML report exports carry the same context receipt summary.
Bounded repair loops rerun only the failed step with the smallest useful
context, so DS Agent can keep ordinary tasks responsive while still leaving a
reviewable trail for longer workflow runs.

Central chat tasks now carry a goal loop contract through the run: DS Agent
packages the user's real goal, constraints, done-when criteria, completion
verifier, stop conditions, and near-miss guardrails before asking DeepSeek for
reasoning. Local, browser, file, Office, and tool outcomes are treated as
complete only when DS Agent can observe evidence that matches the user's goal.
If the user adds supplementary guidance during a running task, DS Agent folds it
into the same task at the next small node and keeps the right-side run status in
sync. Completed or partially completed results can include one short,
task-grounded next-better suggestion.

The current 0.1.0 preview includes the permission loop for built-in local
tools. Built-in local tools cover file, network, browser, email approval
records, local folders, terminal diagnostics, and Computer Use surfaces; access
requests are evaluated through policy, persisted as append-only events, and
resolved through a visible approval queue in the desktop inspector. The first
tool paths let users browse URLs for source evidence, run source-linked web
search, preview local text files and evidence folders, create approved
workspace files, run allowlisted read-only terminal diagnostics, record
approval and audit decisions for mutating terminal and browser-form actions,
record approval and audit decisions for email read/draft/send flows, scan local
folders, export work-package JSON to local folders, capture approved screen
evidence, execute approved local mouse/keyboard actions behind an unlock
window, and inspect recent tool output.

Permission review clarity v1 keeps high-impact actions explicit. Outbound email
and desktop control approvals authorize only the next matching attempt, while
lower-risk approvals can stay reusable when the selected access mode allows it.

Permission state visibility v1 shows whether a grant is reusable, ready for one
use, or already spent, so operators can understand current access without
reading audit internals.

Approval decision traceability v1 keeps high-impact retries tied to the approval
that authorized them and keeps earlier audit history readable. Recent tool
output gives operators a clear path from action back to decision when an
approval record is present.

Tool route settings v1 shows the selected model route and available tool paths
in the runtime inspector. Web search follows the selected model route, uses a
configured local bridge service when it can return source-linked results, and
otherwise requires a selected source-linked web-search route before live search
can run. The current preview keeps email read, draft, and send as approval and audit
surfaces. It also keeps local folders and export packages separate from cloud
accounts, while screen inspection and computer control use the configured bridge
or local Windows/macOS route. No API key or account credential is stored by this
settings slice.

Setup directory clarity v2 keeps program files and app data separate from the
user-selected workspace. DS Agent stores that single setup choice in the current
user's app data directory, uses a native folder picker for the workspace, and
automatically manages evidence, exports, reports, runs, sources, work packages,
memory, and logs under that workspace.

Windows packaging clarity v1 builds the local Windows preview as an NSIS
installer. It keeps macOS packaging configured but pending verification on a
macOS host.

DeepSeek credential status v1 reads only whether `DEEPSEEK_API_KEY` is present in the local process environment and shows that status in the runtime inspector with the API base URL. DeepSeek Chat API readiness v1 also reports the derived Chat Completions endpoint, selected Flash/Pro model names, and whether local chat-completion requests are ready based on key presence. The key value is never returned to the UI, never serialized into events, and never included in exported work packages.

DeepSeek model request path v1 calls the official Chat Completions endpoint when a local API key is configured. It keeps automated tests offline, redacts local API keys from request errors, and keeps source-linked web search evidence on dedicated routes that require source URLs instead of treating plain chat completions as verified web evidence.

DeepSeek cache and usage visibility v1 keeps Operations Briefing synthesis responsive with an in-session request cache and secret-safe usage records. The runtime inspector shows the latest DeepSeek call status, cache hit/miss state, elapsed time, token usage when the provider returns it, estimated cost when local pricing is configured, current cache size, and a clear-cache action. Local pricing stays in the user's app data directory, and public source does not hardcode live DeepSeek prices.

Web search evidence clarity v1 shows the selected search route before a web
search runs. The app uses source-linked search when the selected model route
cannot provide verified web results, requires source URLs before search output
is treated as evidence, keeps approval gates in place and avoids live network
requests while approval is pending. Reserved alpha presets disclose when they
share the same local search implementation until separate local-browser or
aggregator routes are ready.

Optional local web-search bridge readiness v1 uses only a configured local loopback
bridge for supported providers and maps returned source URLs into the same
evidence and audit trail, so ordinary chat-completion text is not treated as web
evidence.

Desktop automation route clarity v1 shows whether screen inspection and desktop
control will use the selected local route or a configured local bridge. It keeps
desktop control visibly approval-gated before any mouse or keyboard action can
run, and it does not silently switch routes when a configured bridge is
unavailable.

Desktop prerequisite clarity v1 shows local screen and input prerequisites
before a tool runs. macOS lists Screen Recording and Accessibility requirements,
while Windows calls out foreground-desktop and secure-desktop limits instead of
pretending every window can be inspected or controlled.

Local bridge evidence safety v1 only accepts local loopback bridge endpoints. It keeps
screen evidence, control actions, and source-linked web search inside the same
approval and audit path, stores returned screen evidence in the selected evidence
folder, and does not expose local file paths through bridge responses.

Work-package readiness summary v1 adds a secret-safe readiness snapshot to
exported work packages. It shows whether DeepSeek requests, source-linked web
search, desktop automation, local folders, and selected tool routes are ready.
The snapshot is built without storing API keys, user machine paths, running live
model calls, capturing screens, or controlling the desktop during export.

Audited desktop input safety v1 keeps real desktop input inside a small reviewed action set, keeps desktop control experimental and visibly gated, and requires a one-shot approval plus a local in-memory unlock code before any mouse or keyboard action can run.

Local computer control unlock window v1 adds a short local unlock window after approval. The local operator unlocks computer control for five minutes in the inspector, computer control stays locked when the local unlock window is not active, and the unlock code stays out of audit events and exported work packages.

Computer tool route selection v1 keeps screen inspection and desktop control on
the route users see before approval. It reports an unavailable bridge as a clear
error, keeps DeepSeek and custom-model routes on local Windows/macOS desktop
automation paths, and avoids silent route switching before an approved screen or
control action runs.

Screen evidence clarity v1 keeps approved screenshots as local evidence files
with readable audit references. It keeps pending and failed attempts in the
approval trail so users can see why screen inspection did not run.

Screen inspection consent v1 treats screen capture as a sensitive desktop read.
It runs screen capture without an extra prompt in the default full-access mode,
while medium-risk reads remain policy-evaluated in limited automation mode.

Local screenshot storage clarity v1 saves approved PNG screenshots under
`computer-screenshots/`. It uses the selected evidence folder, or app data before
first-run setup, and records portable relative references for export and audit.

Operations Briefing clarity v1 turns approved local evidence into a management
brief. It drafts a summary, anomaly leads, and action items from approved local
evidence, can use DeepSeek synthesis when configured and falls back to a local
draft with a visible warning.

Report export clarity v1 exports Markdown, standalone HTML, lightweight PDF,
and work-package JSON through an approved local export flow. Markdown and HTML
preserve full Unicode, while the preview PDF stays lightweight and ASCII-safe.

Briefing handoff safety v1 keeps imported briefing runs read-only and
reviewable, keeps source-machine evidence handles redacted in exported work
packages, keeps memory candidate history local and auditable, and keeps resolved
memory candidates out of exported packages. For handoff safety, exported work packages
redact source-machine evidence handles, and resolved memory candidates stay out
of exported work packages. Evidence-template seeding uses blank operator
templates without overwriting existing local evidence files.

Memory review clarity v1 keeps memory writes explicit and auditable while
moving routine candidate decisions into DS Agent background maintenance. Users
can still propose corrections, edit, expire, and delete long-term memories when
they need to fix the audit trail.
Selected-memory feedback now supports feedback-informed retrieval scoring:
useful feedback can lift later recall, irrelevant, stale, conflicting, or
should_update feedback can lower recall, and stale/conflicting/should_update
feedback surfaces compact review hints without dumping full memory bodies into
receipts.
Automatic Memory maintenance v1 keeps routine memory care out of the user's
workflow. Repeated irrelevant feedback records retrieval tuning, should_update
feedback can create and apply an audited update, and repeated stale feedback can
archive stale memories in the background while leaving an inspectable audit
trail.

Memory conflict clarity v1 surfaces likely overlaps in the audit view and lets
DS Agent apply link, merge, replace, update, archive, or reject decisions in the
background with inspectable relation notes.
Linked memory title/body search, linked memory search match source, and linked
memory relation notes stay visible so related records are understandable during
review. Manual existing-memory links let users connect already accepted
long-term memories without waiting for a new candidate conflict.

Memory import safety v1 keeps imported memory candidates local and auditable
before DS Agent background maintenance resolves them. It ensures imported
memory candidates drop source-machine source links, and package-internal
duplicate ids are counted as skipped. Older task-derived memories remain
readable without exposing local source links.
