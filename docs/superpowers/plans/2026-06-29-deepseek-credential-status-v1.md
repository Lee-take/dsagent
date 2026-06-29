# DeepSeek Credential Status V1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a safe DeepSeek API credential status path without storing or exposing API keys.

**Architecture:** The Rust DeepSeek module exposes a serializable credential-status struct built from environment lookup. Tauri exposes the status to the React inspector. The status reports `DEEPSEEK_API_KEY` presence and the DeepSeek API base URL, but never returns the secret value and never includes credentials in work packages.

**Tech Stack:** Rust, Serde, Tauri commands, React, TypeScript.

---

### Task 1: Rust Credential Status

**Files:**
- Modify: `apps/desktop/src-tauri/src/kernel/deepseek.rs`

- [x] **Step 1: Write failing tests**

Add tests:

```rust
#[test]
fn credential_status_reports_missing_env_key_without_secret() {
    let status = deepseek_credential_status_from_env(|_| None);

    assert_eq!(status.base_url, DEEPSEEK_API_BASE_URL);
    assert_eq!(status.api_key_env_var, DEEPSEEK_API_KEY_ENV);
    assert!(!status.api_key_configured);
}

#[test]
fn credential_status_reports_present_env_key_without_serializing_secret() {
    let status = deepseek_credential_status_from_env(|name| {
        if name == DEEPSEEK_API_KEY_ENV {
            Some("test-secret-token".to_string())
        } else {
            None
        }
    });
    let serialized = serde_json::to_string(&status).expect("status serializes");

    assert!(status.api_key_configured);
    assert!(!serialized.contains("test-secret-token"));
}
```

- [x] **Step 2: Run tests to verify they fail**

Run: `$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test credential_status --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: FAIL because the constants, struct, and function do not exist yet.

- [x] **Step 3: Implement credential status**

Add:

```rust
pub const DEEPSEEK_API_BASE_URL: &str = "https://api.deepseek.com";
pub const DEEPSEEK_API_KEY_ENV: &str = "DEEPSEEK_API_KEY";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeepSeekCredentialStatus {
    pub base_url: String,
    pub api_key_env_var: String,
    pub api_key_configured: bool,
}

pub fn deepseek_credential_status_from_env(
    read_env: impl Fn(&str) -> Option<String>,
) -> DeepSeekCredentialStatus {
    let api_key_configured = read_env(DEEPSEEK_API_KEY_ENV)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);

    DeepSeekCredentialStatus {
        base_url: DEEPSEEK_API_BASE_URL.to_string(),
        api_key_env_var: DEEPSEEK_API_KEY_ENV.to_string(),
        api_key_configured,
    }
}

pub fn current_deepseek_credential_status() -> DeepSeekCredentialStatus {
    deepseek_credential_status_from_env(|name| std::env::var(name).ok())
}
```

- [x] **Step 4: Run tests to verify they pass**

Run: `$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test credential_status --manifest-path apps/desktop/src-tauri/Cargo.toml`

Expected: PASS.

### Task 2: Tauri Command And Frontend

**Files:**
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `apps/desktop/src-tauri/src/main.rs`
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src/i18n.ts`

- [x] **Step 1: Add Tauri command**

Expose:

```rust
#[tauri::command]
pub fn get_deepseek_credential_status() -> DeepSeekCredentialStatus {
    current_deepseek_credential_status()
}
```

Register it in `main.rs`.

- [x] **Step 2: Add TypeScript type**

Add:

```ts
export type DeepSeekCredentialStatus = {
  base_url: string;
  api_key_env_var: string;
  api_key_configured: boolean;
};
```

- [x] **Step 3: Load and render credential status**

In `App.tsx`, load `get_deepseek_credential_status` during initial state load and render a compact row in the backend strategy section:

```tsx
<div>
  <dt>{copy.backendLabels.deepSeekApi}</dt>
  <dd>
    {deepSeekCredentialStatus.api_key_configured
      ? copy.backendLabels.apiKeyConfigured
      : copy.backendLabels.apiKeyMissing}
  </dd>
</div>
```

- [x] **Step 4: Add localized labels**

Add Chinese and English labels for DeepSeek API, configured, missing, base URL, and env var.

### Task 3: Docs And Verification

**Files:**
- Modify: `README.md`
- Modify: `SESSION_HANDOFF.md`
- Modify: `docs/superpowers/plans/2026-06-29-deepseek-credential-status-v1.md`

- [x] **Step 1: Document the credential path**

Document that `DEEPSEEK_API_KEY` is read from the local environment, that the key value is not serialized, and that work packages do not include secrets.

- [x] **Step 2: Mark checkboxes**

Mark completed plan steps with `[x]`.

- [x] **Step 3: Run final verification**

Run:

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml
$env:CARGO_TARGET_DIR='D:\deepseek-ui-target'; cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm run build --prefix apps/desktop
git diff --check
```

Expected: all commands exit 0 except harmless CRLF warnings from Git.
