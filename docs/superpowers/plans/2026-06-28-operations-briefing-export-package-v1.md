# Operations Briefing Export Package V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Include Operations Briefing workflow runs in exported work-package JSON so a briefing run can be handed off with its evidence trace, summary, anomalies, and action plan.

**Architecture:** Extend the existing `deepseek-agent-os.work-package.v1` payload with a serde-defaulted `operations_briefing_runs` array. Export reads persisted workflow runs from the event store; import keeps its existing task-record-only behavior for now, so archived run replay remains a later explicit feature.

**Tech Stack:** Rust/Tauri kernel, serde JSON work packages, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Work-Package Model

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/work_package.rs`

- [x] **Step 1: Write failing tests**

Add tests proving exported packages include briefing runs and legacy package JSON without `operations_briefing_runs` still parses:

```rust
#[test]
fn operations_export_package_includes_briefing_runs() {
    let run = sample_operations_briefing_run();
    let package = export_work_package(FoundationState::default(), Vec::new(), vec![run.clone()]);

    assert_eq!(package.operations_briefing_runs, vec![run]);
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_operations_export_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml operations_export
```

Expected: FAIL because `WorkPackage` has no `operations_briefing_runs` field and `export_work_package` does not accept workflow runs.

- [x] **Step 3: Implement model extension**

Add `operations_briefing_runs: Vec<OperationsBriefingRun>` to `WorkPackage` with `#[serde(default)]`, update `export_work_package(...)`, and keep `parse_work_package_json(...)` on the same package version.

- [x] **Step 4: Verify green**

Run the focused cargo test command again. Expected: PASS.

### Task 2: Command and UI Export Hook

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Include runs in export command**

Update the existing `export_work_package` command to read `store.list_operations_briefing_runs()` and pass those runs into `build_work_package(...)`.

- [x] **Step 2: Update frontend contract**

Add `operations_briefing_runs: OperationsBriefingRun[]` to the TypeScript `WorkPackage` type and add an export button on the latest Operations Briefing run that fills the existing exported package JSON textarea.

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
- Modify: `docs/superpowers/plans/2026-06-28-operations-briefing-export-package-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_operations_export_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the latest Operations Briefing card exposes the export button when a run exists or the page still renders cleanly when no run exists.

- [x] **Step 3: Update docs and mark plan complete**

Record that exported work packages now carry Operations Briefing run history while import/replay of runs remains future work.
