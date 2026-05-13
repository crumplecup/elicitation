//! Retry backoff config trenchcoats for tower.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── TowerExponentialBackoffMaker ─────────────────────────────────────────────

/// Serializable factory for [`tower::retry::backoff::ExponentialBackoffMaker`].
///
/// The concrete type is `ExponentialBackoffMaker<HasherRng>`. Duration fields
/// are stored as millisecond `u64` values; `jitter` is a `f64` in `[0.0, 100.0]`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerExponentialBackoffMaker {
    /// Minimum backoff duration in milliseconds.
    pub min_millis: u64,
    /// Maximum backoff duration in milliseconds.
    pub max_millis: u64,
    /// Jitter factor in `[0.0, 100.0]`.
    pub jitter: f64,
}

crate::default_style!(TowerExponentialBackoffMaker => TowerExponentialBackoffMakerStyle);

impl Prompt for TowerExponentialBackoffMaker {
    fn prompt() -> Option<&'static str> {
        Some("Configure exponential backoff (min/max ms + jitter %):")
    }
}

impl Elicitation for TowerExponentialBackoffMaker {
    type Style = TowerExponentialBackoffMakerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerExponentialBackoffMaker");
        let min_millis = u64::elicit(communicator).await?;
        let max_millis = u64::elicit(communicator).await?;
        let jitter = f64::elicit(communicator).await?;
        Ok(Self {
            min_millis,
            max_millis,
            jitter,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::kani_proof();
        ts.extend(<u64 as Elicitation>::kani_proof());
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::verus_proof();
        ts.extend(<u64 as Elicitation>::verus_proof());
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::creusot_proof();
        ts.extend(<u64 as Elicitation>::creusot_proof());
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerExponentialBackoffMaker {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::retry::backoff::ExponentialBackoffMaker",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "min_millis",
                        type_name: "u64",
                        prompt: Some("Min backoff (ms):"),
                    },
                    FieldInfo {
                        name: "max_millis",
                        type_name: "u64",
                        prompt: Some("Max backoff (ms):"),
                    },
                    FieldInfo {
                        name: "jitter",
                        type_name: "f64",
                        prompt: Some("Jitter factor [0..100]:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerExponentialBackoffMaker {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerExponentialBackoffMaker".to_string(),
            fields: vec![
                ("min_millis".to_string(), Box::new(u64::prompt_tree())),
                ("max_millis".to_string(), Box::new(u64::prompt_tree())),
                ("jitter".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

// ── TowerTpsBudget ───────────────────────────────────────────────────────────

/// Serializable factory config for [`tower::retry::budget::TpsBudget`].
///
/// `TpsBudget` is a runtime-state type (holds atomics + mutex). This struct
/// captures the constructor parameters so the budget can be created on demand.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerTpsBudget {
    /// TTL duration in milliseconds.
    pub ttl_millis: u64,
    /// Minimum number of retries allowed per second regardless of error rate.
    pub min_per_sec: u32,
    /// Ratio of retries to original requests (e.g. `0.1` = 10%).
    pub retry_percent: f32,
}

crate::default_style!(TowerTpsBudget => TowerTpsBudgetStyle);

impl Prompt for TowerTpsBudget {
    fn prompt() -> Option<&'static str> {
        Some("Configure TPS retry budget (TTL, minimum/second, retry ratio):")
    }
}

impl Elicitation for TowerTpsBudget {
    type Style = TowerTpsBudgetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerTpsBudget");
        let ttl_millis = u64::elicit(communicator).await?;
        let min_per_sec = u32::elicit(communicator).await?;
        let retry_percent = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        Ok(Self {
            ttl_millis,
            min_per_sec,
            retry_percent,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::kani_proof();
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts.extend(<f32 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::verus_proof();
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts.extend(<f32 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u64 as Elicitation>::creusot_proof();
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts.extend(<f32 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TowerTpsBudget {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::retry::budget::TpsBudget",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "ttl_millis",
                        type_name: "u64",
                        prompt: Some("TTL (ms):"),
                    },
                    FieldInfo {
                        name: "min_per_sec",
                        type_name: "u32",
                        prompt: Some("Min retries/sec:"),
                    },
                    FieldInfo {
                        name: "retry_percent",
                        type_name: "f32",
                        prompt: Some("Retry ratio [0.0..1.0]:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerTpsBudget {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerTpsBudget".to_string(),
            fields: vec![
                ("ttl_millis".to_string(), Box::new(u64::prompt_tree())),
                ("min_per_sec".to_string(), Box::new(u32::prompt_tree())),
                ("retry_percent".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
