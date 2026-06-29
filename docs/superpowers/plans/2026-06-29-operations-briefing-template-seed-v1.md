# Operations Briefing Template Seed v1

## Goal

Let a local operator seed the sample Operations Briefing evidence templates into the configured evidence folder from the desktop UI without hardcoded developer-machine paths or silent file writes.

## Tests First

- Add a workflow test proving template seeding waits for FileWrite approval and does not touch the filesystem while pending.
- Add a local seeder test proving templates are written and existing files are skipped rather than overwritten.
- Add command helper tests proving the seed target prefers the user's configured evidence folder and falls back to an app-data subdirectory before setup.

## Implementation

- Compile the repository template files into the desktop binary with `include_str!` so installed apps can seed templates without needing the source checkout.
- Add `run_operations_briefing_template_seed` using `CapabilityKind::FileWrite`.
- Add `LocalOperationsBriefingTemplateSeeder` for local filesystem writes.
- Add `seed_operations_briefing_evidence_templates` as a Tauri command.
- Add a Seed Templates button to the Operations Briefing workflow panel.

## Verification

Run focused template seed tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
