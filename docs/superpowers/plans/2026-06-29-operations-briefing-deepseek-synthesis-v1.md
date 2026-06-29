# Operations Briefing DeepSeek Synthesis v1

## Goal

Wire the existing DeepSeek Chat executor into Operations Briefing synthesis without weakening the evidence-folder permission gate or making DeepSeek chat a NetworkSearch evidence source.

## Tests First

- Add a workflow-level fake synthesizer test proving a successful model synthesis replaces the deterministic draft after evidence ingest succeeds.
- Add a workflow-level failure test proving model synthesis errors keep the deterministic draft and append a warning.
- Add a DeepSeek adapter test proving the adapter sends only evidence manifest content and parses strict JSON into `OperationsBriefingSynthesis`.
- Add command helper tests proving DeepSeek synthesis is enabled only for the DeepSeek provider with a non-empty `DEEPSEEK_API_KEY`.

## Implementation

- Add `OperationsBriefingSynthesizer` as an injectable workflow trait.
- Keep `run_operations_briefing` deterministic by default and add `run_operations_briefing_with_synthesizer` for model-backed synthesis.
- Add `DeepSeekOperationsBriefingSynthesizer` in the DeepSeek kernel module.
- Parse the model response as strict JSON containing `summary`, `anomalies`, `action_plan`, and `warnings`.
- Extend the Tauri `run_operations_briefing` command and React invocation to pass the selected large-model provider, model route, and thinking level.
- Use DeepSeek synthesis only when the selected provider is DeepSeek and the local process has a non-empty `DEEPSEEK_API_KEY`; otherwise keep local deterministic behavior.

## Verification

Run focused Operations Briefing tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
