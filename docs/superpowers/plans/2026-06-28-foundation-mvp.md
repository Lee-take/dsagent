# Foundation MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create the first runnable DeepSeek Agent OS desktop foundation: repository baseline, Tauri/React shell, Rust kernel modules, SQLite event store, policy model, DeepSeek route model, and a visible workbench UI skeleton.

**Architecture:** Start with a local-first monorepo. The React app owns the workbench UI, while Tauri/Rust owns local persistence, policy-sensitive logic, and kernel commands. This slice proves the shape of the Agent OS without implementing full email, drive, browser, memory graph, or Computer Use adapters.

**Tech Stack:** Tauri 2, React, TypeScript, Vite, Rust, rusqlite, serde, uuid, chrono, pnpm.

---

## Scope

This plan implements the foundation only:

- Git repository and monorepo metadata.
- Desktop shell under `apps/desktop`.
- Rust kernel modules inside `apps/desktop/src-tauri/src/kernel`.
- SQLite event store with tests.
- Policy engine with tests.
- DeepSeek route/thinking model with tests.
- Tauri commands that expose the foundation state.
- React workbench shell with model, access, thinking, and scope controls.

The following systems get separate plans:

- Memory Kernel and Memory Studio.
- Real DeepSeek API calls.
- Email, drive, browser, and Computer Use adapters.
- Operations Briefing workflow implementation.
- Import/export package implementation.

## File Structure

Create this structure:

```text
D:\deepseek UI
  package.json
  pnpm-workspace.yaml
  .gitignore
  README.md
  apps/
    desktop/
      package.json
      index.html
      vite.config.ts
      tsconfig.json
      src/
        App.tsx
        main.tsx
        styles.css
        types.ts
      src-tauri/
        Cargo.toml
        build.rs
        tauri.conf.json
        src/
          main.rs
          commands.rs
          kernel/
            mod.rs
            event_store.rs
            models.rs
            policy.rs
            deepseek.rs
```

Responsibilities:

- `package.json`: workspace scripts.
- `apps/desktop/src`: React workbench shell only.
- `apps/desktop/src-tauri/src/main.rs`: Tauri bootstrap.
- `commands.rs`: Tauri command boundary.
- `kernel/models.rs`: stable kernel data types.
- `kernel/event_store.rs`: SQLite event persistence.
- `kernel/policy.rs`: capability-specific policy decisions.
- `kernel/deepseek.rs`: DeepSeek route and thinking defaults.

---

### Task 1: Initialize Repository Baseline

**Files:**
- Create: `.gitignore`
- Create: `README.md`
- Create: `package.json`
- Create: `pnpm-workspace.yaml`

- [ ] **Step 1: Initialize git**

Run:

```powershell
git init
```

Expected: command exits with code 0 and `.git` exists in `D:\deepseek UI`.

- [ ] **Step 2: Create root package metadata**

Create `package.json`:

```json
{
  "name": "deepseek-agent-os",
  "private": true,
  "version": "0.1.0",
  "description": "Local-first desktop Agent OS optimized for DeepSeek.",
  "scripts": {
    "dev": "pnpm --filter @deepseek-agent-os/desktop dev",
    "build": "pnpm --filter @deepseek-agent-os/desktop build",
    "tauri": "pnpm --filter @deepseek-agent-os/desktop tauri"
  },
  "packageManager": "pnpm@9.15.9"
}
```

Create `pnpm-workspace.yaml`:

```yaml
packages:
  - "apps/*"
```

Create `.gitignore`:

```gitignore
node_modules/
dist/
target/
.DS_Store
*.log
*.tmp
.env
.env.*
!.env.example
apps/desktop/src-tauri/target/
apps/desktop/src-tauri/.tauri/
```

Create `README.md`:

