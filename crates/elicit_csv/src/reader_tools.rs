//! MCP tools shadowing [`csv::Reader`] methods.
//!
//! Works with both in-memory (`mem_readers`) and file-backed (`file_readers`) readers.

use std::sync::Arc;

use elicitation::{CsvByteRecord, CsvPosition, CsvStringRecord, elicit_tool};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::plugin::{CsvCtx, err_text, ok_json, ok_text, parse_uuid};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderHeadersParams {
    /// UUID from csv__reader_builder__from_reader or __from_path.
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__headers",
    description = "Return the header row from a CSV reader (if has_headers was enabled)."
)]
#[instrument(skip(ctx))]
pub async fn reader_headers(
    ctx: Arc<CsvCtx>,
    params: ReaderHeadersParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    {
        let mut guard = ctx.mem_readers.lock().expect("mem_readers lock");
        if let Some(r) = guard.get_mut(&id) {
            return match r.headers() {
                Ok(h) => ok_json(&CsvStringRecord::from(h.clone())),
                Err(e) => err_text(format!("headers error: {}", e)),
            };
        }
    }
    {
        let mut guard = ctx.file_readers.lock().expect("file_readers lock");
        if let Some(r) = guard.get_mut(&id) {
            return match r.headers() {
                Ok(h) => ok_json(&CsvStringRecord::from(h.clone())),
                Err(e) => err_text(format!("headers error: {}", e)),
            };
        }
    }
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderNextRecordParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__next_record",
    description = "Read the next string record from a CSV reader. Returns null when exhausted."
)]
#[instrument(skip(ctx))]
pub async fn reader_next_record(
    ctx: Arc<CsvCtx>,
    params: ReaderNextRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! try_reader {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get_mut(&id) {
                let mut record = csv::StringRecord::new();
                return match r.read_record(&mut record) {
                    Ok(true) => ok_json(&CsvStringRecord::from(record)),
                    Ok(false) => ok_text("null"),
                    Err(e) => err_text(format!("read error: {}", e)),
                };
            }
        }};
    }
    try_reader!(ctx.mem_readers);
    try_reader!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderNextByteRecordParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__next_byte_record",
    description = "Read the next byte record from a CSV reader. Returns null when exhausted."
)]
#[instrument(skip(ctx))]
pub async fn reader_next_byte_record(
    ctx: Arc<CsvCtx>,
    params: ReaderNextByteRecordParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! try_reader {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get_mut(&id) {
                let mut record = csv::ByteRecord::new();
                return match r.read_byte_record(&mut record) {
                    Ok(true) => ok_json(&CsvByteRecord::from(record)),
                    Ok(false) => ok_text("null"),
                    Err(e) => err_text(format!("read error: {}", e)),
                };
            }
        }};
    }
    try_reader!(ctx.mem_readers);
    try_reader!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderAllRecordsParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__all_records",
    description = "Drain all remaining string records from a CSV reader into a JSON array."
)]
#[instrument(skip(ctx))]
pub async fn reader_all_records(
    ctx: Arc<CsvCtx>,
    params: ReaderAllRecordsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! drain {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get_mut(&id) {
                let mut out: Vec<CsvStringRecord> = Vec::new();
                let mut record = csv::StringRecord::new();
                loop {
                    match r.read_record(&mut record) {
                        Ok(true) => out.push(CsvStringRecord::from(record.clone())),
                        Ok(false) => break,
                        Err(e) => return err_text(format!("read error: {}", e)),
                    }
                }
                return ok_json(&out);
            }
        }};
    }
    drain!(ctx.mem_readers);
    drain!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderAllByteRecordsParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__all_byte_records",
    description = "Drain all remaining byte records from a CSV reader into a JSON array."
)]
#[instrument(skip(ctx))]
pub async fn reader_all_byte_records(
    ctx: Arc<CsvCtx>,
    params: ReaderAllByteRecordsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! drain {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get_mut(&id) {
                let mut out: Vec<CsvByteRecord> = Vec::new();
                let mut record = csv::ByteRecord::new();
                loop {
                    match r.read_byte_record(&mut record) {
                        Ok(true) => out.push(CsvByteRecord::from(record.clone())),
                        Ok(false) => break,
                        Err(e) => return err_text(format!("read error: {}", e)),
                    }
                }
                return ok_json(&out);
            }
        }};
    }
    drain!(ctx.mem_readers);
    drain!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderPositionParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__position",
    description = "Return the current position (byte offset, line, record index) of a CSV reader."
)]
#[instrument(skip(ctx))]
pub async fn reader_position(
    ctx: Arc<CsvCtx>,
    params: ReaderPositionParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! get_pos {
        ($map:expr) => {{
            let guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get(&id) {
                return ok_json(&CsvPosition::from(r.position()));
            }
        }};
    }
    get_pos!(ctx.mem_readers);
    get_pos!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderIsDoneParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__is_done",
    description = "Check whether a CSV reader has been exhausted. Note: advances position by one record."
)]
#[instrument(skip(ctx))]
pub async fn reader_is_done(
    ctx: Arc<CsvCtx>,
    params: ReaderIsDoneParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! check {
        ($map:expr) => {{
            let mut guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get_mut(&id) {
                let mut record = csv::ByteRecord::new();
                return match r.read_byte_record(&mut record) {
                    Ok(has_more) => ok_text((!has_more).to_string()),
                    Err(e) => err_text(format!("read error: {}", e)),
                };
            }
        }};
    }
    check!(ctx.mem_readers);
    check!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderRecordCountParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__record_count",
    description = "Return the number of records read so far (from the reader's current position)."
)]
#[instrument(skip(ctx))]
pub async fn reader_record_count(
    ctx: Arc<CsvCtx>,
    params: ReaderRecordCountParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    macro_rules! count {
        ($map:expr) => {{
            let guard = $map.lock().expect("reader lock");
            if let Some(r) = guard.get(&id) {
                return ok_text(r.position().record().to_string());
            }
        }};
    }
    count!(ctx.mem_readers);
    count!(ctx.file_readers);
    err_text(format!("reader not found: {}", id))
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderSeekParams {
    pub reader_id: String,
    /// Position to seek to (obtained from csv__reader__position).
    pub position: CsvPosition,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__seek",
    description = "Seek an in-memory CSV reader to a previously captured position."
)]
#[instrument(skip(ctx))]
pub async fn reader_seek(
    ctx: Arc<CsvCtx>,
    params: ReaderSeekParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    let csv_pos = csv::Position::from(params.position);
    let mut guard = ctx.mem_readers.lock().expect("mem_readers lock");
    if let Some(r) = guard.get_mut(&id) {
        return match r.seek(csv_pos) {
            Ok(()) => ok_text("seek successful"),
            Err(e) => err_text(format!("seek error: {}", e)),
        };
    }
    err_text("seek only supported for in-memory readers; file readers do not support seek")
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderCloseParams {
    pub reader_id: String,
}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__close",
    description = "Remove a CSV reader from the context, releasing its resources."
)]
#[instrument(skip(ctx))]
pub async fn reader_close(
    ctx: Arc<CsvCtx>,
    params: ReaderCloseParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_uuid(&params.reader_id)?;
    let removed_mem = ctx
        .mem_readers
        .lock()
        .expect("mem_readers lock")
        .remove(&id)
        .is_some();
    let removed_file = ctx
        .file_readers
        .lock()
        .expect("file_readers lock")
        .remove(&id)
        .is_some();
    if removed_mem || removed_file {
        ok_text(format!("reader {} closed", id))
    } else {
        err_text(format!("reader not found: {}", id))
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReaderListParams {}

#[elicit_tool(
    plugin = "csv",
    name = "csv__reader__list",
    description = "List all active CSV reader UUIDs (both in-memory and file-backed)."
)]
#[instrument(skip(ctx))]
pub async fn reader_list(
    ctx: Arc<CsvCtx>,
    _params: ReaderListParams,
) -> Result<CallToolResult, ErrorData> {
    let mem: Vec<String> = ctx
        .mem_readers
        .lock()
        .expect("mem_readers lock")
        .keys()
        .map(|id| format!("mem:{}", id))
        .collect();
    let file: Vec<String> = ctx
        .file_readers
        .lock()
        .expect("file_readers lock")
        .keys()
        .map(|id| format!("file:{}", id))
        .collect();
    let all: Vec<String> = mem.into_iter().chain(file).collect();
    ok_json(&all)
}
