//! `TokioFsPlugin` — MCP tools for async file system operations.
//!
//! Wraps [`tokio::fs`] standalone functions as stateless MCP tools. All
//! parameters and return values are serializable — no handles or futures
//! cross the MCP boundary.
//!
//! # Tool namespace: `tokio_fs__*`
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `read_to_string` | `path` | `{ content }` |
//! | `read_bytes` | `path` | `{ bytes: [u8], len }` |
//! | `write_text` | `path, content` | `{ bytes_written }` |
//! | `write_bytes` | `path, bytes: [u8]` | `{ bytes_written }` |
//! | `create_dir` | `path` | `{ ok }` |
//! | `create_dir_all` | `path` | `{ ok }` |
//! | `remove_dir` | `path` | `{ ok }` |
//! | `remove_dir_all` | `path` | `{ ok }` |
//! | `remove_file` | `path` | `{ ok }` |
//! | `rename` | `from, to` | `{ ok }` |
//! | `copy` | `from, to` | `{ bytes_copied }` |
//! | `metadata` | `path` | `{ size, is_file, is_dir, is_symlink, readonly, modified_unix_ms? }` |
//! | `read_dir` | `path` | `{ entries: [{name, is_file, is_dir, is_symlink}] }` |
//! | `canonicalize` | `path` | `{ canonical_path }` |

use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `tokio_fs__read_to_string`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadToStringParams {
    /// Path to the file to read.
    pub path: String,
}

/// Parameters for `tokio_fs__read_bytes`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadBytesParams {
    /// Path to the file to read.
    pub path: String,
}

/// Parameters for `tokio_fs__write_text`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteTextParams {
    /// Path to write to (created or truncated).
    pub path: String,
    /// UTF-8 text content to write.
    pub content: String,
}

/// Parameters for `tokio_fs__write_bytes`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WriteBytesParams {
    /// Path to write to (created or truncated).
    pub path: String,
    /// Raw bytes to write.
    pub bytes: Vec<u8>,
}

/// Parameters for a single-path `tokio_fs__*` operation.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PathParams {
    /// Target path.
    pub path: String,
}

/// Parameters for a two-path `tokio_fs__*` operation (rename, copy).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FromToParams {
    /// Source path.
    pub from: String,
    /// Destination path.
    pub to: String,
}

// ── Result structs ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ReadToStringResult {
    content: String,
}

#[derive(Serialize)]
struct ReadBytesResult {
    bytes: Vec<u8>,
    len: usize,
}

