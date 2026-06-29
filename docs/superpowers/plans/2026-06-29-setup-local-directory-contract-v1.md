# Setup and local directory contract v1

## Goal

Add a first-run local-directory contract so open-source users do not inherit
developer-machine paths and do not confuse the installation directory with
workspace data.

## Requirements

- Store application data under the OS-provided app data directory.
- Persist user-selected workspace, evidence, and export directories in a small
  settings file under app data.
- Treat missing settings as first-run setup required.
- Reject blank required directory values.
- Surface the contract in the desktop UI before local file/folder workflows.
- Keep local file/folder path inputs as runtime user paths, not install paths.

## Plan

1. Add Rust tests for missing, saved, and invalid local-directory settings.
2. Implement a local-directory kernel module with JSON persistence.
3. Expose Tauri commands for reading and saving setup state.
4. Add a compact first-run setup panel to the React workbench.
5. Update README and handoff, then verify.
