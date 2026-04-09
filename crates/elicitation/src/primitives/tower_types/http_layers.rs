//! tower-http config layer trenchcoats.
//!
//! Serializable factory structs for all public `tower_http` layer types.
//! Complex generic layers use the factory pattern — fields capture
//! constructor parameters, and the shadow crate instantiates the concrete
//! type when building a `ServiceBuilder` stack.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── TowerNormalizePathLayer ──────────────────────────────────────────────────

/// Serializable mirror for [`tower_http::normalize_path::NormalizePathLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerNormalizePathLayer {
    /// `true` = trim trailing slashes (`/foo/` → `/foo`);
    /// `false` = append trailing slashes (`/foo` → `/foo/`).
    pub trim: bool,
}

crate::default_style!(TowerNormalizePathLayer => TowerNormalizePathLayerStyle);

impl Prompt for TowerNormalizePathLayer {
    fn prompt() -> Option<&'static str> {
        Some("Normalize path — trim trailing slashes? (true = trim, false = append):")
    }
}

impl Elicitation for TowerNormalizePathLayer {
    type Style = TowerNormalizePathLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerNormalizePathLayer");
        let trim = bool::elicit(communicator).await?;
        Ok(Self { trim })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerNormalizePathLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::normalize_path::NormalizePathLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "trim",
                    type_name: "bool",
                    prompt: Some("Trim trailing slashes?"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerNormalizePathLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerNormalizePathLayer".to_string(),
            fields: vec![("trim".to_string(), Box::new(bool::prompt_tree()))],
        }
    }
}

// ── TowerPropagateHeaderLayer ────────────────────────────────────────────────

/// Serializable mirror for [`tower_http::propagate_header::PropagateHeaderLayer`].
///
/// Propagates a named request header to the response.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerPropagateHeaderLayer {
    /// HTTP header name to propagate (e.g. `"x-request-id"`).
    pub header: String,
}

crate::default_style!(TowerPropagateHeaderLayer => TowerPropagateHeaderLayerStyle);

impl Prompt for TowerPropagateHeaderLayer {
    fn prompt() -> Option<&'static str> {
        Some("Header name to propagate from request to response:")
    }
}

impl Elicitation for TowerPropagateHeaderLayer {
    type Style = TowerPropagateHeaderLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerPropagateHeaderLayer");
        let header = String::elicit(communicator).await?;
        Ok(Self { header })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerPropagateHeaderLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::propagate_header::PropagateHeaderLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "header",
                    type_name: "String",
                    prompt: Some("Header name:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerPropagateHeaderLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerPropagateHeaderLayer".to_string(),
            fields: vec![("header".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

// ── TowerSetStatusLayer ──────────────────────────────────────────────────────

/// Serializable mirror for [`tower_http::set_status::SetStatusLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSetStatusLayer {
    /// HTTP status code to set on every response (e.g. `200`, `404`).
    pub status_code: u16,
}

crate::default_style!(TowerSetStatusLayer => TowerSetStatusLayerStyle);

impl Prompt for TowerSetStatusLayer {
    fn prompt() -> Option<&'static str> {
        Some("HTTP status code to force on all responses:")
    }
}

impl Elicitation for TowerSetStatusLayer {
    type Style = TowerSetStatusLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSetStatusLayer");
        let status_code = u16::elicit(communicator).await?;
        Ok(Self { status_code })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerSetStatusLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::set_status::SetStatusLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "status_code",
                    type_name: "u16",
                    prompt: Some("Status code:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSetStatusLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSetStatusLayer".to_string(),
            fields: vec![("status_code".to_string(), Box::new(u16::prompt_tree()))],
        }
    }
}

// ── TowerSetSensitiveRequestHeadersLayer ─────────────────────────────────────

/// Serializable mirror for [`tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSetSensitiveRequestHeadersLayer {
    /// Header names to mark as sensitive in requests (e.g. `["authorization"]`).
    pub headers: Vec<String>,
}

crate::default_style!(TowerSetSensitiveRequestHeadersLayer => TowerSetSensitiveRequestHeadersLayerStyle);

impl Prompt for TowerSetSensitiveRequestHeadersLayer {
    fn prompt() -> Option<&'static str> {
        Some("Request header names to mark sensitive (comma-separated):")
    }
}

impl Elicitation for TowerSetSensitiveRequestHeadersLayer {
    type Style = TowerSetSensitiveRequestHeadersLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSetSensitiveRequestHeadersLayer");
        let headers = Vec::<String>::elicit(communicator).await?;
        Ok(Self { headers })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerSetSensitiveRequestHeadersLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "headers",
                    type_name: "Vec<String>",
                    prompt: Some("Header names:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSetSensitiveRequestHeadersLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSetSensitiveRequestHeadersLayer".to_string(),
            fields: vec![(
                "headers".to_string(),
                Box::new(Vec::<String>::prompt_tree()),
            )],
        }
    }
}

