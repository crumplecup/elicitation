//! `tower::limit::Rate` elicitation — serializable config trenchcoat.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Serializable mirror for `tower::limit::Rate`.
///
/// Stores the rate as `num` requests per `per_millis` milliseconds so the
/// fields remain plain `u64` values that satisfy all `ElicitComplete` bounds.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerRate {
    /// Number of requests allowed in the time window.
    pub num: u64,
    /// Duration of the time window in milliseconds.
    pub per_millis: u64,
}

impl From<TowerRate> for tower::limit::rate::Rate {
    fn from(r: TowerRate) -> Self {
        tower::limit::rate::Rate::new(r.num, std::time::Duration::from_millis(r.per_millis))
    }
}

crate::default_style!(TowerRate => TowerRateStyle);

impl Prompt for TowerRate {
    fn prompt() -> Option<&'static str> {
        Some("Configure a rate limit (requests per time window):")
    }
}

impl Elicitation for TowerRate {
    type Style = TowerRateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerRate");
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

impl ElicitIntrospect for TowerRate {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::limit::Rate",
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
                        prompt: Some("Window duration (ms):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerRate {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerRate".to_string(),
            fields: vec![
                ("num".to_string(), Box::new(u64::prompt_tree())),
                ("per_millis".to_string(), Box::new(u64::prompt_tree())),
            ],
        }
    }
}
