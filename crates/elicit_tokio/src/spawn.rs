//! `TokioSpawnPlugin` — runtime spawn factory for tokio tasks.
//!
//! Closures and futures cannot cross the MCP JSON boundary, so this plugin
//! uses a *factory pattern*: callers implement one of two traits
//! ([`BlockingWorkload`] or [`AsyncWorkload`]) whose inputs and outputs are
//! fully serializable, then register the type with the plugin builder.  The
//! plugin dynamically generates one MCP tool per registered workload type.
//!
//! When a workload tool is called:
//! 1. The JSON params are deserialized into `T`.
//! 2. `T::execute` is spawned via [`tokio::task::spawn_blocking`] (blocking)
//!    or [`tokio::spawn`] (async).
//! 3. The output is serialized to `serde_json::Value` and the
//!    [`JoinHandle`](tokio::task::JoinHandle) is stored server-side, keyed by
//!    UUID.
//! 4. The tool returns `{ handle_id }`.
//!
//! Companion tools then let agents await or cancel the handle:
//!
//! # Tool namespace: `tokio_spawn__*`
//!
//! ## Companion tools (always present)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `join` | `handle_id` | `{ output }` — awaits completion |
//! | `try_join` | `handle_id` | `{ output?, ready }` — non-blocking poll |
//! | `abort` | `handle_id` | `{ ok }` — cancel the task |
//!
//! ## Dynamic workload tools (one per registered type)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `tokio_spawn__<name>` | `T`'s fields | `{ handle_id }` |
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_tokio::{BlockingWorkload, TokioSpawnPlugin};
//! use schemars::JsonSchema;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Deserialize, JsonSchema)]
//! struct CompressParams { data: Vec<u8>, level: u32 }
//!
//! #[derive(Serialize)]
//! struct CompressOutput { compressed: Vec<u8> }
//!
//! impl BlockingWorkload for CompressParams {
//!     type Output = CompressOutput;
//!     fn execute(self) -> CompressOutput {
//!         // ... compression logic ...
//!         CompressOutput { compressed: vec![] }
//!     }
//! }
//!
//! let plugin = TokioSpawnPlugin::builder()
//!     .register_blocking::<CompressParams>("compress", "Compress data in a blocking thread")
//!     .build();
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::Elicit;
use elicitation::contracts::{Established, Prop};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uuid::Uuid;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: a task was successfully spawned and a `JoinHandle` is registered.
#[derive(Elicit)]
pub struct TaskSpawned {}
impl Prop for TaskSpawned {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_task_spawned_axiom() {
                let spawn_ok = true;
                assert!(spawn_ok, "tokio::spawn axiom: JoinHandle returned => task accepted by scheduler");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_task_spawned(spawn_returned_handle: bool) -> (result: bool)
                ensures result == spawn_returned_handle,
            {
                spawn_returned_handle
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_task_spawned_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a spawned task completed and its output was retrieved.
#[derive(Elicit)]
pub struct TaskJoined {}
impl Prop for TaskJoined {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_task_joined_axiom() {
                let join_ok: bool = kani::any();
                kani::assume(join_ok);
                assert!(join_ok, "JoinHandle::await axiom: Ok => task completed without panic");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_task_joined(join_returned_ok: bool) -> (result: bool)
                ensures result == join_returned_ok,
            {
                join_returned_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_task_joined_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a spawned task was cancelled via `JoinHandle::abort()`.
#[derive(Elicit)]
pub struct TaskAborted {}
impl Prop for TaskAborted {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_task_aborted_axiom() {
                let abort_scheduled = true;
                assert!(abort_scheduled, "JoinHandle::abort axiom: schedules cancellation (infallible)");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_task_aborted(abort_scheduled: bool) -> (result: bool)
                ensures result == abort_scheduled,
            {
                abort_scheduled
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_task_aborted_contract() -> bool {
                true
            }
        }
    }
}

// ── Workload traits ───────────────────────────────────────────────────────────

/// A unit of blocking work that can be spawned via [`tokio::task::spawn_blocking`].
///
/// Implement this on a struct whose fields carry all the input the task needs.
/// The struct's [`JsonSchema`] + [`Deserialize`] define the MCP tool schema;
/// the [`Output`](BlockingWorkload::Output) type defines what is stored in the
/// handle registry (serialized to [`serde_json::Value`]).
pub trait BlockingWorkload: DeserializeOwned + JsonSchema + Send + 'static {
    /// The type produced when the task completes. Must be serializable.
    type Output: Serialize + Send + 'static;
    /// Execute the blocking work, consuming `self`.
    fn execute(self) -> Self::Output;
}

/// A unit of async work that can be spawned via [`tokio::spawn`].
///
/// Return a boxed future from [`execute`](AsyncWorkload::execute) so the trait
/// is object-safe from the plugin's perspective.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_tokio::AsyncWorkload;
/// use futures::future::BoxFuture;
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Deserialize, JsonSchema)]
/// struct FetchParams { url: String }
///
/// #[derive(Serialize)]
/// struct FetchOutput { body: String }
///
/// impl AsyncWorkload for FetchParams {
///     type Output = FetchOutput;
///     fn execute(self) -> BoxFuture<'static, FetchOutput> {
///         Box::pin(async move {
///             // ... async fetch logic ...
///             FetchOutput { body: String::new() }
///         })
///     }
/// }
/// ```
pub trait AsyncWorkload: DeserializeOwned + JsonSchema + Send + 'static {
    /// The type produced when the future resolves. Must be serializable.
    type Output: Serialize + Send + 'static;
    /// Start the async work, consuming `self`, and return a boxed future.
    fn execute(self) -> BoxFuture<'static, Self::Output>;
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for `tokio_spawn__*` companion tools.
pub struct SpawnCtx {
    handles: Mutex<HashMap<Uuid, JoinHandle<serde_json::Value>>>,
}

impl SpawnCtx {
    fn new() -> Self {
        Self {
            handles: Mutex::new(HashMap::new()),
        }
    }

    async fn insert(&self, handle: JoinHandle<serde_json::Value>) -> Uuid {
        let id = Uuid::new_v4();
        self.handles.lock().await.insert(id, handle);
        id
    }
}

impl elicitation::PluginContext for SpawnCtx {}

// ── Companion param/result types ──────────────────────────────────────────────

/// Parameters for `tokio_spawn__join` — awaits task completion.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct JoinParams {
    /// Handle UUID returned by a workload tool.
    pub handle_id: Uuid,
}

/// Parameters for `tokio_spawn__try_join` — non-blocking poll.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TryJoinParams {
    /// Handle UUID returned by a workload tool.
    pub handle_id: Uuid,
}

/// Parameters for `tokio_spawn__abort` — cancels the task.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AbortParams {
    /// Handle UUID returned by a workload tool.
    pub handle_id: Uuid,
}

