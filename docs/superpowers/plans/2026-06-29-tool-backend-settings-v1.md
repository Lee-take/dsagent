# Tool Backend Settings V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Persist the Phase 2 tool backend decisions in the app state, work package surface, and runtime inspector.

**Architecture:** Add typed backend selection enums to `FoundationState` so backend strategy is visible to Rust tests, Tauri commands, TypeScript types, and exported work packages. The settings describe provider intent only; they do not store credentials or execute external tools.

**Tech Stack:** Rust, Serde, Tauri commands, React, TypeScript, CSS.

---

### Task 1: Rust Backend Settings Model

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/models.rs`

- [x] **Step 1: Write the failing test**

Add model tests that expect `FoundationState::default()` to include the confirmed Phase 2 backend choices:

```rust
#[test]
fn foundation_state_defaults_to_confirmed_tool_backends() {
    let state = FoundationState::default();

    assert_eq!(state.tool_backends.network_search, NetworkSearchBackend::DeepSeek);
    assert_eq!(state.tool_backends.email, EmailBackend::ArchitectureOnly);
    assert_eq!(state.tool_backends.drive, DriveBackend::LocalFolderExportPackage);
    assert_eq!(
        state.tool_backends.computer_screenshot,
        ComputerScreenshotBackend::CodexStyleScreenCapture
    );
    assert_eq!(
        state.tool_backends.computer_control,
        ComputerControlBackend::CodexStyleInputControl
    );
}

#[test]
fn tool_backend_settings_serialize_phase_two_choices() {
    let value = serde_json::to_value(ToolBackendSettings::default())
        .expect("settings serialize");

    assert_eq!(value["network_search"], "deepseek");
    assert_eq!(value["email"], "architecture_only");
    assert_eq!(value["drive"], "local_folder_export_package");
    assert_eq!(value["computer_screenshot"], "codex_style_screen_capture");
    assert_eq!(value["computer_control"], "codex_style_input_control");
}
```

- [x] **Step 2: Run test to verify it fails**

Run: `cargo test foundation_state_defaults_to_confirmed_tool_backends tool_backend_settings_serialize_phase_two_choices --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: FAIL because the backend enums and `tool_backends` field do not exist yet.

- [x] **Step 3: Write minimal implementation**

Add these Serde enums and settings struct before `FoundationState`, add `tool_backends: ToolBackendSettings` to `FoundationState`, and default it:

```rust
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSearchBackend {
    DeepSeek,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmailBackend {
    ArchitectureOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DriveBackend {
    LocalFolderExportPackage,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerScreenshotBackend {
    CodexStyleScreenCapture,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerControlBackend {
    CodexStyleInputControl,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToolBackendSettings {
    pub network_search: NetworkSearchBackend,
    pub email: EmailBackend,
    pub drive: DriveBackend,
    pub computer_screenshot: ComputerScreenshotBackend,
    pub computer_control: ComputerControlBackend,
}

impl Default for ToolBackendSettings {
    fn default() -> Self {
        Self {
            network_search: NetworkSearchBackend::DeepSeek,
            email: EmailBackend::ArchitectureOnly,
            drive: DriveBackend::LocalFolderExportPackage,
            computer_screenshot: ComputerScreenshotBackend::CodexStyleScreenCapture,
            computer_control: ComputerControlBackend::CodexStyleInputControl,
        }
    }
}
```

- [x] **Step 4: Run test to verify it passes**

Run: `cargo test foundation_state_defaults_to_confirmed_tool_backends tool_backend_settings_serialize_phase_two_choices --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: PASS.

### Task 2: Frontend Types And Inspector

**Files:**
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/i18n.ts`
- Modify: `apps/desktop/src/styles.css`

- [x] **Step 1: Update TypeScript types**

Add backend union types and `tool_backends` to `FoundationState`:

```ts
export type NetworkSearchBackend = "deepseek";
export type EmailBackend = "architecture_only";
export type DriveBackend = "local_folder_export_package";
export type ComputerScreenshotBackend = "codex_style_screen_capture";
export type ComputerControlBackend = "codex_style_input_control";

export type ToolBackendSettings = {
  network_search: NetworkSearchBackend;
  email: EmailBackend;
  drive: DriveBackend;
  computer_screenshot: ComputerScreenshotBackend;
  computer_control: ComputerControlBackend;
};
```

- [x] **Step 2: Update fallback state**

Add matching `tool_backends` to `fallbackState` in `App.tsx`.

- [x] **Step 3: Add localized labels**

Add `backendOptions` and `backendLabels` strings to `i18n.ts` for Chinese and English:

```ts
backendOptions: {
  network_search: Record<NetworkSearchBackend, string>;
  email: Record<EmailBackend, string>;
  drive: Record<DriveBackend, string>;
  computer_screenshot: Record<ComputerScreenshotBackend, string>;
  computer_control: Record<ComputerControlBackend, string>;
};
backendLabels: {
  title: string;
  networkSearch: string;
  email: string;
  drive: string;
  computerScreenshot: string;
  computerControl: string;
};
```

- [x] **Step 4: Render backend settings in the inspector**

Add a compact `tool-backend-list` section under runtime controls:

```tsx
<section className="tool-backend-list" aria-labelledby="tool-backend-title">
  <strong id="tool-backend-title">{copy.backendLabels.title}</strong>
  <dl>
    <div>
      <dt>{copy.backendLabels.networkSearch}</dt>
      <dd>{copy.backendOptions.network_search[state.tool_backends.network_search]}</dd>
    </div>
    <div>
      <dt>{copy.backendLabels.email}</dt>
      <dd>{copy.backendOptions.email[state.tool_backends.email]}</dd>
    </div>
    <div>
      <dt>{copy.backendLabels.drive}</dt>
      <dd>{copy.backendOptions.drive[state.tool_backends.drive]}</dd>
    </div>
    <div>
      <dt>{copy.backendLabels.computerScreenshot}</dt>
      <dd>
        {
          copy.backendOptions.computer_screenshot[
            state.tool_backends.computer_screenshot
          ]
        }
      </dd>
    </div>
    <div>
      <dt>{copy.backendLabels.computerControl}</dt>
      <dd>{copy.backendOptions.computer_control[state.tool_backends.computer_control]}</dd>
    </div>
  </dl>
</section>
```

- [x] **Step 5: Style backend settings**

Add CSS that keeps the section compact and prevents long labels from overflowing.

- [x] **Step 6: Run frontend build**

Run: `npm run build --prefix apps/desktop`

Expected: PASS.

### Task 3: Documentation And Handoff

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`

- [x] **Step 1: Document confirmed backend decisions**

Add a Phase 2 note:

```markdown
- Tool backend settings now record the confirmed Phase 2 choices:
  - NetworkSearch: DeepSeek model/backend route, with external network access still governed by NetworkSearch permission/audit.
  - EmailRead/Draft/Send: architecture only in this version.
  - DriveRead/Write: local folder and export-package direction.
  - ComputerScreenshot: Codex-style screen pixel capture backend direction.
  - ComputerControl: Codex-style mouse/keyboard control backend direction.
```

- [x] **Step 2: Update handoff**

Record that no API key is stored yet and the next implementation slice should build either the local Drive export package executor or the DeepSeek search adapter credential path.

- [x] **Step 3: Run final verification**

Run:

```powershell
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run build --prefix apps/desktop
git diff --check
```

Expected: all commands exit 0.
