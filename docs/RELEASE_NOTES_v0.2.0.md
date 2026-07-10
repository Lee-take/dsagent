# DS Agent v0.2.0 Release Notes

Status: Windows-first formal release. The `v0.2.0` release supersedes `v0.1.2`
for ordinary downloads.

Search aliases: DS Agent, DSAgent, dsagent, DeepSeek Agent OS.

Repository: https://github.com/Lee-take/dsagent

## From Chat Tool To AI Work Platform

DS Agent v0.2.0 turns the background-run foundation into a durable local
execution runtime. DeepSeek remains the open-ended reasoning and planning
layer. DS Agent owns local execution, tool contracts, permissions, sandbox
boundaries, run state, evidence, audit, verification, and recovery.

This is the first release where those responsibilities share one runtime
contract instead of being implemented as isolated chat actions. Longer work
can continue in the background while the user keeps typing, and local actions
are accepted as complete only when DS Agent can observe and record suitable
evidence.

Bumps the package, desktop, Tauri, Cargo, and updater metadata to `0.2.0` /
`v0.2.0` so installed Windows clients can detect this release as newer than
`v0.1.2`.

## What Is New

### Durable Background Runs

- Keeps the composer available while a run is queued or active.
- Lets new input become a separate task, durable guidance for the active run,
  or an explicit cancellation request.
- Persists queued guidance, cancellation state, run steps, artifacts, evidence,
  verification results, and terminal outcomes.
- Claims queued work through the background worker and records ownership,
  heartbeat, retry, recovery, and stale-run handling.
- Shows queued, running, waiting, completed, failed, and cancelled work in the
  desktop task area instead of hiding execution inside one chat request.

### Generic Tool And Capability Runtime

- Gives built-in tools and trusted declarative skills a common lifecycle:
  preflight, policy evaluation, approval, resource claim, execution, evidence,
  verification, audit, replay, and recovery.
- Covers file and directory work, Office artifacts, browser and network paths,
  Computer Use, application updates, Operations Briefing, and trusted
  declarative skill entry points.
- Uses exact approval fingerprints so a permission cannot silently authorize a
  different target or mutated action.
- Adds resource locks and heartbeats so two runs cannot silently write the same
  declared resource at the same time.
- Records tool attempts and outcomes as inspectable local events with links to
  the run, approval, evidence, and verification that produced them.

### Permission Sandbox And Computer Use

- Normalizes workspace paths and rejects traversal, protected locations,
  undeclared writes, and targets outside the allowed local boundary.
- Restricts network destinations through an explicit network sandbox and keeps
  source-linked search separate from unverified model text.
- Keeps screen inspection and desktop control behind visible policy and audit
  boundaries; mouse and keyboard control still requires explicit approval and
  a short local unlock window.
- Routes application update checks and downloads through the same evidence and
  validation discipline instead of treating updates as an unrelated bypass.

### Open Skill Foundation

- Executes trusted declarative skill plans through the generic tool runtime.
- Validates manifest identity, source, integrity, permissions, trust state, and
  entry-point shape before execution.
- Fails closed for arbitrary scripts, unsupported native hooks, undeclared
  permissions, unsafe paths, and disallowed network destinations.
- Keeps preview, install, trust reset, enable/disable, uninstall, execution,
  and audit paths locally inspectable.

### Verification And Recovery

- Stores evidence and verification summaries alongside run steps and artifacts.
- Supports replay and recovery from persisted state instead of depending only
  on transient frontend state.
- Marks unsupported model-proposed actions as failed rather than pretending
  they executed. Real mailbox, cloud-drive, and unrestricted terminal-write
  actions remain fail-closed until their runtime contracts exist.
- Preserves the boundary that model output proposes work and DS Agent decides
  whether local work is permitted, executed, and verified.

## For Everyday Users

The practical change is that DS Agent can keep working without taking over the
chat box. You can start a second task, add guidance to work already running, or
cancel it. The task area shows what is queued or active, and completed work can
carry visible artifacts, evidence, and verification rather than only a model
claim that something happened.