#[derive(Serialize)]
struct HandleResult {
    handle_id: Uuid,
}

#[derive(Serialize)]
struct JoinResult {
    output: serde_json::Value,
}

#[derive(Serialize)]
struct TryJoinResult {
    output: Option<serde_json::Value>,
    ready: bool,
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

fn err_not_found(id: Uuid) -> ErrorData {
    ErrorData::invalid_params(format!("handle not found: {id}"), None)
}

// ── Companion tool descriptors ────────────────────────────────────────────────

fn join_descriptor() -> elicitation::ToolDescriptor {
    elicitation::make_descriptor_ctx::<SpawnCtx, JoinParams, _>(
        "tokio_spawn__join",
        "Await a spawned task to completion and return its output. Blocks until the task finishes.",
        |ctx, p| {
            Box::pin(async move {
                let handle = ctx
                    .handles
                    .lock()
                    .await
                    .remove(&p.handle_id)
                    .ok_or_else(|| err_not_found(p.handle_id))?;
                let output = handle
                    .await
                    .map_err(|e| ErrorData::internal_error(format!("task panicked: {e}"), None))?;
                let _proof: Established<TaskJoined> = Established::assert();
                Ok(json_result(&JoinResult { output }))
            })
        },
    )
}

fn try_join_descriptor() -> elicitation::ToolDescriptor {
    elicitation::make_descriptor_ctx::<SpawnCtx, TryJoinParams, _>(
        "tokio_spawn__try_join",
        "Poll a spawned task without blocking. Returns `{ ready: true, output }` if the task has \
         finished, or `{ ready: false }` if it is still running. The handle remains valid after \
         a `false` response so you can poll again or call `join` later.",
        |ctx, p| {
            Box::pin(async move {
                let mut handles = ctx.handles.lock().await;
                let finished = handles
                    .get(&p.handle_id)
                    .ok_or_else(|| err_not_found(p.handle_id))?
                    .is_finished();
                if finished {
                    let handle = handles.remove(&p.handle_id).unwrap();
                    drop(handles);
                    let output = handle.await.map_err(|e| {
                        ErrorData::internal_error(format!("task panicked: {e}"), None)
                    })?;
                    Ok(json_result(&TryJoinResult {
                        output: Some(output),
                        ready: true,
                    }))
                } else {
                    Ok(json_result(&TryJoinResult {
                        output: None,
                        ready: false,
                    }))
                }
            })
        },
    )
}

fn abort_descriptor() -> elicitation::ToolDescriptor {
    elicitation::make_descriptor_ctx::<SpawnCtx, AbortParams, _>(
        "tokio_spawn__abort",
        "Cancel a spawned task. The task is aborted at the next await point. The handle is \
         removed from the registry; calling `join` or `try_join` afterwards returns an error.",
        |ctx, p| {
            Box::pin(async move {
                let handle = ctx
                    .handles
                    .lock()
                    .await
                    .remove(&p.handle_id)
                    .ok_or_else(|| err_not_found(p.handle_id))?;
                handle.abort();
                let _proof: Established<TaskAborted> = Established::assert();
                Ok(json_result(&OkResult { ok: true }))
            })
        },
    )
}

// ── Dynamic workload descriptor builders ─────────────────────────────────────

/// Build a `ToolDescriptor` for a [`BlockingWorkload`] type.
fn blocking_descriptor<T: BlockingWorkload>(
    name: String,
    description: String,
    ctx: Arc<SpawnCtx>,
) -> (Tool, DynHandler) {
    let schema_value = serde_json::to_value(schemars::schema_for!(T))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    let tool = Tool::new(name.clone(), description, schema_obj);
    let handler: DynHandler = Arc::new(move |params: CallToolRequestParams| {
        let ctx = Arc::clone(&ctx);
        let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
        Box::pin(async move {
            let workload: T = serde_json::from_value(value)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
            let handle: JoinHandle<serde_json::Value> = tokio::task::spawn_blocking(move || {
                let output = workload.execute();
                serde_json::to_value(output).unwrap_or(serde_json::Value::Null)
            });
            let handle_id = ctx.insert(handle).await;
            let _proof: Established<TaskSpawned> = Established::assert();
            Ok(json_result(&HandleResult { handle_id }))
        })
    });
    (tool, handler)
}

/// Build a `(Tool, DynHandler)` pair for an [`AsyncWorkload`] type.
fn async_descriptor<T: AsyncWorkload>(
    name: String,
    description: String,
    ctx: Arc<SpawnCtx>,
) -> (Tool, DynHandler) {
    let schema_value = serde_json::to_value(schemars::schema_for!(T))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    let tool = Tool::new(name.clone(), description, schema_obj);
    let handler: DynHandler = Arc::new(move |params: CallToolRequestParams| {
        let ctx = Arc::clone(&ctx);
        let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
        Box::pin(async move {
            let workload: T = serde_json::from_value(value)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
            let future = workload.execute();
            let handle: JoinHandle<serde_json::Value> = tokio::spawn(async move {
                let output = future.await;
                serde_json::to_value(output).unwrap_or(serde_json::Value::Null)
            });
            let handle_id = ctx.insert(handle).await;
            let _proof: Established<TaskSpawned> = Established::assert();
            Ok(json_result(&HandleResult { handle_id }))
        })
    });
    (tool, handler)
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

type DynHandler = Arc<
    dyn Fn(CallToolRequestParams) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
        + Send
        + Sync,
>;

struct DynamicEntry {
    tool: Tool,
    handler: DynHandler,
}

/// MCP plugin for runtime task spawning via the workload factory pattern.
///
/// Construct with [`TokioSpawnPlugin::builder()`].
pub struct TokioSpawnPlugin {
    ctx: Arc<SpawnCtx>,
    /// Static companion descriptors (join, try_join, abort).
    companions: Vec<elicitation::ToolDescriptor>,
    /// Dynamically registered workload tools.
    dynamic: Vec<DynamicEntry>,
}

impl std::fmt::Debug for TokioSpawnPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokioSpawnPlugin")
            .field("dynamic_tool_count", &self.dynamic.len())
            .finish()
    }
}

