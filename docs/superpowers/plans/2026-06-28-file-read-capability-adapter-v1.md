# File Read Capability Adapter V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a local evidence-file read adapter behind the same Phase 2 permission and tool-output loop.

**Architecture:** Keep the adapter local-first. The Rust kernel evaluates `FileRead` through policy, reads a selected local file through an injectable client, emits a generic capability invocation with resource/evidence references, and the UI exposes a compact file-path tool beside browser browse.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: File Read Kernel Runtime

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing tests**

Add tests for `run_file_read` that verify allowed reads return title/excerpt/evidence ref and `ask_every_step` waits for approval without reading.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_file_read_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml file_read
```

Expected: FAIL because the file read runtime does not exist.

- [x] **Step 3: Implement minimal runtime**

Add file read request/source structs, `FileContentClient`, `LocalFileContentClient`, and `run_file_read`.

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

Add a `read_local_file` command that uses `run_file_read`, persists the access request, audit entry, and invocation.

- [x] **Step 2: Wire UI**

Add a compact file path form and make the existing tool output list display browser and file invocation evidence.

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
- Modify: `docs/superpowers/plans/2026-06-28-file-read-capability-adapter-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_file_read_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

- [x] **Step 2: Update handoff docs**

Record FileRead adapter v1, remaining connector order, and current external-decision points.
