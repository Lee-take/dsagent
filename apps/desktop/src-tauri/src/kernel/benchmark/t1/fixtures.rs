use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zip::write::FileOptions;

use super::{FIXTURE_GENERATOR_ID, FIXTURE_SET_ID};

const FIXTURE_SET_VERSION: &str = "t1.fixture-set/v1";
const FIXED_CORE_TIME: &str = "2026-07-16T00:00:00Z";
const XLSX_MEDIA_TYPE: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const DOCX_MEDIA_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const PDF_MEDIA_TYPE: &str = "application/pdf";
const MAX_FIXTURE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct T1FixtureFileSpec {
    pub fixture_id: String,
    pub relative_path: String,
    pub media_type: String,
    pub bytes: u64,
    pub sha256: String,
    pub source_label: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct T1FixtureSetManifest {
    pub version: String,
    pub fixture_set_id: String,
    pub data_class: String,
    pub generator_id: String,
    pub generated_at: String,
    pub files: Vec<T1FixtureFileSpec>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct T1GeneratedFixture {
    pub fixture_id: String,
    pub relative_path: String,
    pub media_type: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct T1GeneratedFixtureSet {
    pub manifest: T1FixtureSetManifest,
    pub files: Vec<T1GeneratedFixture>,
}

impl T1GeneratedFixtureSet {
    pub fn file(&self, fixture_id: &str) -> Option<&T1GeneratedFixture> {
        self.files
            .iter()
            .find(|fixture| fixture.fixture_id == fixture_id)
    }
}

pub fn generate_fixture_set() -> Result<T1GeneratedFixtureSet, String> {
    let manifest: T1FixtureSetManifest = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/benchmarks/t1/fixture-set-v1.json"
    )))
    .map_err(|error| format!("invalid T1 fixture-set manifest: {error}"))?;
    validate_manifest_shape(&manifest)?;

    let files = vec![
        T1GeneratedFixture {
            fixture_id: "monthly-revenue-xlsx".to_string(),
            relative_path: "inputs/01-monthly-revenue.xlsx".to_string(),
            media_type: XLSX_MEDIA_TYPE.to_string(),
            bytes: build_revenue_xlsx()?,
        },
        T1GeneratedFixture {
            fixture_id: "operations-notes-docx".to_string(),
            relative_path: "inputs/02-operations-notes.docx".to_string(),
            media_type: DOCX_MEDIA_TYPE.to_string(),
            bytes: build_operations_notes_docx()?,
        },
        T1GeneratedFixture {
            fixture_id: "risk-summary-pdf".to_string(),
            relative_path: "inputs/03-risk-summary.pdf".to_string(),
            media_type: PDF_MEDIA_TYPE.to_string(),
            bytes: build_risk_summary_pdf(),
        },
    ];
    validate_generated_files(&manifest, &files)?;
    Ok(T1GeneratedFixtureSet { manifest, files })
}

pub fn write_fixture_set(root: &Path) -> Result<T1GeneratedFixtureSet, String> {
    let fixture_set = generate_fixture_set()?;
    fs::create_dir_all(root)
        .map_err(|error| format!("T1 fixture output directory could not be created: {error}"))?;
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("T1 fixture output directory could not be resolved: {error}"))?;
    for fixture in &fixture_set.files {
        let target = canonical_root.join(&fixture.relative_path);
        let parent = target
            .parent()
            .ok_or_else(|| "T1 fixture target has no parent directory".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("T1 fixture directory could not be created: {error}"))?;
        let canonical_parent = parent
            .canonicalize()
            .map_err(|error| format!("T1 fixture directory could not be resolved: {error}"))?;
        if !canonical_parent.starts_with(&canonical_root) {
            return Err("T1 fixture directory escaped the output root".to_string());
        }
        let file_name = target
            .file_name()
            .ok_or_else(|| "T1 fixture target has no file name".to_string())?;
        let safe_target = canonical_parent.join(file_name);
        if fs::symlink_metadata(&safe_target)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err("T1 fixture target symlink is blocked".to_string());
        }
        fs::write(&safe_target, &fixture.bytes)
            .map_err(|error| format!("T1 fixture could not be written: {error}"))?;
    }
    Ok(fixture_set)
}

