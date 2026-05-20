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

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

// --- AxumHttpMethod ----------------------------------------------------------

impl Prompt for AxumHttpMethod {
    fn prompt() -> Option<&'static str> {
        Some("Choose the HTTP method:")
    }
}

impl Select for AxumHttpMethod {
    fn options() -> Vec<Self> {
        vec![
            AxumHttpMethod::Get, AxumHttpMethod::Post, AxumHttpMethod::Put,
            AxumHttpMethod::Delete, AxumHttpMethod::Patch, AxumHttpMethod::Head,
            AxumHttpMethod::Options, AxumHttpMethod::Trace, AxumHttpMethod::Any,
        ]
    }
    fn labels() -> Vec<String> {
        vec!["GET","POST","PUT","DELETE","PATCH","HEAD","OPTIONS","TRACE","ANY"]
            .into_iter().map(|s| s.to_string()).collect()
    }
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "GET" => Some(AxumHttpMethod::Get),
            "POST" => Some(AxumHttpMethod::Post),
            "PUT" => Some(AxumHttpMethod::Put),
            "DELETE" => Some(AxumHttpMethod::Delete),
            "PATCH" => Some(AxumHttpMethod::Patch),
            "HEAD" => Some(AxumHttpMethod::Head),
            "OPTIONS" => Some(AxumHttpMethod::Options),
            "TRACE" => Some(AxumHttpMethod::Trace),
            "ANY" => Some(AxumHttpMethod::Any),
            _ => None,
        }
    }
}

crate::default_style!(AxumHttpMethod => AxumHttpMethodStyle);

impl Elicitation for AxumHttpMethod {
    type Style = AxumHttpMethodStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumHttpMethod");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose HTTP method:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid AxumHttpMethod: {}", label)))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("AxumHttpMethod", "Get")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("AxumHttpMethod", "Get")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("AxumHttpMethod", "Get")
    }
}

impl ElicitIntrospect for AxumHttpMethod {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumHttpMethod",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for AxumHttpMethod {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose HTTP method:").to_string(),
            type_name: "AxumHttpMethod".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- AxumExtractorKind -------------------------------------------------------

impl Prompt for AxumExtractorKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the axum extractor kind:")
    }
}

impl Select for AxumExtractorKind {
    fn options() -> Vec<Self> {
        vec![
            AxumExtractorKind::Path, AxumExtractorKind::Query, AxumExtractorKind::Json,
            AxumExtractorKind::State, AxumExtractorKind::Extension, AxumExtractorKind::Form,
            AxumExtractorKind::Headers, AxumExtractorKind::RawBody, AxumExtractorKind::RawQuery,
            AxumExtractorKind::OriginalUri, AxumExtractorKind::MatchedPath, AxumExtractorKind::ConnectInfo,
        ]
    }
    fn labels() -> Vec<String> {
        vec!["Path","Query","Json","State","Extension","Form","Headers",
             "RawBody","RawQuery","OriginalUri","MatchedPath","ConnectInfo"]
            .into_iter().map(|s| s.to_string()).collect()
    }
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Path" => Some(AxumExtractorKind::Path),
            "Query" => Some(AxumExtractorKind::Query),
            "Json" => Some(AxumExtractorKind::Json),
            "State" => Some(AxumExtractorKind::State),
            "Extension" => Some(AxumExtractorKind::Extension),
            "Form" => Some(AxumExtractorKind::Form),
            "Headers" => Some(AxumExtractorKind::Headers),
            "RawBody" => Some(AxumExtractorKind::RawBody),
            "RawQuery" => Some(AxumExtractorKind::RawQuery),
            "OriginalUri" => Some(AxumExtractorKind::OriginalUri),
            "MatchedPath" => Some(AxumExtractorKind::MatchedPath),
            "ConnectInfo" => Some(AxumExtractorKind::ConnectInfo),
            _ => None,
        }
    }
}

crate::default_style!(AxumExtractorKind => AxumExtractorKindStyle);

impl Elicitation for AxumExtractorKind {
    type Style = AxumExtractorKindStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumExtractorKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose extractor kind:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid AxumExtractorKind: {}", label)))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("AxumExtractorKind", "Path")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("AxumExtractorKind", "Path")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("AxumExtractorKind", "Path")
    }
}

impl ElicitIntrospect for AxumExtractorKind {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumExtractorKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for AxumExtractorKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose extractor kind:").to_string(),
            type_name: "AxumExtractorKind".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- AxumResponseKind --------------------------------------------------------

impl Prompt for AxumResponseKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the axum response kind:")
    }
}

impl Select for AxumResponseKind {
    fn options() -> Vec<Self> {
        vec![
            AxumResponseKind::Json, AxumResponseKind::Html, AxumResponseKind::Redirect,
            AxumResponseKind::NoContent, AxumResponseKind::Status, AxumResponseKind::Custom,
        ]
    }
    fn labels() -> Vec<String> {
        vec!["Json","Html","Redirect","NoContent","Status","Custom"]
            .into_iter().map(|s| s.to_string()).collect()
    }
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Json" => Some(AxumResponseKind::Json),
            "Html" => Some(AxumResponseKind::Html),
            "Redirect" => Some(AxumResponseKind::Redirect),
            "NoContent" => Some(AxumResponseKind::NoContent),
            "Status" => Some(AxumResponseKind::Status),
            "Custom" => Some(AxumResponseKind::Custom),
            _ => None,
        }
    }
}

crate::default_style!(AxumResponseKind => AxumResponseKindStyle);

impl Elicitation for AxumResponseKind {
    type Style = AxumResponseKindStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AxumResponseKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose response kind:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid AxumResponseKind: {}", label)))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("AxumResponseKind", "Json")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("AxumResponseKind", "Json")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("AxumResponseKind", "Json")
    }
}

impl ElicitIntrospect for AxumResponseKind {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::AxumResponseKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for AxumResponseKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose response kind:").to_string(),
            type_name: "AxumResponseKind".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}
