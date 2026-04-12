//! Data export helpers — convert a [`QueryResult`] to CSV, JSON, NDJSON, or TSV.
//!
//! These functions are pure — they take a `QueryResult` by reference and return
//! a formatted `String`.  No database connection is required.

use elicit_db::DbValue;

use crate::archive::{ExportFormat, ExportResult, QueryResult};

/// Format a [`QueryResult`] as the requested [`ExportFormat`].
pub fn export_query_result(result: &QueryResult, format: ExportFormat) -> ExportResult {
    let content = match format {
        ExportFormat::Csv => to_csv(result, ','),
        ExportFormat::Tsv => to_csv(result, '\t'),
        ExportFormat::Json => to_json(result),
        ExportFormat::Ndjson => to_ndjson(result),
    };
    ExportResult {
        format,
        row_count: result.row_count,
        content,
    }
}

// ── CSV / TSV ─────────────────────────────────────────────────────────────────

fn to_csv(result: &QueryResult, sep: char) -> String {
    let mut out = String::new();
    // Header row
    let headers: Vec<String> = result
        .columns
        .iter()
        .map(|c| csv_quote(&c.name, sep))
        .collect();
    out.push_str(&headers.join(&sep.to_string()));
    out.push('\n');
    // Data rows
    for row in &result.rows.rows {
        let cells: Vec<String> = result
            .columns
            .iter()
            .enumerate()
            .map(|(ci, _)| {
                let val = row
                    .0
                    .get(ci)
                    .map(|(_, v)| format_value(v))
                    .unwrap_or_default();
                csv_quote(&val, sep)
            })
            .collect();
        out.push_str(&cells.join(&sep.to_string()));
        out.push('\n');
    }
    out
}

/// Wrap in double-quotes if the value contains the separator, `"`, or newline.
fn csv_quote(s: &str, sep: char) -> String {
    if s.contains(sep) || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

// ── JSON array ────────────────────────────────────────────────────────────────

fn to_json(result: &QueryResult) -> String {
    let mut out = String::from("[\n");
    let last = result.rows.rows.len().saturating_sub(1);
    for (ri, row) in result.rows.rows.iter().enumerate() {
        out.push_str("  {");
        let fields: Vec<String> = result
            .columns
            .iter()
            .enumerate()
            .map(|(ci, col)| {
                let val = row
                    .0
                    .get(ci)
                    .map(|(_, v)| json_value(v))
                    .unwrap_or_else(|| "null".to_string());
                format!("\"{}\":{val}", col.name.replace('"', "\\\""))
            })
            .collect();
        out.push_str(&fields.join(","));
        out.push('}');
        if ri < last {
            out.push(',');
        }
        out.push('\n');
    }
    out.push(']');
    out
}

// ── NDJSON ────────────────────────────────────────────────────────────────────

fn to_ndjson(result: &QueryResult) -> String {
    let mut out = String::new();
    for row in &result.rows.rows {
        out.push('{');
        let fields: Vec<String> = result
            .columns
            .iter()
            .enumerate()
            .map(|(ci, col)| {
                let val = row
                    .0
                    .get(ci)
                    .map(|(_, v)| json_value(v))
                    .unwrap_or_else(|| "null".to_string());
                format!("\"{}\":{val}", col.name.replace('"', "\\\""))
            })
            .collect();
        out.push_str(&fields.join(","));
        out.push_str("}\n");
    }
    out
}

// ── Value formatting ──────────────────────────────────────────────────────────

fn format_value(v: &DbValue) -> String {
    match v {
        DbValue::Null => String::new(),
        DbValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        DbValue::Int(n) => n.to_string(),
        DbValue::Float(f) => format!("{f}"),
        DbValue::Text(s) => s.clone(),
        DbValue::Bytes(b) => format!(
            "\\x{}",
            b.iter().map(|x| format!("{x:02x}")).collect::<String>()
        ),
        DbValue::Json(j) => j.to_string(),
        DbValue::Geometry(s) | DbValue::Geography(s) => match s {
            elicit_db::DbSpatialValue::Wkt(w) => w.clone(),
            elicit_db::DbSpatialValue::Wkb(b) => {
                format!(
                    "\\x{}",
                    b.iter().map(|x| format!("{x:02x}")).collect::<String>()
                )
            }
        },
    }
}

fn json_value(v: &DbValue) -> String {
    match v {
        DbValue::Null => "null".to_string(),
        DbValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        DbValue::Int(n) => n.to_string(),
        DbValue::Float(f) => format!("{f}"),
        DbValue::Text(s) => format!(
            "\"{}\"",
            s.replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
        ),
        DbValue::Json(j) => j.to_string(),
        DbValue::Bytes(b)
        | DbValue::Geometry(elicit_db::DbSpatialValue::Wkb(b))
        | DbValue::Geography(elicit_db::DbSpatialValue::Wkb(b)) => format!(
            "\"\\x{}\"",
            b.iter().map(|x| format!("{x:02x}")).collect::<String>()
        ),
        DbValue::Geometry(elicit_db::DbSpatialValue::Wkt(w))
        | DbValue::Geography(elicit_db::DbSpatialValue::Wkt(w)) => {
            format!("\"{}\"", w.replace('"', "\\\""))
        }
    }
}
