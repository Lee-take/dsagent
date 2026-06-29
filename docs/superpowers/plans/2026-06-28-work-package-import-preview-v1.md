# Work Package Import Preview V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a read-only work-package import preview so users can inspect new/skipped task records and archived Operations Briefing runs before committing an import.

**Architecture:** Extend the work-package model with preview structs, compute duplicate task IDs in the event store without appending events, and expose a Tauri preview command. The React import area gains a preview button and a compact summary; importing still writes only task records while briefing runs remain archived package content.

**Tech Stack:** Rust/Tauri kernel, serde JSON work packages, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Preview Model and Store

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/work_package.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing tests**

Add focused tests named with `import_preview`:

```rust
#[test]
fn import_preview_counts_new_skipped_tasks_and_briefing_archives_without_writing() {
    let store = EventStore::open_memory().expect("memory store opens");
    let existing = TaskRecord::new("Existing".to_string(), "Already local.".to_string())
        .expect("record is valid");
    let incoming = TaskRecord::new("Incoming".to_string(), "New handoff task.".to_string())
        .expect("record is valid");
    store.append_task_record(&existing).expect("existing appends");

    let package = export_work_package(
        FoundationState::default(),
        vec![existing.clone(), incoming],
        vec![sample_operations_briefing_run()],
    );
    let preview = store.preview_work_package_import(&package).expect("preview loads");

    assert_eq!(preview.task_records.total, 2);
    assert_eq!(preview.task_records.new, 1);
    assert_eq!(preview.task_records.skipped, 1);
    assert_eq!(preview.operations_briefing_runs.total, 1);
    assert!(preview.operations_briefing_runs.replay_supported);
    assert_eq!(store.list_task_records().expect("records load"), vec![existing]);
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_import_preview_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml import_preview
```

Expected: FAIL because preview structs and `preview_work_package_import` do not exist.

- [x] **Step 3: Implement preview model/store**

Add `WorkPackageImportPreview`, `WorkPackageTaskImportPreview`, and `WorkPackageOperationsBriefingImportPreview` to `work_package.rs`. Add `EventStore::preview_work_package_import(&WorkPackage)` that compares incoming task IDs to existing task IDs and reports briefing run count with `replay_supported` set according to the current archive-import capability.

- [x] **Step 4: Verify green**

Run the focused cargo test again. Expected: PASS.

### Task 2: Command and Frontend Preview UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Expose preview command**

Add `preview_work_package_import(package_json, state)` that parses the package JSON and returns `WorkPackageImportPreview`.

- [x] **Step 2: Add UI preview**

Add a preview button next to import. Show counts for total/new/skipped task records and archived Operations Briefing runs. Clear stale preview after a successful import or when the pasted JSON changes.

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
- Modify: `docs/superpowers/plans/2026-06-28-work-package-import-preview-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_import_preview_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the import preview button and empty preview state render without console warnings or layout overflow.

- [x] **Step 3: Update docs and mark plan complete**

Record that import preview is read-only and that briefing runs can import as read-only archives once archive replay is enabled.
