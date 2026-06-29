# Imported Memory Candidate Review V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Import memory candidates from work packages into Memory Studio as pending review items without writing long-term memory.

**Architecture:** Extend the work-package schema with `memory_candidates` using serde defaults for legacy packages. Export existing candidate proposals, preview new/skipped candidate IDs, and import only unseen candidates as `MemoryCandidateSource::Import` while leaving them unresolved. Update the desktop UI to show candidate preview/import counts and refresh the Memory Studio candidate list after import.

**Tech Stack:** Rust/Tauri kernel, serde JSON event payloads, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Work Package Candidate Payload

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/work_package.rs`

- [x] **Step 1: Write failing work-package tests**

Add tests named `imported_memory_candidate_export_package_includes_candidates` and `imported_memory_candidate_legacy_package_defaults_candidates`:

```rust
#[test]
fn imported_memory_candidate_export_package_includes_candidates() {
    let candidate = MemoryCandidate::new_with_metadata(
        "Review-safe project rule".to_string(),
        "Imported package candidates must stay pending until accepted locally.".to_string(),
        MemoryCandidateSource::Manual,
        None,
        "Package export should preserve review candidates.".to_string(),
        MemoryType::WorkflowRule,
        MemoryScope::Project,
        MemorySensitivity::Normal,
        MemoryLifecycle::Active,
    )
    .expect("candidate is valid");

    let package = export_work_package(
        FoundationState::default(),
        Vec::new(),
        vec![candidate.clone()],
        Vec::new(),
    );

    assert_eq!(package.memory_candidates, vec![candidate]);
}

#[test]
fn imported_memory_candidate_legacy_package_defaults_candidates() {
    let package_json = serde_json::json!({
        "version": "deepseek-agent-os.work-package.v1",
        "exported_at": chrono::Utc::now(),
        "foundation_state": FoundationState::default(),
        "task_records": [],
        "operations_briefing_runs": []
    })
    .to_string();

    let package = parse_work_package_json(&package_json).expect("legacy package parses");

    assert!(package.memory_candidates.is_empty());
}
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_candidate_import_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml imported_memory_candidate
```

Expected: FAIL because `WorkPackage.memory_candidates` and the expanded `export_work_package` signature do not exist.

- [x] **Step 3: Implement package payload**

Add `MemoryCandidate` to the work-package imports, add `#[serde(default)] pub memory_candidates: Vec<MemoryCandidate>` to `WorkPackage`, and expand `export_work_package` to accept and store `memory_candidates`.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS for the two work-package tests.

### Task 2: Store Preview and Pending Candidate Import

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/work_package.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing store tests**

Add a test named `imported_memory_candidate_imports_new_candidates_as_pending_without_writing_memory`:

```rust
#[test]
fn imported_memory_candidate_imports_new_candidates_as_pending_without_writing_memory() {
    let store = EventStore::open_memory().expect("memory store opens");
    let existing = MemoryCandidate::new(
        "Existing imported rule".to_string(),
        "This candidate is already present locally.".to_string(),
        MemoryCandidateSource::Manual,
        None,
        "Existing local review candidate.".to_string(),
    )
    .expect("candidate is valid");
    let incoming = MemoryCandidate::new_with_metadata(
        "Imported project context".to_string(),
        "Review this package context before saving it as local memory.".to_string(),
        MemoryCandidateSource::Manual,
        None,
        "Imported from a handoff package.".to_string(),
        MemoryType::ProjectContext,
        MemoryScope::Project,
        MemorySensitivity::Sensitive,
        MemoryLifecycle::Active,
    )
    .expect("candidate is valid");

    store
        .append_memory_candidate(&existing)
        .expect("existing candidate appends");

    let summary = store
        .import_memory_candidates(&[existing.clone(), incoming.clone()])
        .expect("candidates import");
    let records = store
        .list_memory_candidate_records()
        .expect("candidate records load");
    let memories = store.list_memory_records().expect("memories load");
    let imported = records
        .iter()
        .find(|record| record.candidate.id == incoming.id)
        .expect("incoming candidate imports");

    assert_eq!(summary.imported, 1);
    assert_eq!(summary.skipped, 1);
    assert_eq!(imported.effective_status, MemoryCandidateStatus::Pending);
    assert_eq!(imported.candidate.source, MemoryCandidateSource::Import);
    assert_eq!(imported.candidate.memory_type, MemoryType::ProjectContext);
    assert_eq!(imported.candidate.scope, MemoryScope::Project);
    assert_eq!(imported.candidate.sensitivity, MemorySensitivity::Sensitive);
    assert!(memories.is_empty());
}
```

Update the import preview test to package one existing and one incoming candidate, then assert:

```rust
assert_eq!(preview.memory_candidates.total, 2);
assert_eq!(preview.memory_candidates.new, 1);
assert_eq!(preview.memory_candidates.skipped, 1);
assert!(preview.memory_candidates.review_supported);
```

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_candidate_import_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml imported_memory_candidate
```

Expected: FAIL because preview/import summary types and `EventStore::import_memory_candidates` do not exist.

- [x] **Step 3: Implement store import**

Add `WorkPackageMemoryCandidateImportSummary { imported, skipped }` and `WorkPackageMemoryCandidateImportPreview { total, new, skipped, review_supported }`. Include them in `WorkPackageImportSummary` and `WorkPackageImportPreview`. Add `EventStore::import_memory_candidates(&[MemoryCandidate])` that dedupes by candidate ID, clones unseen candidates, sets `source = MemoryCandidateSource::Import`, appends only candidate events, and never calls `append_memory_record`.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS for all `imported_memory_candidate` tests.

### Task 3: Command and Frontend Integration

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire commands**

Update `export_work_package` to pass `store.list_memory_candidates()` into `build_work_package`. Update `import_work_package` to set `summary.memory_candidates = store.import_memory_candidates(&package.memory_candidates)?`.

- [x] **Step 2: Update TypeScript types**

Add `memory_candidates: MemoryCandidate[]` to `WorkPackage`, add `WorkPackageMemoryCandidateImportSummary`, add `WorkPackageMemoryCandidateImportPreview`, and include both in the summary and preview types.

- [x] **Step 3: Update UI copy and refresh flow**

Update `copy.package.imported` to accept candidate imported/skipped counts. Add preview labels for memory candidates. In `importWorkPackageJson`, refresh `list_memory_candidate_records` after import and call `setMemoryCandidateRecords`.

- [x] **Step 4: Build frontend**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 4: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-28-imported-memory-candidate-review-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_candidate_import_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the import preview region still renders, includes candidate preview wording in code-backed copy, and has no console warnings or layout overflow on desktop/mobile widths.

- [x] **Step 3: Update docs and mark plan complete**

Record that work-package memory candidates import as pending review candidates and do not write long-term memory until accepted locally.
