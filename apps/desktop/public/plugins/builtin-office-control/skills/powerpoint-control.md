# PowerPoint Control

Use `office_create` for net-new PowerPoint deck requests. DS Agent creates a
real `.pptx` package in the workspace and records it through the FileWrite
audit trail.

## Defaults

- Target extension: `.pptx`
- Default folder: `office/`
- JSON `slides` becomes deck slides.
- Plain text content becomes a simple title/body first slide.
- Keep slide text audience-facing; do not expose internal planning notes.

## Verification Boundary

Render or preview every final slide before making visual quality claims. If a
render or preview path is not available yet, report the file creation result
without claiming visual QA.

If PowerPoint is unavailable, DS Agent can still create the `.pptx` file. Use
desktop control only for existing visible decks, app-specific dialogs, or
manual user context.
