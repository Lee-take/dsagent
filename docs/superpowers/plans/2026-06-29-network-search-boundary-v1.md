# Network Search Boundary V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a NetworkSearch boundary that records public web-search requests through policy, audit, and invocation events without performing network search.

**Architecture:** Add a `NetworkSearchRequest` and boundary runner in the Rust kernel. The runner validates a query and optional source scope, respects the existing NetworkSearch policy, records `PendingApproval` when policy asks, and otherwise records a blocked invocation because search execution is not enabled in v1. The React inspector gets a compact Network Search Boundary form.

**Tech Stack:** Rust/Tauri kernel, append-only SQLite event store, React/TypeScript UI, existing pnpm workspace.

---

### Task 1: Kernel NetworkSearch Boundary

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write failing NetworkSearch tests**

Add focused tests named with `network_search_boundary` for approval pending, allowed-but-blocked execution, and missing query validation.

- [x] **Step 2: Verify red**

Run:

```powershell
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_network_search_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml network_search_boundary
```

Expected: FAIL because `NetworkSearchRequest` and `run_network_search_boundary` do not exist.

- [x] **Step 3: Implement boundary-only runner**

Add `NetworkSearchRequest`, `NetworkSearchOutcome`, `run_network_search_boundary`, and validation helpers. Pending policy decisions record `PendingApproval`; allowed or approved requests record `Failed` with a warning that search execution is not enabled and no network request was made.

- [x] **Step 4: Verify green**

Run the focused cargo command again. Expected: PASS.

### Task 2: Tauri Command and UI Surface

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/App.tsx`

- [x] **Step 1: Wire the Tauri command**

Add `search_network_boundary(access_mode, query, scope, state)` that checks `has_user_approved_capability(CapabilityKind::NetworkSearch)`, calls `run_network_search_boundary`, appends the access request when appropriate, appends a permission audit entry, and appends the invocation.

- [x] **Step 2: Register the command**

Expose `search_network_boundary` in `apps/desktop/src-tauri/src/main.rs`.

- [x] **Step 3: Add UI copy**

Add `networkSearchTool` copy for title, query/scope placeholders, request button, pending hint, blocked result, and failure text in Chinese and English.

- [x] **Step 4: Add inspector form**

Add React state for query/scope, notice/error, and pending. Add a compact Network Search Boundary form in the inspector that invokes `search_network_boundary` and refreshes capability state.

- [x] **Step 5: Build frontend**

Run:

```powershell
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript and Vite build succeed.

### Task 3: Verification and Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-29-network-search-boundary-v1.md`

- [x] **Step 1: Run full verification**

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR = Join-Path $env:TEMP 'deepseek_ui_cargo_test_network_search_boundary_v1'
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npx pnpm@9.15.9 --filter @deepseek-agent-os/desktop build
git diff --check
```

- [x] **Step 2: Browser smoke**

Open `http://127.0.0.1:1420/`, confirm the Network Search Boundary form renders on desktop and mobile without console warnings or layout overflow. In Vite browser mode, do not submit the Tauri command because IPC is unavailable.

- [x] **Step 3: Update docs and mark plan complete**

Record that NetworkSearch v1 is an approval/audit boundary only and does not search the public web until a search provider is selected and explicitly enabled.
