# DS Agent Office Control

This built-in skill pack is the default route for user requests involving
Word, Excel, or PowerPoint.

## Execution Boundary

- Prefer deterministic file creation or file editing before desktop UI control.
- Use `office_create` to create real `.docx`, `.xlsx`, and `.pptx` files in the
  configured DS Agent workspace.
- Use `office_update` to update an existing workspace Office file before using
  desktop UI control. The v1 deterministic update supports appending Word
  paragraphs, Excel rows, and PowerPoint slides.
- Use `office_open` to open a workspace `.docx`, `.xlsx`, or `.pptx` file with
  the matching Microsoft Office app when available. If Microsoft Office is not
  installed, DS Agent should fall back to the system default app and show only
  that compact result in the right rail.
- Use `computer_screenshot` before any screen-dependent computer_control action.
- Use computer_screenshot before any screen-dependent computer_control action.
- Use `computer_control` only for one approved structured input action at a
  time: `click:x,y[,button]`, `move:x,y`, `type:text`, `press:key`,
  `hotkey:key+key`, or `scroll:delta[,axis]`.
- Right rail output should show only compact user-facing steps and their state.
  Only the current failed or blocked step should show problem details.

## office_create Contract

`office_create` uses the existing FileWrite permission, audit, and workspace
path boundary. The action can provide plain text in `content`, or JSON:

```json
{
  "app": "word",
  "title": "Status note",
  "body": "Text to place in the document",
  "path": "office/status-note.docx"
}
```

Excel JSON may include `rows`. PowerPoint JSON may include `slides`.

## office_update Contract

`office_update` uses the existing FileWrite permission, audit, and workspace
path boundary. The action target must be an existing workspace-relative
`.docx`, `.xlsx`, or `.pptx` file.

Word updates append body text as paragraphs. Excel updates append `rows` to the
first worksheet and preserve values beginning with `=` as formulas. PowerPoint
updates append `slides` to the deck.

## office_open Contract

`office_open` uses the existing FileRead permission and workspace path
boundary. The action target must be a workspace-relative `.docx`, `.xlsx`, or
`.pptx` file. Prefer the app implied by the extension; if that Microsoft Office
app is unavailable, use the default file handler instead of blocking the user.

## App UI Control

Use Office desktop UI control only when the user needs an already open app,
existing visible window, ribbon command, dialog interaction, or manual account
context. App UI work must start with observation, then continue one action at a
time through the Computer Use boundary.
