# Excel Control

Use `office_create` for net-new Excel workbook requests. DS Agent creates a real
`.xlsx` package in the workspace and records it through the FileWrite audit
trail.

## Defaults

- Target extension: `.xlsx`
- Default folder: `office/`
- JSON `rows` becomes the first worksheet.
- Plain text content is interpreted as comma- or tab-delimited rows when
  possible.
- Numeric cells are stored as typed numeric values when they are unambiguous.

## Workbook Rules

Keep workbook calculations formula driven when formulas are needed. Keep raw
inputs, assumptions, and outputs easy to inspect. Avoid pretending a workbook
has been recalculated or visually verified unless DS Agent has actually run the
corresponding local check.

If Excel is unavailable, DS Agent can still create the `.xlsx` file. Use desktop
control only for existing windows, user-specific Excel UI actions, or visible
ribbon/dialog workflows.
