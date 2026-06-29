# Email Read Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an EmailRead boundary that records mailbox-read requests through policy, audit, and invocation events without reading a real mailbox.

**Architecture:** Add an `EmailReadRequest` and boundary runner in the Rust kernel. The runner validates mailbox and query text, respects the existing medium-risk EmailRead policy, records `PendingApproval` when policy asks, and otherwise records a blocked invocation because no mailbox connector is wired in v1. The React inspector gets a compact read-boundary form.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel EmailRead Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing EmailRead tests**

Add focused tests named with `email_read_boundary`:

```rust
#[test]
fn email_read_boundary_waits_for_approval_when_policy_asks() {
    let outcome = run_email_read_boundary(EmailReadRequest {
        access_mode: AccessMode::AskOnRisk,
        mailbox: "Inbox".to_string(),
        query: "weekly brief".to_string(),
        approval_granted: false,
    })
    .expect("email read boundary returns pending result");

    assert_eq!(outcome.access_request.capability, CapabilityKind::EmailRead);
    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("Inbox: weekly brief")
    );
}

#[test]
fn email_read_boundary_blocks_read_after_policy_allows() {
    let outcome = run_email_read_boundary(EmailReadRequest {
        access_mode: AccessMode::FullAccess,
        mailbox: "Inbox".to_string(),
        query: "weekly brief".to_string(),
        approval_granted: false,
    })
    .expect("email read boundary records blocked read");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(outcome.invocation.capability, CapabilityKind::EmailRead);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert_eq!(
        outcome.invocation.title.as_deref(),
        Some("Email read blocked: Inbox")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("not enabled")));
}

#[test]
fn email_read_boundary_rejects_missing_fields() {
    let error = run_email_read_boundary(EmailReadRequest {
        access_mode: AccessMode::AskOnRisk,
        mailbox: " ".to_string(),
        query: " ".to_string(),
        approval_granted: false,
    })
    .expect_err("blank email read should fail validation");

    assert!(error.contains("email mailbox is required"));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_email_read_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml email_read_boundary
```

Expected: FAIL because `EmailReadRequest` and `run_email_read_boundary` do not exist.

- [ ] **Step 3: Implement boundary-only runner**

Add `EmailReadRequest`, `EmailReadOutcome`, `run_email_read_boundary`, and validation helpers. Pending policy decisions record `PendingApproval`; allowed requests record `Failed` with a warning that mailbox reading is not enabled and no email was read.

- [ ] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `read_email_boundary(access_mode, mailbox, query, state)` that checks `has_user_approved_capability(CapabilityKind::EmailRead)`, calls `run_email_read_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `read_email_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `emailReadTool` copy for title, mailbox/query placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector form**

Add React state for mailbox/query, notice/error, and pending. Add a compact Email Read Boundary form in the inspector that invokes `read_email_boundary` and refreshes capability state.

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
- Modify: `docs/superpowers/plans/2026-06-28-email-read-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_email_read_boundary_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Email Read Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that EmailRead v1 is an approval/audit boundary only and does not read mail until a mailbox connector is selected.
