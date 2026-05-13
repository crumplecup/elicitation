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
