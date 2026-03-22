//! `TokioRuntimePlugin` — MCP tools for inspecting the active tokio runtime
//! and emitting runtime construction code.
//!
//! Uses [`tokio::runtime::Handle::current`] to introspect the runtime the MCP
//! server is already running on. No futures or runtime objects cross the MCP
//! boundary — results are serializable values.
//!
//! `build_current_thread`, `build_multi_thread`, and `block_on` are
//! **emit-only** tools: they cannot execute at runtime (you cannot build a new
//! tokio runtime inside an existing one), but they participate in `emit_binary`
//! composition to generate valid runtime setup code at the top of `main`.
//!
//! # Tool namespace: `tokio_runtime__*`
//!
//! | Tool | Params | Returns | Notes |
//! |---|---|---|---|
//! | `inspect_flavor` | — | `{ flavor }` | |
//! | `build_current_thread` | `enable_all, max_blocking_threads?` | error at runtime | emit-only |
//! | `build_multi_thread` | `worker_threads?, enable_all, max_blocking_threads?` | error at runtime | emit-only |
//! | `block_on` | `runtime_var, body` | error at runtime | emit-only |

use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Runtime flavor mirror ─────────────────────────────────────────────────────

/// Serializable mirror of [`tokio::runtime::RuntimeFlavor`].
///
/// Returned by `tokio_runtime__inspect_flavor` to describe the threading
/// model of the active tokio runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFlavorKind {
    /// Single-threaded scheduler (`Builder::new_current_thread`).
    CurrentThread,
    /// Multi-threaded work-stealing scheduler (`Builder::new_multi_thread`).
    MultiThread,
    /// Unknown or future variant.
    Other,
}

impl From<tokio::runtime::RuntimeFlavor> for RuntimeFlavorKind {
    fn from(f: tokio::runtime::RuntimeFlavor) -> Self {
        match f {
            tokio::runtime::RuntimeFlavor::CurrentThread => RuntimeFlavorKind::CurrentThread,
            tokio::runtime::RuntimeFlavor::MultiThread => RuntimeFlavorKind::MultiThread,
            _ => RuntimeFlavorKind::Other,
        }
    }
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `tokio_runtime__inspect_flavor`.
///
/// No configuration needed — inspects the current runtime context.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InspectFlavorParams {}

fn default_true() -> bool {
    true
}

fn default_runtime_var() -> String {
    "runtime".to_string()
}

/// Parameters for `tokio_runtime__build_current_thread`.
///
/// This tool is **emit-only**: use it inside `emit_binary` to generate
/// `tokio::runtime::Builder::new_current_thread()…build().unwrap()` at the
/// start of `main`. Calling it directly returns an error.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BuildCurrentThreadParams {
    /// Enable all I/O and timer drivers. Equivalent to calling `.enable_all()`.
    /// Defaults to `true`.
    #[serde(default = "default_true")]
    pub enable_all: bool,
    /// Maximum number of blocking threads. `null` uses tokio's default (512).
    pub max_blocking_threads: Option<usize>,
    /// Variable name to bind the runtime to (default: `"runtime"`).
    #[serde(default = "default_runtime_var")]
    pub runtime_var: String,
}

/// Parameters for `tokio_runtime__build_multi_thread`.
///
/// This tool is **emit-only**: use it inside `emit_binary` to generate
/// `tokio::runtime::Builder::new_multi_thread()…build().unwrap()` at the
/// start of `main`. Calling it directly returns an error.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BuildMultiThreadParams {
    /// Number of worker threads. `null` uses the number of logical CPU cores.
    pub worker_threads: Option<usize>,
    /// Enable all I/O and timer drivers. Defaults to `true`.
    #[serde(default = "default_true")]
    pub enable_all: bool,
    /// Maximum number of blocking threads. `null` uses tokio's default (512).
    pub max_blocking_threads: Option<usize>,
    /// Variable name to bind the runtime to (default: `"runtime"`).
    #[serde(default = "default_runtime_var")]
    pub runtime_var: String,
}

/// Parameters for `tokio_runtime__block_on`.
///
/// This tool is **emit-only**: use it inside `emit_binary` to generate
/// `<runtime_var>.block_on(async { <body> })`. Calling it directly returns an
/// error.
///
/// Pair with `build_current_thread` or `build_multi_thread` to form a
/// complete synchronous entry point around an async workload.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockOnParams {
    /// The async body to run (Rust expression or statement sequence).
    pub body: String,
    /// Name of the runtime variable created by a prior `build_*` step
    /// (default: `"runtime"`).
    #[serde(default = "default_runtime_var")]
    pub runtime_var: String,
}

// ── Emit impls ────────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
mod emit_impls {
    use super::{BlockOnParams, BuildCurrentThreadParams, BuildMultiThreadParams};
    use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
    use elicitation::proc_macro2::TokenStream;