```markdown
# DeepSeek Agent OS

Local-first open-source desktop Agent OS optimized for DeepSeek.

Read first:

- `PROJECT_CONTEXT.md`
- `DECISIONS.md`
- `SESSION_HANDOFF.md`
- `docs/superpowers/specs/2026-06-28-deepseek-agent-os-architecture-design.md`

## Development

```powershell
pnpm install
pnpm dev
```

## Architecture

The app uses a stable Agent OS Kernel with Workflow Packs. The first implementation slice builds the desktop shell, local event store, policy model, and DeepSeek route model.
```

- [ ] **Step 3: Verify root files**

Run:

```powershell
rg --files -g "!_reference_repos/**"
```

Expected output includes:

```text
package.json
pnpm-workspace.yaml
README.md
PROJECT_CONTEXT.md
DECISIONS.md
SESSION_HANDOFF.md
docs\superpowers\specs\2026-06-28-deepseek-agent-os-architecture-design.md
docs\superpowers\plans\2026-06-28-foundation-mvp.md
```

- [ ] **Step 4: Commit repository baseline**

Run:

```powershell
git add .gitignore README.md package.json pnpm-workspace.yaml PROJECT_CONTEXT.md DECISIONS.md SESSION_HANDOFF.md docs
git commit -m "docs: establish agent os project baseline"
```

Expected: commit succeeds.

---

### Task 2: Create Tauri React Desktop Shell

**Files:**
- Create: `apps/desktop/package.json`
- Create: `apps/desktop/index.html`
- Create: `apps/desktop/vite.config.ts`
- Create: `apps/desktop/tsconfig.json`
- Create: `apps/desktop/src/main.tsx`
- Create: `apps/desktop/src/App.tsx`
- Create: `apps/desktop/src/styles.css`
- Create: `apps/desktop/src/types.ts`
- Create: `apps/desktop/src-tauri/Cargo.toml`
- Create: `apps/desktop/src-tauri/build.rs`
- Create: `apps/desktop/src-tauri/tauri.conf.json`
- Create: `apps/desktop/src-tauri/src/main.rs`

- [ ] **Step 1: Write desktop package files**

Create `apps/desktop/package.json`:

```json
{
  "name": "@deepseek-agent-os/desktop",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite --host 127.0.0.1",
    "build": "tsc --noEmit && vite build",
    "preview": "vite preview --host 127.0.0.1",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.5.0",
    "lucide-react": "^0.468.0",
    "react": "^18.3.1",
    "react-dom": "^18.3.1"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.5.0",
    "@types/react": "^18.3.12",
    "@types/react-dom": "^18.3.1",
    "@vitejs/plugin-react": "^4.3.4",
    "typescript": "^5.6.3",
    "vite": "^5.4.11"
  }
}
```

Create `apps/desktop/index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>DeepSeek Agent OS</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

Create `apps/desktop/vite.config.ts`:

```ts
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
});
```

Create `apps/desktop/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["DOM", "DOM.Iterable", "ES2020"],
    "allowJs": false,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "module": "ESNext",
    "moduleResolution": "Node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx"
  },
  "include": ["src"],
  "references": []
}
```

- [ ] **Step 2: Write initial React files**

Create `apps/desktop/src/types.ts`:

```ts
export type ModelRoute = "auto" | "deepseek-v4-flash" | "deepseek-v4-pro";

export type ThinkingLevel = "auto" | "fast" | "standard" | "deep";

export type AccessMode = "ask_every_step" | "ask_on_risk" | "limited_auto" | "full_access";

export type WorkspaceScope = "current_file" | "current_folder" | "workspace";

export type FoundationState = {
  appName: string;
  modelRoute: ModelRoute;
  thinkingLevel: ThinkingLevel;
  accessMode: AccessMode;
  workspaceScope: WorkspaceScope;
};
```

Create `apps/desktop/src/main.tsx`:

```tsx
import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

Create `apps/desktop/src/App.tsx`:

