//! `BevyDerivePlugin` — fragment tools that emit Bevy derive-code skeletons.
//!
//! These tools cover the first Phase 3 macro surface for `elicit_bevy`:
//! generating Rust items annotated with the standard Bevy derive macros used
//! for ECS types, schedules, and reflection.

use elicitation::emit_code::{CrateDep, EmitCode};
use elicitation::{ElicitPlugin, elicit_tool};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// Common item template used by Bevy derive tools.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ItemTemplate {
    /// Type name in Rust `PascalCase`.
    pub name: String,
    /// Optional visibility, e.g. `"pub"` or `"pub(crate)"`.
    #[serde(default)]
    pub visibility: Option<String>,
    /// Optional rustdoc lines emitted as `#[doc = "..."]`.
    #[serde(default)]
    pub docs: Vec<String>,
    /// Extra outer attributes emitted before the item, e.g. `#[require(Foo)]`.
    #[serde(default)]
    pub attrs: Vec<String>,
    /// Additional derives appended after the Bevy derives for the tool.
    #[serde(default)]
    pub extra_derives: Vec<String>,
    /// The emitted Rust item shape.
    pub shape: ItemShape,
}

/// Shape of the emitted Rust item.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ItemShape {
    /// A unit struct, e.g. `struct Marker;`.
    UnitStruct,
    /// A tuple struct, e.g. `struct Wrapper(f32, String);`.
    TupleStruct {
        /// Tuple field types.
        #[serde(default)]
        types: Vec<String>,
    },
    /// A named-field struct, e.g. `struct Position { x: f32, y: f32 }`.
    NamedStruct {
        /// Struct fields.
        #[serde(default)]
        fields: Vec<NamedFieldSpec>,
    },
    /// An enum, e.g. `enum GameState { Loading, Playing }`.
    Enum {
        /// Enum variants.
        #[serde(default)]
        variants: Vec<EnumVariantSpec>,
    },
}

/// A named struct field.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NamedFieldSpec {
    /// Field name.
    pub name: String,
    /// Field type as Rust source.
    pub ty: String,
    /// Optional rustdoc lines for the field.
    #[serde(default)]
    pub docs: Vec<String>,
    /// Optional field attributes, e.g. `#[reflect(ignore)]`.
    #[serde(default)]
    pub attrs: Vec<String>,
}

/// An enum variant definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EnumVariantSpec {
    /// Variant name.
    pub name: String,
    /// Optional rustdoc lines for the variant.
    #[serde(default)]
    pub docs: Vec<String>,
    /// Optional variant attributes.
    #[serde(default)]
    pub attrs: Vec<String>,
    /// Variant payload shape.
    #[serde(default)]
    pub shape: VariantShape,
}

/// Shape of an enum variant payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VariantShape {
    /// A unit variant.
    #[default]
    Unit,
    /// A tuple variant.
    Tuple {
        /// Tuple payload types.
        #[serde(default)]
        types: Vec<String>,
    },
    /// A struct-like variant.
    Named {
        /// Named fields for the variant.
        #[serde(default)]
        fields: Vec<NamedFieldSpec>,
    },
}

/// Parameters for `bevy_derive__component`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComponentDeriveParams {
    /// Item template to annotate with `#[derive(Component)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__resource`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResourceDeriveParams {
    /// Item template to annotate with `#[derive(Resource)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__asset`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssetDeriveParams {
    /// Item template to annotate with `#[derive(Asset, TypePath)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__event`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EventDeriveParams {
    /// Item template to annotate with `#[derive(Event)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__bundle`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BundleDeriveParams {
    /// Item template to annotate with `#[derive(Bundle)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__states`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatesDeriveParams {
    /// Item template to annotate with `#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__system_set`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemSetDeriveParams {
    /// Item template to annotate with `#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__schedule_label`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScheduleLabelDeriveParams {
    /// Item template to annotate with `#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]`.
    pub item: ItemTemplate,
}

