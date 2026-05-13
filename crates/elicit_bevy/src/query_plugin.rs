//! `BevyQueryPlugin` — generic ECS parameter and query codegen tools.
//!
//! This plugin covers the generic Bevy ECS surfaces that cannot be represented
//! as concrete MCP values: `Query`, `Res`, `EventReader`, `Handle`, `Local`,
//! and complete system signatures assembled from those parameter fragments.

use elicitation::emit_code::{CrateDep, EmitCode};
use elicitation::{ElicitPlugin, elicit_tool};
use proc_macro2::TokenStream;
use quote::quote;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// How a query item should appear inside `Query<...>`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyQueryItemAccess {
    /// Emit the type by value, e.g. `Entity`.
    Value,
    /// Emit a shared reference, e.g. `&Transform`.
    #[default]
    Shared,
    /// Emit a mutable reference, e.g. `&mut Transform`.
    Mutable,
}

/// A single item inside a Bevy query.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyQueryItemSpec {
    /// Rust type path for the query item.
    pub ty: String,
    /// Access mode used for this item.
    #[serde(default)]
    pub access: BevyQueryItemAccess,
}

/// Supported Bevy query filter wrappers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyQueryFilterKind {
    /// Emit `With<T>`.
    With,
    /// Emit `Without<T>`.
    Without,
    /// Emit `Added<T>`.
    Added,
    /// Emit `Changed<T>`.
    Changed,
}

/// Parameters for `bevy_query__define_component_query`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineComponentQueryParams {
    /// Parameter binding name.
    pub binding: String,
    /// Whether to emit `mut binding`.
    #[serde(default)]
    pub mutable_binding: bool,
    /// Query items to include in the data position.
    pub items: Vec<BevyQueryItemSpec>,
    /// Optional query filter type fragments.
    #[serde(default)]
    pub filters: Vec<String>,
}

/// Parameters for `bevy_query__define_resource`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineResourceParams {
    /// Parameter binding name.
    pub binding: String,
    /// Resource type used inside `Res<T>` / `ResMut<T>`.
    pub resource_type: String,
    /// Whether to emit `ResMut<T>` and a mutable binding.
    #[serde(default)]
    pub mutable: bool,
}

/// Parameters for `bevy_query__define_event_reader`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineEventReaderParams {
    /// Parameter binding name.
    pub binding: String,
    /// Event type read by the parameter.
    pub event_type: String,
}

/// Parameters for `bevy_query__define_event_writer`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineEventWriterParams {
    /// Parameter binding name.
    pub binding: String,
    /// Event type written by the parameter.
    pub event_type: String,
}

/// Parameters for `bevy_query__define_handle`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineHandleParams {
    /// Optional field visibility such as `pub`.
    #[serde(default)]
    pub visibility: Option<String>,
    /// Field name.
    pub binding: String,
    /// Asset type used in `Handle<T>`.
    pub asset_type: String,
}

/// Parameters for `bevy_query__define_local`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineLocalParams {
    /// Parameter binding name.
    pub binding: String,
    /// Local state type.
    pub local_type: String,
    /// Whether to emit `mut binding`.
    #[serde(default)]
    pub mutable_binding: bool,
}

/// Parameters for `bevy_query__define_time`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DefineTimeParams {
    /// Parameter binding name.
    pub binding: String,
    /// Optional time generic such as `Fixed`, `Virtual`, or `Real`.
    #[serde(default)]
    pub time_generic: Option<String>,
    /// Whether to emit `ResMut<...>` and a mutable binding.
    #[serde(default)]
    pub mutable: bool,
}

/// Parameters for `bevy_query__filter`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FilterParams {
    /// Filter wrapper to emit.
    pub kind: BevyQueryFilterKind,
    /// Inner type for the filter.
    pub type_name: String,
}

