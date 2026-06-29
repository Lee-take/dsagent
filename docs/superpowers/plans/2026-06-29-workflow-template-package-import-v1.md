# Workflow Template Package Import v1

## Goal

Let work packages carry reusable workflow template packages, preview them before import, and register imported templates locally without writing user folders.

## Tests First

- Add an event-store test proving workflow template import preview counts new and skipped template packages.
- Add an event-store test proving importing template packages appends only new packages once.
- Add a work-package test proving default exports include the Operations Briefing evidence template package.
- Extend legacy package parsing checks so older packages without `workflow_templates` still parse with an empty list.

## Implementation

- Add `WorkflowTemplatePackage` and `WorkflowTemplateFile` to the workflow layer.
- Export the compiled Operations Briefing evidence templates as a default workflow template package.
- Add `workflow_templates` to `WorkPackage` with serde defaults for legacy compatibility.
- Add workflow-template import preview and summary structs.
- Add append-only `workflow_template_package.imported` events, list/import helpers, and duplicate skipping by template package ID.
- Extend `import_work_package` and `preview_work_package_import` to include workflow-template package counts.
- Extend the React package preview and import completion copy with workflow-template counts.

## Verification

Run focused workflow-template/work-package tests, full Rust tests, desktop build, Tauri debug build, and `git diff --check`.
