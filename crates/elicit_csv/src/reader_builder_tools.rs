//! MCP tools shadowing [`csv::ReaderBuilder`] methods.

use std::sync::Arc;

use elicitation::elicit_tool;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use elicitation::{CsvTerminator, CsvTrim};

use crate::plugin::{CsvCtx, err_text, ok_text, parse_uuid};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderNewParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__new",
    description = "Create a new CSV ReaderBuilder with default settings. Returns a builder UUID."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_new(
    ctx: Arc<CsvCtx>,
    _params: ReaderBuilderNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.reader_builders
        .lock()
        .expect("reader_builders lock")
        .insert(id, csv::ReaderBuilder::new());
    ok_text(format!("reader_builder created: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderDelimiterParams {
    /// Builder UUID returned by csv__reader_builder__new.
    pub builder_id: String,
    /// Field delimiter byte (ASCII, e.g. 44 = comma, 9 = tab).
    pub delimiter: u8,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__delimiter",
    description = "Set the field delimiter byte on a CSV ReaderBuilder. Default is 44 (comma)."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_delimiter(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderDelimiterParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.delimiter(params.delimiter);
    ok_text(format!("delimiter set to {}", params.delimiter))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderHasHeadersParams {
    pub builder_id: String,
    /// Whether the first record is a header row. Default: true.
    pub has_headers: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__has_headers",
    description = "Configure whether the first CSV record is treated as a header row."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_has_headers(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderHasHeadersParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.has_headers(params.has_headers);
    ok_text(format!("has_headers set to {}", params.has_headers))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderFlexibleParams {
    pub builder_id: String,
    /// Allow records with variable field counts. Default: false.
    pub flexible: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__flexible",
    description = "Allow variable-length records when reading CSV."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_flexible(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderFlexibleParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.flexible(params.flexible);
    ok_text(format!("flexible set to {}", params.flexible))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderTrimParams {
    pub builder_id: String,
    /// Trim strategy: All, Fields, Headers, or None.
    pub trim: CsvTrim,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__trim",
    description = "Set whitespace trimming behaviour for CSV fields/headers."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_trim(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderTrimParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.trim(csv::Trim::from(params.trim));
    ok_text("trim set")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderTerminatorParams {
    pub builder_id: String,
    /// Record terminator: Crlf, Any, or AnyByte.
    pub terminator: CsvTerminator,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__terminator",
    description = "Set the record terminator on the CSV ReaderBuilder."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_terminator(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderTerminatorParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.terminator(csv::Terminator::from(params.terminator));
    ok_text("terminator set")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderQuoteParams {
    pub builder_id: String,
    /// Quote character byte (ASCII, default 34 = double-quote).
    pub quote: u8,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__quote",
    description = "Set the quote character used when parsing CSV fields."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_quote(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderQuoteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.quote(params.quote);
    ok_text(format!("quote set to {}", params.quote))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderDoubleQuoteParams {
    pub builder_id: String,
    /// Whether doubled quotes are interpreted as escaped quotes. Default: true.
    pub double_quote: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__double_quote",
    description = "Configure whether consecutive quote characters are treated as an escaped quote."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_double_quote(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderDoubleQuoteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.double_quote(params.double_quote);
    ok_text(format!("double_quote set to {}", params.double_quote))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderEscapeParams {
    pub builder_id: String,
    /// Optional escape byte; when set, disables double-quote escaping.
    pub escape: Option<u8>,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__escape",
    description = "Set an explicit escape byte for CSV fields (disables double-quote escaping)."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_escape(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderEscapeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.escape(params.escape);
    ok_text(format!("escape set to {:?}", params.escape))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderCommentParams {
    pub builder_id: String,
    /// If set, lines starting with this byte are treated as comments (e.g. 35 = '#').
    pub comment: Option<u8>,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__comment",
    description = "Set a comment character: lines starting with this byte are skipped."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_comment(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderCommentParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.comment(params.comment);
    ok_text(format!("comment set to {:?}", params.comment))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderQuotingParams {
    pub builder_id: String,
    /// Enable or disable quoting entirely. Default: true.
    pub quoting: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__quoting",
    description = "Enable or disable quoting when parsing CSV."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_quoting(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderQuotingParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.quoting(params.quoting);
    ok_text(format!("quoting set to {}", params.quoting))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderBufferCapacityParams {
    pub builder_id: String,
    /// Internal read buffer capacity in bytes. Default: 8192.
    pub capacity: usize,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__buffer_capacity",
    description = "Set the internal read buffer capacity for the CSV reader."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_buffer_capacity(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderBufferCapacityParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.reader_builders.lock().expect("reader_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.buffer_capacity(params.capacity);
    ok_text(format!("buffer_capacity set to {}", params.capacity))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderFromReaderParams {
    /// Builder UUID to consume.
    pub builder_id: String,
    /// CSV data as a UTF-8 string.
    pub csv_data: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__from_reader",
    description = "Consume a ReaderBuilder and in-memory CSV data to create a live Reader. Returns a reader UUID."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_from_reader(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderFromReaderParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let builder = ctx
        .reader_builders
        .lock()
        .expect("reader_builders lock")
        .remove(&id);
    let Some(builder) = builder else {
        return err_text(format!("builder not found: {}", id));
    };
    let reader = builder.from_reader(std::io::Cursor::new(params.csv_data.into_bytes()));
    let reader_id = Uuid::new_v4();
    ctx.mem_readers
        .lock()
        .expect("mem_readers lock")
        .insert(reader_id, reader);
    ok_text(format!("mem_reader created: {}", reader_id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderBuilderFromPathParams {
    /// Builder UUID to consume.
    pub builder_id: String,
    /// Filesystem path to a CSV file.
    pub path: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader_builder__from_path",
    description = "Consume a ReaderBuilder and open a CSV file at the given path. Returns a file reader UUID."
)]
#[instrument(skip(ctx))]
pub async fn reader_builder_from_path(
    ctx: Arc<CsvCtx>,
    params: ReaderBuilderFromPathParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let builder = ctx
        .reader_builders
        .lock()
        .expect("reader_builders lock")
        .remove(&id);
    let Some(builder) = builder else {
        return err_text(format!("builder not found: {}", id));
    };
    match builder.from_path(&params.path) {
        Ok(reader) => {
            let reader_id = Uuid::new_v4();
            ctx.file_readers
                .lock()
                .expect("file_readers lock")
                .insert(reader_id, reader);
            ok_text(format!("file_reader created: {}", reader_id))
        }
        Err(e) => err_text(format!("failed to open '{}': {}", params.path, e)),
    }
}