fn validate_manifest_shape(manifest: &T1FixtureSetManifest) -> Result<(), String> {
    if manifest.version != FIXTURE_SET_VERSION
        || manifest.fixture_set_id != FIXTURE_SET_ID
        || manifest.data_class != "synthetic_benchmark_only"
        || manifest.generator_id != FIXTURE_GENERATOR_ID
        || manifest.generated_at != FIXED_CORE_TIME
        || manifest.files.len() != 3
    {
        return Err("T1 fixture-set manifest identity is invalid".to_string());
    }
    let expected = [
        (
            "monthly-revenue-xlsx",
            "inputs/01-monthly-revenue.xlsx",
            XLSX_MEDIA_TYPE,
        ),
        (
            "operations-notes-docx",
            "inputs/02-operations-notes.docx",
            DOCX_MEDIA_TYPE,
        ),
        (
            "risk-summary-pdf",
            "inputs/03-risk-summary.pdf",
            PDF_MEDIA_TYPE,
        ),
    ];
    for (entry, (fixture_id, path, media_type)) in manifest.files.iter().zip(expected) {
        if entry.fixture_id != fixture_id
            || entry.relative_path != path
            || entry.media_type != media_type
            || entry.bytes == 0
            || entry.bytes > MAX_FIXTURE_BYTES as u64
            || !is_sha256(&entry.sha256)
            || entry.source_label.trim().is_empty()
        {
            return Err("T1 fixture-set manifest entry is invalid".to_string());
        }
        validate_relative_path(&entry.relative_path)?;
    }
    Ok(())
}

fn validate_generated_files(
    manifest: &T1FixtureSetManifest,
    files: &[T1GeneratedFixture],
) -> Result<(), String> {
    if files.len() != manifest.files.len() {
        return Err("T1 generated fixture count changed".to_string());
    }
    for (generated, expected) in files.iter().zip(&manifest.files) {
        let actual_hash = sha256(&generated.bytes);
        if generated.fixture_id != expected.fixture_id
            || generated.relative_path != expected.relative_path
            || generated.media_type != expected.media_type
            || generated.bytes.len() as u64 != expected.bytes
            || actual_hash != expected.sha256
        {
            return Err(format!(
                "T1 fixture identity mismatch for {}: bytes={} sha256={actual_hash}",
                generated.fixture_id,
                generated.bytes.len()
            ));
        }
    }
    Ok(())
}

fn build_revenue_xlsx() -> Result<Vec<u8>, String> {
    let sheet = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><dimension ref="A1:B13"/><sheetData>
<row r="1"><c r="A1" t="inlineStr"><is><t>period</t></is></c><c r="B1" t="inlineStr"><is><t>2026-06</t></is></c></row>
<row r="2"><c r="A2" t="inlineStr"><is><t>available_room_nights</t></is></c><c r="B2"><v>3000</v></c></row>
<row r="3"><c r="A3" t="inlineStr"><is><t>sold_room_nights</t></is></c><c r="B3"><v>2040</v></c></row>
<row r="4"><c r="A4" t="inlineStr"><is><t>reported_occupancy_rate</t></is></c><c r="B4"><v>0.68</v></c></row>
<row r="5"><c r="A5" t="inlineStr"><is><t>rooms_revenue_cny</t></is></c><c r="B5"><v>1142400</v></c></row>
<row r="6"><c r="A6" t="inlineStr"><is><t>reported_adr_cny</t></is></c><c r="B6"><v>560</v></c></row>
<row r="7"><c r="A7" t="inlineStr"><is><t>food_beverage_revenue_cny</t></is></c><c r="B7"><v>480000</v></c></row>
<row r="8"><c r="A8" t="inlineStr"><is><t>other_revenue_cny</t></is></c><c r="B8"><v>80000</v></c></row>
<row r="9"><c r="A9" t="inlineStr"><is><t>reported_total_revenue_cny</t></is></c><c r="B9"><v>1702400</v></c></row>
<row r="10"><c r="A10" t="inlineStr"><is><t>budget_total_revenue_cny</t></is></c><c r="B10"><v>1850000</v></c></row>
<row r="11"><c r="A11" t="inlineStr"><is><t>prior_period_total_revenue_cny</t></is></c><c r="B11"><v>1760000</v></c></row>
<row r="12"><c r="A12" t="inlineStr"><is><t>budget_occupancy_rate</t></is></c><c r="B12"><v>0.74</v></c></row>
<row r="13"><c r="A13" t="inlineStr"><is><t>synthetic_notice</t></is></c><c r="B13" t="inlineStr"><is><t>Fictional synthetic benchmark data only; not operational data.</t></is></c></row>
</sheetData></worksheet>"#;
    write_deterministic_zip(&[
        (
            "[Content_Types].xml",
            br#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/><Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/></Types>"#,
        ),
        ("_rels/.rels", root_relationships("xl/workbook.xml").as_bytes()),
        ("docProps/core.xml", core_properties("T1 monthly revenue").as_bytes()),
        (
            "xl/workbook.xml",
            br#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Monthly Revenue" sheetId="1" r:id="rId1"/></sheets><calcPr calcMode="auto"/></workbook>"#,
        ),
        (
            "xl/_rels/workbook.xml.rels",
            br#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#,
        ),
        ("xl/worksheets/sheet1.xml", sheet.as_bytes()),
    ])
}

