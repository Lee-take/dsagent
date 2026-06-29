# Model-driven tool strategy v1

## Goal

Wire Phase 2 tool backends to the user-selected large model instead of fixed
defaults.

## Requirements

- NetworkSearch follows the selected large model.
- If the selected large model does not support source-backed NetworkSearch,
  the UI must prompt for a NetworkSearch source model and provide free preset
  options in a dropdown.
- ComputerScreenshot and ComputerControl use the Codex bridge contract when the
  selected large model is ChatGPT or Codex.
- Other large models use local OS screenshot/input libraries.
- Keep a macOS backend path in the strategy model even while this Windows build
  remains the primary target.

## Plan

1. Add model/provider and NetworkSearch source-model types to the shared kernel
   model.
2. Add a `ModelDrivenToolStrategy` kernel service that derives NetworkSearch and
   computer-use backends from provider plus platform.
3. Expose the strategy through a Tauri command and include it in work-package
   readiness.
4. Wire the desktop UI to show provider selection, NetworkSearch source-model
   prompt/dropdown, and strategy-derived backend status.
5. Verify with Rust tests, desktop TypeScript build, and diff hygiene.
