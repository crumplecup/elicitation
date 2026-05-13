//! MCP tools shadowing [`csv::Writer`] methods.

use std::sync::Arc;

use elicitation::{CsvByteRecord, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{CsvCtx, err_text, ok_json, ok_text, parse_uuid};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterWriteRecordParams {
    pub writer_id: String,
    /// Fields to write as a single CSV record.
    pub fields: Vec<String>,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__write_record",
    description = "Write a record (array of string fields) to a CSV writer."
)]
#[instrument(skip(ctx))]
pub async fn writer_write_record(
    ctx: Arc<CsvCtx>,
    params: WriterWriteRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    macro_rules! write_rec {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("writer lock");
            if let Some(w) = guard.get_mut(&id) {
                return match w.write_record(&params.fields) {
                    Ok(()) => ok_text("record written"),
                    Err(e) => err_text(format!("write error: {}", e)),
                };
            }
        }};
    }
    write_rec!(ctx.mem_writers);
    write_rec!(ctx.file_writers);
    err_text(format!("writer not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterWriteByteRecordParams {
    pub writer_id: String,
    /// Byte record fields (Vec<Vec<u8>>).
    pub record: CsvByteRecord,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__write_byte_record",
    description = "Write a byte record to a CSV writer."
)]
#[instrument(skip(ctx))]
pub async fn writer_write_byte_record(
    ctx: Arc<CsvCtx>,
    params: WriterWriteByteRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    let csv_record = csv::ByteRecord::from(params.record);
    macro_rules! write_rec {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("writer lock");
            if let Some(w) = guard.get_mut(&id) {
                return match w.write_byte_record(&csv_record) {
                    Ok(()) => ok_text("byte record written"),
                    Err(e) => err_text(format!("write error: {}", e)),
                };
            }
        }};
    }
    write_rec!(ctx.mem_writers);
    write_rec!(ctx.file_writers);
    err_text(format!("writer not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterWriteFieldParams {
    pub writer_id: String,
    /// Single field value; call csv__writer__terminate_record after all fields.
    pub field: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__write_field",
    description = "Write a single field to a CSV writer. Call csv__writer__terminate_record after all fields."
)]
#[instrument(skip(ctx))]
pub async fn writer_write_field(
    ctx: Arc<CsvCtx>,
    params: WriterWriteFieldParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    macro_rules! write_field {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("writer lock");
            if let Some(w) = guard.get_mut(&id) {
                return match w.write_field(params.field.as_bytes()) {
                    Ok(()) => ok_text("field written"),
                    Err(e) => err_text(format!("write error: {}", e)),
                };
            }
        }};
    }
    write_field!(ctx.mem_writers);
    write_field!(ctx.file_writers);
    err_text(format!("writer not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterTerminateRecordParams {
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__terminate_record",
    description = "Terminate the current record after individual field writes."
)]
#[instrument(skip(ctx))]
pub async fn writer_terminate_record(
    ctx: Arc<CsvCtx>,
    params: WriterTerminateRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    macro_rules! terminate {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("writer lock");
            if let Some(w) = guard.get_mut(&id) {
                return match w.write_record(std::iter::empty::<&str>()) {
                    Ok(()) => ok_text("record terminated"),
                    Err(e) => err_text(format!("terminate error: {}", e)),
                };
            }
        }};
    }
    terminate!(ctx.mem_writers);
    terminate!(ctx.file_writers);
    err_text(format!("writer not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterFlushParams {
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__flush",
    description = "Flush the CSV writer's internal buffer to the underlying writer."
)]
#[instrument(skip(ctx))]
pub async fn writer_flush(
    ctx: Arc<CsvCtx>,
    params: WriterFlushParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    macro_rules! flush {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("writer lock");
            if let Some(w) = guard.get_mut(&id) {
                return match w.flush() {
                    Ok(()) => ok_text("flushed"),
                    Err(e) => err_text(format!("flush error: {}", e)),
                };
            }
        }};
    }
    flush!(ctx.mem_writers);
    flush!(ctx.file_writers);
    err_text(format!("writer not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterGetRefParams {
    /// In-memory writer UUID only.
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__get_ref",
    description = "Peek at the current contents of an in-memory CSV writer as a UTF-8 string (without consuming it)."
)]
#[instrument(skip(ctx))]
pub async fn writer_get_ref(
    ctx: Arc<CsvCtx>,
    params: WriterGetRefParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    let mut guard = ctx.mem_writers.lock().expect("mem_writers lock");
    if let Some(w) = guard.get_mut(&id) {
        if let Err(e) = w.flush() {
            return err_text(format!("flush error: {}", e));
        }
        return match String::from_utf8(w.get_ref().clone()) {
            Ok(s) => ok_text(s),
            Err(e) => err_text(format!("utf8 error: {}", e)),
        };
    }
    err_text("get_ref is only supported for in-memory writers")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterIntoInnerParams {
    /// In-memory writer UUID to consume.
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__into_inner",
    description = "Flush and consume an in-memory CSV writer, returning its CSV string. The UUID is removed from context."
)]
#[instrument(skip(ctx))]
pub async fn writer_into_inner(
    ctx: Arc<CsvCtx>,
    params: WriterIntoInnerParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    let writer = ctx
        .mem_writers
        .lock()
        .expect("mem_writers lock")
        .remove(&id);
    let Some(writer) = writer else {
        return err_text("into_inner only supported for in-memory writers");
    };
    match writer.into_inner() {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => ok_text(s),
            Err(e) => err_text(format!("utf8 error: {}", e)),
        },
        Err(e) => err_text(format!("into_inner error: {}", e.error())),
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterCloseParams {
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__close",
    description = "Remove a CSV writer from context without consuming its contents."
)]
#[instrument(skip(ctx))]
pub async fn writer_close(
    ctx: Arc<CsvCtx>,
    params: WriterCloseParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.writer_id)?;
    let mem = ctx
        .mem_writers
        .lock()
        .expect("mem_writers lock")
        .remove(&id)
        .is_some();
    let file = ctx
        .file_writers
        .lock()
        .expect("file_writers lock")
        .remove(&id)
        .is_some();
    if mem || file {
        ok_text(format!("writer {} closed", id))
    } else {
        err_text(format!("writer not found: {}", id))
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterListParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__list",
    description = "List all active CSV writer UUIDs (both in-memory and file-backed)."
)]
#[instrument(skip(ctx))]
pub async fn writer_list(
    ctx: Arc<CsvCtx>,
    _params: WriterListParams,
) -> Result<CallToolResult, ErrorData> {
    let mem: Vec<String> = ctx
        .mem_writers
        .lock()
        .expect("mem_writers lock")
        .keys()
        .map(|id| format!("mem:{}", id))
        .collect();
    let file: Vec<String> = ctx
        .file_writers
        .lock()
        .expect("file_writers lock")
        .keys()
        .map(|id| format!("file:{}", id))
        .collect();
    let all: Vec<String> = mem.into_iter().chain(file).collect();
    ok_json(&all)
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriterRecordCountParams {
    pub writer_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__writer__record_count",
    description = "Note: csv::Writer does not track record count internally. Count write_record calls in your workflow."
)]
#[instrument(skip(_ctx))]
pub async fn writer_record_count(
    _ctx: Arc<CsvCtx>,
    _params: WriterRecordCountParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(
        "csv::Writer does not track record count internally; count write_record calls in your workflow",
    )
}
