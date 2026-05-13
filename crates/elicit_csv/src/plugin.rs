//! [`CsvPlugin`] — stateful MCP plugin holding live csv objects.
//!
//! Six UUID-keyed maps:
//! - `reader_builders` — in-progress [`csv::ReaderBuilder`] configurations
//! - `writer_builders` — in-progress [`csv::WriterBuilder`] configurations
//! - `mem_readers`     — live [`csv::Reader`]`<`[`Cursor`]`<`[`Vec`]`<u8>>>` (in-memory)
//! - `file_readers`    — live [`csv::Reader`]`<`[`File`]`>` (file-backed)
//! - `mem_writers`     — live [`csv::Writer`]`<`[`Vec`]`<u8>>` (in-memory)
//! - `file_writers`    — live [`csv::Writer`]`<`[`File`]`>` (file-backed)

use std::{
    collections::HashMap,
    fs::File,
    io::Cursor,
    sync::{Arc, Mutex},
};

use elicitation::{ElicitPlugin, plugin::PluginContext};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use uuid::Uuid;

pub struct CsvCtx {
    pub reader_builders: Mutex<HashMap<Uuid, csv::ReaderBuilder>>,
    pub writer_builders: Mutex<HashMap<Uuid, csv::WriterBuilder>>,
    pub mem_readers: Mutex<HashMap<Uuid, csv::Reader<Cursor<Vec<u8>>>>>,
    pub file_readers: Mutex<HashMap<Uuid, csv::Reader<File>>>,
    pub mem_writers: Mutex<HashMap<Uuid, csv::Writer<Vec<u8>>>>,
    pub file_writers: Mutex<HashMap<Uuid, csv::Writer<File>>>,
}

impl std::fmt::Debug for CsvCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CsvCtx")
            .field(
                "reader_builders",
                &self.reader_builders.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field(
                "writer_builders",
                &self.writer_builders.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field(
                "mem_readers",
                &self.mem_readers.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field(
                "file_readers",
                &self.file_readers.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field(
                "mem_writers",
                &self.mem_writers.lock().map(|g| g.len()).unwrap_or(0),
            )
            .field(
                "file_writers",
                &self.file_writers.lock().map(|g| g.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl Default for CsvCtx {
    fn default() -> Self {
        CsvCtx {
            reader_builders: Mutex::new(HashMap::new()),
            writer_builders: Mutex::new(HashMap::new()),
            mem_readers: Mutex::new(HashMap::new()),
            file_readers: Mutex::new(HashMap::new()),
            mem_writers: Mutex::new(HashMap::new()),
            file_writers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for CsvCtx {}

/// MCP plugin for csv reader/writer operations.
///
/// Holds live [`csv::ReaderBuilder`], [`csv::WriterBuilder`], [`csv::Reader`],
/// and [`csv::Writer`] instances keyed by UUID.
#[derive(ElicitPlugin)]
#[plugin(name = "csv")]
pub struct CsvPlugin(pub Arc<CsvCtx>);

impl CsvPlugin {
    pub fn new() -> Self {
        CsvPlugin(Arc::new(CsvCtx::default()))
    }
}

impl Default for CsvPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CsvPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CsvPlugin").field(&self.0).finish()
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Parse a UUID string, returning an [`ErrorData`] on failure.
pub(crate) fn parse_uuid(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|e| ErrorData::invalid_params(format!("invalid UUID '{}': {}", s, e), None))
}

/// Wrap a plain text message in a successful [`CallToolResult`].
pub(crate) fn ok_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(msg.into())]))
}

/// Serialize a value as JSON and wrap in a successful [`CallToolResult`].
pub(crate) fn ok_json<T: serde::Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    match serde_json::to_string(value) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => ok_text(format!("serialization error: {}", e)),
    }
}

/// Return an error result with an invalid_params [`ErrorData`].
pub(crate) fn err_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::invalid_params(msg.into(), None))
}
