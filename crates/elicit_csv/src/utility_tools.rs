//! Utility MCP tools: `invalid_option` snippet, position construction, result type alias.

use std::sync::Arc;

use elicitation::{CsvPosition, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{CsvCtx, ok_json, ok_text};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InvalidOptionSnippetParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__invalid_option__snippet",
    description = "Return a Rust code snippet demonstrating csv::invalid_option as a Serde deserializer."
)]
#[instrument(skip(_ctx))]
pub async fn invalid_option_snippet(
    _ctx: Arc<CsvCtx>,
    _params: InvalidOptionSnippetParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(
        r#"use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    // Fields that may be invalid/empty become None instead of causing a parse error.
    #[serde(deserialize_with = "csv::invalid_option")]
    age: Option<u32>,
    #[serde(deserialize_with = "csv::invalid_option")]
    score: Option<f64>,
}

// Usage:
let mut rdr = csv::Reader::from_reader(data.as_bytes());
for result in rdr.deserialize::<Record>() {
    let record = result?;
    println!("{:?}", record);
}"#,
    )
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PositionNewParams {
    /// Byte offset into the CSV data.
    pub byte: u64,
    /// Line number (1-based).
    pub line: u64,
    /// Record index (0-based).
    pub record: u64,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__position__new",
    description = "Construct a CsvPosition from byte/line/record fields. Useful for seek operations."
)]
#[instrument(skip(_ctx))]
pub async fn position_new(
    _ctx: Arc<CsvCtx>,
    params: PositionNewParams,
) -> Result<CallToolResult, ErrorData> {
    ok_json(&CsvPosition {
        byte: params.byte,
        line: params.line,
        record: params.record,
    })
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResultTypeAliasParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__result__type_alias",
    description = "Return documentation and a code snippet for the csv::Result<T> type alias."
)]
#[instrument(skip(_ctx))]
pub async fn result_type_alias(
    _ctx: Arc<CsvCtx>,
    _params: ResultTypeAliasParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(
        r#"csv::Result<T> is a type alias for std::result::Result<T, csv::Error>.

All csv reader/writer methods return csv::Result<T>. Use the ? operator to propagate errors:

use csv::Result;

fn read_records(path: &str) -> Result<Vec<csv::StringRecord>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut records = Vec::new();
    for result in rdr.records() {
        records.push(result?);
    }
    Ok(records)
}

// csv::Error wraps csv::ErrorKind, which includes:
// - Io(std::io::Error)
// - Utf8 { pos, err }
// - UnequalLengths { expected_len, pos, len }
// - Serialize(String)
// - Deserialize { pos, err: DeserializeError }
"#,
    )
}
