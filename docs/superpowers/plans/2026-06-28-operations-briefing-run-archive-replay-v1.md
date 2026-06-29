# Operations Briefing Run Archive Replay V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Import Operations Briefing runs from work packages as read-only archived runs so handoff packages can be replayed in the local run list without re-executing tools.

**Architecture:** Add an `archived_from_package` marker to `OperationsBriefingRun` with serde defaulting for legacy runs. Extend work-package import summaries with briefing-run import/skip counts, dedupe imported runs by run ID, and mark imported runs as archived before appending them to the event store.

**Tech Stack:** Rust/Tauri kernel, serde JSON event payloads, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Archived Run Model and Store Import

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/workflow.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/work_package.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing tests**

Add focused tests with `archive_replay` in their names:

```rust
#[test]
fn archive_replay_legacy_run_json_defaults_to_local_run() {
    let run: OperationsBriefingRun = serde_json::from_value(serde_json::json!({
        "id": uuid::Uuid::new_v4(),
        "workflow_id": OPERATIONS_BRIEFING_WORKFLOW_ID,
        "status": "draft_ready",
        "evidence_folder_path": "D:\\evidence",
        "evidence_invocation_id": uuid::Uuid::new_v4(),
        "title": "Operations Briefing Draft",
        "summary": "Legacy run payload.",
        "anomalies": [],
        "action_plan": [],
        "warnings": [],
        "created_at": chrono::Utc::now()
    })).expect("legacy run deserializes");

    assert!(!run.archived_from_package);
}
```

Add an event-store test that imports two package runs where one already exists locally, then asserts one run is imported, one skipped, and the imported run has `archived_from_package=true`.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_archive_replay_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml archive_replay
```

Expected: FAIL because `archived_from_package` and run import summary/store methods do not exist.

- [x] **Step 3: Implement archived run import**

Add `archived_from_package: bool` with `#[serde(default)]` to `OperationsBriefingRun`. Add `WorkPackageOperationsBriefingImportSummary`, include it in `WorkPackageImportSummary`, and add `EventStore::import_operations_briefing_runs(...)` that skips duplicate IDs and appends imported clones marked as archived.

- [x] **Step 4: Verify green**

Run the focused cargo test again. Expected: PASS.

### Task 2: Command and Frontend Integration

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Import archived runs from command**

Update `import_work_package` to import task records and archived briefing runs, returning both summaries.

- [x] **Step 2: Surface archived runs in UI**

Refresh `operationsBriefingRuns` after package import. Show an archive badge on imported runs and include run import/skip counts in the import success message.

- [x] **Step 3: Build frontend**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-operations-briefing-run-archive-replay-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_archive_replay_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the import controls render without console warnings or layout overflow on desktop and mobile widths. The archived-run data path is covered by the Rust store test and TypeScript build because the Vite browser smoke does not have live Tauri IPC.

- [x] **Step 3: Update docs and mark plan complete**

Record that imported Operations Briefing runs are archived read-only records and that replay does not rerun tools.
