//! Axum descriptor enum types.
//!
//! Small unit-variant enums that classify HTTP methods, extractors, and
//! response kinds in axum router/handler/response descriptors.
//!
//! Available with the `axum-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// HTTP methods supported by axum routers.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    ToCodeLiteral,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AxumHttpMethod {
    /// HTTP GET
    Get,
    /// HTTP POST
    Post,
    /// HTTP PUT
    Put,
    /// HTTP DELETE
    Delete,
    /// HTTP PATCH
    Patch,
    /// HTTP HEAD
    Head,
    /// HTTP OPTIONS
    Options,
    /// HTTP TRACE
    Trace,
    /// Match any HTTP method
    Any,
}

/// Axum extractor kinds.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum AxumExtractorKind {
    /// Path parameter extractor (`Path<T>`)
    Path,
    /// Query string extractor (`Query<T>`)
    Query,
    /// JSON body extractor (`Json<T>`)
    Json,
    /// Shared application state (`State<T>`)
    State,
    /// Request extension (`Extension<T>`)
    Extension,
    /// Form body extractor (`Form<T>`)
    Form,
    /// Request headers (`HeaderMap`)
    Headers,
    /// Raw bytes body (`Bytes`)
    RawBody,
    /// Raw query string (`RawQuery`)
    RawQuery,
    /// Original request URI (`OriginalUri`)
    OriginalUri,
    /// Matched route path pattern (`MatchedPath`)
    MatchedPath,
    /// Remote connection info (`ConnectInfo<T>`)
    ConnectInfo,
}

/// Axum response kinds.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum AxumResponseKind {
    /// JSON response (`Json<T>`)
    Json,
    /// HTML response (`Html<T>`)
    Html,
    /// Redirect response (`Redirect`)
    Redirect,
    /// No content (`StatusCode::NO_CONTENT`)
    NoContent,
    /// Status-only response
    Status,
    /// Custom / arbitrary response expression
    Custom,
}
