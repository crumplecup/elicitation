//! Axum descriptor struct types.
//!
//! Serializable survey structs that describe axum router, handler, response,
//! and serve configurations without requiring live axum instances.
//!
//! Available with the `axum-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AxumExtractorKind, AxumHttpMethod, AxumResponseKind};

/// A single route entry in a router descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumRouteEntry {
    /// HTTP method.
    pub method: AxumHttpMethod,
    /// URL path pattern (e.g. `"/users/:id"`).
    pub path: String,
    /// Handler function name or expression (e.g. `"get_user"`).
    pub handler: String,
}

/// Structured description of a db pool (or state struct) injected via
/// `.with_state()` and optionally threaded into Leptos server functions via
/// `provide_context`.
///
/// This is the common-case sugar.  For fully custom state expressions use
/// [`AxumRouterDescriptor::custom_state_expr`] instead.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumDbSlot {
    /// Rust type expression for the pool or state struct (e.g. `"sqlx::AnyPool"`
    /// or `"Arc<AppState>"`).
    pub pool_type: String,
    /// Variable name used in generated code (e.g. `"pool"`).
    ///
    /// Emitted as `.with_state({var_name})`.
    pub var_name: String,
    /// When `true` the bridge emits `leptos_routes_with_context` and injects
    /// `provide_context({var_name}.clone())` so that every Leptos server
    /// function can call `use_context::<{pool_type}>()`.
    pub provide_leptos_context: bool,
}

/// Descriptor for a `Router<S>` configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumRouterDescriptor {
    /// Rust type name of the router state (e.g. `"AppState"`).
    pub state_type: String,
    /// Routes registered on this router.
    pub routes: Vec<AxumRouteEntry>,
    /// Raw method-call expressions emitted verbatim between routes and layers,
    /// as `.{expr}` (e.g. `"leptos_routes(&opts, routes, App)"`).
    ///
    /// Use this for integration methods that are not standard
    /// `.route()` calls — such as `.leptos_routes()` or `.nest()`.
    pub raw_method_calls: Vec<String>,
    /// Layer expressions applied in order (e.g. `"TraceLayer::new_for_http()"`).
    pub layers: Vec<String>,
    /// Optional fallback handler expression.
    pub fallback: Option<String>,
    /// Optional db pool / state slot.
    ///
    /// When set, `emit_router` appends `.with_state({var_name})` as the
    /// terminal call and sets `state_type` from `pool_type`.  Takes
    /// precedence over [`custom_state_expr`][Self::custom_state_expr].
    pub db_slot: Option<AxumDbSlot>,
    /// Arbitrary `.with_state(expr)` for non-pool custom state.
    ///
    /// Ignored when [`db_slot`][Self::db_slot] is set.
    pub custom_state_expr: Option<String>,
}

/// A single extractor argument in a handler signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumExtractorEntry {
    /// Rust variable name (e.g. `"payload"`).
    pub var_name: String,
    /// Extractor kind.
    pub kind: AxumExtractorKind,
    /// Inner Rust type name (e.g. `"CreateUserRequest"`).
    pub type_name: String,
}

/// Descriptor for an async axum handler function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumHandlerDescriptor {
    /// Handler function name (e.g. `"create_user"`).
    pub name: String,
    /// Extractor parameters in order.
    pub extractors: Vec<AxumExtractorEntry>,
    /// Return type expression (e.g. `"impl IntoResponse"`).
    pub return_type: String,
    /// Optional body Rust expression or block. If `None`, emits a `todo!()` stub.
    pub body: Option<String>,
}

/// Descriptor for an axum response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumResponseDescriptor {
    /// Response kind.
    pub kind: AxumResponseKind,
    /// HTTP status code.
    pub status_code: u16,
    /// Body expression (e.g. `"serde_json::json!({\"ok\": true})"`). Optional.
    pub body_expr: Option<String>,
}

/// Descriptor for an axum serve configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AxumServeDescriptor {
    /// Bind address (e.g. `"0.0.0.0:3000"`).
    pub addr: String,
    /// UUID of the router descriptor this server wraps.
    pub router_id: Uuid,
    /// Optional graceful shutdown signal expression (e.g. `"tokio::signal::ctrl_c()"`).
    pub graceful_shutdown: Option<String>,
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    FieldInfo, PatternDetails, Prompt, TypeMetadata,
};

// --- AxumRouteEntry ----------------------------------------------------------

