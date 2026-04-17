//! Bevy animation type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyRepeatAnimation ───────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::animation::RepeatAnimation`].
///
/// `Count(u32)` carries the number of repetitions; `Never` and `Forever` are unit variants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum BevyRepeatAnimation {
    /// Play once, then stop.
    Never,
    /// Repeat a fixed number of times.
    Count(u32),
    /// Repeat indefinitely.
    Forever,
}

impl From<bevy::animation::RepeatAnimation> for BevyRepeatAnimation {
    fn from(r: bevy::animation::RepeatAnimation) -> Self {
        match r {
            bevy::animation::RepeatAnimation::Never => Self::Never,
            bevy::animation::RepeatAnimation::Count(n) => Self::Count(n),
            bevy::animation::RepeatAnimation::Forever => Self::Forever,
        }
    }
}

impl From<BevyRepeatAnimation> for bevy::animation::RepeatAnimation {
    fn from(r: BevyRepeatAnimation) -> Self {
        match r {
            BevyRepeatAnimation::Never => Self::Never,
            BevyRepeatAnimation::Count(n) => Self::Count(n),
            BevyRepeatAnimation::Forever => Self::Forever,
        }
    }
}

impl Prompt for BevyRepeatAnimation {
    fn prompt() -> Option<&'static str> {
        Some("Animation repeat mode:")
    }
}

crate::default_style!(BevyRepeatAnimation => BevyRepeatAnimationStyle);

impl Elicitation for BevyRepeatAnimation {
    type Style = BevyRepeatAnimationStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyRepeatAnimation"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose repeat mode:"),
                &["Never", "Count", "Forever"],
            );
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(params),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            match label.as_str() {
                "Never" => Ok(Self::Never),
                "Count" => {
                    let n = u32::elicit(communicator).await?;
                    Ok(Self::Count(n))
                }
                "Forever" => Ok(Self::Forever),
                _ => Err(crate::ElicitError::new(
                    crate::ElicitErrorKind::InvalidSelection(label),
                )),
            }
        })
        .await
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u32 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u32 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyRepeatAnimation {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyRepeatAnimation",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Never".into(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Count".into(),
                        fields: vec![FieldInfo {
                            name: "count",
                            type_name: "u32",
                            prompt: Some("Repeat count:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Forever".into(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyRepeatAnimation {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose repeat mode:").to_string(),
            type_name: "BevyRepeatAnimation".to_string(),
            options: vec!["Never".into(), "Count".into(), "Forever".into()],
            branches: vec![
                None,
                Some(Box::new(crate::PromptTree::Leaf {
                    prompt: "Repeat count:".to_string(),
                    type_name: "u32".to_string(),
                })),
                None,
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyRepeatAnimation {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Never => {
                quote::quote! { bevy::animation::RepeatAnimation::Never }
            }
            Self::Count(n) => {
                quote::quote! { bevy::animation::RepeatAnimation::Count(#n) }
            }
            Self::Forever => {
                quote::quote! { bevy::animation::RepeatAnimation::Forever }
            }
        }
    }
}
