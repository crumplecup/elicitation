//! Polars pipeline descriptor types.
//!
//! Available with the `polars-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::PolarsPipelineOp;

/// A single named step in a polars pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct PolarsPipelineStep {
    /// Step UUID.
    pub step_id: Uuid,
    /// The operation to perform.
    pub op: PolarsPipelineOp,
}

/// Descriptor for a complete polars LazyFrame pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct PolarsPipelineDescriptor {
    /// Pipeline UUID.
    pub pipeline_id: Uuid,
    /// Human-readable pipeline name.
    pub name: String,
    /// Ordered list of pipeline steps.
    pub steps: Vec<PolarsPipelineStep>,
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// --- PolarsPipelineStep ------------------------------------------------------

impl Prompt for PolarsPipelineStep {
    fn prompt() -> Option<&'static str> {
        Some("Describe a single named step in a Polars pipeline:")
    }
}

crate::default_style!(PolarsPipelineStep => PolarsPipelineStepStyle);

impl Elicitation for PolarsPipelineStep {
    type Style = PolarsPipelineStepStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PolarsPipelineStep");
        let step_id = uuid::Uuid::elicit(communicator).await?;
        let op = PolarsPipelineOp::elicit(communicator).await?;
        Ok(Self { step_id, op })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for PolarsPipelineStep {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::PolarsPipelineStep",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "step_id",
                        type_name: "Uuid",
                        prompt: Some("Step UUID:"),
                    },
                    FieldInfo {
                        name: "op",
                        type_name: "PolarsPipelineOp",
                        prompt: Some("Pipeline operation:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for PolarsPipelineStep {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "PolarsPipelineStep".to_string(),
            fields: vec![
                ("step_id".to_string(), Box::new(uuid::Uuid::prompt_tree())),
                ("op".to_string(), Box::new(PolarsPipelineOp::prompt_tree())),
            ],
        }
    }
}

// --- PolarsPipelineDescriptor ------------------------------------------------

impl Prompt for PolarsPipelineDescriptor {
    fn prompt() -> Option<&'static str> {
        Some("Describe a complete Polars LazyFrame pipeline:")
    }
}

crate::default_style!(PolarsPipelineDescriptor => PolarsPipelineDescriptorStyle);

impl Elicitation for PolarsPipelineDescriptor {
    type Style = PolarsPipelineDescriptorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PolarsPipelineDescriptor");
        let pipeline_id = uuid::Uuid::elicit(communicator).await?;
        let name = String::elicit(communicator).await?;
        let steps = Vec::<PolarsPipelineStep>::elicit(communicator).await?;
        Ok(Self {
            pipeline_id,
            name,
            steps,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for PolarsPipelineDescriptor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::PolarsPipelineDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "pipeline_id",
                        type_name: "Uuid",
                        prompt: Some("Pipeline UUID:"),
                    },
                    FieldInfo {
                        name: "name",
                        type_name: "String",
                        prompt: Some("Pipeline name:"),
                    },
                    FieldInfo {
                        name: "steps",
                        type_name: "Vec<PolarsPipelineStep>",
                        prompt: Some("Pipeline steps:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for PolarsPipelineDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "PolarsPipelineDescriptor".to_string(),
            fields: vec![
                (
                    "pipeline_id".to_string(),
                    Box::new(uuid::Uuid::prompt_tree()),
                ),
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("steps".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}