```tsx
import { invoke } from "@tauri-apps/api/core";
import { Brain, Database, FolderOpen, ShieldCheck } from "lucide-react";
import { useEffect, useState } from "react";
import type { FoundationState } from "./types";

const fallbackState: FoundationState = {
  appName: "DeepSeek Agent OS",
  modelRoute: "auto",
  thinkingLevel: "auto",
  accessMode: "ask_on_risk",
  workspaceScope: "workspace",
};

export function App() {
  const [state, setState] = useState<FoundationState>(fallbackState);
  const keepInitialControl = () => undefined;

  useEffect(() => {
    void invoke<FoundationState>("get_foundation_state")
      .then(setState)
      .catch(() => setState(fallbackState));
  }, []);

  return (
    <main className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">D</div>
          <div>
            <strong>{state.appName}</strong>
            <span>Local-first Agent OS</span>
          </div>
        </div>
        <nav className="nav-list" aria-label="Primary">
          <button className="nav-item active" type="button">
            <FolderOpen size={18} /> Workbench
          </button>
          <button className="nav-item" type="button">
            <Database size={18} /> Memory
          </button>
          <button className="nav-item" type="button">
            <ShieldCheck size={18} /> Approvals
          </button>
        </nav>
      </aside>

      <section className="workspace">
        <header className="toolbar">
          <select value={state.modelRoute} aria-label="Model route" onChange={keepInitialControl}>
            <option value="auto">DeepSeek Auto</option>
            <option value="deepseek-v4-flash">DeepSeek Flash</option>
            <option value="deepseek-v4-pro">DeepSeek Pro</option>
          </select>
          <select value={state.accessMode} aria-label="Access mode" onChange={keepInitialControl}>
            <option value="ask_every_step">Every step asks</option>
            <option value="ask_on_risk">Ask on risk</option>
            <option value="limited_auto">Limited auto</option>
            <option value="full_access">Full access</option>
          </select>
          <select value={state.thinkingLevel} aria-label="Thinking level" onChange={keepInitialControl}>
            <option value="auto">Thinking auto</option>
            <option value="fast">Fast</option>
            <option value="standard">Standard</option>
            <option value="deep">Deep</option>
          </select>
        </header>

        <section className="workbench">
          <div className="timeline">
            <p className="eyebrow">Foundation MVP</p>
            <h1>Operations Briefing Workbench</h1>
            <p className="summary">
              The first runnable slice proves the desktop shell, policy controls,
              DeepSeek routing defaults, and local kernel boundary.
            </p>
          </div>
          <aside className="inspector">
            <div className="inspector-header">
              <Brain size={18} />
              <strong>Runtime Controls</strong>
            </div>
            <dl>
              <div>
                <dt>Model</dt>
                <dd>{state.modelRoute}</dd>
              </div>
              <div>
                <dt>Access</dt>
                <dd>{state.accessMode}</dd>
              </div>
              <div>
                <dt>Thinking</dt>
                <dd>{state.thinkingLevel}</dd>
              </div>
              <div>
                <dt>Scope</dt>
                <dd>{state.workspaceScope}</dd>
              </div>
            </dl>
          </aside>
        </section>
      </section>
    </main>
  );
}
```

Create `apps/desktop/src/styles.css`:

