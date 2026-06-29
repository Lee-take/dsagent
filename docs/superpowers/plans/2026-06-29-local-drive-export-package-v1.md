# Local Drive Export Package V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn DriveRead and DriveWrite from approval-only stubs into local-folder read and work-package export executors.

**Architecture:** Add a `DriveLocalFolderClient` trait to the capability kernel. `DriveRead` uses the client to scan bounded local text files in a selected folder, while `DriveWrite` writes the current work package JSON into a selected local folder after policy approval. Cloud drive accounts remain out of scope for this version.

**Tech Stack:** Rust, Tauri commands, Serde work packages, React copy updates, local filesystem.

---

### Task 1: Local Drive Capability Tests

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`

- [x] **Step 1: Write the failing DriveRead test**

Add a test that uses a real temporary folder and expects DriveRead to return a successful manifest for files matching the query:

```rust
#[test]
fn drive_read_local_folder_returns_matching_manifest_after_policy_allows() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let budget_path = temp_dir.path().join("budget-plan.md");
    let ops_path = temp_dir.path().join("operations.md");
    std::fs::write(&budget_path, "Budget assumptions for 2026.").expect("write budget");
    std::fs::write(&ops_path, "Operations notes.").expect("write ops");
    let client = LocalDriveFolderClient::new(10, 512 * 1024);

    let outcome = run_drive_read_boundary(
        DriveReadRequest {
            access_mode: AccessMode::AskOnRisk,
            location: temp_dir.path().to_string_lossy().to_string(),
            query: "budget".to_string(),
            approval_granted: false,
        },
        &client,
    )
    .expect("drive read returns local folder manifest");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(outcome.invocation.capability, CapabilityKind::DriveRead);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Succeeded);
    assert!(outcome.invocation.excerpt.as_deref().unwrap_or_default().contains("budget-plan.md"));
    assert!(!outcome.invocation.excerpt.as_deref().unwrap_or_default().contains("operations.md"));
}
```

- [x] **Step 2: Write the failing DriveWrite test**

Add a test that writes package JSON to a real temporary folder after policy allows:

```rust
#[test]
fn drive_write_local_export_package_writes_json_after_policy_allows() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let client = LocalDriveFolderClient::new(10, 512 * 1024);
    let package_json = r#"{"version":"deepseek-agent-os.work-package.v1"}"#.to_string();

    let outcome = run_drive_write_boundary(
        DriveWriteRequest {
            access_mode: AccessMode::FullAccess,
            location: temp_dir.path().to_string_lossy().to_string(),
            summary: "Export current work package".to_string(),
            package_json: Some(package_json.clone()),
            approval_granted: false,
        },
        &client,
    )
    .expect("drive write exports local package");

    assert_eq!(outcome.access_request.decision, PolicyDecision::Allow);
    assert_eq!(outcome.invocation.capability, CapabilityKind::DriveWrite);
    assert_eq!(outcome.invocation.status, CapabilityInvocationStatus::Succeeded);
    let output_path = outcome.invocation.evidence_ref.expect("output path");
    assert!(output_path.ends_with(".json"));
    assert_eq!(std::fs::read_to_string(output_path).expect("read package"), package_json);
}
```

- [x] **Step 3: Run tests to verify they fail**

Run: `$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test drive_read_local_folder_returns_matching_manifest_after_policy_allows drive_write_local_export_package_writes_json_after_policy_allows --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: FAIL because `LocalDriveFolderClient`, the injected client signature, and `package_json` field do not exist yet.

### Task 2: Local Drive Kernel Implementation

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/capability.rs`
- Modify: `apps/desktop/src-tauri/src/commands.rs`

- [x] **Step 1: Add Drive local types and trait**

Add:

```rust
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DriveFolderEntry {
    pub path: String,
    pub title: String,
    pub bytes: u64,
    pub excerpt: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DriveReadResult {
    pub location: String,
    pub query: String,
    pub entries: Vec<DriveFolderEntry>,
    pub total_bytes: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DriveWriteResult {
    pub path: String,
    pub bytes: u64,
}

pub trait DriveLocalFolderClient {
    fn read_local_folder(&self, location: &str, query: &str) -> Result<DriveReadResult, String>;
    fn write_export_package(
        &self,
        location: &str,
        summary: &str,
        package_json: &str,
    ) -> Result<DriveWriteResult, String>;
}
```

- [x] **Step 2: Implement `LocalDriveFolderClient`**

Implement `LocalDriveFolderClient::new(max_files, max_file_bytes)`. It should:

- require `location` to be an existing folder,
- scan top-level supported text files using the existing `is_supported_text_file`,
- ignore files over `max_file_bytes`,
- match query case-insensitively against file name or text,
- return bounded entries with short excerpts,
- write export package JSON to `deepseek-agent-os-work-package-<uuid>.json` in the target folder.

- [x] **Step 3: Update DriveRead boundary to execute local folder reads**

Change `run_drive_read_boundary(request)` to `run_drive_read_boundary(request, client)`. Keep the existing pending-approval path unchanged. When policy allows or approval exists, call `client.read_local_folder` and return:

- `status=Succeeded` on success,
- `status=Failed` with the client error as warning on failure,
- `requested_resource` and `evidence_ref` pointing at the local folder.

- [x] **Step 4: Update DriveWrite boundary to execute local package export**

Add `package_json: Option<String>` to `DriveWriteRequest`. Keep pending approval unchanged. When policy allows or approval exists:

- require `package_json` to be present and non-empty,
- call `client.write_export_package`,
- return `status=Succeeded`, `evidence_ref=<written file path>`, and title `Drive export package written`.

- [x] **Step 5: Update commands**

In `read_drive_boundary`, instantiate `LocalDriveFolderClient::new(20, 512 * 1024)` and pass it to `run_drive_read_boundary`.

In `write_drive_boundary`, build the current `WorkPackage` from the event store, serialize it with `serde_json::to_string_pretty`, instantiate `LocalDriveFolderClient`, pass `package_json: Some(package_json)` to `run_drive_write_boundary`, then record the access request, audit entry, and invocation as before.

- [x] **Step 6: Update existing Drive tests**

Update pending and validation tests to pass `&LocalDriveFolderClient` and `package_json` where needed. Replace the old "blocks write/read after policy allows" expectations with the successful local read/write behavior from Task 1.

- [x] **Step 7: Run focused Rust tests**

Run: `$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test drive_read_ drive_write_ --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: PASS.

### Task 3: UI Copy, Docs, And Verification

**Files:**
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-29-local-drive-export-package-v1.md`

- [x] **Step 1: Update UI copy**

Update DriveRead/DriveWrite copy so it says local folder/export package rather than cloud-drive account:

```ts
driveReadTool: {
  title: "Drive Read Local Folder",
  locationPlaceholder: "Local folder path",
  blocked: "DriveRead reads a bounded local folder after approval.",
}
driveWriteTool: {
  title: "Drive Export Package",
  locationPlaceholder: "Target local export folder",
  summaryPlaceholder: "Export package summary",
  blocked: "DriveWrite exports the current work package JSON to a local folder after approval.",
}
```

- [x] **Step 2: Update README and handoff**

Document that DriveRead now scans local folders and DriveWrite now writes local work-package JSON exports after policy approval. Note that cloud drive accounts are still deferred.

- [x] **Step 3: Mark plan checkboxes**

Mark completed plan steps with `[x]`.

- [x] **Step 4: Run final verification**

Run:

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run build --prefix apps/desktop
git diff --check
```

Expected: all commands exit 0 except harmless CRLF warnings from Git.