#[derive(Serialize)]
struct BytesWrittenResult {
    bytes_written: usize,
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

#[derive(Serialize)]
struct CopyResult {
    bytes_copied: u64,
}

#[derive(Serialize)]
struct MetadataResult {
    size: u64,
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
    readonly: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified_unix_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessed_unix_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_unix_ms: Option<u64>,
}

#[derive(Serialize)]
struct DirEntry {
    name: String,
    is_file: bool,
    is_dir: bool,
    is_symlink: bool,
}

#[derive(Serialize)]
struct ReadDirResult {
    entries: Vec<DirEntry>,
}

#[derive(Serialize)]
struct CanonicalizeResult {
    canonical_path: String,
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn io_err(e: std::io::Error) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

fn system_time_to_unix_ms(t: SystemTime) -> Option<u64> {
    t.duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_millis() as u64)
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_fs__*` tools for async file system operations.
///
/// All tools wrap [`tokio::fs`] standalone functions — no handles or futures
/// cross the MCP boundary.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tokio_fs")]
pub struct TokioFsPlugin;

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__read_to_string",
    description = "Read the entire contents of a file as a UTF-8 string. \
                   Assumes: path exists and is a valid UTF-8 file.",
    emit = Auto
)]
async fn fs_read_to_string(p: ReadToStringParams) -> Result<CallToolResult, ErrorData> {
    let content = tokio::fs::read_to_string(&p.path).await.map_err(io_err)?;
    Ok(json_result(&ReadToStringResult { content }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__read_bytes",
    description = "Read the entire contents of a file as raw bytes. \
                   Bytes are returned as a JSON array of integers (0–255). \
                   Assumes: path exists and is readable.",
    emit = Auto
)]
async fn fs_read_bytes(p: ReadBytesParams) -> Result<CallToolResult, ErrorData> {
    let bytes = tokio::fs::read(&p.path).await.map_err(io_err)?;
    let len = bytes.len();
    Ok(json_result(&ReadBytesResult { bytes, len }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__write_text",
    description = "Write a UTF-8 string to a file, creating or truncating it. \
                   Returns the number of bytes written. \
                   Assumes: parent directory exists.",
    emit = Auto
)]
async fn fs_write_text(p: WriteTextParams) -> Result<CallToolResult, ErrorData> {
    let bytes = p.content.as_bytes();
    let bytes_written = bytes.len();
    tokio::fs::write(&p.path, bytes).await.map_err(io_err)?;
    Ok(json_result(&BytesWrittenResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__write_bytes",
    description = "Write raw bytes to a file, creating or truncating it. \
                   Bytes are provided as a JSON array of integers (0–255). \
                   Returns the number of bytes written. \
                   Assumes: parent directory exists.",
    emit = Auto
)]
async fn fs_write_bytes(p: WriteBytesParams) -> Result<CallToolResult, ErrorData> {
    let bytes_written = p.bytes.len();
    tokio::fs::write(&p.path, &p.bytes).await.map_err(io_err)?;
    Ok(json_result(&BytesWrittenResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__create_dir",
    description = "Create a directory. Fails if the parent does not exist or the \
                   directory already exists. Use `create_dir_all` to create \
                   intermediate directories. \
                   Assumes: parent directory exists.",
    emit = Auto
)]
async fn fs_create_dir(p: PathParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::create_dir(&p.path).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__create_dir_all",
    description = "Recursively create a directory and all missing parent directories. \
                   No-op if the directory already exists.",
    emit = Auto
)]
async fn fs_create_dir_all(p: PathParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::create_dir_all(&p.path).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__remove_dir",
    description = "Remove an empty directory. Fails if the directory is not empty. \
                   Use `remove_dir_all` to remove a directory and its contents. \
                   Assumes: path is an existing empty directory.",
    emit = Auto
)]
async fn fs_remove_dir(p: PathParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::remove_dir(&p.path).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__remove_dir_all",
    description = "Recursively remove a directory and all its contents. \
                   ⚠ Irreversible — use with caution. \
                   Assumes: path is an existing directory.",
    emit = Auto
)]
async fn fs_remove_dir_all(p: PathParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::remove_dir_all(&p.path).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__remove_file",
    description = "Remove a file. \
                   Assumes: path is an existing file (not a directory).",
    emit = Auto
)]
async fn fs_remove_file(p: PathParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::remove_file(&p.path).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__rename",
    description = "Rename or move a file or directory. Overwrites the destination \
                   if it already exists (on most platforms). \
                   Assumes: `from` exists; `to` parent directory exists.",
    emit = Auto
)]
async fn fs_rename(p: FromToParams) -> Result<CallToolResult, ErrorData> {
    tokio::fs::rename(&p.from, &p.to).await.map_err(io_err)?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__copy",
    description = "Copy the contents of one file to another. Returns the number of \
                   bytes copied. Destination is created or truncated. \
                   Assumes: `from` is an existing file; `to` parent directory exists.",
    emit = Auto
)]
async fn fs_copy(p: FromToParams) -> Result<CallToolResult, ErrorData> {
    let bytes_copied = tokio::fs::copy(&p.from, &p.to).await.map_err(io_err)?;
    Ok(json_result(&CopyResult { bytes_copied }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__metadata",
    description = "Query metadata for a file or directory: size, type flags, \
                   read-only status, and timestamps (milliseconds since Unix epoch). \
                   Follows symlinks (use `symlink_metadata` for the link itself). \
                   Assumes: path exists.",
    emit = Auto
)]
async fn fs_metadata(p: PathParams) -> Result<CallToolResult, ErrorData> {
    let m = tokio::fs::metadata(&p.path).await.map_err(io_err)?;
    Ok(json_result(&MetadataResult {
        size: m.len(),
        is_file: m.is_file(),
        is_dir: m.is_dir(),
        is_symlink: m.is_symlink(),
        readonly: m.permissions().readonly(),
        modified_unix_ms: m.modified().ok().and_then(system_time_to_unix_ms),
        accessed_unix_ms: m.accessed().ok().and_then(system_time_to_unix_ms),
        created_unix_ms: m.created().ok().and_then(system_time_to_unix_ms),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__read_dir",
    description = "List the entries in a directory. Returns name, file/dir/symlink \
                   flags for each entry. Does not recurse into subdirectories. \
                   Assumes: path is an existing directory.",
    emit = Auto
)]
async fn fs_read_dir(p: PathParams) -> Result<CallToolResult, ErrorData> {
    let mut reader = tokio::fs::read_dir(&p.path).await.map_err(io_err)?;
    let mut entries = Vec::new();
    while let Some(entry) = reader.next_entry().await.map_err(io_err)? {
        let file_type = entry.file_type().await.map_err(io_err)?;
        entries.push(DirEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            is_file: file_type.is_file(),
            is_dir: file_type.is_dir(),
            is_symlink: file_type.is_symlink(),
        });
    }
    Ok(json_result(&ReadDirResult { entries }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_fs",
    name = "tokio_fs__canonicalize",
    description = "Resolve a path to its absolute canonical form, resolving all \
                   symlinks and `.` / `..` components. \
                   Assumes: path exists.",
    emit = Auto
)]
async fn fs_canonicalize(p: PathParams) -> Result<CallToolResult, ErrorData> {
    let canonical = tokio::fs::canonicalize(&p.path).await.map_err(io_err)?;
    Ok(json_result(&CanonicalizeResult {
        canonical_path: canonical.to_string_lossy().into_owned(),
    }))
}