Local execution remains intentionally visible. File changes, network access,
desktop control, and skill actions stay inside permission, sandbox, and audit
boundaries. When DS Agent cannot execute a requested capability safely, it
reports the missing runtime instead of silently simulating success.

## For Practitioners

`v0.2.0` establishes the execution substrate for a general agent loop. Durable
`AgentRunRecord` state is separate from chat messages; a scheduler claims work;
the tool runtime coordinates capabilities, approvals, resources, evidence, and
verification; and append-only events make outcomes recoverable and reviewable.

The architecture keeps the model boundary explicit. DeepSeek performs
open-ended interpretation, decomposition, planning, and generation. DS Agent
performs deterministic local preflight, protocol handling, permission checks,
sandbox enforcement, execution, state transitions, evidence capture,
verification, audit, replay, and recovery.

The release remains deliberately single-writer where resources conflict. It
does not introduce hidden concurrent mutation of the same file or desktop
target. Wider parallel scheduling can build on declared resource ownership
without weakening approvals or auditability.

Operations Briefing keeps the existing evidence-directory discipline. Live
smoke tests use `docs/templates/operations-briefing-smoke-evidence` by default,
and every bundled file is marked for test use. The bundled smoke files are
marked as `SMOKE SAMPLE evidence for local verification only` and
`Replace before operational use`. The desktop seed-template action
uses `docs/templates/operations-briefing-evidence` as blank operator templates
instead of real business evidence. Replace smoke data before using a workflow
with operational material.

## Current Limits

- Real mailbox connectors and real cloud-drive connectors are not complete.
- DS Agent does not automatically install or manage an optional local bridge
  service.
- Public skill marketplace discovery, package signing, publisher reputation,
  and arbitrary third-party native/script execution are not complete.
- The sandbox is an application policy boundary, not a separate OS virtual
  machine for untrusted code.
- macOS packaging is configured but is not validated or released from this
  Windows release path.
- The Windows installer is unsigned, so Windows may show an unknown-publisher
  warning.

## 中文说明

`v0.2.0` 是面向普通下载的 Windows 正式版本，替代 `v0.1.2`。这次升级把 DS Agent 从
“可后台运行的聊天工具”进一步推进为“可验证的本地 AI 工作平台”地基：DeepSeek 负责
开放式理解、拆解、规划和生成，DS Agent 负责本地执行、工具契约、权限、沙箱、状态、
证据、审计、验证和恢复。

用户可以在任务运行期间继续输入，并明确选择把新输入作为独立任务、追加给当前任务，
或取消当前任务。queued guidance、取消状态、步骤、产物、证据、验证和最终结果都会持久
化，任务区会显示 queued、running、waiting、completed、failed 和 cancelled 状态。

这一版加入通用工具与能力 runtime。文件、Office、浏览器、网络、Computer Use、应用
更新、经营简报和受信任声明式 skill 共享同一套 preflight、策略判断、审批、资源领取、
执行、留证、验证、审计、回放和恢复流程。精确审批指纹防止权限被挪用；资源锁和心跳防
止多个 run 隐藏并发写同一资源。

权限沙箱会规范化本地路径，拦截目录穿越、保护路径、未声明写入和工作区边界外目标；
网络目标也经过显式限制。屏幕读取和鼠标键盘控制继续保留可见权限与审计边界，真实桌
面控制仍需要一次性审批和短时本地解锁。

skill 生态从“只能预览和准备”推进到“受信任声明式计划可执行”。manifest 的身份、来
源、完整性、权限、信任状态和入口形态都会在执行前校验；任意脚本、不支持的 native
hook、未声明权限、不安全路径和不允许的网络目标会 fail closed。

当前边界仍需说清楚：真实邮箱和云盘连接器、自动管理本地桥接服务、公共 skill 市场发
现、包签名与发布者信誉、任意第三方 native/script 代码的强隔离、macOS 实机发布和
Windows 安装包签名尚未完成。遇到这些尚未接线的能力，DS Agent 会明确失败，不会假装
已经执行成功。

For historical notes, see `docs/RELEASE_NOTES_v0.1.2.md` and
`docs/RELEASE_NOTES_v0.1.0.md`.
