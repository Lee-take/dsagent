# Memory Studio Metadata V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add reviewed memory metadata for type, scope, sensitivity, and lifecycle so accepted Memory Studio candidates keep explicit governance tags.

**Architecture:** Extend `MemoryCandidate` and `MemoryRecord` with serde-defaulted metadata fields to keep old event payloads readable. The Tauri command accepts explicit metadata from the Memory Studio form, and accepting a candidate copies those metadata tags into the long-term memory record.

**Tech Stack:** Rust/Tauri kernel, serde JSON event payloads, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Metadata Model Compatibility

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/models.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing Rust tests**

Add tests named with the `memory_metadata` prefix:

```rust
#[test]
fn memory_metadata_candidate_defaults_are_review_safe() {
    let candidate = MemoryCandidate::new(
        "Preferred report tone".to_string(),
        "Use concise operating language with clear owners.".to_string(),
        MemoryCandidateSource::Manual,
        None,
        "User proposed this as reusable guidance.".to_string(),
    )
    .expect("candidate is valid");

    assert_eq!(candidate.memory_type, MemoryType::Preference);
    assert_eq!(candidate.scope, MemoryScope::Workspace);
    assert_eq!(candidate.sensitivity, MemorySensitivity::Normal);
    assert_eq!(candidate.lifecycle, MemoryLifecycle::Active);
}
```

Add a store test that creates a candidate with `MemoryCandidate::new_with_metadata(...)`, accepts it, and verifies the resulting `MemoryRecord` preserves the four metadata fields.

Add legacy JSON tests that deserialize a `MemoryCandidate` and `MemoryRecord` without the new fields and verify the same default values are applied.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_metadata_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory_metadata
```

Expected: FAIL because `MemoryType`, `MemoryScope`, `MemorySensitivity`, `MemoryLifecycle`, and metadata fields do not exist.

- [x] **Step 3: Implement minimal metadata model**

Add enums to `models.rs`:

```rust
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    Preference,
    ProjectContext,
    WorkflowRule,
    Artifact,
    FailurePattern,
}
```

Use the same pattern for `MemoryScope`, `MemorySensitivity`, and `MemoryLifecycle`. Add default helper functions and `#[serde(default = "...")]` on the four new fields in both `MemoryCandidate` and `MemoryRecord`. Add `MemoryCandidate::new_with_metadata(...)`; keep `MemoryCandidate::new(...)` as the defaulting constructor.

- [x] **Step 4: Verify green**

Run the focused cargo test command again. Expected: PASS.

### Task 2: Commands and Frontend Metadata Controls

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Extend command and TypeScript types**

Expose `memory_type`, `scope`, `sensitivity`, and `lifecycle` in `propose_memory_candidate`, and add matching TypeScript unions and object fields.

- [x] **Step 2: Add Memory Studio selectors**

Add compact selects for the four metadata fields in the candidate form. Send camelCase Tauri args:

```ts
await invoke<MemoryCandidateRecord>("propose_memory_candidate", {
  title: candidateTitle,
  body: candidateBody,
  memoryType: candidateMemoryType,
  scope: candidateMemoryScope,
  sensitivity: candidateSensitivity,
  lifecycle: candidateLifecycle,
});
```

Render metadata badges on candidate rows and memory rows so review state is visible before and after acceptance.

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
- Modify: `docs/superpowers/plans/2026-06-28-memory-studio-metadata-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_metadata_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Memory Studio form shows metadata selectors, the page has no framework overlay, and the mobile viewport keeps the form readable.

- [x] **Step 3: Update docs and mark plan complete**

Record that Memory Studio metadata v1 preserves governance tags through candidate review and legacy event loading.
