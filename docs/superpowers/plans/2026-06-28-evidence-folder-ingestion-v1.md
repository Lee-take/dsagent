# Evidence Folder Ingestion V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a local evidence-folder ingestion adapter that summarizes selected text files for the future Operations Briefing workflow.

**Architecture:** Build on the existing FileRead capability. The Rust kernel evaluates folder ingestion through `FileRead`, scans a local folder for bounded UTF-8 text files through an injectable client, emits one capability invocation with a manifest-style excerpt, and the UI exposes a compact folder path tool.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Evidence Folder Runtime

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing tests**

Add tests for `run_evidence_folder_ingest` that verify allowed ingestion returns a manifest excerpt and `ask_every_step` waits for approval without scanning.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_evidence_folder_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml evidence_folder
```

Expected: FAIL because the evidence folder runtime does not exist.

- [x] **Step 3: Implement minimal runtime**

Add evidence folder request/source structs, `EvidenceFolderClient`, `LocalEvidenceFolderClient`, and `run_evidence_folder_ingest`.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 2: Tauri Command and UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Expose command**

Add an `ingest_evidence_folder` command that uses the folder runtime and persists the access request, audit entry, and invocation.

- [x] **Step 2: Wire UI**

Add a compact evidence folder path form and reuse the existing tool output list.

- [x] **Step 3: Build**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-evidence-folder-ingestion-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_evidence_folder_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

- [x] **Step 2: Update handoff docs**

Record evidence folder ingestion v1 and remaining Operations Briefing prerequisites.
