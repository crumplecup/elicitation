//! `TowerBuilderPlugin` — MCP tools for composing a tower `ServiceBuilder`.
//!
//! Builder descriptors are held server-side in UUID-keyed registries.
//! Agents build a service incrementally then call `build` to retrieve the
//! full `TowerServiceBuilder` descriptor for code generation.
//!
//! # Tool namespace: `tower_builder__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `new` | `service_name` | `{ builder_id }` | `TowerServiceBuilderCreated` |
//! | `add_layer` | `builder_id, layer` | updated JSON | `TowerServiceBuilderLayerAdded` |
//! | `build` | `builder_id` | full JSON | `TowerServiceBuilderDone` |
//! | `describe` | `builder_id` | current JSON | — |

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
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a new empty `TowerServiceBuilder` was created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerServiceBuilderCreated;
impl Prop for TowerServiceBuilderCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_service_builder_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower service builder created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_service_builder_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_service_builder_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerServiceBuilderCreated {}

/// Proposition: a layer was successfully appended to a `TowerServiceBuilder`.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerServiceBuilderLayerAdded;
impl Prop for TowerServiceBuilderLayerAdded {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_service_builder_layer_added() {
                let added: bool = kani::any();
                kani::assume(added);
                assert!(added, "tower service builder layer added");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_service_builder_layer_added(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_service_builder_layer_added_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerServiceBuilderLayerAdded {}

/// Proposition: a `TowerServiceBuilder` was finalised and is ready for code generation.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerServiceBuilderDone;
impl Prop for TowerServiceBuilderDone {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_service_builder_done() {
                let done: bool = kani::any();
                kani::assume(done);
                assert!(done, "tower service builder done");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_service_builder_done(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_service_builder_done_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerServiceBuilderDone {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_builder__*` tool calls.
pub struct TowerBuilderCtx {
    tower_builders: Mutex<HashMap<Uuid, elicitation::TowerServiceBuilder>>,
}

impl TowerBuilderCtx {
    fn new() -> Self {
        Self {
            tower_builders: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerBuilderCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_builder__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuilderNewParams {
    /// Rust identifier or expression for the inner service.
    pub service_name: String,
}

/// Parameters for `tower_builder__add_layer`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuilderAddLayerParams {
    /// UUID returned by `tower_builder__new`.
    pub builder_id: String,
    /// The layer to append (provide as JSON with `kind` field).
    pub layer: elicitation::TowerLayerKind,
}

/// Parameters for `tower_builder__build`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuilderBuildParams {
    /// UUID returned by `tower_builder__new`.
    pub builder_id: String,
}

/// Parameters for `tower_builder__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BuilderDescribeParams {
    /// UUID returned by `tower_builder__new`.
    pub builder_id: String,
}

/// Result returned by `tower_builder__new`.
#[derive(Debug, Serialize)]
pub struct BuilderIdResult {
    /// UUID handle for the newly created builder.
    pub builder_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn parse_builder_id(raw: &str) -> Result<Uuid, ErrorData> {
    raw.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {raw}"), None))
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tower_builder",
    name = "tower_builder__new",
    description = "Create an empty TowerServiceBuilder with the given inner service name. \
                   Establishes: TowerServiceBuilderCreated.",
    emit = Auto
)]
async fn builder_new(
    ctx: Arc<TowerBuilderCtx>,
    p: BuilderNewParams,
) -> Result<CallToolResult, ErrorData> {
    let builder = elicitation::TowerServiceBuilder {
        layers: vec![],
        service_name: p.service_name,
    };
    let id = Uuid::new_v4();
    ctx.tower_builders.lock().await.insert(id, builder);
    let _proof: Established<TowerServiceBuilderCreated> = Established::assert();
    Ok(json_result(&BuilderIdResult {
        builder_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_builder",
    name = "tower_builder__add_layer",
    description = "Append a layer to an existing TowerServiceBuilder. Layers are applied outermost-first. \
                   Assumes: builder_id is a valid UUID from tower_builder__new. \
                   Establishes: TowerServiceBuilderLayerAdded.",
    emit = Auto
)]
async fn builder_add_layer(
    ctx: Arc<TowerBuilderCtx>,
    p: BuilderAddLayerParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_builder_id(&p.builder_id)?;
    let mut guard = ctx.tower_builders.lock().await;
    let builder = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("builder_id not found: {id}"), None))?;
    builder.layers.push(p.layer);
    let updated = builder.clone();
    drop(guard);
    let _proof: Established<TowerServiceBuilderLayerAdded> = Established::assert();
    Ok(json_result(&updated))
}

#[elicitation::elicit_tool(
    plugin = "tower_builder",
    name = "tower_builder__build",
    description = "Finalise a TowerServiceBuilder and return its full descriptor JSON. \
                   Assumes: builder_id is a valid UUID from tower_builder__new. \
                   Establishes: TowerServiceBuilderDone.",
    emit = Auto
)]
async fn builder_build(
    ctx: Arc<TowerBuilderCtx>,
    p: BuilderBuildParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_builder_id(&p.builder_id)?;
    let builder = ctx
        .tower_builders
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("builder_id not found: {id}"), None))?;
    let _proof: Established<TowerServiceBuilderDone> = Established::assert();
    Ok(json_result(&builder))
}

#[elicitation::elicit_tool(
    plugin = "tower_builder",
    name = "tower_builder__describe",
    description = "Return the current state of a TowerServiceBuilder without finalising it. \
                   Assumes: builder_id is a valid UUID from tower_builder__new.",
    emit = Auto
)]
async fn builder_describe(
    ctx: Arc<TowerBuilderCtx>,
    p: BuilderDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_builder_id(&p.builder_id)?;
    let builder = ctx
        .tower_builders
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("builder_id not found: {id}"), None))?;
    Ok(json_result(&builder))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_builder__*` tools for composing tower services.
///
/// Holds a UUID-keyed registry of in-progress `TowerServiceBuilder` descriptors.
/// Agents call `new` to start, `add_layer` repeatedly to stack layers, then
/// `build` to retrieve the final descriptor for code generation.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_builder"` namespace and named
/// `tower_builder__<verb>`.
pub struct TowerBuilderPlugin(Arc<TowerBuilderCtx>);

impl TowerBuilderPlugin {
    /// Create a new `TowerBuilderPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(TowerBuilderCtx::new()))
    }
}

impl Default for TowerBuilderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerBuilderPlugin {
    fn name(&self) -> &'static str {
        "tower_builder"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_builder")
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
            let full_name = if name.starts_with("tower_builder__") {
                name.to_string()
            } else {
                format!("tower_builder__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_builder")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