fn build_operations_notes_docx() -> Result<Vec<u8>, String> {
    let paragraphs = [
        "period=2026-06",
        "synthetic_notice=Fictional synthetic benchmark data only; not operational data.",
        "breakfast_queue_complaints=12",
        "overdue_invoice_corrections_over_48h=3",
        "group_leads_deferred_to_july=2",
    ]
    .iter()
    .map(|text| format!("<w:p><w:r><w:t>{}</w:t></w:r></w:p>", xml_escape(text)))
    .collect::<Vec<_>>()
    .join("");
    let document = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body>{paragraphs}<w:sectPr/></w:body></w:document>"#
    );
    write_deterministic_zip(&[
        (
            "[Content_Types].xml",
            br#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/></Types>"#,
        ),
        ("_rels/.rels", root_relationships("word/document.xml").as_bytes()),
        ("docProps/core.xml", core_properties("T1 operations notes").as_bytes()),
        ("word/document.xml", document.as_bytes()),
    ])
}

fn build_risk_summary_pdf() -> Vec<u8> {
    let lines = [
        "period=2026-06",
        "synthetic_notice=Fictional synthetic benchmark data only; not operational data.",
        "elevator_2_unplanned_outages=4",
        "overdue_fire_door_closing_checks=2",
        "temporary_food_staff_retraining_incomplete=6",
    ];
    let mut stream = String::from("BT /F1 12 Tf 72 740 Td ");
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            stream.push_str("0 -22 Td ");
        }
        stream.push_str(&format!("({}) Tj ", pdf_escape(line)));
    }
    stream.push_str("ET");
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>".to_string(),
        format!("<< /Length {} >>\nstream\n{stream}\nendstream", stream.len()),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
    ];
    let mut pdf = String::from("%PDF-1.4\n% T1 synthetic benchmark only\n");
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.push_str(&format!("{} 0 obj\n{object}\nendobj\n", index + 1));
    }
    let xref = pdf.len();
    pdf.push_str(&format!(
        "xref\n0 {}\n0000000000 65535 f \n",
        objects.len() + 1
    ));
    for offset in offsets {
        pdf.push_str(&format!("{offset:010} 00000 n \n"));
    }
    pdf.push_str(&format!(
        "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
        objects.len() + 1
    ));
    pdf.into_bytes()
}

pub(super) fn write_deterministic_zip(parts: &[(&str, &[u8])]) -> Result<Vec<u8>, String> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let timestamp = zip::DateTime::from_date_and_time(2026, 7, 16, 0, 0, 0)
        .map_err(|_| "T1 ZIP timestamp is invalid".to_string())?;
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(timestamp)
        .unix_permissions(0o644);
    for (path, bytes) in parts {
        validate_relative_path(path)?;
        writer
            .start_file(*path, options)
            .map_err(|error| format!("T1 ZIP part could not start: {error}"))?;
        writer
            .write_all(bytes)
            .map_err(|error| format!("T1 ZIP part could not be written: {error}"))?;
    }
    writer
        .finish()
        .map(|finished| finished.into_inner())
        .map_err(|error| format!("T1 ZIP could not finish: {error}"))
}

fn root_relationships(target: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="{target}"/><Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/></Relationships>"#
    )
}

fn core_properties(title: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"><dc:title>{}</dc:title><dc:creator>DS Agent benchmark</dc:creator><cp:lastModifiedBy>DS Agent benchmark</cp:lastModifiedBy><dcterms:created xsi:type="dcterms:W3CDTF">{FIXED_CORE_TIME}</dcterms:created><dcterms:modified xsi:type="dcterms:W3CDTF">{FIXED_CORE_TIME}</dcterms:modified></cp:coreProperties>"#,
        xml_escape(title)
    )
}

pub(super) fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn pdf_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

fn validate_relative_path(path: &str) -> Result<(), String> {
    if path.is_empty()
        || path.starts_with('/')
        || path.starts_with('\\')
        || path.contains('\\')
        || path.contains(':')
        || path
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err("T1 fixture path is unsafe".to_string());
    }
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_generation_is_byte_stable_and_manifest_bound() {
        let first = vec![
            build_revenue_xlsx().unwrap(),
            build_operations_notes_docx().unwrap(),
            build_risk_summary_pdf(),
        ];
        let second = vec![
            build_revenue_xlsx().unwrap(),
            build_operations_notes_docx().unwrap(),
            build_risk_summary_pdf(),
        ];
        assert_eq!(first, second);
        let manifest: T1FixtureSetManifest = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/benchmarks/t1/fixture-set-v1.json"
        )))
        .unwrap();
        let actual = first
            .iter()
            .map(|bytes| (bytes.len() as u64, sha256(bytes)))
            .collect::<Vec<_>>();
        let expected = manifest
            .files
            .iter()
            .map(|file| (file.bytes, file.sha256.clone()))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
        generate_fixture_set().unwrap();
    }

    #[test]
    fn fixture_writer_emits_only_the_three_manifest_bound_inputs() {
        let temp = tempfile::tempdir().unwrap();
        let fixture_set = write_fixture_set(temp.path()).unwrap();
        for fixture in fixture_set.files {
            let bytes = fs::read(temp.path().join(&fixture.relative_path)).unwrap();
            assert_eq!(bytes, fixture.bytes);
        }
        assert!(!temp.path().join("outputs").exists());
    }
}