/// Parameters for `bevy_derive__reflect`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReflectDeriveParams {
    /// Item template to annotate with `#[derive(Reflect)]`.
    pub item: ItemTemplate,
    /// Optional trait list for the `#[reflect(...)]` attribute.
    #[serde(default)]
    pub reflect_traits: Vec<String>,
    /// Optional Bevy `#[type_path = "..."]` attribute payload.
    #[serde(default)]
    pub type_path: Option<String>,
}

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_source(source: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

fn parse_ident(src: &str, context: &str) -> Result<syn::Ident, ErrorData> {
    syn::parse_str::<syn::Ident>(src)
        .map_err(|error| tool_err(format!("invalid {context} `{src}`: {error}")))
}

fn parse_visibility(src: &str) -> Result<syn::Visibility, ErrorData> {
    syn::parse_str::<syn::Visibility>(src)
        .map_err(|error| tool_err(format!("invalid visibility `{src}`: {error}")))
}

fn parse_type(src: &str, context: &str) -> Result<syn::Type, ErrorData> {
    syn::parse_str::<syn::Type>(src)
        .map_err(|error| tool_err(format!("invalid {context} type `{src}`: {error}")))
}

fn parse_path(src: &str, context: &str) -> Result<syn::Path, ErrorData> {
    syn::parse_str::<syn::Path>(src)
        .map_err(|error| tool_err(format!("invalid {context} path `{src}`: {error}")))
}

fn parse_attr_tokens(src: &str, context: &str) -> Result<TokenStream, ErrorData> {
    src.parse::<TokenStream>()
        .map_err(|error| tool_err(format!("invalid {context} attribute `{src}`: {error}")))
}

fn validate_fields(fields: &[NamedFieldSpec], context: &str) -> Result<(), ErrorData> {
    for field in fields {
        let _ = parse_ident(&field.name, &format!("{context} field name"))?;
        let _ = parse_type(&field.ty, &format!("{context} field"))?;
        for attr in &field.attrs {
            let _ = parse_attr_tokens(attr, &format!("{context} field"))?;
        }
    }
    Ok(())
}

fn validate_variant_shape(shape: &VariantShape, context: &str) -> Result<(), ErrorData> {
    match shape {
        VariantShape::Unit => Ok(()),
        VariantShape::Tuple { types } => {
            for ty in types {
                let _ = parse_type(ty, context)?;
            }
            Ok(())
        }
        VariantShape::Named { fields } => validate_fields(fields, context),
    }
}

fn validate_item_template(item: &ItemTemplate) -> Result<(), ErrorData> {
    let _ = parse_ident(&item.name, "item name")?;
    if let Some(visibility) = &item.visibility {
        let _ = parse_visibility(visibility)?;
    }
    for attr in &item.attrs {
        let _ = parse_attr_tokens(attr, "item")?;
    }
    for derive in &item.extra_derives {
        let _ = parse_path(derive, "extra derive")?;
    }
    match &item.shape {
        ItemShape::UnitStruct => Ok(()),
        ItemShape::TupleStruct { types } => {
            for ty in types {
                let _ = parse_type(ty, "tuple field")?;
            }
            Ok(())
        }
        ItemShape::NamedStruct { fields } => validate_fields(fields, "struct"),
        ItemShape::Enum { variants } => {
            for variant in variants {
                let _ = parse_ident(&variant.name, "variant name")?;
                for attr in &variant.attrs {
                    let _ = parse_attr_tokens(attr, "variant")?;
                }
                validate_variant_shape(&variant.shape, &format!("variant `{}`", variant.name))?;
            }
            Ok(())
        }
    }
}

fn validate_reflect_params(params: &ReflectDeriveParams) -> Result<(), ErrorData> {
    validate_item_template(&params.item)?;
    for reflect_trait in &params.reflect_traits {
        let _ = parse_path(reflect_trait, "reflect trait")?;
    }
    if let Some(type_path) = &params.type_path
        && type_path.trim().is_empty()
    {
        return Err(tool_err("type_path must not be empty"));
    }
    Ok(())
}

fn docs_tokens(docs: &[String]) -> Vec<TokenStream> {
    docs.iter().map(|line| quote! { #[doc = #line] }).collect()
}

fn attr_tokens(attrs: &[String]) -> Vec<TokenStream> {
    attrs
        .iter()
        .map(|attr| {
            attr.parse::<TokenStream>()
                .expect("validated attributes must parse")
        })
        .collect()
}

fn field_tokens(fields: &[NamedFieldSpec]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|field| {
            let docs = docs_tokens(&field.docs);
            let attrs = attr_tokens(&field.attrs);
            let name = format_ident!("{}", field.name);
            let ty =
                syn::parse_str::<syn::Type>(&field.ty).expect("validated field types must parse");
            quote! {
                #(#docs)*
                #(#attrs)*
                #name: #ty
            }
        })
        .collect()
}

fn variant_tokens(variants: &[EnumVariantSpec]) -> Vec<TokenStream> {
    variants
        .iter()
        .map(|variant| {
            let docs = docs_tokens(&variant.docs);
            let attrs = attr_tokens(&variant.attrs);
            let name = format_ident!("{}", variant.name);
            match &variant.shape {
                VariantShape::Unit => quote! {
                    #(#docs)*
                    #(#attrs)*
                    #name
                },
                VariantShape::Tuple { types } => {
                    let tys: Vec<syn::Type> = types
                        .iter()
                        .map(|ty| {
                            syn::parse_str::<syn::Type>(ty)
                                .expect("validated tuple types must parse")
                        })
                        .collect();
                    quote! {
                        #(#docs)*
                        #(#attrs)*
                        #name(#(#tys),*)
                    }
                }
                VariantShape::Named { fields } => {
                    let rendered_fields = field_tokens(fields);
                    quote! {
                        #(#docs)*
                        #(#attrs)*
                        #name { #(#rendered_fields),* }
                    }
                }
            }
        })
        .collect()
}

fn emit_item(
    item: &ItemTemplate,
    derive_paths: &[&str],
    extra_attrs: Vec<TokenStream>,
) -> TokenStream {
    let docs = docs_tokens(&item.docs);
    let attrs = attr_tokens(&item.attrs);
    let visibility = item
        .visibility
        .as_deref()
        .map(|src| parse_visibility(src).expect("validated visibility must parse"))
        .unwrap_or(syn::Visibility::Inherited);
    let derives: Vec<syn::Path> = derive_paths
        .iter()
        .map(|path| parse_path(path, "derive").expect("static derive path must parse"))
        .chain(
            item.extra_derives
                .iter()
                .map(|path| parse_path(path, "derive").expect("validated derive path must parse")),
        )
        .collect();
    let name = format_ident!("{}", item.name);
    let derive_attr = quote! { #[derive(#(#derives),*)] };
    let body = match &item.shape {
        ItemShape::UnitStruct => quote! { #visibility struct #name; },
        ItemShape::TupleStruct { types } => {
            let tys: Vec<syn::Type> = types
                .iter()
                .map(|ty| {
                    syn::parse_str::<syn::Type>(ty).expect("validated tuple types must parse")
                })
                .collect();
            quote! { #visibility struct #name(#(#tys),*); }
        }
        ItemShape::NamedStruct { fields } => {
            let rendered_fields = field_tokens(fields);
            quote! {
                #visibility struct #name {
                    #(#rendered_fields),*
                }
            }
        }
        ItemShape::Enum { variants } => {
            let rendered_variants = variant_tokens(variants);
            quote! {
                #visibility enum #name {
                    #(#rendered_variants),*
                }
            }
        }
    };
    quote! {
        #(#docs)*
        #(#attrs)*
        #derive_attr
        #(#extra_attrs)*
        #body
    }
}

fn bevy_dep() -> Vec<CrateDep> {
    vec![CrateDep::new("bevy", "0.18")]
}

macro_rules! impl_derive_emit {
    ($ty:ty, [$($derive:literal),+ $(,)?]) => {
        impl EmitCode for $ty {
            fn emit_code(&self) -> TokenStream {
                emit_item(&self.item, &[$($derive),+], vec![])
            }

            fn crate_deps(&self) -> Vec<CrateDep> {
                bevy_dep()
            }
        }
    };
}

impl_derive_emit!(ComponentDeriveParams, ["bevy::prelude::Component"]);
impl_derive_emit!(ResourceDeriveParams, ["bevy::prelude::Resource"]);
impl_derive_emit!(
    AssetDeriveParams,
    ["bevy::asset::Asset", "bevy::reflect::TypePath"]
);
impl_derive_emit!(EventDeriveParams, ["bevy::prelude::Event"]);
impl_derive_emit!(BundleDeriveParams, ["bevy::prelude::Bundle"]);
impl_derive_emit!(
    StatesDeriveParams,
    [
        "bevy::prelude::States",
        "Debug",
        "Clone",
        "PartialEq",
        "Eq",
        "Hash"
    ]
);
impl_derive_emit!(
    SystemSetDeriveParams,
    [
        "bevy::prelude::SystemSet",
        "Debug",
        "Clone",
        "PartialEq",
        "Eq",
        "Hash"
    ]
);
impl_derive_emit!(
    ScheduleLabelDeriveParams,
    [
        "bevy::prelude::ScheduleLabel",
        "Debug",
        "Clone",
        "PartialEq",
        "Eq",
        "Hash"
    ]
);

impl EmitCode for ReflectDeriveParams {
    fn emit_code(&self) -> TokenStream {
        let mut attrs = Vec::new();
        if !self.reflect_traits.is_empty() {
            let traits: Vec<syn::Path> = self
                .reflect_traits
                .iter()
                .map(|path| {
                    parse_path(path, "reflect trait").expect("validated reflect path must parse")
                })
                .collect();
            attrs.push(quote! { #[reflect(#(#traits),*)] });
        }
        if let Some(type_path) = &self.type_path {
            attrs.push(quote! { #[type_path = #type_path] });
        }
        emit_item(&self.item, &["bevy::reflect::Reflect"], attrs)
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

elicitation::register_emit!("component", ComponentDeriveParams);
elicitation::register_emit!("resource", ResourceDeriveParams);
elicitation::register_emit!("asset", AssetDeriveParams);
elicitation::register_emit!("event", EventDeriveParams);
elicitation::register_emit!("bundle", BundleDeriveParams);
elicitation::register_emit!("states", StatesDeriveParams);
elicitation::register_emit!("system_set", SystemSetDeriveParams);
elicitation::register_emit!("schedule_label", ScheduleLabelDeriveParams);
elicitation::register_emit!("reflect", ReflectDeriveParams);

/// MCP plugin exposing Bevy derive-code fragment tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_derive")]
pub struct BevyDerivePlugin;

impl BevyDerivePlugin {
    /// Creates a new Bevy derive plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyDerivePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "component",
    description = "Emit a Rust item annotated with `#[derive(Component)]`."
)]
#[instrument(skip_all)]
async fn emit_component(p: ComponentDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "resource",
    description = "Emit a Rust item annotated with `#[derive(Resource)]`."
)]
#[instrument(skip_all)]
async fn emit_resource(p: ResourceDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "asset",
    description = "Emit a Rust item annotated with `#[derive(Asset, TypePath)]`."
)]
#[instrument(skip_all)]
async fn emit_asset(p: AssetDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "event",
    description = "Emit a Rust item annotated with `#[derive(Event)]`."
)]
#[instrument(skip_all)]
async fn emit_event(p: EventDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "bundle",
    description = "Emit a Rust item annotated with `#[derive(Bundle)]`."
)]
#[instrument(skip_all)]
async fn emit_bundle(p: BundleDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "states",
    description = "Emit a Rust item annotated with `#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]`."
)]
#[instrument(skip_all)]
async fn emit_states(p: StatesDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "system_set",
    description = "Emit a Rust item annotated with `#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]`."
)]
#[instrument(skip_all)]
async fn emit_system_set(p: SystemSetDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "schedule_label",
    description = "Emit a Rust item annotated with `#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]`."
)]
#[instrument(skip_all)]
async fn emit_schedule_label(p: ScheduleLabelDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_item_template(&p.item)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_derive",
    emit = None,
    name = "reflect",
    description = "Emit a Rust item annotated with `#[derive(Reflect)]`, optional `#[reflect(...)]`, and optional `#[type_path = \"...\"]`."
)]
#[instrument(skip_all)]
async fn emit_reflect(p: ReflectDeriveParams) -> Result<CallToolResult, ErrorData> {
    validate_reflect_params(&p)?;
    ok_source(p.emit_code().to_string())
}
