# Memory Candidate Review V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first Memory Studio review loop: propose memory candidates, accept or reject them explicitly, and only write accepted candidates into long-term memory.

**Architecture:** Keep existing `MemoryRecord` behavior intact for compatibility, and add a separate candidate event stream with pending/accepted/rejected status. A Tauri command creates candidates, another resolves them, and accepting a candidate appends a normal memory record derived from the candidate.

**Tech Stack:** Rust/Tauri kernel, SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Memory Candidate Model and Store

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/models.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/event_store.rs`

- [x] **Step 1: Write failing tests**

Add model/event-store tests that verify a pending memory candidate can be appended/listed and accepting it writes one memory record.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_candidate_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml memory_candidate
```

Expected: FAIL because memory candidate types and store methods do not exist.

- [x] **Step 3: Implement minimal model/store**

Add `MemoryCandidate`, `MemoryCandidateStatus`, `MemoryCandidateSource`, `MemoryCandidateResolution`, append/list/resolve methods, and accepted-candidate conversion to `MemoryRecord`.

- [x] **Step 4: Verify green**

Run the focused cargo test. Expected: PASS.

### Task 2: Tauri Commands and UI

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Expose commands**

Add `propose_memory_candidate`, `list_memory_candidate_records`, and `resolve_memory_candidate`.

- [x] **Step 2: Wire UI**

Add a compact Memory Studio candidate panel with title/body inputs and accept/reject buttons for pending candidates.

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
- Modify: `docs/superpowers/plans/2026-06-28-memory-candidate-review-v1.md`

- [x] **Step 1: Run full verification**

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_memory_candidate_v1_final'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Update handoff docs**

Record Memory Candidate v1 behavior and remaining Memory Studio metadata gaps.