```css
:root {
  color-scheme: dark;
  font-family:
    Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  background: #111315;
  color: #f4f1ea;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 960px;
  min-height: 720px;
  background: #111315;
}

button,
select {
  font: inherit;
}

.app-shell {
  display: grid;
  grid-template-columns: 248px minmax(0, 1fr);
  min-height: 100vh;
}

.sidebar {
  border-right: 1px solid #2c3034;
  background: #17191c;
  padding: 18px 14px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 26px;
}

.brand-mark {
  display: grid;
  width: 38px;
  height: 38px;
  place-items: center;
  border: 1px solid #3a3f45;
  border-radius: 8px;
  background: #202327;
  color: #8fd0bc;
  font-weight: 700;
}

.brand strong,
.brand span {
  display: block;
}

.brand span {
  margin-top: 2px;
  color: #a7adb5;
  font-size: 12px;
}

.nav-list {
  display: grid;
  gap: 6px;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 38px;
  width: 100%;
  border: 1px solid transparent;
  border-radius: 7px;
  background: transparent;
  color: #c7ccd2;
  padding: 0 10px;
  text-align: left;
}

.nav-item.active {
  border-color: #3a4c48;
  background: #202b28;
  color: #f4f1ea;
}

.workspace {
  min-width: 0;
  background: #111315;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 10px;
  min-height: 58px;
  border-bottom: 1px solid #2c3034;
  padding: 0 18px;
}

.toolbar select {
  height: 34px;
  border: 1px solid #3a3f45;
  border-radius: 7px;
  background: #1a1d20;
  color: #f4f1ea;
  padding: 0 10px;
}

.workbench {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 360px;
  gap: 18px;
  padding: 18px;
}

.timeline,
.inspector {
  border: 1px solid #2c3034;
  border-radius: 8px;
  background: #17191c;
}

.timeline {
  min-height: 520px;
  padding: 24px;
}

.eyebrow {
  margin: 0 0 10px;
  color: #8fd0bc;
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
}

h1 {
  margin: 0;
  font-size: 30px;
  line-height: 1.2;
}

.summary {
  max-width: 680px;
  color: #b8bec5;
  font-size: 15px;
  line-height: 1.6;
}

.inspector {
  align-self: start;
  padding: 16px;
}

.inspector-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
}

dl {
  display: grid;
  gap: 10px;
  margin: 0;
}

dl div {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  border-top: 1px solid #2c3034;
  padding-top: 10px;
}

dt {
  color: #a7adb5;
}

dd {
  margin: 0;
  color: #f4f1ea;
  font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", monospace;
  font-size: 12px;
}
```

- [ ] **Step 3: Write Tauri Rust shell files**

Create `apps/desktop/src-tauri/Cargo.toml`:

```toml
[package]
name = "deepseek-agent-os-desktop"
version = "0.1.0"
description = "DeepSeek Agent OS desktop shell"
edition = "2021"

[lib]
name = "deepseek_agent_os_desktop"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2.2.0", features = [] }

[dependencies]
chrono = { version = "0.4", features = ["serde"] }
rusqlite = { version = "0.32", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tauri = { version = "2.5.0", features = [] }
thiserror = "1"
uuid = { version = "1", features = ["v4", "serde"] }

[dev-dependencies]
tempfile = "3"
```

Create `apps/desktop/src-tauri/build.rs`:

```rust
fn main() {
    tauri_build::build();
}
```

Create `apps/desktop/src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "DeepSeek Agent OS",
  "version": "0.1.0",
  "identifier": "ai.deepseek-agent-os.desktop",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build",
    "devUrl": "http://127.0.0.1:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "DeepSeek Agent OS",
        "width": 1280,
        "height": 820,
        "minWidth": 960,
        "minHeight": 720
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": false,
    "targets": "all",
    "icon": []
  }
}
```

Create `apps/desktop/src-tauri/src/main.rs`:

```rust
mod commands;
mod kernel;

use commands::get_foundation_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_foundation_state])
        .run(tauri::generate_context!())
        .expect("failed to run DeepSeek Agent OS desktop app");
}

fn main() {
    run();
}
```

- [ ] **Step 4: Install dependencies and run frontend build**

Run:

```powershell
pnpm install
pnpm --filter @deepseek-agent-os/desktop build
```

Expected: TypeScript build and Vite build complete without errors.

- [ ] **Step 5: Commit desktop shell**

Run:

```powershell
git add apps package.json pnpm-lock.yaml
git commit -m "feat: scaffold tauri desktop shell"
```

Expected: commit succeeds.

---

### Task 3: Add Kernel Models and DeepSeek Defaults

