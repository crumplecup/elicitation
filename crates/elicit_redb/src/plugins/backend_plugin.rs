//! `RedbBackendPlugin` — `StorageBackend` implementation skeletons.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn ok(text: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(text.into())]))
}

// ── params ────────────────────────────────────────────────────────────────────

/// Parameters for `redb_backend__impl_storage`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImplStorageParams {
    /// Rust type name to implement `redb::StorageBackend` for.
    pub type_name: String,
}

/// Parameters for `redb_backend__read_impl`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadImplParams {
    /// Rust type name containing the in-memory `Vec<u8>` backing store.
    pub type_name: String,
}

/// Parameters for `redb_backend__write_impl`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WriteImplParams {
    /// Rust type name containing the in-memory `Vec<u8>` backing store.
    pub type_name: String,
}

/// Parameters for `redb_backend__in_memory_struct`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InMemoryParams {
    /// Rust type name for the complete in-memory `StorageBackend` struct.
    pub type_name: String,
}

// ── tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "redb_backend",
    name = "redb_backend__impl_storage",
    description = "Emit a `impl redb::StorageBackend for MyBackend` skeleton with all required method signatures."
)]
#[instrument]
async fn redb_backend_impl_storage(p: ImplStorageParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "impl redb::StorageBackend for {t} {{\n\
    fn len(&self) -> Result<u64, redb::StorageError> {{\n\
        todo!()\n\
    }}\n\
\n\
    fn read(&self, offset: u64, len: usize) -> Result<Vec<u8>, redb::StorageError> {{\n\
        todo!()\n\
    }}\n\
\n\
    fn set_len(&self, len: u64) -> Result<(), redb::StorageError> {{\n\
        todo!()\n\
    }}\n\
\n\
    fn write(&self, offset: u64, data: &[u8]) -> Result<(), redb::StorageError> {{\n\
        todo!()\n\
    }}\n\
\n\
    fn sync_data(&self, _eventual: bool) -> Result<(), redb::StorageError> {{\n\
        todo!()\n\
    }}\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_backend",
    name = "redb_backend__read_impl",
    description = "Emit a `fn read` body that reads bytes from an in-memory `Vec<u8>` slice."
)]
#[instrument]
async fn redb_backend_read_impl(p: ReadImplParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "// Inside impl redb::StorageBackend for {t}:\n\
fn read(&self, offset: u64, len: usize) -> Result<Vec<u8>, redb::StorageError> {{\n\
    let data = self.data.lock().unwrap();\n\
    let start = offset as usize;\n\
    let end = start + len;\n\
    if end > data.len() {{\n\
        return Err(redb::StorageError::Io(std::io::Error::new(\n\
            std::io::ErrorKind::UnexpectedEof,\n\
            \"read past end of in-memory store\",\n\
        )));\n\
    }}\n\
    Ok(data[start..end].to_vec())\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_backend",
    name = "redb_backend__write_impl",
    description = "Emit `fn write` and `fn sync_data` bodies for an in-memory `StorageBackend`."
)]
#[instrument]
async fn redb_backend_write_impl(p: WriteImplParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "// Inside impl redb::StorageBackend for {t}:\n\
fn write(&self, offset: u64, data: &[u8]) -> Result<(), redb::StorageError> {{\n\
    let mut store = self.data.lock().unwrap();\n\
    let start = offset as usize;\n\
    let end = start + data.len();\n\
    if end > store.len() {{\n\
        store.resize(end, 0);\n\
    }}\n\
    store[start..end].copy_from_slice(data);\n\
    Ok(())\n\
}}\n\
\n\
fn sync_data(&self, _eventual: bool) -> Result<(), redb::StorageError> {{\n\
    // No-op for in-memory backend.\n\
    Ok(())\n\
}}"
    ))
}

#[elicit_tool(
    plugin = "redb_backend",
    name = "redb_backend__in_memory_struct",
    description = "Emit a complete in-memory `StorageBackend` struct and full `impl StorageBackend` for testing or ephemeral use."
)]
#[instrument]
async fn redb_backend_in_memory_struct(p: InMemoryParams) -> Result<CallToolResult, ErrorData> {
    let t = &p.type_name;
    ok(format!(
        "use std::sync::Mutex;\n\
\n\
/// In-memory `redb::StorageBackend` — useful for tests or ephemeral databases.\n\
pub struct {t} {{\n\
    data: Mutex<Vec<u8>>,\n\
}}\n\
\n\
impl {t} {{\n\
    pub fn new() -> Self {{\n\
        Self {{ data: Mutex::new(Vec::new()) }}\n\
    }}\n\
}}\n\
\n\
impl Default for {t} {{\n\
    fn default() -> Self {{ Self::new() }}\n\
}}\n\
\n\
impl redb::StorageBackend for {t} {{\n\
    fn len(&self) -> Result<u64, redb::StorageError> {{\n\
        Ok(self.data.lock().unwrap().len() as u64)\n\
    }}\n\
\n\
    fn read(&self, offset: u64, len: usize) -> Result<Vec<u8>, redb::StorageError> {{\n\
        let data = self.data.lock().unwrap();\n\
        let start = offset as usize;\n\
        let end = start + len;\n\
        if end > data.len() {{\n\
            return Err(redb::StorageError::Io(std::io::Error::new(\n\
                std::io::ErrorKind::UnexpectedEof,\n\
                \"read past end\",\n\
            )));\n\
        }}\n\
        Ok(data[start..end].to_vec())\n\
    }}\n\
\n\
    fn set_len(&self, len: u64) -> Result<(), redb::StorageError> {{\n\
        self.data.lock().unwrap().resize(len as usize, 0);\n\
        Ok(())\n\
    }}\n\
\n\
    fn write(&self, offset: u64, data: &[u8]) -> Result<(), redb::StorageError> {{\n\
        let mut store = self.data.lock().unwrap();\n\
        let start = offset as usize;\n\
        let end = start + data.len();\n\
        if end > store.len() {{\n\
            store.resize(end, 0);\n\
        }}\n\
        store[start..end].copy_from_slice(data);\n\
        Ok(())\n\
    }}\n\
\n\
    fn sync_data(&self, _eventual: bool) -> Result<(), redb::StorageError> {{\n\
        Ok(())\n\
    }}\n\
}}"
    ))
}

// ── plugin ────────────────────────────────────────────────────────────────────

/// Plugin providing `redb::StorageBackend` implementation skeletons.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "redb_backend")]
pub struct RedbBackendPlugin;
