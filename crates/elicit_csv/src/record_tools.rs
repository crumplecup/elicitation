//! Stateless MCP tools for [`csv::StringRecord`] and [`csv::ByteRecord`] operations.

use std::sync::Arc;

use elicitation::{CsvByteRecord, CsvStringRecord, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{CsvCtx, err_text, ok_json, ok_text};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringRecordLenParams {
    pub record: CsvStringRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__string_record__len",
    description = "Return the number of fields in a CsvStringRecord."
)]
#[instrument(skip(_ctx))]
pub async fn string_record_len(
    _ctx: Arc<CsvCtx>,
    params: StringRecordLenParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(params.record.0.len().to_string())
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringRecordGetParams {
    pub record: CsvStringRecord,
    /// Zero-based field index.
    pub index: usize,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__string_record__get",
    description = "Return the field at a given index from a CsvStringRecord, or null if out of bounds."
)]
#[instrument(skip(_ctx))]
pub async fn string_record_get(
    _ctx: Arc<CsvCtx>,
    params: StringRecordGetParams,
) -> Result<CallToolResult, ErrorData> {
    match params.record.0.get(params.index) {
        Some(s) => ok_text(s.clone()),
        None => ok_text("null"),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringRecordIsEmptyParams {
    pub record: CsvStringRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__string_record__is_empty",
    description = "Return true if a CsvStringRecord has no fields."
)]
#[instrument(skip(_ctx))]
pub async fn string_record_is_empty(
    _ctx: Arc<CsvCtx>,
    params: StringRecordIsEmptyParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(params.record.0.is_empty().to_string())
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringRecordTrimParams {
    pub record: CsvStringRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__string_record__trim",
    description = "Return a copy of a CsvStringRecord with whitespace trimmed from each field."
)]
#[instrument(skip(_ctx))]
pub async fn string_record_trim(
    _ctx: Arc<CsvCtx>,
    params: StringRecordTrimParams,
) -> Result<CallToolResult, ErrorData> {
    let trimmed: Vec<String> = params
        .record
        .0
        .iter()
        .map(|s| s.trim().to_string())
        .collect();
    ok_json(&CsvStringRecord(trimmed))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringRecordFromLineParams {
    /// A single CSV line (no newline required).
    pub line: String,
    /// Optional delimiter byte (default 44 = comma).
    pub delimiter: Option<u8>,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__string_record__from_line",
    description = "Parse a single CSV line into a CsvStringRecord."
)]
#[instrument(skip(_ctx))]
pub async fn string_record_from_line(
    _ctx: Arc<CsvCtx>,
    params: StringRecordFromLineParams,
) -> Result<CallToolResult, ErrorData> {
    let mut builder = csv::ReaderBuilder::new();
    builder.has_headers(false);
    if let Some(d) = params.delimiter {
        builder.delimiter(d);
    }
    let mut rdr = builder.from_reader(params.line.as_bytes());
    match rdr.records().next() {
        Some(Ok(rec)) => ok_json(&CsvStringRecord::from(rec)),
        Some(Err(e)) => err_text(format!("parse error: {}", e)),
        None => ok_json(&CsvStringRecord(vec![])),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ByteRecordLenParams {
    pub record: CsvByteRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__byte_record__len",
    description = "Return the number of fields in a CsvByteRecord."
)]
#[instrument(skip(_ctx))]
pub async fn byte_record_len(
    _ctx: Arc<CsvCtx>,
    params: ByteRecordLenParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(params.record.0.len().to_string())
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ByteRecordGetParams {
    pub record: CsvByteRecord,
    pub index: usize,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__byte_record__get",
    description = "Return the field at a given index from a CsvByteRecord as a JSON array of u8, or null if out of bounds."
)]
#[instrument(skip(_ctx))]
pub async fn byte_record_get(
    _ctx: Arc<CsvCtx>,
    params: ByteRecordGetParams,
) -> Result<CallToolResult, ErrorData> {
    match params.record.0.get(params.index) {
        Some(bytes) => ok_json(bytes),
        None => ok_text("null"),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ByteRecordIsEmptyParams {
    pub record: CsvByteRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__byte_record__is_empty",
    description = "Return true if a CsvByteRecord has no fields."
)]
#[instrument(skip(_ctx))]
pub async fn byte_record_is_empty(
    _ctx: Arc<CsvCtx>,
    params: ByteRecordIsEmptyParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(params.record.0.is_empty().to_string())
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ByteRecordTrimParams {
    pub record: CsvByteRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__byte_record__trim",
    description = "Return a copy of a CsvByteRecord with ASCII whitespace trimmed from each field."
)]
#[instrument(skip(_ctx))]
pub async fn byte_record_trim(
    _ctx: Arc<CsvCtx>,
    params: ByteRecordTrimParams,
) -> Result<CallToolResult, ErrorData> {
    let mut brecord = csv::ByteRecord::from(params.record);
    brecord.trim();
    ok_json(&CsvByteRecord::from(brecord))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ByteRecordToStringRecordParams {
    pub record: CsvByteRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__byte_record__to_string_record",
    description = "Convert a CsvByteRecord to a CsvStringRecord, interpreting each field as UTF-8."
)]
#[instrument(skip(_ctx))]
pub async fn byte_record_to_string_record(
    _ctx: Arc<CsvCtx>,
    params: ByteRecordToStringRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let brecord = csv::ByteRecord::from(params.record);
    match csv::StringRecord::from_byte_record(brecord) {
        Ok(srec) => ok_json(&CsvStringRecord::from(srec)),
        Err(e) => err_text(format!("utf8 conversion error: {}", e)),
    }
}
