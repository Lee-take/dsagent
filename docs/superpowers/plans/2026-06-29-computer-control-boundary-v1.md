# Computer Control Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a ComputerControl boundary that records mouse/keyboard control requests through policy, audit, and invocation events without controlling the desktop.

**Architecture:** Add a `ComputerControlRequest` and boundary runner in the Rust kernel. The runner validates a target/context and action summary, respects the existing critical ComputerControl policy, records `PendingApproval` when policy asks, and otherwise records a blocked invocation because desktop control execution is not enabled in v1. The React inspector gets a compact Computer Control Boundary form.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel ComputerControl Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing ComputerControl tests**

Add focused tests named with `computer_control_boundary` for full-access approval pending, approved-but-blocked execution, and missing field validation.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_computer_control_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml computer_control_boundary
```

Expected: FAIL because `ComputerControlRequest` and `run_computer_control_boundary` do not exist.

- [x] **Step 3: Implement boundary-only runner**

Add `ComputerControlRequest`, `ComputerControlOutcome`, and `run_computer_control_boundary`. Pending policy decisions record `PendingApproval`; approved requests record `Failed` with a warning that desktop control is not enabled and no mouse or keyboard action was performed.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `control_computer_boundary(access_mode, target, action, state)` that checks `has_user_approved_capability(CapabilityKind::ComputerControl)`, calls `run_computer_control_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `control_computer_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `computerControlTool` copy for title, target/action placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector form**

Add React state for target/action summary, notice/error, and pending. Add a compact Computer Control Boundary form in the inspector that invokes `control_computer_boundary` and refreshes capability state.

- [x] **Step 5: Build frontend**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-29-computer-control-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_computer_control_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Computer Control Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that ComputerControl v1 is an approval/audit boundary only and does not move the mouse, type, click, or otherwise control the desktop until a desktop-control executor is designed and explicitly enabled.