// ── TowerSetSensitiveResponseHeadersLayer ────────────────────────────────────

/// Serializable mirror for [`tower_http::sensitive_headers::SetSensitiveResponseHeadersLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSetSensitiveResponseHeadersLayer {
    /// Header names to mark as sensitive in responses (e.g. `["set-cookie"]`).
    pub headers: Vec<String>,
}

crate::default_style!(TowerSetSensitiveResponseHeadersLayer => TowerSetSensitiveResponseHeadersLayerStyle);

impl Prompt for TowerSetSensitiveResponseHeadersLayer {
    fn prompt() -> Option<&'static str> {
        Some("Response header names to mark sensitive (comma-separated):")
    }
}

impl Elicitation for TowerSetSensitiveResponseHeadersLayer {
    type Style = TowerSetSensitiveResponseHeadersLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSetSensitiveResponseHeadersLayer");
        let headers = Vec::<String>::elicit(communicator).await?;
        Ok(Self { headers })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <Vec<String> as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerSetSensitiveResponseHeadersLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::sensitive_headers::SetSensitiveResponseHeadersLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "headers",
                    type_name: "Vec<String>",
                    prompt: Some("Header names:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSetSensitiveResponseHeadersLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSetSensitiveResponseHeadersLayer".to_string(),
            fields: vec![(
                "headers".to_string(),
                Box::new(Vec::<String>::prompt_tree()),
            )],
        }
    }
}

// ── TowerCatchPanicLayer ─────────────────────────────────────────────────────

/// Serializable factory for [`tower_http::catch_panic::CatchPanicLayer<DefaultResponseForPanic>`].
///
/// Always uses the default panic handler (returns `500 Internal Server Error`).
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerCatchPanicLayer;

crate::default_style!(TowerCatchPanicLayer => TowerCatchPanicLayerStyle);

impl Prompt for TowerCatchPanicLayer {
    fn prompt() -> Option<&'static str> {
        Some("Add catch-panic layer (returns 500 on handler panics):")
    }
}

impl Elicitation for TowerCatchPanicLayer {
    type Style = TowerCatchPanicLayerStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerCatchPanicLayer (unit)");
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("TowerCatchPanicLayer")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("TowerCatchPanicLayer")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("TowerCatchPanicLayer")
    }
}

impl ElicitIntrospect for TowerCatchPanicLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::catch_panic::CatchPanicLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![] },
        }
    }
}

impl crate::ElicitPromptTree for TowerCatchPanicLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerCatchPanicLayer".to_string(),
            fields: vec![],
        }
    }
}

// ── TowerCompressionLayer ────────────────────────────────────────────────────

/// Serializable factory for [`tower_http::compression::CompressionLayer`].
///
/// Each flag enables the corresponding encoding algorithm. Requires the
/// matching feature flag on the `tower-http` dependency.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerCompressionLayer {
    /// Enable gzip compression.
    pub gzip: bool,
    /// Enable deflate compression.
    pub deflate: bool,
    /// Enable Brotli compression.
    pub br: bool,
    /// Enable Zstandard compression.
    pub zstd: bool,
}

crate::default_style!(TowerCompressionLayer => TowerCompressionLayerStyle);

impl Prompt for TowerCompressionLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure response compression (enable each algorithm):")
    }
}

