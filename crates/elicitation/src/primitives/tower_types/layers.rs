//! Config layer trenchcoats for tower middleware.
//!
//! Each type is a serializable mirror struct for a [`tower`] layer type,
//! storing `Duration` fields as millisecond `u64` values.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── TowerTimeoutLayer ────────────────────────────────────────────────────────

/// Serializable mirror for [`tower::timeout::TimeoutLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerTimeoutLayer {
    /// Request timeout in milliseconds.
    pub timeout_millis: u64,
}

impl From<TowerTimeoutLayer> for tower::timeout::TimeoutLayer {
    fn from(t: TowerTimeoutLayer) -> Self {
        tower::timeout::TimeoutLayer::new(std::time::Duration::from_millis(t.timeout_millis))
    }
}

crate::default_style!(TowerTimeoutLayer => TowerTimeoutLayerStyle);

impl Prompt for TowerTimeoutLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure request timeout:")
    }
}

impl Elicitation for TowerTimeoutLayer {
    type Style = TowerTimeoutLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerTimeoutLayer");
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

impl ElicitIntrospect for TowerTimeoutLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::timeout::TimeoutLayer",
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

impl crate::ElicitPromptTree for TowerTimeoutLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerTimeoutLayer".to_string(),
            fields: vec![("timeout_millis".to_string(), Box::new(u64::prompt_tree()))],
        }
    }
}

// ── TowerConcurrencyLimitLayer ───────────────────────────────────────────────

/// Serializable mirror for [`tower::limit::ConcurrencyLimitLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerConcurrencyLimitLayer {
    /// Maximum number of concurrent in-flight requests.
    pub max: usize,
}

impl From<TowerConcurrencyLimitLayer> for tower::limit::ConcurrencyLimitLayer {
    fn from(c: TowerConcurrencyLimitLayer) -> Self {
        tower::limit::ConcurrencyLimitLayer::new(c.max)
    }
}

crate::default_style!(TowerConcurrencyLimitLayer => TowerConcurrencyLimitLayerStyle);

impl Prompt for TowerConcurrencyLimitLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure concurrency limit:")
    }
}

impl Elicitation for TowerConcurrencyLimitLayer {
    type Style = TowerConcurrencyLimitLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerConcurrencyLimitLayer");
        let max = usize::elicit(communicator).await?;
        Ok(Self { max })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerConcurrencyLimitLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::limit::ConcurrencyLimitLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "max",
                    type_name: "usize",
                    prompt: Some("Max concurrent requests:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerConcurrencyLimitLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerConcurrencyLimitLayer".to_string(),
            fields: vec![("max".to_string(), Box::new(usize::prompt_tree()))],
        }
    }
}

// ── TowerRateLimitLayer ──────────────────────────────────────────────────────

/// Serializable mirror for [`tower::limit::RateLimitLayer`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerRateLimitLayer {
    /// Number of requests allowed in the time window.
    pub num: u64,
    /// Duration of the time window in milliseconds.
    pub per_millis: u64,
}

impl From<TowerRateLimitLayer> for tower::limit::RateLimitLayer {
    fn from(r: TowerRateLimitLayer) -> Self {
        tower::limit::RateLimitLayer::new(r.num, std::time::Duration::from_millis(r.per_millis))
    }
}

crate::default_style!(TowerRateLimitLayer => TowerRateLimitLayerStyle);

impl Prompt for TowerRateLimitLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure rate limit layer:")
    }
}

impl Elicitation for TowerRateLimitLayer {
    type Style = TowerRateLimitLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerRateLimitLayer");
        let num = u64::elicit(communicator).await?;
        let per_millis = u64::elicit(communicator).await?;
        Ok(Self { num, per_millis })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::kani_proof();
        ts.extend(<u64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::verus_proof();
        ts.extend(<u64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::creusot_proof();
        ts.extend(<u64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerRateLimitLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::limit::RateLimitLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "num",
                        type_name: "u64",
                        prompt: Some("Requests per window:"),
                    },
                    FieldInfo {
                        name: "per_millis",
                        type_name: "u64",
                        prompt: Some("Window (ms):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerRateLimitLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerRateLimitLayer".to_string(),
            fields: vec![
                ("num".to_string(), Box::new(u64::prompt_tree())),
                ("per_millis".to_string(), Box::new(u64::prompt_tree())),
            ],
        }
    }
}

// ── TowerBufferLayer ─────────────────────────────────────────────────────────

/// Serializable factory for [`tower::buffer::BufferLayer<Request>`].
///
/// `BufferLayer` is generic over `Request`; this factory captures only the
/// `bound` parameter and constructs the layer for any `Request` type.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerBufferLayer {
    /// Channel capacity — maximum queued requests before backpressure applies.
    pub bound: usize,
}

crate::default_style!(TowerBufferLayer => TowerBufferLayerStyle);

impl Prompt for TowerBufferLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure buffer layer channel capacity:")
    }
}

