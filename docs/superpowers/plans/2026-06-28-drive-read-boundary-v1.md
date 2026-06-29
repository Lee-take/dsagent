# Drive Read Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a DriveRead boundary that records cloud-drive read requests through policy, audit, and invocation events without reading a real drive account.

**Architecture:** Add a `DriveReadRequest` and boundary runner in the Rust kernel. The runner validates drive location and query text, respects the existing low-risk DriveRead policy, records `PendingApproval` when policy asks, and otherwise records a blocked invocation because no cloud-drive connector is wired in v1. The React inspector gets a compact read-boundary form.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel DriveRead Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing DriveRead tests**

Add focused tests named with `drive_read_boundary`:

```rust
#[test]
fn drive_read_boundary_waits_for_approval_when_policy_asks() {
    let outcome = run_drive_read_boundary(DriveReadRequest {
        access_mode: AccessMode::AskEveryStep,
        location: "Shared drive".to_string(),
        query: "2026 budget".to_string(),
        approval_granted: false,
    })
    .expect("drive read boundary returns pending result");

    assert_eq!(outcome.access_request.capability, CapabilityKind::DriveRead);
    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("Shared drive: 2026 budget")
    );
}

#[test]
fn drive_read_boundary_blocks_read_after_policy_allows() {
    let outcome = run_drive_read_boundary(DriveReadRequest {
        access_mode: AccessMode::AskOnRisk,
        location: "Shared drive".to_string(),
        query: "2026 budget".to_string(),
        approval_granted: false,
    })
    .expect("drive read boundary records blocked read");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(outcome.invocation.capability, CapabilityKind::DriveRead);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert_eq!(
        outcome.invocation.title.as_deref(),
        Some("Drive read blocked: Shared drive")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("not enabled")));
}

#[test]
fn drive_read_boundary_rejects_missing_fields() {
    let error = run_drive_read_boundary(DriveReadRequest {
        access_mode: AccessMode::AskOnRisk,
        location: " ".to_string(),
        query: " ".to_string(),
        approval_granted: false,
    })
    .expect_err("blank drive read should fail validation");

    assert!(error.contains("drive location is required"));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_drive_read_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml drive_read_boundary
```

Expected: FAIL because `DriveReadRequest` and `run_drive_read_boundary` do not exist.

- [x] **Step 3: Implement boundary-only runner**

Add `DriveReadRequest`, `DriveReadOutcome`, `run_drive_read_boundary`, and validation helpers. Pending policy decisions record `PendingApproval`; allowed requests record `Failed` with a warning that drive reading is not enabled and no drive file was read.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [ ] **Step 1: Wire the Tauri command**

Add `read_drive_boundary(access_mode, location, query, state)` that checks `has_user_approved_capability(CapabilityKind::DriveRead)`, calls `run_drive_read_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [ ] **Step 2: Register the command**

Expose `read_drive_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [ ] **Step 3: Add UI copy**

Add `driveReadTool` copy for title, location/query placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [ ] **Step 4: Add inspector form**

Add React state for drive location/query, notice/error, and pending. Add a compact Drive Read Boundary form in the inspector that invokes `read_drive_boundary` and refreshes capability state.

- [ ] **Step 5: Build frontend**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-drive-read-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_drive_read_boundary_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Drive Read Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that DriveRead v1 is an approval/audit boundary only and does not read cloud-drive files until a drive connector is selected.
