# DeepSeek Agent OS v0.1.2 Release Notes

Status: Windows-first formal release upgrade. The `v0.1.2` release supersedes
`v0.1.1` for ordinary downloads.

Search aliases: DS Agent, DSAgent, dsagent, DeepSeek Agent OS.

Repository: https://github.com/Lee-take/dsagent

## v0.1.2 Upgrade

DS Agent now treats user instructions as background Agent runs instead of a
single blocking chat turn. This is the first release where DS Agent starts to
behave like an agent workbench: work can be queued, claimed by a worker,
tracked through state transitions, cancelled, and audited while the user keeps
the composer available for follow-up instructions.

Bumps the package, Tauri, Cargo, and updater current-release tag to `0.1.2` /
`v0.1.2` so installed Windows clients can detect this release as newer than
`v0.1.1`.

## Background Agent Runs

- Persists user instructions as queued Agent runs instead of treating every
  instruction as one blocking chat request.
- Adds a worker command that claims the next queued run, records execution
  state, writes step events, attaches artifacts, and records a terminal
  completed, failed, or cancelled outcome.
- Keeps the input composer usable while background work is running, so users
  can add follow-up instructions and let DS Agent queue the next task.
- Records cancellation requests and cancellation outcomes in the local event
  store so the user can inspect what happened after a stop request.
- Keeps run history local-first and auditable, matching the existing DS Agent
  pattern for permissions, receipts, artifacts, and recovery.

## Safe Skill Ecosystem Foundation

- Adds a skill manifest contract for id, name, version, description, entry
  points, declared permissions, source identity, integrity metadata, and trust
  state.
- Adds local and remote preview paths before installation, so a package can be
  inspected before it becomes available to the runtime.
- Blocks unsafe or unsupported execution shapes during preflight, including
  undeclared permissions, missing source identity, unsafe script/native hooks,
  and packages that fail manifest validation.
- Adds trust reset, enable/disable, uninstall, and execution-plan preparation
  controls so installed skills remain reviewable after installation.
- Records skill install, trust, disable, uninstall, and execution-preparation
  actions through the local audit/event model.

## For Everyday Users

The visible change is simple: DS Agent should no longer feel like a single
blocked chat box when a longer Agent task is running. You can keep typing,
queue a next request, check status, and cancel work without losing the local
audit trail.

The skill work is intentionally careful. DS Agent is not yet a broad third-party
marketplace, and users should not treat arbitrary public code as safe to run.
This release creates the safety rails needed before future high-quality skills
from open-source sources can be installed with clear permissions, source
identity, trust state, and uninstall paths.

## For Practitioners

`v0.1.2` adds the first durable split between the chat surface and the Agent
run lifecycle. The frontend can enqueue work, the backend event store owns run
state, and the worker command advances one queued run at a time. That makes
future scheduling, interruption, resumption, and multi-run inspection possible
without overloading the message composer as the only execution state.

The skill foundation is deliberately smaller than a marketplace. The important
architectural step is the trust boundary: DS Agent now has a manifest shape,
permission declaration, source verification hooks, integrity fields, audit
events, and execution-plan preflight before skill code is treated as usable.

Operations Briefing keeps the existing evidence-directory discipline. Live
smoke tests use `docs/templates/operations-briefing-smoke-evidence` by default,
and the desktop seed-template action uses
`docs/templates/operations-briefing-evidence` as blank operator templates
instead of real business evidence. The bundled smoke files are marked as
`SMOKE SAMPLE evidence for local verification only` and
`Replace before operational use`; replace them before pointing any workflow at
real business evidence.

## Current Limits

- Third-party skill marketplace discovery is not complete.
- Cryptographic signing and publisher reputation are not complete.
- Strong sandbox isolation for arbitrary third-party code is not complete.
- Remote skill sources are previewed conservatively; ordinary users should only
  install skills from sources they trust and can review.
- Windows installer signing is not complete, so Windows can still show an
  unknown-publisher warning.

## 中文说明

`v0.1.2` 是面向普通下载的 Windows 正式升级版本，替代 `v0.1.1`。这次不是单纯修版本号，
而是把 DS Agent 从“单次阻塞聊天”推进到“可后台执行、可排队、可取消、可审计”的 Agent
工作台方向。

后台 Agent run 的变化包括：用户指令会先作为 queued run 记录下来，由 worker 领取执行；
执行过程会记录状态、步骤、产物和最终结果；用户可以在任务运行期间继续输入后续要求；
取消请求和取消结果也会进入本地审计记录。

skill 生态的变化包括：新增 manifest 规范、权限声明、来源身份、完整性信息和信任状态；
安装前可以做本地/远程预览；执行前会做计划预检；未声明权限、缺少来源身份、不安全脚本
或不支持的 native hook 会被拦截；已安装 skill 可以重置信任、启用/禁用、卸载，并留下
审计记录。

当前仍要说清楚边界：这还不是完整第三方 marketplace，也还没有完成签名分发、发布者信誉
体系和强沙箱隔离。`v0.1.2` 的价值，是先把未来开放 skill 生态所需的安全边界、审计口径
和可卸载路径落到代码里。

For full historical feature notes, see `docs/RELEASE_NOTES_v0.1.0.md` and
`docs/RELEASE_NOTES_v0.1.1.md`.