impl Prompt for AxumRouteEntry {
    fn prompt() -> Option<&'static str> { Some("Describe an axum route entry (method, path, handler):") }
}
crate::default_style!(AxumRouteEntry => AxumRouteEntryStyle);
impl Elicitation for AxumRouteEntry {
    type Style = AxumRouteEntryStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumRouteEntry");
        let method = AxumHttpMethod::elicit(communicator).await?;
        let path = String::elicit(communicator).await?;
        let handler = String::elicit(communicator).await?;
        Ok(Self { method, path, handler })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumRouteEntry {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumRouteEntry",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "method", type_name: "AxumHttpMethod", prompt: None },
                FieldInfo { name: "path", type_name: "String", prompt: None },
                FieldInfo { name: "handler", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumRouteEntry {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumRouteEntry".to_string(),
            fields: vec![
                ("method".to_string(), Box::new(AxumHttpMethod::prompt_tree())),
                ("path".to_string(), Box::new(String::prompt_tree())),
                ("handler".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- AxumDbSlot --------------------------------------------------------------

impl Prompt for AxumDbSlot {
    fn prompt() -> Option<&'static str> { Some("Describe the axum db pool / state slot:") }
}
crate::default_style!(AxumDbSlot => AxumDbSlotStyle);
impl Elicitation for AxumDbSlot {
    type Style = AxumDbSlotStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumDbSlot");
        let pool_type = String::elicit(communicator).await?;
        let var_name = String::elicit(communicator).await?;
        let provide_leptos_context = bool::elicit(communicator).await?;
        Ok(Self { pool_type, var_name, provide_leptos_context })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumDbSlot {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumDbSlot",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "pool_type", type_name: "String", prompt: None },
                FieldInfo { name: "var_name", type_name: "String", prompt: None },
                FieldInfo { name: "provide_leptos_context", type_name: "bool", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumDbSlot {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumDbSlot".to_string(),
            fields: vec![
                ("pool_type".to_string(), Box::new(String::prompt_tree())),
                ("var_name".to_string(), Box::new(String::prompt_tree())),
                ("provide_leptos_context".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

// --- AxumRouterDescriptor ----------------------------------------------------

impl Prompt for AxumRouterDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe an axum Router<S> configuration:") }
}
crate::default_style!(AxumRouterDescriptor => AxumRouterDescriptorStyle);
impl Elicitation for AxumRouterDescriptor {
    type Style = AxumRouterDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumRouterDescriptor");
        let state_type = String::elicit(communicator).await?;
        let routes = Vec::<AxumRouteEntry>::elicit(communicator).await?;
        let raw_method_calls = Vec::<String>::elicit(communicator).await?;
        let layers = Vec::<String>::elicit(communicator).await?;
        let fallback = Option::<String>::elicit(communicator).await?;
        let db_slot = Option::<AxumDbSlot>::elicit(communicator).await?;
        let custom_state_expr = Option::<String>::elicit(communicator).await?;
        Ok(Self { state_type, routes, raw_method_calls, layers, fallback, db_slot, custom_state_expr })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumRouterDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumRouterDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "state_type", type_name: "String", prompt: None },
                FieldInfo { name: "routes", type_name: "Vec<AxumRouteEntry>", prompt: None },
                FieldInfo { name: "raw_method_calls", type_name: "Vec<String>", prompt: None },
                FieldInfo { name: "layers", type_name: "Vec<String>", prompt: None },
                FieldInfo { name: "fallback", type_name: "Option<String>", prompt: None },
                FieldInfo { name: "db_slot", type_name: "Option<AxumDbSlot>", prompt: None },
                FieldInfo { name: "custom_state_expr", type_name: "Option<String>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumRouterDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumRouterDescriptor".to_string(),
            fields: vec![
                ("state_type".to_string(), Box::new(String::prompt_tree())),
                ("routes".to_string(), Box::new(AxumRouteEntry::prompt_tree())),
                ("raw_method_calls".to_string(), Box::new(String::prompt_tree())),
                ("layers".to_string(), Box::new(String::prompt_tree())),
                ("fallback".to_string(), Box::new(String::prompt_tree())),
                ("db_slot".to_string(), Box::new(String::prompt_tree())),
                ("custom_state_expr".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- AxumExtractorEntry ------------------------------------------------------

impl Prompt for AxumExtractorEntry {
    fn prompt() -> Option<&'static str> { Some("Describe an axum extractor argument:") }
}
crate::default_style!(AxumExtractorEntry => AxumExtractorEntryStyle);
impl Elicitation for AxumExtractorEntry {
    type Style = AxumExtractorEntryStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumExtractorEntry");
        let var_name = String::elicit(communicator).await?;
        let kind = AxumExtractorKind::elicit(communicator).await?;
        let type_name = String::elicit(communicator).await?;
        Ok(Self { var_name, kind, type_name })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumExtractorEntry {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumExtractorEntry",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "var_name", type_name: "String", prompt: None },
                FieldInfo { name: "kind", type_name: "AxumExtractorKind", prompt: None },
                FieldInfo { name: "type_name", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumExtractorEntry {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumExtractorEntry".to_string(),
            fields: vec![
                ("var_name".to_string(), Box::new(String::prompt_tree())),
                ("kind".to_string(), Box::new(AxumExtractorKind::prompt_tree())),
                ("type_name".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- AxumHandlerDescriptor ---------------------------------------------------

impl Prompt for AxumHandlerDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe an async axum handler function:") }
}
crate::default_style!(AxumHandlerDescriptor => AxumHandlerDescriptorStyle);
impl Elicitation for AxumHandlerDescriptor {
    type Style = AxumHandlerDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumHandlerDescriptor");
        let name = String::elicit(communicator).await?;
        let extractors = Vec::<AxumExtractorEntry>::elicit(communicator).await?;
        let return_type = String::elicit(communicator).await?;
        let body = Option::<String>::elicit(communicator).await?;
        Ok(Self { name, extractors, return_type, body })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumHandlerDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumHandlerDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "name", type_name: "String", prompt: None },
                FieldInfo { name: "extractors", type_name: "Vec<AxumExtractorEntry>", prompt: None },
                FieldInfo { name: "return_type", type_name: "String", prompt: None },
                FieldInfo { name: "body", type_name: "Option<String>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumHandlerDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumHandlerDescriptor".to_string(),
            fields: vec![
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("extractors".to_string(), Box::new(AxumExtractorEntry::prompt_tree())),
                ("return_type".to_string(), Box::new(String::prompt_tree())),
                ("body".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- AxumResponseDescriptor --------------------------------------------------

impl Prompt for AxumResponseDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe an axum response:") }
}
crate::default_style!(AxumResponseDescriptor => AxumResponseDescriptorStyle);
impl Elicitation for AxumResponseDescriptor {
    type Style = AxumResponseDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumResponseDescriptor");
        let kind = AxumResponseKind::elicit(communicator).await?;
        let status_code = u16::elicit(communicator).await?;
        let body_expr = Option::<String>::elicit(communicator).await?;
        Ok(Self { kind, status_code, body_expr })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumResponseDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumResponseDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "kind", type_name: "AxumResponseKind", prompt: None },
                FieldInfo { name: "status_code", type_name: "u16", prompt: None },
                FieldInfo { name: "body_expr", type_name: "Option<String>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumResponseDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumResponseDescriptor".to_string(),
            fields: vec![
                ("kind".to_string(), Box::new(AxumResponseKind::prompt_tree())),
                ("status_code".to_string(), Box::new(u16::prompt_tree())),
                ("body_expr".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- AxumServeDescriptor -----------------------------------------------------

impl Prompt for AxumServeDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe an axum serve configuration:") }
}
crate::default_style!(AxumServeDescriptor => AxumServeDescriptorStyle);
impl Elicitation for AxumServeDescriptor {
    type Style = AxumServeDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumServeDescriptor");
        let addr = String::elicit(communicator).await?;
        let router_id = uuid::Uuid::elicit(communicator).await?;
        let graceful_shutdown = Option::<String>::elicit(communicator).await?;
        Ok(Self { addr, router_id, graceful_shutdown })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for AxumServeDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumServeDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "addr", type_name: "String", prompt: None },
                FieldInfo { name: "router_id", type_name: "Uuid", prompt: None },
                FieldInfo { name: "graceful_shutdown", type_name: "Option<String>", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for AxumServeDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "AxumServeDescriptor".to_string(),
            fields: vec![
                ("addr".to_string(), Box::new(String::prompt_tree())),
                ("router_id".to_string(), Box::new(uuid::Uuid::prompt_tree())),
                ("graceful_shutdown".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
