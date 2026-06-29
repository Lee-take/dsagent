# Browser Capability Adapter V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first real capability adapter behind the Phase 2 permission loop: browser browse/open URL with structured tool output.

**Architecture:** Keep the adapter local and policy-gated. The Rust kernel owns a browser capability runtime that evaluates `BrowserBrowse`, records the access request and audit entry, fetches or summarizes a URL through an injectable client, persists the resulting capability invocation event, and returns a structured result to the UI.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Browser Adapter Kernel

**Files:**
- Create: `apps/desktop/src-tauri/src/kernel/capability.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/mod.rs`

- [x] **Step 1: Write failing tests**

Add tests that verify a browser browse request is policy-gated and produces a structured result with title, excerpt, elapsed time, and evidence URL.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_browser_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml browser_browse_returns_structured_tool_result
```

Expected: FAIL because the capability runtime does not exist.

- [x] **Step 3: Implement minimal kernel runtime**

Add browser request/result structs, an injectable `BrowserPageClient` trait, text extraction helpers, and a `run_browser_browse` function.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 2: Event Store Invocation Records

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing tests**

Add tests that append and list browser capability invocation records.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_browser_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml appends_and_lists_capability_invocations
```

Expected: FAIL because invocation persistence helpers do not exist.

- [x] **Step 3: Implement event helpers**

Add a `capability_invocation.recorded` event type and append/list helpers.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 3: Tauri Command and UI Tool Output

**Files:**
- Modify: `apps/desktop/src-tauri/Cargo.toml`
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Expose command**

Add a `browse_url` command that uses the browser adapter and persists the invocation.

- [x] **Step 2: Wire UI**

Add a compact browser URL form and a recent tool-output list in the inspector.

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
- Modify: `docs/superpowers/plans/2026-06-28-browser-capability-adapter-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_browser_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

- [x] **Step 2: Update handoff docs**

Record browser adapter v1, remaining connector order, and any limits.
