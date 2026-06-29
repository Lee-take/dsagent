# Operations Briefing Skeleton V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first Operations Management workflow skeleton: run an Operations Briefing draft from a local evidence folder manifest, with permissioned FileRead ingestion and a persisted workflow run record.

**Architecture:** Keep workflow logic outside the generic kernel models. Add an Operations Briefing runtime that calls evidence-folder ingestion, converts the bounded manifest into a deterministic draft summary/anomaly/action-plan scaffold, and stores the result as an append-only event.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Workflow Runtime

**Files:**
- Add: `apps/desktop/src-tauri/src/kernel/workflow.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/mod.rs`

- [x] **Step 1: Write failing tests**

Add tests for `run_operations_briefing` that verify a successful folder manifest produces a draft-ready run and that `ask_every_step` waits for approval without scanning.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_ops_briefing_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml operations_briefing
```

Expected: FAIL because the workflow runtime does not exist.

- [x] **Step 3: Implement minimal runtime**

Add request/run/section structs and deterministic draft generation from the evidence-folder invocation.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 2: Event Store, Commands, and UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Persist workflow runs**

Add append/list support for Operations Briefing run records.

- [x] **Step 2: Expose commands**

Add `run_operations_briefing` and `list_operations_briefing_runs`, persisting the access request, audit entry, capability invocation, and workflow run.

- [x] **Step 3: Wire UI**

Add a compact Operations Briefing workflow panel that starts from an evidence folder path and displays the latest run output.

- [x] **Step 4: Build**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-operations-briefing-skeleton-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_ops_briefing_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

- [x] **Step 2: Update handoff docs**

Record the Operations Briefing skeleton and the remaining prerequisites for model-backed report generation/export.
