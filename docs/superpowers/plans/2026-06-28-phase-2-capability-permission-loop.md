# Phase 2 Capability Permission Loop Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first runnable tool-access and permission-closure slice for DeepSeek Agent OS.

**Architecture:** Keep real adapters deferred, but make the kernel contract real: built-in capability catalog, policy-backed access request, pending approval, user resolution, and append-only audit events. The React workbench consumes those commands so users can see which connector families exist and close approval requests from the inspector.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel Capability Contract

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/policy.rs`

- [x] **Step 1: Write failing tests**

Add tests for the built-in capability catalog and access-request status mapping:

```rust
#[test]
fn builtin_catalog_declares_phase_two_connector_families() {
    let catalog = builtin_capability_catalog();

    assert!(catalog.iter().any(|capability| capability.capability == CapabilityKind::BrowserBrowse));
    assert!(catalog.iter().any(|capability| capability.capability == CapabilityKind::EmailRead));
    assert!(catalog.iter().any(|capability| capability.capability == CapabilityKind::DriveRead));
    assert!(catalog.iter().any(|capability| capability.capability == CapabilityKind::ComputerScreenshot));
    assert!(catalog.iter().any(|capability| capability.capability == CapabilityKind::ComputerControl && capability.experimental));
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_target'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml builtin_catalog_declares_phase_two_connector_families
```

Expected: FAIL because the catalog and new request types do not exist yet.

- [x] **Step 3: Implement minimal kernel types**

Add `CapabilityFamily`, `CapabilityDescriptor`, `CapabilityAccessStatus`, `CapabilityAccessRequest`, `PermissionResolution`, and `builtin_capability_catalog`.

- [x] **Step 4: Verify green**

Run the same focused cargo test. Expected: PASS.

### Task 2: Event Store Closure

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing tests**

Add tests that append a pending critical request, resolve it, and compute no pending requests afterwards.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_target'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml resolves_pending_capability_access_request
```

Expected: FAIL because persistence helpers do not exist yet.

- [x] **Step 3: Implement event helpers**

Add request/resolution event types plus append/list/pending/resolve helpers.

- [x] **Step 4: Verify green**

Run the same focused cargo test. Expected: PASS.

### Task 3: Tauri Commands and UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Expose commands**

Add commands for listing capabilities, requesting access, listing access records, and approving/rejecting requests.

- [x] **Step 2: Wire UI**

Replace the old three-button preflight with a connector matrix and pending approval list.

- [x] **Step 3: Build**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 4: Verification and Handoff

**Files:**
- Modify: `SESSION_HANDOFF.md`
- Modify: `README.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_target'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

- [x] **Step 2: Update handoff docs**

Record the completed Phase 2 slice and next adapter priorities.
