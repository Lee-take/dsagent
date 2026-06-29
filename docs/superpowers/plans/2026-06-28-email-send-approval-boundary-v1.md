# Email Send Approval Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an EmailSend approval boundary that records outbound-email send requests through policy, audit, and invocation events without sending mail.

**Architecture:** Add an `EmailSendRequest` and boundary runner in the Rust kernel. The runner validates recipient, subject, and body, always respects EmailSend critical approval policy, and records a blocked invocation after approval because no mailbox connector is wired in v1. The React inspector gets a compact send-boundary form so users can exercise the approval loop.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel EmailSend Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing EmailSend tests**

Add focused tests named with `email_send_boundary`:

```rust
#[test]
fn email_send_boundary_waits_for_approval_even_in_full_access() {
    let outcome = run_email_send_boundary(EmailSendRequest {
        access_mode: AccessMode::FullAccess,
        to: "ops@example.com".to_string(),
        subject: "Weekly brief".to_string(),
        body: "Please review the attached operating notes.".to_string(),
        approval_granted: false,
    })
    .expect("email send boundary returns pending result");

    assert_eq!(outcome.access_request.capability, CapabilityKind::EmailSend);
    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(
        outcome.invocation.requested_resource.as_deref(),
        Some("ops@example.com")
    );
}

#[test]
fn email_send_boundary_blocks_send_after_approval() {
    let outcome = run_email_send_boundary(EmailSendRequest {
        access_mode: AccessMode::FullAccess,
        to: "ops@example.com".to_string(),
        subject: "Weekly brief".to_string(),
        body: "Please review the attached operating notes.".to_string(),
        approval_granted: true,
    })
    .expect("email send boundary records blocked send");

    assert_eq!(outcome.invocation.capability, CapabilityKind::EmailSend);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert_eq!(
        outcome.invocation.title.as_deref(),
        Some("Email send blocked: Weekly brief")
    );
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("not enabled")));
}

#[test]
fn email_send_boundary_rejects_missing_fields() {
    let error = run_email_send_boundary(EmailSendRequest {
        access_mode: AccessMode::AskOnRisk,
        to: " ".to_string(),
        subject: " ".to_string(),
        body: " ".to_string(),
        approval_granted: false,
    })
    .expect_err("blank email should fail validation");

    assert!(error.contains("email recipient is required"));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_email_send_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml email_send_boundary
```

Expected: FAIL because `EmailSendRequest` and `run_email_send_boundary` do not exist.

- [x] **Step 3: Implement boundary-only runner**

Add `EmailSendRequest`, `EmailSendOutcome`, `run_email_send_boundary`, and validation helpers. Pending policy decisions record `PendingApproval`; approved requests record `Failed` with a warning that email sending is not enabled and no email was sent.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `send_email_boundary(access_mode, to, subject, body, state)` that checks `has_user_approved_capability(CapabilityKind::EmailSend)`, calls `run_email_send_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `send_email_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `emailTool` copy for title, to/subject/body placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector form**

Add React state for email to/subject/body, notice/error, and pending. Add a compact Email Send Boundary form in the inspector that invokes `send_email_boundary` and refreshes capability state.

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
- Modify: `docs/superpowers/plans/2026-06-28-email-send-approval-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_email_send_boundary_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Email Send Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that EmailSend v1 is an approval/audit boundary only and does not send mail until a mailbox connector is selected.