/// Parameters for `bevy_query__system_signature`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemSignatureParams {
    /// Optional function visibility such as `pub`.
    #[serde(default)]
    pub visibility: Option<String>,
    /// System function name.
    pub function_name: String,
    /// Full parameter fragments such as `query: Query<&Transform>`.
    #[serde(default)]
    pub params: Vec<String>,
    /// Optional return type.
    #[serde(default)]
    pub return_type: Option<String>,
    /// Optional function body statements.
    #[serde(default)]
    pub body: Option<String>,
}

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_source(source: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

fn parse_ident(src: &str, context: &str) -> Result<syn::Ident, ErrorData> {
    syn::parse_str::<syn::Ident>(src)
        .map_err(|error| tool_err(format!("invalid {context} identifier `{src}`: {error}")))
}

fn parse_type(src: &str, context: &str) -> Result<syn::Type, ErrorData> {
    syn::parse_str::<syn::Type>(src)
        .map_err(|error| tool_err(format!("invalid {context} type `{src}`: {error}")))
}

fn parse_visibility(src: &str) -> Result<syn::Visibility, ErrorData> {
    syn::parse_str::<syn::Visibility>(src)
        .map_err(|error| tool_err(format!("invalid visibility `{src}`: {error}")))
}

fn parse_param_fragment(src: &str) -> Result<syn::FnArg, ErrorData> {
    syn::parse_str::<syn::FnArg>(src)
        .map_err(|error| tool_err(format!("invalid system parameter `{src}`: {error}")))
}

fn parse_body_statements(src: &str) -> Result<Vec<syn::Stmt>, ErrorData> {
    let wrapped = format!("{{{src}}}");
    syn::parse_str::<syn::Block>(&wrapped)
        .map(|block| block.stmts)
        .map_err(|error| tool_err(format!("invalid function body `{src}`: {error}")))
}

fn validate_non_empty<T>(values: &[T], context: &str) -> Result<(), ErrorData> {
    if values.is_empty() {
        Err(tool_err(format!("{context} must not be empty")))
    } else {
        Ok(())
    }
}

fn binding_tokens(binding: &str, mutable: bool) -> syn::Pat {
    let binding = parse_ident(binding, "binding").expect("validated bindings must parse");
    if mutable {
        syn::parse_quote!(mut #binding)
    } else {
        syn::parse_quote!(#binding)
    }
}

fn type_tokens(src: &str, context: &str) -> syn::Type {
    parse_type(src, context).expect("validated types must parse")
}

fn visibility_tokens(visibility: &Option<String>) -> syn::Visibility {
    visibility
        .as_deref()
        .map(|src| parse_visibility(src).expect("validated visibility must parse"))
        .unwrap_or_else(|| syn::parse_quote!())
}

fn fn_arg_tokens(src: &str) -> syn::FnArg {
    parse_param_fragment(src).expect("validated function args must parse")
}

fn filter_kind_ident(kind: BevyQueryFilterKind) -> syn::Ident {
    let name = match kind {
        BevyQueryFilterKind::With => "With",
        BevyQueryFilterKind::Without => "Without",
        BevyQueryFilterKind::Added => "Added",
        BevyQueryFilterKind::Changed => "Changed",
    };
    syn::Ident::new(name, proc_macro2::Span::call_site())
}

fn render_query_item(item: &BevyQueryItemSpec) -> TokenStream {
    let ty = type_tokens(&item.ty, "query item");
    match item.access {
        BevyQueryItemAccess::Value => quote! { #ty },
        BevyQueryItemAccess::Shared => quote! { &#ty },
        BevyQueryItemAccess::Mutable => quote! { &mut #ty },
    }
}

fn render_query_filters(filters: &[String]) -> TokenStream {
    let filter_tokens: Vec<syn::Type> = filters
        .iter()
        .map(|filter| type_tokens(filter, "query filter"))
        .collect();
    match filter_tokens.len() {
        0 => TokenStream::new(),
        1 => {
            let filter = &filter_tokens[0];
            quote! { , #filter }
        }
        _ => quote! { , (#(#filter_tokens),*) },
    }
}

fn time_type_tokens(time_generic: &Option<String>) -> TokenStream {
    match time_generic {
        Some(generic) => {
            let generic = type_tokens(generic, "time generic");
            quote! { ::bevy::prelude::Time<#generic> }
        }
        None => quote! { ::bevy::prelude::Time },
    }
}

fn bevy_dep() -> Vec<CrateDep> {
    vec![CrateDep::new("bevy", "0.18")]
}

impl EmitCode for DefineComponentQueryParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, self.mutable_binding);
        let items: Vec<TokenStream> = self.items.iter().map(render_query_item).collect();
        let data = if items.len() == 1 {
            let item = &items[0];
            quote! { #item }
        } else {
            quote! { (#(#items),*) }
        };
        let filters = render_query_filters(&self.filters);
        quote! { #binding: ::bevy::ecs::system::Query<#data #filters> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineResourceParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, self.mutable);
        let resource = type_tokens(&self.resource_type, "resource");
        if self.mutable {
            quote! { #binding: ::bevy::ecs::system::ResMut<#resource> }
        } else {
            quote! { #binding: ::bevy::ecs::system::Res<#resource> }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineEventReaderParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, false);
        let event = type_tokens(&self.event_type, "event");
        quote! { #binding: ::bevy::ecs::event::EventReader<#event> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineEventWriterParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, true);
        let event = type_tokens(&self.event_type, "event");
        quote! { #binding: ::bevy::ecs::event::EventWriter<#event> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineHandleParams {
    fn emit_code(&self) -> TokenStream {
        let visibility = visibility_tokens(&self.visibility);
        let binding = parse_ident(&self.binding, "field").expect("validated field must parse");
        let asset = type_tokens(&self.asset_type, "asset");
        quote! { #visibility #binding: ::bevy::asset::Handle<#asset> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineLocalParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, self.mutable_binding);
        let local = type_tokens(&self.local_type, "local");
        quote! { #binding: ::bevy::ecs::system::Local<#local> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DefineTimeParams {
    fn emit_code(&self) -> TokenStream {
        let binding = binding_tokens(&self.binding, self.mutable);
        let time = time_type_tokens(&self.time_generic);
        if self.mutable {
            quote! { #binding: ::bevy::ecs::system::ResMut<#time> }
        } else {
            quote! { #binding: ::bevy::ecs::system::Res<#time> }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for FilterParams {
    fn emit_code(&self) -> TokenStream {
        let kind = filter_kind_ident(self.kind);
        let ty = type_tokens(&self.type_name, "filter");
        quote! { ::bevy::ecs::query::#kind<#ty> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for SystemSignatureParams {
    fn emit_code(&self) -> TokenStream {
        let visibility = visibility_tokens(&self.visibility);
        let name = parse_ident(&self.function_name, "function")
            .expect("validated function name must parse");
        let params: Vec<syn::FnArg> = self
            .params
            .iter()
            .map(|param| fn_arg_tokens(param))
            .collect();
        let output = self
            .return_type
            .as_deref()
            .map(|return_type| {
                let ty = type_tokens(return_type, "return");
                quote! { -> #ty }
            })
            .unwrap_or_default();
        let body = self
            .body
            .as_deref()
            .map(|body| parse_body_statements(body).expect("validated body must parse"))
            .unwrap_or_default();
        quote! {
            #visibility fn #name(#(#params),*) #output {
                #(#body)*
            }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

elicitation::register_emit!("define_component_query", DefineComponentQueryParams);
elicitation::register_emit!("define_resource", DefineResourceParams);
elicitation::register_emit!("define_event_reader", DefineEventReaderParams);
elicitation::register_emit!("define_event_writer", DefineEventWriterParams);
elicitation::register_emit!("define_handle", DefineHandleParams);
elicitation::register_emit!("define_local", DefineLocalParams);
elicitation::register_emit!("define_time", DefineTimeParams);
elicitation::register_emit!("system_signature", SystemSignatureParams);
elicitation::register_emit!("filter", FilterParams);

/// MCP plugin exposing generic Bevy ECS parameter fragment tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_query")]
pub struct BevyQueryPlugin;

impl BevyQueryPlugin {
    /// Creates a new Bevy query fragment plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyQueryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_define_component_query(params: &DefineComponentQueryParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    validate_non_empty(&params.items, "query items")?;
    for item in &params.items {
        let _ = parse_type(&item.ty, "query item")?;
    }
    for filter in &params.filters {
        let _ = parse_type(filter, "query filter")?;
    }
    Ok(())
}

fn validate_define_resource(params: &DefineResourceParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    let _ = parse_type(&params.resource_type, "resource")?;
    Ok(())
}

fn validate_define_event_reader(params: &DefineEventReaderParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    let _ = parse_type(&params.event_type, "event")?;
    Ok(())
}

fn validate_define_event_writer(params: &DefineEventWriterParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    let _ = parse_type(&params.event_type, "event")?;
    Ok(())
}

fn validate_define_handle(params: &DefineHandleParams) -> Result<(), ErrorData> {
    if let Some(visibility) = &params.visibility {
        let _ = parse_visibility(visibility)?;
    }
    let _ = parse_ident(&params.binding, "field")?;
    let _ = parse_type(&params.asset_type, "asset")?;
    Ok(())
}

fn validate_define_local(params: &DefineLocalParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    let _ = parse_type(&params.local_type, "local")?;
    Ok(())
}

fn validate_define_time(params: &DefineTimeParams) -> Result<(), ErrorData> {
    let _ = parse_ident(&params.binding, "binding")?;
    if let Some(time_generic) = &params.time_generic {
        let _ = parse_type(time_generic, "time generic")?;
    }
    Ok(())
}

fn validate_filter(params: &FilterParams) -> Result<(), ErrorData> {
    let _ = parse_type(&params.type_name, "filter")?;
    Ok(())
}

fn validate_system_signature(params: &SystemSignatureParams) -> Result<(), ErrorData> {
    if let Some(visibility) = &params.visibility {
        let _ = parse_visibility(visibility)?;
    }
    let _ = parse_ident(&params.function_name, "function")?;
    for param in &params.params {
        let _ = parse_param_fragment(param)?;
    }
    if let Some(return_type) = &params.return_type {
        let _ = parse_type(return_type, "return")?;
    }
    if let Some(body) = &params.body {
        let _ = parse_body_statements(body)?;
    }
    Ok(())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_component_query",
    description = "Emit a `Query<...>` system parameter fragment from query items and optional filters.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_component_query(
    p: DefineComponentQueryParams,
) -> Result<CallToolResult, ErrorData> {
    validate_define_component_query(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_resource",
    description = "Emit a `Res<T>` or `ResMut<T>` system parameter fragment.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_resource(p: DefineResourceParams) -> Result<CallToolResult, ErrorData> {
    validate_define_resource(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_event_reader",
    description = "Emit an `EventReader<E>` system parameter fragment.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_event_reader(p: DefineEventReaderParams) -> Result<CallToolResult, ErrorData> {
    validate_define_event_reader(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_event_writer",
    description = "Emit an `EventWriter<E>` system parameter fragment with a mutable binding.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_event_writer(p: DefineEventWriterParams) -> Result<CallToolResult, ErrorData> {
    validate_define_event_writer(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_handle",
    description = "Emit a `Handle<A>` field declaration fragment.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_handle(p: DefineHandleParams) -> Result<CallToolResult, ErrorData> {
    validate_define_handle(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_local",
    description = "Emit a `Local<T>` system parameter fragment.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_local(p: DefineLocalParams) -> Result<CallToolResult, ErrorData> {
    validate_define_local(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "define_time",
    description = "Emit a `Res<Time>` or `Res<Time<Fixed>>`-style system parameter fragment.",
    emit = None
)]
#[instrument(skip_all)]
async fn define_time(p: DefineTimeParams) -> Result<CallToolResult, ErrorData> {
    validate_define_time(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "system_signature",
    description = "Emit a full Bevy system function signature from previously generated parameter fragments.",
    emit = None
)]
#[instrument(skip_all)]
async fn system_signature(p: SystemSignatureParams) -> Result<CallToolResult, ErrorData> {
    validate_system_signature(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_query",
    name = "filter",
    description = "Emit a Bevy query filter such as `With<T>` or `Changed<T>`.",
    emit = None
)]
#[instrument(skip_all)]
async fn filter(p: FilterParams) -> Result<CallToolResult, ErrorData> {
    validate_filter(&p)?;
    ok_source(p.emit_code().to_string())
}
