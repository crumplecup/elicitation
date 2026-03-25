//! `TokioTaskPlugin` — MCP tools for tokio task utilities.
//!
//! Spawn / closure APIs cannot execute directly over MCP (closures and futures
//! can't cross the JSON boundary). `spawn`, `spawn_blocking`, and
//! `block_in_place` are therefore **emit-only** tools: calling them at runtime
//! returns a clear error; using them inside `emit_binary` generates real
//! `tokio::spawn(...)` code in the emitted binary.
//!
//! `tokio::task::id()` requires `tokio_unstable`; omitted for stable builds.
//!
//! # Tool namespace: `tokio_task__*`
//!
//! | Tool | Params | Returns | Notes |
//! |---|---|---|---|
//! | `yield_now` | — | `{ ok }` | |
//! | `spawn` | `body` | error at runtime | emit-only |
//! | `spawn_blocking` | `body` | error at runtime | emit-only |
//! | `block_in_place` | `body` | error at runtime | emit-only |

use elicitation::Elicit;
use elicitation::contracts::{Established, Prop};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Param / result types ──────────────────────────────────────────────────────

/// Proposition: `tokio::task::yield_now()` returned — the task yielded to the scheduler.
#[derive(Elicit)]
pub struct TaskYielded {}
impl Prop for TaskYielded {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_task_yielded_axiom() {
                let yielded = true;
                assert!(yielded, "tokio::task::yield_now axiom: always returns after yielding");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_task_yielded(call_completed: bool) -> (result: bool)
                ensures result == call_completed,
            {
                call_completed
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_task_yielded_contract() -> bool {
                true
            }
        }
    }
}

/// Parameters for `tokio_task__yield_now` (none required).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct YieldNowParams {}

/// Parameters for `tokio_task__spawn`.
///
/// This tool is **emit-only**: it cannot execute at runtime. Use it inside
/// `emit_binary` to generate `tokio::spawn(async { … })` in a Rust binary.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnParams {
    /// The body of the async block to spawn, as a Rust expression or statement
    /// sequence (e.g. `"my_future.await"` or `"println!(\"hello\"); sleep(Duration::from_secs(1)).await"`).
    pub body: String,
}

/// Parameters for `tokio_task__spawn_blocking`.
///
/// This tool is **emit-only**: it cannot execute at runtime. Use it inside
/// `emit_binary` to generate `tokio::task::spawn_blocking(|| { … })`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnBlockingParams {
    /// The body of the blocking closure (e.g. `"std::fs::read_to_string(\"/etc/hosts\").unwrap()"`).
    pub body: String,
}

/// Parameters for `tokio_task__block_in_place`.
///
/// This tool is **emit-only**: it cannot execute at runtime. Use it inside
/// `emit_binary` to generate `tokio::task::block_in_place(|| { … })`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BlockInPlaceParams {
    /// The body of the blocking closure.
    pub body: String,
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    use rmcp::model::Content;
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

// ── Emit impls ────────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
mod emit_impls {
    use super::{BlockInPlaceParams, SpawnBlockingParams, SpawnParams};
    use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
    use elicitation::proc_macro2::TokenStream;

    fn parse_body(body: &str) -> TokenStream {
        body.parse().unwrap_or_else(|_| {
            let lit = body;
            ::quote::quote! { compile_error!(#lit) }
        })
    }

    impl EmitCode for SpawnParams {
        fn emit_code(&self) -> TokenStream {
            let body = parse_body(&self.body);
            ::quote::quote! {
                tokio::spawn(async move { #body })
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![CrateDep::new("tokio", "1")]
        }
    }

    impl EmitCode for SpawnBlockingParams {
        fn emit_code(&self) -> TokenStream {
            let body = parse_body(&self.body);
            ::quote::quote! {
                tokio::task::spawn_blocking(move || { #body })
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![CrateDep::new("tokio", "1")]
        }
    }

    impl EmitCode for BlockInPlaceParams {
        fn emit_code(&self) -> TokenStream {
            let body = parse_body(&self.body);
            ::quote::quote! {
                tokio::task::block_in_place(move || { #body })
            }
        }
        fn crate_deps(&self) -> Vec<CrateDep> {
            vec![CrateDep::new("tokio", "1")]
        }
    }

    elicitation::inventory::submit! { EmitEntry {
        tool: "tokio_task__spawn",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<SpawnParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}

    elicitation::inventory::submit! { EmitEntry {
        tool: "tokio_task__spawn_blocking",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<SpawnBlockingParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}

    elicitation::inventory::submit! { EmitEntry {
        tool: "tokio_task__block_in_place",
        crate_name: "elicit_tokio",
        constructor: |v| serde_json::from_value::<BlockInPlaceParams>(v)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| e.to_string()),
    }}
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_task",
    name = "tokio_task__yield_now",
    description = "Yield execution back to the tokio scheduler, allowing other tasks to run \
                   before this one continues. Useful in tight loops to avoid starving the \
                   runtime.",
    emit = Auto
)]
async fn task_yield_now(_p: YieldNowParams) -> Result<CallToolResult, ErrorData> {
    tokio::task::yield_now().await;
    let _proof: Established<TaskYielded> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_task",
    name = "tokio_task__spawn",
    description = "EMIT-ONLY: generates `tokio::spawn(async move { <body> })` in an emitted \
                   Rust binary. Cannot execute directly over MCP — calling this tool at runtime \
                   returns an error. Use via `emit_binary` to compose asynchronous task \
                   spawning in generated code.",
    emit = None
)]
async fn task_spawn(_p: SpawnParams) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::invalid_params(
        "tokio_task__spawn is emit-only: use it as a step in emit_binary to generate \
         tokio::spawn(async move { … }) in a Rust binary",
        None,
    ))
}

#[elicitation::elicit_tool(
    plugin = "tokio_task",
    name = "tokio_task__spawn_blocking",
    description = "EMIT-ONLY: generates `tokio::task::spawn_blocking(move || { <body> })` in \
                   an emitted Rust binary. Cannot execute directly over MCP. Use via \
                   `emit_binary` to offload blocking work to a dedicated thread pool in \
                   generated code.",
    emit = None
)]
async fn task_spawn_blocking(_p: SpawnBlockingParams) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::invalid_params(
        "tokio_task__spawn_blocking is emit-only: use it as a step in emit_binary to generate \
         tokio::task::spawn_blocking(move || { … }) in a Rust binary",
        None,
    ))
}

#[elicitation::elicit_tool(
    plugin = "tokio_task",
    name = "tokio_task__block_in_place",
    description = "EMIT-ONLY: generates `tokio::task::block_in_place(move || { <body> })` in \
                   an emitted Rust binary. Cannot execute directly over MCP. Use via \
                   `emit_binary` to run blocking code inside a multi-threaded tokio runtime \
                   without starving async tasks.",
    emit = None
)]
async fn task_block_in_place(_p: BlockInPlaceParams) -> Result<CallToolResult, ErrorData> {
    Err(ErrorData::invalid_params(
        "tokio_task__block_in_place is emit-only: use it as a step in emit_binary to generate \
         tokio::task::block_in_place(move || { … }) in a Rust binary",
        None,
    ))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_task__*` tools.
///
/// `spawn`, `spawn_blocking`, and `block_in_place` are **emit-only** — they
/// return an error when called directly but participate in `emit_binary`
/// composition to generate real tokio task code in Rust binaries.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tokio_task")]
pub struct TokioTaskPlugin;
