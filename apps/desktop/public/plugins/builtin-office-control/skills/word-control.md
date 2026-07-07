# Word Control

Use `office_create` for net-new Word document requests. DS Agent creates a real
`.docx` package in the workspace and records it through the FileWrite audit
trail.

## Defaults

- Target extension: `.docx`
- Default folder: `office/`
- Plain `content` becomes document body text.
- JSON `title` becomes the first document paragraph.
- Preserve Unicode text exactly.

## Fallback And Desktop Control

If Microsoft Word is unavailable, DS Agent can still create the `.docx` file.
Do not block net-new Word creation just because the desktop app is not
installed. Use desktop control only when the user specifically needs the Word
app opened or an existing Word window edited.

Before typing into Word through Computer Use, inspect the window, focus a stable
editing surface, send one structured input action, and verify with another
observation before claiming success.