**Files:**
- Create: `apps/desktop/src-tauri/src/kernel/mod.rs`
- Create: `apps/desktop/src-tauri/src/kernel/models.rs`
- Create: `apps/desktop/src-tauri/src/kernel/deepseek.rs`
- Create: `apps/desktop/src-tauri/src/commands.rs`

- [ ] **Step 1: Write kernel module declarations**

Create `apps/desktop/src-tauri/src/kernel/mod.rs`:

```rust
pub mod deepseek;
pub mod event_store;
pub mod models;
pub mod policy;
```

- [ ] **Step 2: Write shared kernel models**

Create `apps/desktop/src-tauri/src/kernel/models.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelRoute {
    Auto,
    DeepSeekV4Flash,
    DeepSeekV4Pro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThinkingLevel {
    Auto,
    Fast,
    Standard,
    Deep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
    AskEveryStep,
    AskOnRisk,
    LimitedAuto,
    FullAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceScope {
    CurrentFile,
    CurrentFolder,
    Workspace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoundationState {
    pub app_name: String,
    pub model_route: ModelRoute,
    pub thinking_level: ThinkingLevel,
    pub access_mode: AccessMode,
    pub workspace_scope: WorkspaceScope,
}

impl Default for FoundationState {
    fn default() -> Self {
        Self {
            app_name: "DeepSeek Agent OS".to_string(),
            model_route: ModelRoute::Auto,
            thinking_level: ThinkingLevel::Auto,
            access_mode: AccessMode::AskOnRisk,
            workspace_scope: WorkspaceScope::Workspace,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload_json: String,
    pub created_at: DateTime<Utc>,
}

impl KernelEvent {
    pub fn new(event_type: impl Into<String>, payload: &impl Serialize) -> Result<Self, serde_json::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            event_type: event_type.into(),
            payload_json: serde_json::to_string(payload)?,
            created_at: Utc::now(),
        })
    }
}
```

- [ ] **Step 3: Write DeepSeek defaults**

Create `apps/desktop/src-tauri/src/kernel/deepseek.rs`:

```rust
use super::models::{ModelRoute, ThinkingLevel};

pub const DEEPSEEK_AUTO_LABEL: &str = "DeepSeek Auto";
pub const DEEPSEEK_FLASH_MODEL: &str = "deepseek-v4-flash";
pub const DEEPSEEK_PRO_MODEL: &str = "deepseek-v4-pro";

pub fn effective_model(route: ModelRoute, thinking: ThinkingLevel) -> &'static str {
    match route {
        ModelRoute::DeepSeekV4Flash => DEEPSEEK_FLASH_MODEL,
        ModelRoute::DeepSeekV4Pro => DEEPSEEK_PRO_MODEL,
        ModelRoute::Auto => match thinking {
            ThinkingLevel::Fast => DEEPSEEK_FLASH_MODEL,
            ThinkingLevel::Auto | ThinkingLevel::Standard | ThinkingLevel::Deep => DEEPSEEK_PRO_MODEL,
        },
    }
}

pub fn thinking_budget_name(thinking: ThinkingLevel) -> &'static str {
    match thinking {
        ThinkingLevel::Auto => "auto",
        ThinkingLevel::Fast => "none",
        ThinkingLevel::Standard => "high",
        ThinkingLevel::Deep => "max",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_fast_uses_flash() {
        assert_eq!(effective_model(ModelRoute::Auto, ThinkingLevel::Fast), DEEPSEEK_FLASH_MODEL);
    }

    #[test]
    fn auto_deep_uses_pro() {
        assert_eq!(effective_model(ModelRoute::Auto, ThinkingLevel::Deep), DEEPSEEK_PRO_MODEL);
    }

    #[test]
    fn thinking_budget_names_are_stable() {
        assert_eq!(thinking_budget_name(ThinkingLevel::Fast), "none");
        assert_eq!(thinking_budget_name(ThinkingLevel::Standard), "high");
        assert_eq!(thinking_budget_name(ThinkingLevel::Deep), "max");
    }
}
```