impl Elicitation for TowerCompressionLayer {
    type Style = TowerCompressionLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerCompressionLayer");
        let gzip = bool::elicit(communicator).await?;
        let deflate = bool::elicit(communicator).await?;
        let br = bool::elicit(communicator).await?;
        let zstd = bool::elicit(communicator).await?;
        Ok(Self {
            gzip,
            deflate,
            br,
            zstd,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::kani_proof();
        ts.extend(<bool as Elicitation>::kani_proof());
        ts.extend(<bool as Elicitation>::kani_proof());
        ts.extend(<bool as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::verus_proof();
        ts.extend(<bool as Elicitation>::verus_proof());
        ts.extend(<bool as Elicitation>::verus_proof());
        ts.extend(<bool as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::creusot_proof();
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerCompressionLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::compression::CompressionLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "gzip",
                        type_name: "bool",
                        prompt: Some("Enable gzip?"),
                    },
                    FieldInfo {
                        name: "deflate",
                        type_name: "bool",
                        prompt: Some("Enable deflate?"),
                    },
                    FieldInfo {
                        name: "br",
                        type_name: "bool",
                        prompt: Some("Enable brotli?"),
                    },
                    FieldInfo {
                        name: "zstd",
                        type_name: "bool",
                        prompt: Some("Enable zstd?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerCompressionLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerCompressionLayer".to_string(),
            fields: vec![
                ("gzip".to_string(), Box::new(bool::prompt_tree())),
                ("deflate".to_string(), Box::new(bool::prompt_tree())),
                ("br".to_string(), Box::new(bool::prompt_tree())),
                ("zstd".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

// ── TowerDecompressionLayer ──────────────────────────────────────────────────

/// Serializable factory for [`tower_http::decompression::DecompressionLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerDecompressionLayer {
    /// Enable gzip decompression.
    pub gzip: bool,
    /// Enable deflate decompression.
    pub deflate: bool,
    /// Enable Brotli decompression.
    pub br: bool,
    /// Enable Zstandard decompression.
    pub zstd: bool,
}

crate::default_style!(TowerDecompressionLayer => TowerDecompressionLayerStyle);

impl Prompt for TowerDecompressionLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure response decompression (enable each algorithm):")
    }
}

impl Elicitation for TowerDecompressionLayer {
    type Style = TowerDecompressionLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerDecompressionLayer");
        let gzip = bool::elicit(communicator).await?;
        let deflate = bool::elicit(communicator).await?;
        let br = bool::elicit(communicator).await?;
        let zstd = bool::elicit(communicator).await?;
        Ok(Self {
            gzip,
            deflate,
            br,
            zstd,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::kani_proof();
        ts.extend(<bool as Elicitation>::kani_proof());
        ts.extend(<bool as Elicitation>::kani_proof());
        ts.extend(<bool as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::verus_proof();
        ts.extend(<bool as Elicitation>::verus_proof());
        ts.extend(<bool as Elicitation>::verus_proof());
        ts.extend(<bool as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as Elicitation>::creusot_proof();
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerDecompressionLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::decompression::DecompressionLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "gzip",
                        type_name: "bool",
                        prompt: Some("Enable gzip?"),
                    },
                    FieldInfo {
                        name: "deflate",
                        type_name: "bool",
                        prompt: Some("Enable deflate?"),
                    },
                    FieldInfo {
                        name: "br",
                        type_name: "bool",
                        prompt: Some("Enable brotli?"),
                    },
                    FieldInfo {
                        name: "zstd",
                        type_name: "bool",
                        prompt: Some("Enable zstd?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerDecompressionLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerDecompressionLayer".to_string(),
            fields: vec![
                ("gzip".to_string(), Box::new(bool::prompt_tree())),
                ("deflate".to_string(), Box::new(bool::prompt_tree())),
                ("br".to_string(), Box::new(bool::prompt_tree())),
                ("zstd".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

// ── TowerHttpTimeoutLayer ────────────────────────────────────────────────────

/// Serializable mirror for [`tower_http::timeout::TimeoutLayer`].
///
/// Note: distinct from `tower::timeout::TimeoutLayer` — this version
/// returns an HTTP `408 Request Timeout` response rather than an error.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerHttpTimeoutLayer {
    /// Request timeout in milliseconds.
    pub timeout_millis: u64,
}

crate::default_style!(TowerHttpTimeoutLayer => TowerHttpTimeoutLayerStyle);

impl Prompt for TowerHttpTimeoutLayer {
    fn prompt() -> Option<&'static str> {
        Some("HTTP request timeout (returns 408 on expiry):")
    }
}

impl Elicitation for TowerHttpTimeoutLayer {
    type Style = TowerHttpTimeoutLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerHttpTimeoutLayer");
        let timeout_millis = u64::elicit(communicator).await?;
        Ok(Self { timeout_millis })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerHttpTimeoutLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::timeout::TimeoutLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "timeout_millis",
                    type_name: "u64",
                    prompt: Some("Timeout (ms):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerHttpTimeoutLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerHttpTimeoutLayer".to_string(),
            fields: vec![("timeout_millis".to_string(), Box::new(u64::prompt_tree()))],
        }
    }
}

// ── TowerTraceLayer ──────────────────────────────────────────────────────────

/// Serializable factory for [`tower_http::trace::TraceLayer`].
///
/// Uses all defaults (`SharedClassifier<ServerErrorsAsFailures>`,
/// `DefaultMakeSpan`, `DefaultOnRequest`, `DefaultOnResponse`, etc.).
/// The shadow crate instantiates with `TraceLayer::new_for_http()`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerTraceLayer;

crate::default_style!(TowerTraceLayer => TowerTraceLayerStyle);

impl Prompt for TowerTraceLayer {
    fn prompt() -> Option<&'static str> {
        Some("Add HTTP tracing layer (uses default span/classifier):")
    }
}

impl Elicitation for TowerTraceLayer {
    type Style = TowerTraceLayerStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerTraceLayer (unit)");
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("TowerTraceLayer")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("TowerTraceLayer")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("TowerTraceLayer")
    }
}

impl ElicitIntrospect for TowerTraceLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::trace::TraceLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![] },
        }
    }
}

impl crate::ElicitPromptTree for TowerTraceLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerTraceLayer".to_string(),
            fields: vec![],
        }
    }
}

// ── TowerCorsLayer ───────────────────────────────────────────────────────────

/// Serializable factory for [`tower_http::cors::CorsLayer`].
///
/// Captures the essential CORS policy parameters. The shadow crate
/// constructs the full `CorsLayer` from these fields.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerCorsLayer {
    /// Allowed origins. Use `["*"]` for a permissive policy.
    pub allow_origins: Vec<String>,
    /// Allowed HTTP methods (e.g. `["GET", "POST"]`).
    pub allow_methods: Vec<String>,
    /// Allowed request headers (e.g. `["content-type", "authorization"]`).
    pub allow_headers: Vec<String>,
    /// Whether credentials (cookies, auth headers) are allowed.
    pub allow_credentials: bool,
    /// Preflight cache duration in seconds (`None` = browser default).
    pub max_age_secs: Option<u64>,
}

