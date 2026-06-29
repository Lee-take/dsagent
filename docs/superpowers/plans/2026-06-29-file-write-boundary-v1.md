# File Write Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a FileWrite boundary that records local file-write requests through policy, audit, and invocation events without modifying files.

**Architecture:** Add a `FileWriteRequest` and boundary runner in the Rust kernel. The runner validates a target path and change summary, respects the existing high-risk FileWrite policy, records `PendingApproval` when policy asks, and otherwise records a blocked invocation because file mutation is not enabled in v1. The React inspector gets a compact write-boundary form.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel FileWrite Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing FileWrite tests**

Add focused tests named with `file_write_boundary`:

```rust
#[test]
fn file_write_boundary_waits_for_approval_when_policy_asks() {
    let outcome = run_file_write_boundary(FileWriteRequest {
        access_mode: AccessMode::AskOnRisk,
        path: "docs/brief.md".to_string(),
        summary: "Update the briefing draft.".to_string(),
        approval_granted: false,
    })
    .expect("file write boundary returns pending result");

    assert_eq!(outcome.access_request.capability, CapabilityKind::FileWrite);
    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("docs/brief.md")
    );
}

#[test]
fn file_write_boundary_blocks_write_after_policy_allows() {
    let outcome = run_file_write_boundary(FileWriteRequest {
        access_mode: AccessMode::FullAccess,
        path: "docs/brief.md".to_string(),
        summary: "Update the briefing draft.".to_string(),
        approval_granted: false,
    })
    .expect("file write boundary records blocked write");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(outcome.invocation.capability, CapabilityKind::FileWrite);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert_eq!(
        outcome.invocation.title.as_deref(),
        Some("File write blocked: docs/brief.md")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("not enabled")));
}

#[test]
fn file_write_boundary_rejects_missing_fields() {
    let error = run_file_write_boundary(FileWriteRequest {
        access_mode: AccessMode::AskOnRisk,
        path: " ".to_string(),
        summary: " ".to_string(),
        approval_granted: false,
    })
    .expect_err("blank file write should fail validation");

    assert!(error.contains("file write path is required"));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_file_write_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml file_write_boundary
```

Expected: FAIL because `FileWriteRequest` and `run_file_write_boundary` do not exist.

- [x] **Step 3: Implement boundary-only runner**

Add `FileWriteRequest`, `FileWriteOutcome`, `run_file_write_boundary`, and validation helpers. Pending policy decisions record `PendingApproval`; allowed or approved requests record `Failed` with a warning that file writing is not enabled and no file was modified.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `write_file_boundary(access_mode, path, summary, state)` that checks `has_user_approved_capability(CapabilityKind::FileWrite)`, calls `run_file_write_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `write_file_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `fileWriteTool` copy for title, path/summary placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector form**

Add React state for target path/change summary, notice/error, and pending. Add a compact File Write Boundary form in the inspector that invokes `write_file_boundary` and refreshes capability state.

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
- Modify: `docs/superpowers/plans/2026-06-29-file-write-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_file_write_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the File Write Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that FileWrite v1 is an approval/audit boundary only and does not modify local files until a file-write executor is designed and explicitly enabled.
