//! `TowerLayerKind` discriminated enum and `TowerServiceBuilder` descriptor.
//!
//! These types let agents compose a full tower service as a data structure
//! before any runtime code is generated.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, TypeMetadata, VariantMetadata,
};

// ── TowerLayerKind ────────────────────────────────────────────────────────────

/// Discriminated enum covering all serializable tower layer configurations.
///
/// Use the `"kind"` tag field in JSON to select a variant, e.g.:
/// ```json
/// {"kind": "timeout", "millis": 5000}
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TowerLayerKind {
    /// `tower::limit::ConcurrencyLimitLayer`
    ConcurrencyLimit {
        /// Maximum concurrent requests allowed.
        max: usize,
    },
    /// `tower::limit::RateLimitLayer`
    RateLimit {
        /// Requests allowed per window.
        num: u64,
        /// Window duration in milliseconds.
        per_millis: u64,
    },
    /// `tower::timeout::TimeoutLayer`
    Timeout {
        /// Timeout in milliseconds.
        millis: u64,
    },
    /// `tower::buffer::BufferLayer`
    Buffer {
        /// Channel capacity before backpressure applies.
        bound: usize,
    },
    /// `tower::load_shed::LoadShedLayer`
    LoadShed,
    /// `tower::spawn_ready::SpawnReadyLayer`
    SpawnReady,
    /// `tower::filter::FilterLayer`
    Filter {
        /// Name of the registered predicate type.
        predicate_name: String,
    },
    /// `tower::retry::RetryLayer`
    Retry {
        /// Name of the registered retry policy type.
        policy_name: String,
    },
    /// `tower::util::MapErrLayer`
    MapErr {
        /// Rust identifier for the error mapping fn.
        mapper_fn: String,
    },
    /// `tower::util::MapRequestLayer`
    MapRequest {
        /// Rust identifier for the request mapping fn.
        mapper_fn: String,
    },
    /// `tower::util::MapResponseLayer`
    MapResponse {
        /// Rust identifier for the response mapping fn.
        mapper_fn: String,
    },
    /// `tower::util::MapResultLayer`
    MapResult {
        /// Rust identifier for the result mapping fn.
        mapper_fn: String,
    },
    /// `tower::util::AndThenLayer`
    AndThen {
        /// Rust identifier for the async and_then fn.
        f: String,
    },
    /// `tower::util::ThenLayer`
    Then {
        /// Rust identifier for the async then fn.
        f: String,
    },
    /// `tower::util::MapFutureLayer`
    MapFuture {
        /// Rust identifier for the map_future fn.
        f: String,
    },
}

crate::default_style!(TowerLayerKind => TowerLayerKindStyle);

impl TowerLayerKind {
    fn variant_labels() -> Vec<&'static str> {
        vec![
            "concurrency_limit",
            "rate_limit",
            "timeout",
            "buffer",
            "load_shed",
            "spawn_ready",
            "filter",
            "retry",
            "map_err",
            "map_request",
            "map_response",
            "map_result",
            "and_then",
            "then",
            "map_future",
        ]
    }
}

impl Prompt for TowerLayerKind {
    fn prompt() -> Option<&'static str> {
        Some("Select a tower layer kind (provide as JSON with a `kind` discriminant field):")
    }
}

impl Elicitation for TowerLayerKind {
    type Style = TowerLayerKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerLayerKind");
        let json = String::elicit(communicator).await?;
        serde_json::from_str(&json)
            .map_err(|e| ElicitError::new(ElicitErrorKind::ParseError(e.to_string())))
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

impl ElicitIntrospect for TowerLayerKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "TowerLayerKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::variant_labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label: label.to_string(),
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerLayerKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("").to_string(),
            type_name: "TowerLayerKind".to_string(),
            options: Self::variant_labels()
                .into_iter()
                .map(str::to_string)
                .collect(),
            branches: Self::variant_labels().iter().map(|_| None).collect(),
        }
    }
}

// ── TowerServiceBuilder ───────────────────────────────────────────────────────

/// Descriptor for a composed tower service: an ordered list of layers plus an
/// inner service name.
///
/// Agents build this incrementally via the `tower_builder__*` tools, then
/// retrieve the final descriptor for code generation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerServiceBuilder {
    /// Ordered tower layers (outermost first).
    pub layers: Vec<TowerLayerKind>,
    /// Rust identifier or expression for the inner service.
    pub service_name: String,
}

crate::default_style!(TowerServiceBuilder => TowerServiceBuilderStyle);

impl Prompt for TowerServiceBuilder {
    fn prompt() -> Option<&'static str> {
        Some("Configure a tower service builder (layers + inner service name):")
    }
}

impl Elicitation for TowerServiceBuilder {
    type Style = TowerServiceBuilderStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerServiceBuilder");
        let service_name = String::elicit(communicator).await?;
        let layers = Vec::<TowerLayerKind>::elicit(communicator).await?;
        Ok(Self {
            layers,
            service_name,
        })
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

impl ElicitIntrospect for TowerServiceBuilder {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "TowerServiceBuilder",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "service_name",
                        type_name: "String",
                        prompt: Some("Inner service Rust identifier:"),
                    },
                    FieldInfo {
                        name: "layers",
                        type_name: "Vec<TowerLayerKind>",
                        prompt: Some("Tower layers (outermost first):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerServiceBuilder {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerServiceBuilder".to_string(),
            fields: vec![
                ("service_name".to_string(), Box::new(String::prompt_tree())),
                (
                    "layers".to_string(),
                    Box::new(TowerLayerKind::prompt_tree()),
                ),
            ],
        }
    }
}
