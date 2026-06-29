# Computer Screenshot Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a Computer Screenshot read-only capability boundary that flows through policy, audit, invocation recording, and UI without enabling computer control.

**Architecture:** Add a `ComputerScreenshotClient` trait and capability runner in the Rust kernel. Tests use a fake client to verify success, pending approval, and capture failure behavior; the desktop command uses a local boundary client that records a clear unsupported warning until a platform screenshot backend is selected. The React inspector gets a compact button to exercise the read-only screenshot boundary.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel Screenshot Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing ComputerScreenshot tests**

Add focused tests named with `computer_screenshot_boundary`:

```rust
#[test]
fn computer_screenshot_boundary_returns_structured_capture_result() {
    let client = FakeComputerScreenshotClient::new();
    let outcome = run_computer_screenshot(
        ComputerScreenshotRequest {
            access_mode: AccessMode::AskOnRisk,
            approval_granted: false,
        },
        &client,
    )
    .expect("computer screenshot succeeds");

    assert_eq!(
        outcome.access_request.capability,
        CapabilityKind::ComputerScreenshot
    );
    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(
        outcome.invocation.capability,
        CapabilityKind::ComputerScreenshot
    );
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::Succeeded
    );
    assert_eq!(
        outcome.invocation.title.as_deref(),
        Some("Computer screenshot: Primary display")
    );
    assert!(outcome
        .invocation
        .excerpt
        .as_deref()
        .expect("excerpt exists")
        .contains("1920x1080"));
    assert_eq!(client.calls.get(), 1);
}

#[test]
fn computer_screenshot_boundary_waits_for_approval_when_policy_asks() {
    let client = FakeComputerScreenshotClient::new();
    let outcome = run_computer_screenshot(
        ComputerScreenshotRequest {
            access_mode: AccessMode::AskEveryStep,
            approval_granted: false,
        },
        &client,
    )
    .expect("computer screenshot returns pending result");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Ask);
    assert_eq!(
        outcome.invocation.status,
        CapabilityInvocationStatus::PendingApproval
    );
    assert_eq!(client.calls.get(), 0);
}

#[test]
fn computer_screenshot_boundary_records_capture_failure() {
    let client = FakeComputerScreenshotClient::failing();
    let outcome = run_computer_screenshot(
        ComputerScreenshotRequest {
            access_mode: AccessMode::AskOnRisk,
            approval_granted: false,
        },
        &client,
    )
    .expect("computer screenshot records failure");

    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Failed);
    assert!(outcome
        .invocation
        .warnings
        .iter()
        .any(|warning| warning.contains("capture backend unavailable")));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_computer_screenshot_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml computer_screenshot_boundary
```

Expected: FAIL because `ComputerScreenshotRequest`, `ComputerScreenshotClient`, and `run_computer_screenshot` do not exist.

- [x] **Step 3: Implement kernel boundary**

Add `ComputerScreenshotRequest`, `ComputerScreenshot`, `ComputerScreenshotClient`, `LocalComputerScreenshotClient`, `ComputerScreenshotOutcome`, and `run_computer_screenshot`. Pending policy decisions must not call the client. Successful captures record display label and dimensions in the invocation excerpt. Capture failures record a failed invocation with warnings.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `capture_computer_screenshot(access_mode, state)` that checks `has_user_approved_capability(CapabilityKind::ComputerScreenshot)`, calls `run_computer_screenshot` with `LocalComputerScreenshotClient`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `capture_computer_screenshot` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `computerTool` copy for title, capture button, capturing state, pending hint, unavailable result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector control**

Add React state for `computerNotice`, `computerError`, and `computerPending`. Add a compact Computer Screenshot form/button in the inspector that invokes `capture_computer_screenshot` and refreshes capability state.

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
- Modify: `docs/superpowers/plans/2026-06-28-computer-screenshot-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_computer_screenshot_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Computer Screenshot control renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that Computer Screenshot v1 is wired through policy/audit/invocation flow, with local pixel capture awaiting a selected platform backend.
