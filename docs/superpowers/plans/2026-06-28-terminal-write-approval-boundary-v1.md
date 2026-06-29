# TerminalWrite Approval Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a TerminalWrite permission boundary that records high-risk write-command requests without executing arbitrary mutating shell commands.

**Architecture:** Reuse the existing capability policy, access-request, audit, and invocation event flow. Add a boundary-only `run_terminal_write` path that normalizes a requested command, asks for approval when policy requires it, and records a blocked invocation after approval or auto-allow because TerminalWrite execution is intentionally not enabled in v1. The React inspector gets a small request surface so users can exercise and review the boundary.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing TerminalWrite tests**

Add tests named with `terminal_write_boundary`:

```rust
#[test]
fn terminal_write_boundary_waits_for_approval_when_policy_asks() {
    let outcome = run_terminal_write_boundary(TerminalWriteRequest {
        access_mode: AccessMode::AskOnRisk,
        command: "npm install".to_string(),
        approval_granted: false,
    })
    .expect("terminal write boundary returns pending result");

    assert_eq!(outcome.access_request.capability, CapabilityKind::TerminalWrite);
    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("npm install")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("requires approval")));
}

#[test]
fn terminal_write_boundary_blocks_execution_after_approval() {
    let outcome = run_terminal_write_boundary(TerminalWriteRequest {
        access_mode: AccessMode::AskOnRisk,
        command: "npm install".to_string(),
        approval_granted: true,
    })
    .expect("terminal write boundary records blocked execution");

    assert_eq!(outcome.access_request.capability, CapabilityKind::TerminalWrite);
    assert_eq!(outcome.invocation.capability, CapabilityKind::TerminalWrite);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("npm install")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("not enabled")));
}

#[test]
fn terminal_write_boundary_rejects_blank_commands() {
    let error = run_terminal_write_boundary(TerminalWriteRequest {
        access_mode: AccessMode::FullAccess,
        command: "   ".to_string(),
        approval_granted: false,
    })
    .expect_err("blank command should fail validation");

    assert!(error.contains("terminal write command is required"));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_terminal_write_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml terminal_write_boundary
```

Expected: FAIL because `TerminalWriteRequest` and `run_terminal_write_boundary` do not exist.

- [x] **Step 3: Implement boundary-only capability**

Add `TerminalWriteRequest`, `TerminalWriteOutcome`, `run_terminal_write_boundary`, and `normalize_terminal_write_command`. Pending requests use status `PendingApproval`; approved or auto-allowed requests use status `Failed` with a warning that TerminalWrite execution is not enabled and no command was run.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `run_terminal_write(access_mode, command, state)` that checks `has_user_approved_capability(CapabilityKind::TerminalWrite)`, calls `run_terminal_write_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `run_terminal_write` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Extend `terminalTool` copy with write-boundary labels, request/pending/blocked messages, and a placeholder. Chinese copy should say the command will not run in v1.

- [x] **Step 4: Add inspector form**

Add React state for `terminalWriteCommand`, `terminalWriteNotice`, `terminalWriteError`, and `terminalWritePending`. Add a compact Terminal Write Boundary form under Terminal Read that invokes `run_terminal_write`; show pending approval when the invocation status is `pending_approval`, otherwise show the boundary blocked message.

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
- Modify: `docs/superpowers/plans/2026-06-28-terminal-write-approval-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_terminal_write_boundary_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Terminal Read and Terminal Write Boundary controls render on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that TerminalWrite v1 is an approval/audit boundary only and does not execute mutating commands.