    fn parse_body(body: &str) -> TokenStream {
        body.parse().unwrap_or_else(|_| {
            let lit = body;
            ::quote::quote! { compile_error!(#lit) }
        })
    }

    fn tokio_dep() -> CrateDep {
        CrateDep::new("tokio", "1")
    }

    impl EmitCode for BuildCurrentThreadParams {
        fn emit_code(&self) -> TokenStream {
            let var: TokenStream = self
                .runtime_var
                .parse()
                .unwrap_or_else(|_| ::quote::quote! { runtime });
            let mut chain = ::quote::quote! {
                tokio::runtime::Builder::new_current_thread()
            };
            if self.enable_all {
                chain = ::quote::quote! { #chain.enable_all() };
            }
            if let Some(n) = self.max_blocking_threads {
                chain = ::quote::quote! { #chain.max_blocking_threads(#n) };
            }
            ::quote::quote! {
                let #var = #chain.build().unwrap();
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![tokio_dep()]
        }
    }

    impl EmitCode for BuildMultiThreadParams {
        fn emit_code(&self) -> TokenStream {
            let var: TokenStream = self
                .runtime_var
                .parse()
                .unwrap_or_else(|_| ::quote::quote! { runtime });
            let mut chain = ::quote::quote! {
                tokio::runtime::Builder::new_multi_thread()
            };
            if let Some(n) = self.worker_threads {
                chain = ::quote::quote! { #chain.worker_threads(#n) };
            }
            if self.enable_all {
                chain = ::quote::quote! { #chain.enable_all() };
            }
            if let Some(n) = self.max_blocking_threads {
                chain = ::quote::quote! { #chain.max_blocking_threads(#n) };
            }
            ::quote::quote! {
                let #var = #chain.build().unwrap();
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![tokio_dep()]
        }
    }

    impl EmitCode for BlockOnParams {
        fn emit_code(&self) -> TokenStream {
            let var: TokenStream = self
                .runtime_var
                .parse()
                .unwrap_or_else(|_| ::quote::quote! { runtime });
            let body = parse_body(&self.body);
            ::quote::quote! {
                #var.block_on(async { #body })
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![tokio_dep()]
        }
    }

    inventory::submit! { EmitEntry {
        tool: "tokio_runtime__build_current_thread",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<BuildCurrentThreadParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}

    inventory::submit! { EmitEntry {
        tool: "tokio_runtime__build_multi_thread",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<BuildMultiThreadParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}

    inventory::submit! { EmitEntry {
        tool: "tokio_runtime__block_on",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<BlockOnParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn emit_only_error(tool: &str) -> ErrorData {
    ErrorData::invalid_params(
        format!(
            "{tool} is emit-only: use it as a step in emit_binary to generate runtime \
             construction code in a Rust binary"
        ),
        None,
    )
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_runtime__*` tools.
///
/// `build_current_thread`, `build_multi_thread`, and `block_on` are
/// **emit-only** — they return an error when called directly but participate
/// in `emit_binary` composition to generate tokio runtime setup code.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tokio_runtime")]
pub struct TokioRuntimePlugin;

#[elicitation::elicit_tool(
    plugin = "tokio_runtime",
    name = "tokio_runtime__inspect_flavor",
    description = "Inspect the threading model of the active tokio runtime. \
                   Returns 'current_thread' (single-threaded scheduler) or \
                   'multi_thread' (work-stealing multi-threaded scheduler). \
                   Useful for understanding the execution context of the MCP server.",
    emit = Auto
)]
async fn runtime_inspect_flavor(p: InspectFlavorParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    let flavor = RuntimeFlavorKind::from(tokio::runtime::Handle::current().runtime_flavor());
    let result = serde_json::to_string(&InspectFlavorResult { flavor })
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        result,
    )]))
}

#[derive(Serialize)]
struct InspectFlavorResult {
    flavor: RuntimeFlavorKind,
}

#[elicitation::elicit_tool(
    plugin = "tokio_runtime",
    name = "tokio_runtime__build_current_thread",
    description = "EMIT-ONLY: generates `tokio::runtime::Builder::new_current_thread()…\
                   build().unwrap()` in an emitted Rust binary. Cannot execute inside an \
                   existing tokio runtime. Use via `emit_binary` to set up a single-threaded \
                   tokio runtime at the start of `main`."
)]
async fn runtime_build_current_thread(
    _p: BuildCurrentThreadParams,
) -> Result<CallToolResult, ErrorData> {
    Err(emit_only_error("tokio_runtime__build_current_thread"))
}

#[elicitation::elicit_tool(
    plugin = "tokio_runtime",
    name = "tokio_runtime__build_multi_thread",
    description = "EMIT-ONLY: generates `tokio::runtime::Builder::new_multi_thread()…\
                   build().unwrap()` in an emitted Rust binary. Cannot execute inside an \
                   existing tokio runtime. Use via `emit_binary` to set up a multi-threaded \
                   work-stealing tokio runtime at the start of `main`."
)]
async fn runtime_build_multi_thread(
    _p: BuildMultiThreadParams,
) -> Result<CallToolResult, ErrorData> {
    Err(emit_only_error("tokio_runtime__build_multi_thread"))
}

#[elicitation::elicit_tool(
    plugin = "tokio_runtime",
    name = "tokio_runtime__block_on",
    description = "EMIT-ONLY: generates `<runtime_var>.block_on(async { <body> })` in an \
                   emitted Rust binary. Cannot execute inside an existing tokio runtime. \
                   Use via `emit_binary` after `build_current_thread` or `build_multi_thread` \
                   to drive an async workload from synchronous `main`."
)]
async fn runtime_block_on(_p: BlockOnParams) -> Result<CallToolResult, ErrorData> {
    Err(emit_only_error("tokio_runtime__block_on"))
}
