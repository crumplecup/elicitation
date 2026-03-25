//! `TokioProcessPlugin` — MCP tools for tokio process spawning.
//!
//! Provides a high-level `process_run` for one-shot command execution and a
//! set of stateful tools for long-running interactive processes. Spawned child
//! handles are held server-side in a UUID-keyed registry.
//!
//! # Tool namespace: `tokio_process__*`
//!
//! | Tool | Params | Returns | Notes |
//! |---|---|---|---|
//! | `process_run` | `program, args?, stdin_bytes?, env?, cwd?` | `{ stdout_bytes, stderr_bytes, exit_code, success }` | Run-to-completion; no registry entry |
//! | `process_spawn` | `program, args?, env?, cwd?, pipe_stdin, pipe_stdout, pipe_stderr` | `{ child_id, pid }` | Background child; I/O via pipe tools |
//! | `process_stdin_write` | `child_id, data` | `{ bytes_written }` | Requires `pipe_stdin = true` at spawn |
//! | `process_stdout_read` | `child_id, max_bytes?` | `{ data, bytes_read, eof }` | Requires `pipe_stdout = true` at spawn |
//! | `process_stderr_read` | `child_id, max_bytes?` | `{ data, bytes_read, eof }` | Requires `pipe_stderr = true` at spawn |
//! | `process_wait` | `child_id` | `{ exit_code, success }` | Blocks until child exits |
//! | `process_try_wait` | `child_id` | `{ exited, exit_code?, success? }` | Non-blocking exit poll |
//! | `process_kill` | `child_id` | `{ ok }` | Sends SIGKILL (unix) / TerminateProcess (windows) |
//! | `process_id` | `child_id` | `{ pid }` | OS process ID |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: `Command::spawn()` succeeded — the child process is running.
#[derive(Elicit)]
pub struct ProcessSpawned {}
impl Prop for ProcessSpawned {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_process_spawned_axiom() {
                let spawn_ok: bool = kani::any();
                kani::assume(spawn_ok);
                assert!(spawn_ok, "tokio::process::Command::spawn axiom: Ok => OS process is running");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_process_spawned(spawn_returned_ok: bool) -> (result: bool)
                ensures result == spawn_returned_ok,
            {
                spawn_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_process_spawned_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for ProcessSpawned {}


/// Proposition: `child.wait()` completed — the child process has exited.
#[derive(Elicit)]
pub struct ProcessExited {}
impl Prop for ProcessExited {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_process_exited_axiom() {
                let wait_ok: bool = kani::any();
                kani::assume(wait_ok);
                assert!(wait_ok, "tokio::process::Child::wait axiom: Ok => child process has exited");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_process_exited(wait_returned_ok: bool) -> (result: bool)
                ensures result == wait_returned_ok,
            {
                wait_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_process_exited_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for ProcessExited {}


/// Proposition: bytes were written to a child process's stdin pipe.
#[derive(Elicit)]
pub struct StdinWritten {}
impl Prop for StdinWritten {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_stdin_written_axiom() {
                let write_ok: bool = kani::any();
                kani::assume(write_ok);
                assert!(write_ok, "AsyncWriteExt::write_all(stdin) axiom: Ok => all bytes written to pipe");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_stdin_written(write_returned_ok: bool) -> (result: bool)
                ensures result == write_returned_ok,
            {
                write_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_stdin_written_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for StdinWritten {}


// ── Plugin context ────────────────────────────────────────────────────────────

/// Per-child I/O handles decomposed from the spawned [`Child`].
///
/// `stdin`/`stdout`/`stderr` are `None` when the corresponding pipe was not
/// requested at spawn time or has already been closed.
struct ChildEntry {
    child: Mutex<Child>,
    stdin: Mutex<Option<ChildStdin>>,
    stdout: Mutex<Option<ChildStdout>>,
    stderr: Mutex<Option<ChildStderr>>,
}

/// Shared state for all `tokio_process__*` tool calls.
pub struct ProcessCtx {
    children: Mutex<HashMap<Uuid, Arc<ChildEntry>>>,
}

impl ProcessCtx {
    fn new() -> Self {
        Self {
            children: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for ProcessCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

fn err_not_found(label: &str, id: Uuid) -> ErrorData {
    ErrorData::invalid_params(format!("{label} not found: {id}"), None)
}

// ── Param / result types ──────────────────────────────────────────────────────

/// Parameters for `tokio_process__process_run`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessRunParams {
    /// Executable name or path, e.g. `"echo"` or `"/usr/bin/cat"`.
    pub program: String,
    /// Command-line arguments (default: empty).
    #[serde(default)]
    pub args: Vec<String>,
    /// Optional bytes to feed to stdin before closing it (JSON array of u8).
    pub stdin_bytes: Option<Vec<u8>>,
    /// Environment variable overrides as `["KEY=VALUE", ...]`.
    #[serde(default)]
    pub env: Vec<String>,
    /// Working directory for the child process.
    pub cwd: Option<String>,
}

#[derive(Serialize)]
struct ProcessRunResult {
    /// Captured stdout as a JSON array of u8 bytes.
    stdout_bytes: Vec<u8>,
    /// Captured stderr as a JSON array of u8 bytes.
    stderr_bytes: Vec<u8>,
    /// Exit status code, or `null` if the process was terminated by a signal.
    exit_code: Option<i32>,
    success: bool,
}

/// Parameters for `tokio_process__process_spawn`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessSpawnParams {
    /// Executable name or path.
    pub program: String,
    /// Command-line arguments (default: empty).
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variable overrides as `["KEY=VALUE", ...]`.
    #[serde(default)]
    pub env: Vec<String>,
    /// Working directory for the child process.
    pub cwd: Option<String>,
    /// Pipe stdin so `process_stdin_write` can be used (default: false).
    #[serde(default)]
    pub pipe_stdin: bool,
    /// Pipe stdout so `process_stdout_read` can be used (default: true).
    #[serde(default = "default_true")]
    pub pipe_stdout: bool,
    /// Pipe stderr so `process_stderr_read` can be used (default: true).
    #[serde(default = "default_true")]
    pub pipe_stderr: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Serialize)]
struct ProcessSpawnResult {
    child_id: Uuid,
    /// OS-assigned process ID.
    pid: Option<u32>,
}

/// Parameters for `tokio_process__process_stdin_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessStdinWriteParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
    /// Raw bytes to write to stdin (JSON array of u8).
    pub data: Vec<u8>,
}

#[derive(Serialize)]
struct ProcessStdinWriteResult {
    bytes_written: usize,
}

/// Parameters for `tokio_process__process_stdout_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessStdoutReadParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
    /// Maximum bytes to read (default 65536).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct ProcessPipeReadResult {
    data: Vec<u8>,
    bytes_read: usize,
    eof: bool,
}

/// Parameters for `tokio_process__process_stderr_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessStderrReadParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
    /// Maximum bytes to read (default 65536).
    pub max_bytes: Option<usize>,
}

/// Parameters for `tokio_process__process_wait`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessWaitParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
}

#[derive(Serialize)]
struct ProcessExitResult {
    exit_code: Option<i32>,
    success: bool,
}

/// Parameters for `tokio_process__process_try_wait`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessTryWaitParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
}

#[derive(Serialize)]
struct ProcessTryWaitResult {
    exited: bool,
    exit_code: Option<i32>,
    success: Option<bool>,
}

/// Parameters for `tokio_process__process_kill`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessKillParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
}

/// Parameters for `tokio_process__process_id`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProcessIdParams {
    /// Child UUID returned by `process_spawn`.
    pub child_id: Uuid,
}

#[derive(Serialize)]
struct ProcessIdResult {
    pid: Option<u32>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_run",
    description = "Run a command to completion and capture its output. Optionally feed bytes \
                   to stdin before closing it. Returns stdout/stderr as JSON arrays of u8 bytes, \
                   the exit code, and a success flag. Blocks until the process exits.",
    emit = Auto
)]
async fn process_run(
    _ctx: Arc<ProcessCtx>,
    p: ProcessRunParams,
) -> Result<CallToolResult, ErrorData> {
    let mut cmd = build_command(&p.program, &p.args, &p.env, p.cwd.as_deref());
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd
        .spawn()
        .map_err(|e| ErrorData::invalid_params(format!("spawn failed: {e}"), None))?;

    // Write stdin if provided, then close it.
    if let Some(bytes) = p.stdin_bytes {
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&bytes)
                .await
                .map_err(|e| ErrorData::invalid_params(format!("stdin write failed: {e}"), None))?;
        }
    } else {
        drop(child.stdin.take());
    }

    let output = child
        .wait_with_output()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("wait_with_output failed: {e}"), None))?;

    Ok(json_result(&ProcessRunResult {
        stdout_bytes: output.stdout,
        stderr_bytes: output.stderr,
        exit_code: output.status.code(),
        success: output.status.success(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_spawn",
    description = "Spawn a child process in the background. Returns a child_id UUID and the OS \
                   pid. Use `pipe_stdout = true` (default) and `pipe_stderr = true` (default) \
                   to enable reading output. Use `pipe_stdin = true` to enable writing to stdin. \
                   Call `process_wait` or `process_kill` when done.",
    emit = Auto
)]
async fn process_spawn(
    ctx: Arc<ProcessCtx>,
    p: ProcessSpawnParams,
) -> Result<CallToolResult, ErrorData> {
    let mut cmd = build_command(&p.program, &p.args, &p.env, p.cwd.as_deref());
    cmd.stdin(if p.pipe_stdin {
        std::process::Stdio::piped()
    } else {
        std::process::Stdio::null()
    });
    cmd.stdout(if p.pipe_stdout {
        std::process::Stdio::piped()
    } else {
        std::process::Stdio::null()
    });
    cmd.stderr(if p.pipe_stderr {
        std::process::Stdio::piped()
    } else {
        std::process::Stdio::null()
    });

    let mut child = cmd
        .spawn()
        .map_err(|e| ErrorData::invalid_params(format!("spawn failed: {e}"), None))?;

    let pid = child.id();
    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let entry = Arc::new(ChildEntry {
        child: Mutex::new(child),
        stdin: Mutex::new(stdin),
        stdout: Mutex::new(stdout),
        stderr: Mutex::new(stderr),
    });
    let child_id = Uuid::new_v4();
    ctx.children.lock().await.insert(child_id, entry);
    let _proof: Established<ProcessSpawned> = Established::assert();
    Ok(json_result(&ProcessSpawnResult { child_id, pid }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_stdin_write",
    description = "Write bytes to a spawned child's stdin. Requires `pipe_stdin = true` at \
                   spawn time. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_stdin_write(
    ctx: Arc<ProcessCtx>,
    p: ProcessStdinWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .get(&p.child_id)
        .cloned()
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    let mut stdin_guard = entry.stdin.lock().await;
    let stdin = stdin_guard
        .as_mut()
        .ok_or_else(|| ErrorData::invalid_params("stdin not piped or already closed", None))?;
    let bytes_written = p.data.len();
    stdin
        .write_all(&p.data)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("stdin write failed: {e}"), None))?;
    let _proof: Established<StdinWritten> = Established::assert();
    Ok(json_result(&ProcessStdinWriteResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_stdout_read",
    description = "Read up to `max_bytes` (default 65536) from a spawned child's stdout. \
                   Returns raw bytes as a JSON array of u8 values. `eof = true` means the \
                   process has closed stdout. Requires `pipe_stdout = true` at spawn time. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_stdout_read(
    ctx: Arc<ProcessCtx>,
    p: ProcessStdoutReadParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .get(&p.child_id)
        .cloned()
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    let mut stdout_guard = entry.stdout.lock().await;
    let stdout = stdout_guard
        .as_mut()
        .ok_or_else(|| ErrorData::invalid_params("stdout not piped", None))?;
    let max = p.max_bytes.unwrap_or(65536).min(1 << 20);
    let mut buf = vec![0u8; max];
    let bytes_read = stdout
        .read(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("stdout read failed: {e}"), None))?;
    buf.truncate(bytes_read);
    Ok(json_result(&ProcessPipeReadResult {
        data: buf,
        bytes_read,
        eof: bytes_read == 0,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_stderr_read",
    description = "Read up to `max_bytes` (default 65536) from a spawned child's stderr. \
                   Returns raw bytes as a JSON array of u8 values. `eof = true` means the \
                   process has closed stderr. Requires `pipe_stderr = true` at spawn time. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_stderr_read(
    ctx: Arc<ProcessCtx>,
    p: ProcessStderrReadParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .get(&p.child_id)
        .cloned()
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    let mut stderr_guard = entry.stderr.lock().await;
    let stderr = stderr_guard
        .as_mut()
        .ok_or_else(|| ErrorData::invalid_params("stderr not piped", None))?;
    let max = p.max_bytes.unwrap_or(65536).min(1 << 20);
    let mut buf = vec![0u8; max];
    let bytes_read = stderr
        .read(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("stderr read failed: {e}"), None))?;
    buf.truncate(bytes_read);
    Ok(json_result(&ProcessPipeReadResult {
        data: buf,
        bytes_read,
        eof: bytes_read == 0,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_wait",
    description = "Block until a spawned child exits and return its exit status. Removes the \
                   child from the registry. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_wait(
    ctx: Arc<ProcessCtx>,
    p: ProcessWaitParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .remove(&p.child_id)
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    let status = entry
        .child
        .lock()
        .await
        .wait()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("wait failed: {e}"), None))?;
    let _proof: Established<ProcessExited> = Established::assert();
    Ok(json_result(&ProcessExitResult {
        exit_code: status.code(),
        success: status.success(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_try_wait",
    description = "Non-blocking check whether a spawned child has exited. Returns `exited = false` \
                   if the child is still running. Does not remove the child from the registry. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_try_wait(
    ctx: Arc<ProcessCtx>,
    p: ProcessTryWaitParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .get(&p.child_id)
        .cloned()
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    let maybe = entry
        .child
        .lock()
        .await
        .try_wait()
        .map_err(|e| ErrorData::invalid_params(format!("try_wait failed: {e}"), None))?;
    Ok(json_result(&ProcessTryWaitResult {
        exited: maybe.is_some(),
        exit_code: maybe.and_then(|s| s.code()),
        success: maybe.map(|s| s.success()),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_kill",
    description = "Kill a spawned child process (SIGKILL on Unix, TerminateProcess on Windows). \
                   Removes the child from the registry. \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_kill(
    ctx: Arc<ProcessCtx>,
    p: ProcessKillParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .children
        .lock()
        .await
        .remove(&p.child_id)
        .ok_or_else(|| err_not_found("child_id", p.child_id))?;
    entry
        .child
        .lock()
        .await
        .kill()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("kill failed: {e}"), None))?;
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_process",
    name = "tokio_process__process_id",
    description = "Return the OS process ID (pid) of a spawned child. Returns `null` if the \
                   pid is no longer available (process already reaped). \
                   Assumes: child_id was returned by `process_spawn`.",
    emit = Auto
)]
async fn process_id(ctx: Arc<ProcessCtx>, p: ProcessIdParams) -> Result<CallToolResult, ErrorData> {
    let pid = ctx
        .children
        .lock()
        .await
        .get(&p.child_id)
        .ok_or_else(|| err_not_found("child_id", p.child_id))?
        .child
        .lock()
        .await
        .id();
    Ok(json_result(&ProcessIdResult { pid }))
}

// ── Command builder helper ────────────────────────────────────────────────────

fn build_command(program: &str, args: &[String], env: &[String], cwd: Option<&str>) -> Command {
    let mut cmd = Command::new(program);
    cmd.args(args);
    for kv in env {
        if let Some((k, v)) = kv.split_once('=') {
            cmd.env(k, v);
        }
    }
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    cmd
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_process__*` tools for child process management.
///
/// Holds a UUID-keyed registry of spawned [`Child`] entries with decomposed
/// stdin/stdout/stderr pipes. One-shot commands (`process_run`) bypass the
/// registry entirely.
///
/// # Tool namespace
///
/// All tools are registered under the `"tokio_process"` namespace and named
/// `tokio_process__<verb>`.
pub struct TokioProcessPlugin(Arc<ProcessCtx>);

impl TokioProcessPlugin {
    /// Create a new `TokioProcessPlugin` with an empty child registry.
    pub fn new() -> Self {
        Self(Arc::new(ProcessCtx::new()))
    }
}

impl Default for TokioProcessPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioProcessPlugin {
    fn name(&self) -> &'static str {
        "tokio_process"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_process")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("tokio_process__") {
                name.to_string()
            } else {
                format!("tokio_process__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_process")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