crate::default_style!(TowerCorsLayer => TowerCorsLayerStyle);

impl Prompt for TowerCorsLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure CORS policy:")
    }
}

impl Elicitation for TowerCorsLayer {
    type Style = TowerCorsLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerCorsLayer");
        let allow_origins = Vec::<String>::elicit(communicator).await?;
        let allow_methods = Vec::<String>::elicit(communicator).await?;
        let allow_headers = Vec::<String>::elicit(communicator).await?;
        let allow_credentials = bool::elicit(communicator).await?;
        let max_age_secs = Option::<u64>::elicit(communicator).await?;
        Ok(Self {
            allow_origins,
            allow_methods,
            allow_headers,
            allow_credentials,
            max_age_secs,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<String> as Elicitation>::kani_proof();
        ts.extend(<Vec<String> as Elicitation>::kani_proof());
        ts.extend(<Vec<String> as Elicitation>::kani_proof());
        ts.extend(<bool as Elicitation>::kani_proof());
        ts.extend(<Option<u64> as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<String> as Elicitation>::verus_proof();
        ts.extend(<Vec<String> as Elicitation>::verus_proof());
        ts.extend(<Vec<String> as Elicitation>::verus_proof());
        ts.extend(<bool as Elicitation>::verus_proof());
        ts.extend(<Option<u64> as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <Vec<String> as Elicitation>::creusot_proof();
        ts.extend(<Vec<String> as Elicitation>::creusot_proof());
        ts.extend(<Vec<String> as Elicitation>::creusot_proof());
        ts.extend(<bool as Elicitation>::creusot_proof());
        ts.extend(<Option<u64> as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerCorsLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::cors::CorsLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "allow_origins",
                        type_name: "Vec<String>",
                        prompt: Some("Allowed origins:"),
                    },
                    FieldInfo {
                        name: "allow_methods",
                        type_name: "Vec<String>",
                        prompt: Some("Allowed methods:"),
                    },
                    FieldInfo {
                        name: "allow_headers",
                        type_name: "Vec<String>",
                        prompt: Some("Allowed headers:"),
                    },
                    FieldInfo {
                        name: "allow_credentials",
                        type_name: "bool",
                        prompt: Some("Allow credentials?"),
                    },
                    FieldInfo {
                        name: "max_age_secs",
                        type_name: "Option<u64>",
                        prompt: Some("Max-age (secs):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerCorsLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerCorsLayer".to_string(),
            fields: vec![
                (
                    "allow_origins".to_string(),
                    Box::new(Vec::<String>::prompt_tree()),
                ),
                (
                    "allow_methods".to_string(),
                    Box::new(Vec::<String>::prompt_tree()),
                ),
                (
                    "allow_headers".to_string(),
                    Box::new(Vec::<String>::prompt_tree()),
                ),
                (
                    "allow_credentials".to_string(),
                    Box::new(bool::prompt_tree()),
                ),
                (
                    "max_age_secs".to_string(),
                    Box::new(Option::<u64>::prompt_tree()),
                ),
            ],
        }
    }
}

// ── TowerValidateRequestHeaderLayer ─────────────────────────────────────────

/// Serializable factory for [`tower_http::validate_request::ValidateRequestHeaderLayer`].
///
/// Validates that requests carry a specific header with an expected value.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerValidateRequestHeaderLayer {
    /// Header name to validate (e.g. `"authorization"`).
    pub header: String,
    /// Expected header value (e.g. `"Bearer token123"`).
    pub expected_value: String,
}

crate::default_style!(TowerValidateRequestHeaderLayer => TowerValidateRequestHeaderLayerStyle);

impl Prompt for TowerValidateRequestHeaderLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure request header validation:")
    }
}

impl Elicitation for TowerValidateRequestHeaderLayer {
    type Style = TowerValidateRequestHeaderLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerValidateRequestHeaderLayer");
        let header = String::elicit(communicator).await?;
        let expected_value = String::elicit(communicator).await?;
        Ok(Self {
            header,
            expected_value,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::kani_proof();
        ts.extend(<String as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::verus_proof();
        ts.extend(<String as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::creusot_proof();
        ts.extend(<String as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerValidateRequestHeaderLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::validate_request::ValidateRequestHeaderLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "header",
                        type_name: "String",
                        prompt: Some("Header name:"),
                    },
                    FieldInfo {
                        name: "expected_value",
                        type_name: "String",
                        prompt: Some("Expected value:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerValidateRequestHeaderLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerValidateRequestHeaderLayer".to_string(),
            fields: vec![
                ("header".to_string(), Box::new(String::prompt_tree())),
                (
                    "expected_value".to_string(),
                    Box::new(String::prompt_tree()),
                ),
            ],
        }
    }
}

// ── TowerSetRequestHeaderLayer ───────────────────────────────────────────────

/// Serializable factory for [`tower_http::set_header::SetRequestHeaderLayer`].
///
/// Always inserts a static header value (override mode).
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSetRequestHeaderLayer {
    /// Header name (e.g. `"x-api-version"`).
    pub header: String,
    /// Static header value to insert.
    pub value: String,
}

crate::default_style!(TowerSetRequestHeaderLayer => TowerSetRequestHeaderLayerStyle);

impl Prompt for TowerSetRequestHeaderLayer {
    fn prompt() -> Option<&'static str> {
        Some("Set a static request header:")
    }
}

impl Elicitation for TowerSetRequestHeaderLayer {
    type Style = TowerSetRequestHeaderLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSetRequestHeaderLayer");
        let header = String::elicit(communicator).await?;
        let value = String::elicit(communicator).await?;
        Ok(Self { header, value })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::kani_proof();
        ts.extend(<String as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::verus_proof();
        ts.extend(<String as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::creusot_proof();
        ts.extend(<String as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerSetRequestHeaderLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::set_header::SetRequestHeaderLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "header",
                        type_name: "String",
                        prompt: Some("Header name:"),
                    },
                    FieldInfo {
                        name: "value",
                        type_name: "String",
                        prompt: Some("Header value:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSetRequestHeaderLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSetRequestHeaderLayer".to_string(),
            fields: vec![
                ("header".to_string(), Box::new(String::prompt_tree())),
                ("value".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// ── TowerSetResponseHeaderLayer ──────────────────────────────────────────────

/// Serializable factory for [`tower_http::set_header::SetResponseHeaderLayer`].
///
/// Always inserts a static header value (override mode).
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSetResponseHeaderLayer {
    /// Header name (e.g. `"x-content-type-options"`).
    pub header: String,
    /// Static header value to insert.
    pub value: String,
}

crate::default_style!(TowerSetResponseHeaderLayer => TowerSetResponseHeaderLayerStyle);

impl Prompt for TowerSetResponseHeaderLayer {
    fn prompt() -> Option<&'static str> {
        Some("Set a static response header:")
    }
}

impl Elicitation for TowerSetResponseHeaderLayer {
    type Style = TowerSetResponseHeaderLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSetResponseHeaderLayer");
        let header = String::elicit(communicator).await?;
        let value = String::elicit(communicator).await?;
        Ok(Self { header, value })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::kani_proof();
        ts.extend(<String as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::verus_proof();
        ts.extend(<String as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::creusot_proof();
        ts.extend(<String as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerSetResponseHeaderLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http::set_header::SetResponseHeaderLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "header",
                        type_name: "String",
                        prompt: Some("Header name:"),
                    },
                    FieldInfo {
                        name: "value",
                        type_name: "String",
                        prompt: Some("Header value:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSetResponseHeaderLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSetResponseHeaderLayer".to_string(),
            fields: vec![
                ("header".to_string(), Box::new(String::prompt_tree())),
                ("value".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
