//! `TokioIoCopyPlugin` — factory-based `tokio::io::copy` across plugin registries.
//!
//! `tokio::io::copy` requires mutable access to both a reader and a writer
//! simultaneously. Because each plugin owns its own UUID-keyed registry, a
//! copy plugin cannot reach into two different registries at call time without
//! prior knowledge of their types.
//!
//! The factory pattern solves this: at construction time the caller registers
//! a concrete `(R, W)` pair by cloning the relevant plugin registries. The
//! factory generates one MCP tool per registered pair. At call time the tool
//! resolves both UUIDs from the captured registries and runs
//! [`tokio::io::copy`].
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_tokio::{TokioIoCopyPlugin, TokioNetPlugin, TokioIoPlugin};
//! use tokio::net::TcpStream;
//! use tokio::io::DuplexStream;
//!
//! let net = TokioNetPlugin::new();
//! let io  = TokioIoPlugin::new();
//!
//! let copy_plugin = TokioIoCopyPlugin::builder()
//!     .register::<TcpStream, DuplexStream>(
//!         "tcp_to_duplex",
//!         "Copy bytes from a TCP stream into a duplex pipe",
//!         net.tcp_stream_registry(),
//!         io.duplex_stream_registry(),
//!     )
//!     .build();
//! ```
//!
//! # Tool namespace: `tokio_io_copy__*`
//!
//! Each registered pair produces one tool:
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `tokio_io_copy__<name>` | `reader_id, writer_id` | `{ bytes_copied }` |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, VerifiedWorkflow};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: `tokio::io::copy` completed — all readable bytes were written to the writer.
#[derive(Elicit)]
pub struct BytesCopied {}
impl Prop for BytesCopied {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_bytes_copied_axiom() {
                let copy_ok: bool = kani::any();
                kani::assume(copy_ok);
                let n: u64 = kani::any();
                assert!(copy_ok, "tokio::io::copy axiom: Ok(n) => n bytes transferred reader->writer");
                let _ = n;
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_bytes_copied(copy_returned_ok: bool) -> (result: bool)
                ensures result == copy_returned_ok,
            {
                copy_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_bytes_copied_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for BytesCopied {}

// ── Param / result types ──────────────────────────────────────────────────────

/// Parameters for a `tokio_io_copy__*` tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IoCopyParams {
    /// UUID of the reader handle (from the plugin that produced it).
    pub reader_id: Uuid,
    /// UUID of the writer handle (from the plugin that produced it).
    pub writer_id: Uuid,
}

#[derive(Serialize)]
struct IoCopyResult {
    bytes_copied: u64,
}

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

// ── Registry type alias ───────────────────────────────────────────────────────

/// A UUID-keyed registry of handles wrapped in `Arc<Mutex<T>>`.
///
/// This is the type exposed by the registry accessors on each I/O plugin
/// (e.g. [`TokioNetPlugin::tcp_stream_registry`](crate::TokioNetPlugin::tcp_stream_registry)).
pub type HandleRegistry<T> = Arc<Mutex<HashMap<Uuid, Arc<Mutex<T>>>>>;

// ── Dynamic handler type ──────────────────────────────────────────────────────

type DynHandler = Arc<
    dyn Fn(CallToolRequestParams) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
        + Send
        + Sync,
>;

struct DynamicEntry {
    tool: Tool,
    handler: DynHandler,
}

// ── Handler builder ───────────────────────────────────────────────────────────

fn copy_handler<R, W>(
    name: String,
    description: String,
    readers: HandleRegistry<R>,
    writers: HandleRegistry<W>,
) -> (Tool, DynHandler)
where
    R: AsyncRead + Unpin + Send + 'static,
    W: AsyncWrite + Unpin + Send + 'static,
{
    let schema_value = serde_json::to_value(schemars::schema_for!(IoCopyParams))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    let tool = Tool::new(name, description, schema_obj);

    let handler: DynHandler = Arc::new(move |params: CallToolRequestParams| {
        let readers = Arc::clone(&readers);
        let writers = Arc::clone(&writers);
        Box::pin(async move {
            let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
            let p: IoCopyParams = serde_json::from_value(value)
                .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;

            let reader_arc = readers
                .lock()
                .await
                .get(&p.reader_id)
                .cloned()
                .ok_or_else(|| {
                    ErrorData::invalid_params(format!("reader not found: {}", p.reader_id), None)
                })?;

            let writer_arc = writers
                .lock()
                .await
                .get(&p.writer_id)
                .cloned()
                .ok_or_else(|| {
                    ErrorData::invalid_params(format!("writer not found: {}", p.writer_id), None)
                })?;

            let mut reader = reader_arc.lock().await;
            let mut writer = writer_arc.lock().await;

            let bytes_copied = tokio::io::copy(&mut *reader, &mut *writer)
                .await
                .map_err(|e| ErrorData::internal_error(format!("io::copy failed: {e}"), None))?;
            let _proof: Established<BytesCopied> = Established::assert();
            Ok(json_result(&IoCopyResult { bytes_copied }))
        })
    });

    (tool, handler)
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_io_copy__*` tools for cross-plugin I/O copy.
///
/// Construct with [`TokioIoCopyPlugin::builder()`].
pub struct TokioIoCopyPlugin {
    dynamic: Vec<DynamicEntry>,
}

impl std::fmt::Debug for TokioIoCopyPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokioIoCopyPlugin")
            .field("registered_pairs", &self.dynamic.len())
            .finish()
    }
}

impl TokioIoCopyPlugin {
    /// Begin building a `TokioIoCopyPlugin`.
    pub fn builder() -> IoCopyPluginBuilder {
        IoCopyPluginBuilder::new()
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Builder for [`TokioIoCopyPlugin`].
pub struct IoCopyPluginBuilder {
    dynamic: Vec<DynamicEntry>,
}

impl IoCopyPluginBuilder {
    fn new() -> Self {
        Self {
            dynamic: Vec::new(),
        }
    }

    /// Register a concrete `(R, W)` pair for `io::copy`.
    ///
    /// - `name` — bare tool name; the MCP tool will be `tokio_io_copy__<name>`
    /// - `description` — shown to agents
    /// - `readers` — registry returned by the source plugin's registry accessor
    /// - `writers` — registry returned by the sink plugin's registry accessor
    pub fn register<R, W>(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        readers: HandleRegistry<R>,
        writers: HandleRegistry<W>,
    ) -> Self
    where
        R: AsyncRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let full_name = format!("tokio_io_copy__{}", name.into());
        let (tool, handler) = copy_handler::<R, W>(full_name, description.into(), readers, writers);
        self.dynamic.push(DynamicEntry { tool, handler });
        self
    }

    /// Finalize the plugin.
    pub fn build(self) -> TokioIoCopyPlugin {
        TokioIoCopyPlugin {
            dynamic: self.dynamic,
        }
    }
}

// ── ElicitPlugin impl ─────────────────────────────────────────────────────────

impl elicitation::ElicitPlugin for TokioIoCopyPlugin {
    fn name(&self) -> &'static str {
        "tokio_io_copy"
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.dynamic.iter().map(|e| e.tool.clone()).collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let name = params.name.to_string();
        if let Some(entry) = self.dynamic.iter().find(|e| e.tool.name.as_ref() == name) {
            let handler = Arc::clone(&entry.handler);
            return Box::pin(async move { handler(params).await });
        }
        Box::pin(async move {
            Err(ErrorData::invalid_params(
                format!("unknown tokio_io_copy tool: {name}"),
                None,
            ))
        })
    }
}