- [ ] **Step 4: Write Tauri command boundary**

Create `apps/desktop/src-tauri/src/commands.rs`:

```rust
use crate::kernel::models::FoundationState;

#[tauri::command]
pub fn get_foundation_state() -> FoundationState {
    FoundationState::default()
}
```

- [ ] **Step 5: Run Rust tests**

Run:

```powershell
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

Expected: tests pass, including `kernel::deepseek::tests`.

- [ ] **Step 6: Commit kernel model defaults**

Run:

```powershell
git add apps/desktop/src-tauri/src
git commit -m "feat: add kernel models and deepseek defaults"
```

Expected: commit succeeds.

---

### Task 4: Add SQLite Event Store

**Files:**
- Create: `apps/desktop/src-tauri/src/kernel/event_store.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/mod.rs`

- [ ] **Step 1: Write failing event store tests with implementation**

Create `apps/desktop/src-tauri/src/kernel/event_store.rs`:

```rust
use rusqlite::{params, Connection};
use thiserror::Error;

use super::models::KernelEvent;

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("time parse error: {0}")]
    Time(#[from] chrono::ParseError),
    #[error("uuid parse error: {0}")]
    Uuid(#[from] uuid::Error),
}

pub type EventStoreResult<T> = Result<T, EventStoreError>;

pub struct EventStore {
    conn: Connection,
}

impl EventStore {
    pub fn open(path: impl AsRef<std::path::Path>) -> EventStoreResult<Self> {
        let conn = Connection::open(path)?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    pub fn open_memory() -> EventStoreResult<Self> {
        let conn = Connection::open_in_memory()?;
        let store = Self { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> EventStoreResult<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS kernel_events (
                id TEXT PRIMARY KEY NOT NULL,
                event_type TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_kernel_events_created_at
            ON kernel_events(created_at);
            ",
        )?;
        Ok(())
    }

    pub fn append(&self, event: &KernelEvent) -> EventStoreResult<()> {
        self.conn.execute(
            "
            INSERT INTO kernel_events (id, event_type, payload_json, created_at)
            VALUES (?1, ?2, ?3, ?4)
            ",
            params![
                event.id.to_string(),
                event.event_type,
                event.payload_json,
                event.created_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }

    pub fn list_recent(&self, limit: usize) -> EventStoreResult<Vec<KernelEvent>> {
        let mut stmt = self.conn.prepare(
            "
            SELECT id, event_type, payload_json, created_at
            FROM kernel_events
            ORDER BY created_at DESC
            LIMIT ?1
            ",
        )?;
        let rows = stmt.query_map([limit as i64], |row| {
            let id: String = row.get(0)?;
            let event_type: String = row.get(1)?;
            let payload_json: String = row.get(2)?;
            let created_at: String = row.get(3)?;
            Ok((id, event_type, payload_json, created_at))
        })?;

        let mut events = Vec::new();
        for row in rows {
            let (id, event_type, payload_json, created_at) = row?;
            events.push(KernelEvent {
                id: uuid::Uuid::parse_str(&id)?,
                event_type,
                payload_json,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&chrono::Utc),
            });
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestPayload {
        value: String,
    }

    #[test]
    fn appends_and_reads_recent_events() {
        let store = EventStore::open_memory().expect("store should open");
        let event = KernelEvent::new(
            "foundation.started",
            &TestPayload {
                value: "ok".to_string(),
            },
        )
        .expect("event should serialize");

        store.append(&event).expect("append should succeed");

        let events = store.list_recent(10).expect("read should succeed");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, event.id);
        assert_eq!(events[0].event_type, "foundation.started");
        assert_eq!(events[0].payload_json, "{\"value\":\"ok\"}");
    }
}
```

- [ ] **Step 2: Run event store test**

Run:

```powershell
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml event_store
```

Expected: `appends_and_reads_recent_events` passes.

- [ ] **Step 3: Commit event store**

Run:

```powershell
git add apps/desktop/src-tauri/src/kernel/event_store.rs apps/desktop/src-tauri/src/kernel/mod.rs
git commit -m "feat: add sqlite kernel event store"
```

Expected: commit succeeds.

---

### Task 5: Add Policy Engine Foundation

**Files:**
- Create: `apps/desktop/src-tauri/src/kernel/policy.rs`
- Modify: `apps/desktop/src-tauri/src/kernel/mod.rs`

- [ ] **Step 1: Write policy foundation**

Create `apps/desktop/src-tauri/src/kernel/policy.rs`:

```rust
use serde::{Deserialize, Serialize};

use super::models::AccessMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    FileRead,
    FileWrite,
    NetworkSearch,
    BrowserBrowse,
    BrowserSubmit,
    EmailRead,
    EmailDraft,
    EmailSend,
    DriveRead,
    DriveWrite,
    TerminalRead,
    TerminalWrite,
    ComputerScreenshot,
    ComputerControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
    Allow,
    Ask,
    Deny,
}

pub fn capability_risk(capability: CapabilityKind) -> RiskLevel {
    match capability {
        CapabilityKind::FileRead
        | CapabilityKind::NetworkSearch
        | CapabilityKind::EmailDraft
        | CapabilityKind::DriveRead
        | CapabilityKind::TerminalRead
        | CapabilityKind::ComputerScreenshot => RiskLevel::Low,
        CapabilityKind::BrowserBrowse | CapabilityKind::EmailRead => RiskLevel::Medium,
        CapabilityKind::FileWrite
        | CapabilityKind::BrowserSubmit
        | CapabilityKind::DriveWrite
        | CapabilityKind::TerminalWrite => RiskLevel::High,
        CapabilityKind::EmailSend | CapabilityKind::ComputerControl => RiskLevel::Critical,
    }
}

pub fn decide(access_mode: AccessMode, capability: CapabilityKind) -> PolicyDecision {
    let risk = capability_risk(capability);
    match access_mode {
        AccessMode::AskEveryStep => PolicyDecision::Ask,
        AccessMode::AskOnRisk => match risk {
            RiskLevel::Low => PolicyDecision::Allow,
            RiskLevel::Medium | RiskLevel::High | RiskLevel::Critical => PolicyDecision::Ask,
        },
        AccessMode::LimitedAuto => match risk {
            RiskLevel::Low | RiskLevel::Medium => PolicyDecision::Allow,
            RiskLevel::High | RiskLevel::Critical => PolicyDecision::Ask,
        },
        AccessMode::FullAccess => match capability {
            CapabilityKind::EmailSend | CapabilityKind::ComputerControl => PolicyDecision::Ask,
            _ => PolicyDecision::Allow,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_every_step_always_asks() {
        assert_eq!(decide(AccessMode::AskEveryStep, CapabilityKind::FileRead), PolicyDecision::Ask);
        assert_eq!(decide(AccessMode::AskEveryStep, CapabilityKind::ComputerControl), PolicyDecision::Ask);
    }

    #[test]
    fn ask_on_risk_allows_low_risk_only() {
        assert_eq!(decide(AccessMode::AskOnRisk, CapabilityKind::FileRead), PolicyDecision::Allow);
        assert_eq!(decide(AccessMode::AskOnRisk, CapabilityKind::EmailRead), PolicyDecision::Ask);
        assert_eq!(decide(AccessMode::AskOnRisk, CapabilityKind::EmailSend), PolicyDecision::Ask);
    }

    #[test]
    fn full_access_still_asks_for_email_send_and_computer_control() {
        assert_eq!(decide(AccessMode::FullAccess, CapabilityKind::FileWrite), PolicyDecision::Allow);
        assert_eq!(decide(AccessMode::FullAccess, CapabilityKind::EmailSend), PolicyDecision::Ask);
        assert_eq!(decide(AccessMode::FullAccess, CapabilityKind::ComputerControl), PolicyDecision::Ask);
    }
}
```

- [ ] **Step 2: Run policy tests**

Run:

```powershell
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml policy
```

Expected: policy tests pass.

- [ ] **Step 3: Commit policy foundation**

Run:

```powershell
git add apps/desktop/src-tauri/src/kernel/policy.rs apps/desktop/src-tauri/src/kernel/mod.rs
git commit -m "feat: add capability policy foundation"
```

Expected: commit succeeds.

---

### Task 6: Wire Foundation State into UI Build

**Files:**
- Modify: `apps/desktop/src/types.ts`
- Modify: `apps/desktop/src/App.tsx`
- Modify: `apps/desktop/src-tauri/src/commands.rs`

- [ ] **Step 1: Align TypeScript enum values with Rust serde**

Modify `apps/desktop/src/types.ts`:

```ts
export type ModelRoute = "auto" | "deep_seek_v4_flash" | "deep_seek_v4_pro";

export type ThinkingLevel = "auto" | "fast" | "standard" | "deep";

export type AccessMode = "ask_every_step" | "ask_on_risk" | "limited_auto" | "full_access";

export type WorkspaceScope = "current_file" | "current_folder" | "workspace";

export type FoundationState = {
  app_name: string;
  model_route: ModelRoute;
  thinking_level: ThinkingLevel;
  access_mode: AccessMode;
  workspace_scope: WorkspaceScope;
};
```

- [ ] **Step 2: Update React component field names**

Modify `apps/desktop/src/App.tsx` so the fallback state and all reads use snake_case fields:

```tsx
const fallbackState: FoundationState = {
  app_name: "DeepSeek Agent OS",
  model_route: "auto",
  thinking_level: "auto",
  access_mode: "ask_on_risk",
  workspace_scope: "workspace",
};
```

Replace:

```tsx
{state.appName}
state.modelRoute
state.thinkingLevel
state.accessMode
state.workspaceScope
```

with:

```tsx
{state.app_name}
state.model_route
state.thinking_level
state.access_mode
state.workspace_scope
```

Expected: TypeScript no longer expects camelCase fields.

- [ ] **Step 3: Run full foundation checks**

Run:

```powershell
pnpm --filter @deepseek-agent-os/desktop build
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

Expected: Vite/TypeScript build passes and all Rust tests pass.

- [ ] **Step 4: Commit UI/kernel wiring**

Run:

```powershell
git add apps/desktop/src apps/desktop/src-tauri/src
git commit -m "feat: wire foundation state into workbench"
```

Expected: commit succeeds.

---

### Task 7: Update Project Handoff

**Files:**
- Modify: `SESSION_HANDOFF.md`

- [ ] **Step 1: Update current stage**

Modify `SESSION_HANDOFF.md` current stage to:

```markdown
## Current Stage

Foundation MVP implementation has started. The repository baseline, desktop shell, kernel models, policy foundation, DeepSeek route defaults, and SQLite event store are the first implementation slice.
```

- [ ] **Step 2: Add implementation startup command section**

Add this section to `SESSION_HANDOFF.md`:

```markdown
## Foundation MVP Commands

```powershell
pnpm install
pnpm --filter @deepseek-agent-os/desktop build
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
pnpm dev
```
```

- [ ] **Step 3: Commit handoff update**

Run:

```powershell
git add SESSION_HANDOFF.md
git commit -m "docs: update handoff for foundation implementation"
```

Expected: commit succeeds.

---

## Self-Review Checklist

- Spec coverage: this plan covers only the first implementation slice and leaves memory, connectors, import/export implementation, and Computer Use for separate plans.
- Placeholder scan: no task uses placeholder markers or vague implementation steps.
- Type consistency: TypeScript `FoundationState` uses the same serde snake_case field names as Rust.
- Verification: every code task has a concrete build or test command.
- Commit cadence: each task ends with a focused commit.