impl TokioSpawnPlugin {
    /// Begin building a `TokioSpawnPlugin`.
    pub fn builder() -> SpawnPluginBuilder {
        SpawnPluginBuilder::new()
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Builder for [`TokioSpawnPlugin`].
///
/// Call [`register_blocking`](SpawnPluginBuilder::register_blocking) or
/// [`register_async`](SpawnPluginBuilder::register_async) for each workload
/// type, then call [`build`](SpawnPluginBuilder::build).
pub struct SpawnPluginBuilder {
    ctx: Arc<SpawnCtx>,
    dynamic: Vec<DynamicEntry>,
}

impl SpawnPluginBuilder {
    fn new() -> Self {
        Self {
            ctx: Arc::new(SpawnCtx::new()),
            dynamic: Vec::new(),
        }
    }

    /// Register a [`BlockingWorkload`] type as a named MCP tool.
    ///
    /// The tool will be named `tokio_spawn__<name>` and its input schema is
    /// derived from `T`'s [`JsonSchema`] impl.
    pub fn register_blocking<T: BlockingWorkload>(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let name = format!("tokio_spawn__{}", name.into());
        let (tool, handler) =
            blocking_descriptor::<T>(name, description.into(), Arc::clone(&self.ctx));
        self.dynamic.push(DynamicEntry { tool, handler });
        self
    }

    /// Register an [`AsyncWorkload`] type as a named MCP tool.
    ///
    /// The tool will be named `tokio_spawn__<name>` and its input schema is
    /// derived from `T`'s [`JsonSchema`] impl.
    pub fn register_async<T: AsyncWorkload>(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        let name = format!("tokio_spawn__{}", name.into());
        let (tool, handler) =
            async_descriptor::<T>(name, description.into(), Arc::clone(&self.ctx));
        self.dynamic.push(DynamicEntry { tool, handler });
        self
    }

    /// Finalize the plugin, returning a [`TokioSpawnPlugin`] ready for
    /// registration with an [`ElicitServer`](elicitation::ElicitServer).
    pub fn build(self) -> TokioSpawnPlugin {
        let ctx = Arc::clone(&self.ctx);
        TokioSpawnPlugin {
            companions: vec![join_descriptor(), try_join_descriptor(), abort_descriptor()],
            ctx,
            dynamic: self.dynamic,
        }
    }
}

// ── ElicitPlugin impl ─────────────────────────────────────────────────────────

impl elicitation::ElicitPlugin for TokioSpawnPlugin {
    fn name(&self) -> &'static str {
        "tokio_spawn"
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools: Vec<Tool> = self.companions.iter().map(|d| d.as_tool()).collect();
        tools.extend(self.dynamic.iter().map(|e| e.tool.clone()));
        tools
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = Arc::clone(&self.ctx);
        let name = params.name.to_string();

        // Check companion tools first (join, try_join, abort).
        if let Some(descriptor) = self
            .companions
            .iter()
            .find(|d| d.name == name || format!("tokio_spawn__{}", d.name) == name)
        {
            let descriptor = descriptor.clone();
            return Box::pin(async move {
                descriptor
                    .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                    .await
            });
        }

        // Check dynamic workload tools.
        if let Some(entry) = self.dynamic.iter().find(|e| e.tool.name.as_ref() == name) {
            let handler = Arc::clone(&entry.handler);
            return Box::pin(async move { handler(params).await });
        }

        Box::pin(async move {
            Err(ErrorData::invalid_params(
                format!("unknown tokio_spawn tool: {name}"),
                None,
            ))
        })
    }
}
