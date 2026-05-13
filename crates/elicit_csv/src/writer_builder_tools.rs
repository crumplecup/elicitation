//! MCP tools shadowing [`csv::WriterBuilder`] methods.

use std::sync::Arc;

use elicitation::{CsvQuoteStyle, CsvTerminator, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::plugin::{CsvCtx, err_text, ok_text, parse_uuid};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderNewParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__new",
    description = "Create a new CSV WriterBuilder with default settings. Returns a builder UUID."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_new(
    ctx: Arc<CsvCtx>,
    _params: WriterBuilderNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.writer_builders
        .lock()
        .expect("writer_builders lock")
        .insert(id, csv::WriterBuilder::new());
    ok_text(format!("writer_builder created: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderDelimiterParams {
    pub builder_id: String,
    /// Field delimiter byte (ASCII, default 44 = comma).
    pub delimiter: u8,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__delimiter",
    description = "Set the field delimiter byte on a CSV WriterBuilder."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_delimiter(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderDelimiterParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.delimiter(params.delimiter);
    ok_text(format!("delimiter set to {}", params.delimiter))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderHasHeadersParams {
    pub builder_id: String,
    /// Whether serialize() should write a header row automatically. Default: true.
    pub has_headers: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__has_headers",
    description = "Configure whether serialized structs write their field names as a header row."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_has_headers(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderHasHeadersParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.has_headers(params.has_headers);
    ok_text(format!("has_headers set to {}", params.has_headers))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderQuoteParams {
    pub builder_id: String,
    /// Quote character byte (ASCII, default 34 = double-quote).
    pub quote: u8,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__quote",
    description = "Set the quote character used when writing CSV fields."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_quote(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderQuoteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.quote(params.quote);
    ok_text(format!("quote set to {}", params.quote))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderQuoteStyleParams {
    pub builder_id: String,
    /// QuoteStyle: Always, Necessary, NonNumeric, or Never.
    pub quote_style: CsvQuoteStyle,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__quote_style",
    description = "Set when fields are quoted: Always, Necessary (default), NonNumeric, or Never."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_quote_style(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderQuoteStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.quote_style(csv::QuoteStyle::from(params.quote_style));
    ok_text("quote_style set")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderDoubleQuoteParams {
    pub builder_id: String,
    /// Escape quotes by doubling them. Default: true.
    pub double_quote: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__double_quote",
    description = "Configure whether quotes are escaped by doubling them."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_double_quote(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderDoubleQuoteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.double_quote(params.double_quote);
    ok_text(format!("double_quote set to {}", params.double_quote))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderEscapeParams {
    pub builder_id: String,
    /// Escape byte (ASCII, default 92 = backslash). Only used when double_quote is false.
    pub escape: u8,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__escape",
    description = "Set the escape byte for CSV fields when double-quoting is disabled."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_escape(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderEscapeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.escape(params.escape);
    ok_text(format!("escape set to {}", params.escape))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderTerminatorParams {
    pub builder_id: String,
    /// Record terminator: Crlf (default), Any, or AnyByte.
    pub terminator: CsvTerminator,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__terminator",
    description = "Set the record terminator on the CSV WriterBuilder."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_terminator(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderTerminatorParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.terminator(csv::Terminator::from(params.terminator));
    ok_text("terminator set")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderBufferCapacityParams {
    pub builder_id: String,
    /// Write buffer capacity in bytes. Default: 8192.
    pub capacity: usize,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__buffer_capacity",
    description = "Set the internal write buffer capacity for the CSV writer."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_buffer_capacity(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderBufferCapacityParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.buffer_capacity(params.capacity);
    ok_text(format!("buffer_capacity set to {}", params.capacity))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderFlexibleParams {
    pub builder_id: String,
    /// Allow records of variable length. Default: false.
    pub flexible: bool,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__flexible",
    description = "Allow variable-length records when writing CSV."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_flexible(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderFlexibleParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let mut guard = ctx.writer_builders.lock().expect("writer_builders lock");
    let Some(b) = guard.get_mut(&id) else {
        return err_text(format!("builder not found: {}", id));
    };
    b.flexible(params.flexible);
    ok_text(format!("flexible set to {}", params.flexible))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderFromWriterParams {
    /// Builder UUID to consume.
    pub builder_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__from_writer",
    description = "Consume a WriterBuilder to create an in-memory CSV Writer. Returns a writer UUID."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_from_writer(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderFromWriterParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let builder = ctx
        .writer_builders
        .lock()
        .expect("writer_builders lock")
        .remove(&id);
    let Some(builder) = builder else {
        return err_text(format!("builder not found: {}", id));
    };
    let writer = builder.from_writer(Vec::<u8>::new());
    let writer_id = Uuid::new_v4();
    ctx.mem_writers
        .lock()
        .expect("mem_writers lock")
        .insert(writer_id, writer);
    ok_text(format!("mem_writer created: {}", writer_id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterBuilderFromPathParams {
    /// Builder UUID to consume.
    pub builder_id: String,
    /// Filesystem path to write the CSV file to.
    pub path: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer_builder__from_path",
    description = "Consume a WriterBuilder and create a file-backed CSV Writer at the given path. Returns a writer UUID."
)]
#[instrument(skip(ctx))]
pub async fn writer_builder_from_path(
    ctx: Arc<CsvCtx>,
    params: WriterBuilderFromPathParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.builder_id)?;
    let builder = ctx
        .writer_builders
        .lock()
        .expect("writer_builders lock")
        .remove(&id);
    let Some(builder) = builder else {
        return err_text(format!("builder not found: {}", id));
    };
    match builder.from_path(&params.path) {
        Ok(writer) => {
            let writer_id = Uuid::new_v4();
            ctx.file_writers
                .lock()
                .expect("file_writers lock")
                .insert(writer_id, writer);
            ok_text(format!("file_writer created: {}", writer_id))
        }
        Err(e) => err_text(format!("failed to create '{}': {}", params.path, e)),
    }
}