impl Elicitation for TowerBufferLayer {
    type Style = TowerBufferLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerBufferLayer");
        let bound = usize::elicit(communicator).await?;
        Ok(Self { bound })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <usize as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerBufferLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::buffer::BufferLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "bound",
                    type_name: "usize",
                    prompt: Some("Channel capacity:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerBufferLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerBufferLayer".to_string(),
            fields: vec![("bound".to_string(), Box::new(usize::prompt_tree()))],
        }
    }
}

// ── TowerLoadShedLayer ───────────────────────────────────────────────────────

/// Serializable mirror for [`tower::load_shed::LoadShedLayer`] (unit struct).
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerLoadShedLayer;

impl From<TowerLoadShedLayer> for tower::load_shed::LoadShedLayer {
    fn from(_: TowerLoadShedLayer) -> Self {
        tower::load_shed::LoadShedLayer::new()
    }
}

crate::default_style!(TowerLoadShedLayer => TowerLoadShedLayerStyle);

impl Prompt for TowerLoadShedLayer {
    fn prompt() -> Option<&'static str> {
        Some("Add a load-shedding layer (drops requests when the service is not ready):")
    }
}

impl Elicitation for TowerLoadShedLayer {
    type Style = TowerLoadShedLayerStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerLoadShedLayer (unit)");
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("TowerLoadShedLayer")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("TowerLoadShedLayer")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("TowerLoadShedLayer")
    }
}

impl ElicitIntrospect for TowerLoadShedLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::load_shed::LoadShedLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![] },
        }
    }
}

impl crate::ElicitPromptTree for TowerLoadShedLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerLoadShedLayer".to_string(),
            fields: vec![],
        }
    }
}

// ── TowerSpawnReadyLayer ─────────────────────────────────────────────────────

/// Serializable mirror for [`tower::spawn_ready::SpawnReadyLayer`] (unit struct).
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSpawnReadyLayer;

impl From<TowerSpawnReadyLayer> for tower::spawn_ready::SpawnReadyLayer {
    fn from(_: TowerSpawnReadyLayer) -> Self {
        tower::spawn_ready::SpawnReadyLayer::new()
    }
}

crate::default_style!(TowerSpawnReadyLayer => TowerSpawnReadyLayerStyle);

impl Prompt for TowerSpawnReadyLayer {
    fn prompt() -> Option<&'static str> {
        Some("Add a spawn-ready layer (drives services to readiness on a background task):")
    }
}

impl Elicitation for TowerSpawnReadyLayer {
    type Style = TowerSpawnReadyLayerStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSpawnReadyLayer (unit)");
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_unit_struct("TowerSpawnReadyLayer")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_unit_struct("TowerSpawnReadyLayer")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_unit_struct("TowerSpawnReadyLayer")
    }
}

impl ElicitIntrospect for TowerSpawnReadyLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::spawn_ready::SpawnReadyLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![] },
        }
    }
}

impl crate::ElicitPromptTree for TowerSpawnReadyLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSpawnReadyLayer".to_string(),
            fields: vec![],
        }
    }
}

// ── TowerFilterLayer ─────────────────────────────────────────────────────────

/// Serializable factory config for [`tower::filter::FilterLayer<U>`].
///
/// `FilterLayer` is generic over the predicate `U`. This factory stores a
/// string predicate name; concrete predicate types are registered at runtime
/// in the shadow crate's plugin registry.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerFilterLayer {
    /// Name of the registered predicate type to use.
    pub predicate_name: String,
}

crate::default_style!(TowerFilterLayer => TowerFilterLayerStyle);

impl Prompt for TowerFilterLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure filter layer (specify predicate name):")
    }
}

impl Elicitation for TowerFilterLayer {
    type Style = TowerFilterLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerFilterLayer");
        let predicate_name = String::elicit(communicator).await?;
        Ok(Self { predicate_name })
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

impl ElicitIntrospect for TowerFilterLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::filter::FilterLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "predicate_name",
                    type_name: "String",
                    prompt: Some("Predicate type name:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerFilterLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerFilterLayer".to_string(),
            fields: vec![(
                "predicate_name".to_string(),
                Box::new(String::prompt_tree()),
            )],
        }
    }
}

// ── TowerRetryLayer ──────────────────────────────────────────────────────────

/// Serializable factory config for [`tower::retry::RetryLayer<P>`].
///
/// `RetryLayer` is generic over the retry policy `P`. This factory stores a
/// string policy name; concrete policy types are registered at runtime in the
/// shadow crate's plugin registry.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerRetryLayer {
    /// Name of the registered retry-policy type to use.
    pub policy_name: String,
}

crate::default_style!(TowerRetryLayer => TowerRetryLayerStyle);

impl Prompt for TowerRetryLayer {
    fn prompt() -> Option<&'static str> {
        Some("Configure retry layer (specify policy name):")
    }
}

impl Elicitation for TowerRetryLayer {
    type Style = TowerRetryLayerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerRetryLayer");
        let policy_name = String::elicit(communicator).await?;
        Ok(Self { policy_name })
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

impl ElicitIntrospect for TowerRetryLayer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::retry::RetryLayer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "policy_name",
                    type_name: "String",
                    prompt: Some("Retry policy type name:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerRetryLayer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerRetryLayer".to_string(),
            fields: vec![("policy_name".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}
