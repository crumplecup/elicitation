//! Tower load-balancing descriptor types.
//!
//! Serializable mirrors for `tower::load::PeakEwma`, `tower::load::PendingRequests`,
//! `tower::steer::Steer`, and `tower::balance::p2c::Balance`.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── TowerPeakEwma ─────────────────────────────────────────────────────────────

/// Serializable descriptor for `tower::load::PeakEwma`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerPeakEwma {
    /// Rust identifier or expression for the wrapped service.
    pub service_name: String,
    /// Default RTT estimate in microseconds (used before measurements arrive).
    pub default_rtt_micros: u64,
    /// Decay time constant in nanoseconds for the EWMA filter.
    pub decay_nanos: f64,
}

crate::default_style!(TowerPeakEwma => TowerPeakEwmaStyle);

impl Prompt for TowerPeakEwma {
    fn prompt() -> Option<&'static str> {
        Some("Configure PeakEwma load estimator:")
    }
}

impl Elicitation for TowerPeakEwma {
    type Style = TowerPeakEwmaStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerPeakEwma");
        let service_name = String::elicit(communicator).await?;
        let default_rtt_micros = u64::elicit(communicator).await?;
        let decay_nanos = f64::elicit(communicator).await?;
        Ok(Self {
            service_name,
            default_rtt_micros,
            decay_nanos,
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

impl ElicitIntrospect for TowerPeakEwma {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::load::PeakEwma",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "service_name",
                        type_name: "String",
                        prompt: Some("Inner service identifier:"),
                    },
                    FieldInfo {
                        name: "default_rtt_micros",
                        type_name: "u64",
                        prompt: Some("Default RTT estimate (µs):"),
                    },
                    FieldInfo {
                        name: "decay_nanos",
                        type_name: "f64",
                        prompt: Some("Decay constant (ns):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerPeakEwma {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerPeakEwma".to_string(),
            fields: vec![
                ("service_name".to_string(), Box::new(String::prompt_tree())),
                (
                    "default_rtt_micros".to_string(),
                    Box::new(u64::prompt_tree()),
                ),
                ("decay_nanos".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

// ── TowerPendingRequests ──────────────────────────────────────────────────────

/// Serializable descriptor for `tower::load::PendingRequests`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerPendingRequests {
    /// Rust identifier or expression for the wrapped service.
    pub service_name: String,
}

crate::default_style!(TowerPendingRequests => TowerPendingRequestsStyle);

impl Prompt for TowerPendingRequests {
    fn prompt() -> Option<&'static str> {
        Some("Configure PendingRequests load estimator:")
    }
}

impl Elicitation for TowerPendingRequests {
    type Style = TowerPendingRequestsStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerPendingRequests");
        let service_name = String::elicit(communicator).await?;
        Ok(Self { service_name })
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

impl ElicitIntrospect for TowerPendingRequests {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::load::PendingRequests",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "service_name",
                    type_name: "String",
                    prompt: Some("Inner service identifier:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerPendingRequests {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerPendingRequests".to_string(),
            fields: vec![("service_name".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

// ── TowerSteer ────────────────────────────────────────────────────────────────

/// Serializable descriptor for `tower::steer::Steer`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerSteer {
    /// Rust identifiers for each service in the steering pool.
    pub service_names: Vec<String>,
    /// Rust identifier for the `Picker` implementation.
    pub picker_name: String,
}

crate::default_style!(TowerSteer => TowerSteerStyle);

impl Prompt for TowerSteer {
    fn prompt() -> Option<&'static str> {
        Some("Configure Steer service (pool of services + picker):")
    }
}

impl Elicitation for TowerSteer {
    type Style = TowerSteerStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerSteer");
        let service_names = Vec::<String>::elicit(communicator).await?;
        let picker_name = String::elicit(communicator).await?;
        Ok(Self {
            service_names,
            picker_name,
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

impl ElicitIntrospect for TowerSteer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::steer::Steer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "service_names",
                        type_name: "Vec<String>",
                        prompt: Some("Service identifiers:"),
                    },
                    FieldInfo {
                        name: "picker_name",
                        type_name: "String",
                        prompt: Some("Picker implementation identifier:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerSteer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerSteer".to_string(),
            fields: vec![
                (
                    "service_names".to_string(),
                    Box::new(Vec::<String>::prompt_tree()),
                ),
                ("picker_name".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// ── TowerBalance ──────────────────────────────────────────────────────────────

/// Serializable descriptor for `tower::balance::p2c::Balance`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerBalance {
    /// Rust identifier for the service discovery stream.
    pub discovery_name: String,
    /// Rust type expression for the request type.
    pub req_type: String,
}

crate::default_style!(TowerBalance => TowerBalanceStyle);

impl Prompt for TowerBalance {
    fn prompt() -> Option<&'static str> {
        Some("Configure p2c Balance (power-of-two-choices load balancer):")
    }
}

impl Elicitation for TowerBalance {
    type Style = TowerBalanceStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerBalance");
        let discovery_name = String::elicit(communicator).await?;
        let req_type = String::elicit(communicator).await?;
        Ok(Self {
            discovery_name,
            req_type,
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

impl ElicitIntrospect for TowerBalance {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::balance::p2c::Balance",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "discovery_name",
                        type_name: "String",
                        prompt: Some("Service discovery identifier:"),
                    },
                    FieldInfo {
                        name: "req_type",
                        type_name: "String",
                        prompt: Some("Request type (Rust expression):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerBalance {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerBalance".to_string(),
            fields: vec![
                (
                    "discovery_name".to_string(),
                    Box::new(String::prompt_tree()),
                ),
                ("req_type".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
