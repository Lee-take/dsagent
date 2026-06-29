# Terminal Read Adapter V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a permissioned TerminalRead adapter that runs bounded, read-only workspace diagnostics and records command output as tool evidence.

**Architecture:** Extend the existing capability runtime with a TerminalRead request/client/outcome. The runtime evaluates `CapabilityKind::TerminalRead`, allows only a small preset allowlist of read-only commands, emits a generic capability invocation, and the UI exposes a compact command selector rather than arbitrary shell input.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: TerminalRead Runtime

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing tests**

Add tests for `run_terminal_read` that verify an allowed read-only command returns structured output and that `ask_every_step` waits for approval without running the command.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_terminal_read_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml terminal_read
```

Expected: FAIL because the TerminalRead runtime does not exist.

- [x] **Step 3: Implement minimal runtime**

Add `TerminalReadRequest`, `TerminalCommandOutput`, `TerminalReadClient`, `LocalTerminalReadClient`, and `run_terminal_read`.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 2: Tauri Command and UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Expose command**

Add a `run_terminal_read` command that uses the terminal runtime and persists the access request, audit entry, and invocation.

- [x] **Step 2: Wire UI**

Add a compact Terminal Tool preset selector and reuse the existing tool output list.

- [x] **Step 3: Build**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-terminal-read-adapter-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_terminal_read_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Update handoff docs**

Record TerminalRead v1 behavior, allowlisted commands, and remaining TerminalWrite boundary.
