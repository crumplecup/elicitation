//! Bevy time type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── TimerMode ─────────────────────────────────────────────────────────────────

impl Prompt for bevy::time::TimerMode {
    fn prompt() -> Option<&'static str> {
        Some("Timer mode (Once or Repeating):")
    }
}

impl Select for bevy::time::TimerMode {
    fn options() -> Vec<Self> {
        vec![Self::Once, Self::Repeating]
    }

    fn labels() -> Vec<String> {
        vec!["Once".into(), "Repeating".into()]
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(bevy::time::TimerMode => TimerModeStyle);

impl Elicitation for bevy::time::TimerMode {
    type Style = TimerModeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::time::TimerMode"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose timer mode:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TimerMode: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("bevy::time::TimerMode", "Once")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("bevy::time::TimerMode", "Once")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("bevy::time::TimerMode", "Once")
    }
}

impl ElicitIntrospect for bevy::time::TimerMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::time::TimerMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

crate::select_trenchcoat!(bevy::time::TimerMode, as BevyTimerMode, serde);
crate::select_trenchcoat_traits!(BevyTimerMode, bevy::time::TimerMode, [eq]);

// ── BevyTimer ─────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for a Bevy timer configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyTimer {
    /// Duration in seconds.
    pub duration_secs: f64,
    /// Timer mode.
    pub mode: BevyTimerMode,
    /// Start paused.
    pub paused: bool,
}

crate::default_style!(BevyTimer => BevyTimerStyle);

impl BevyTimer {
    /// Convert into a [`bevy::time::Timer`].
    pub fn into_inner(self) -> bevy::time::Timer {
        self.into()
    }
}

impl From<BevyTimer> for bevy::time::Timer {
    fn from(t: BevyTimer) -> Self {
        let mut timer =
            bevy::time::Timer::from_seconds(t.duration_secs as f32, t.mode.into_inner());
        if t.paused {
            timer.pause();
        }
        timer
    }
}

impl Prompt for BevyTimer {
    fn prompt() -> Option<&'static str> {
        Some("Bevy timer (duration, mode, paused):")
    }
}

impl Elicitation for BevyTimer {
    type Style = BevyTimerStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyTimer"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            duration_secs: f64::elicit(communicator).await?,
            mode: BevyTimerMode::elicit(communicator).await?,
            paused: bool::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f64 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f64 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f64 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyTimer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyTimer",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "duration_secs",
                        type_name: "f64",
                        prompt: Some("Duration (seconds):"),
                    },
                    FieldInfo {
                        name: "mode",
                        type_name: "bevy::time::TimerMode",
                        prompt: Some("Mode:"),
                    },
                    FieldInfo {
                        name: "paused",
                        type_name: "bool",
                        prompt: Some("Start paused?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyTimer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyTimer".to_string(),
            fields: vec![
                ("duration_secs".to_string(), Box::new(f64::prompt_tree())),
                ("mode".to_string(), Box::new(BevyTimerMode::prompt_tree())),
                ("paused".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyTimer {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let d = self.duration_secs as f32;
        let mode = match self.mode.0 {
            bevy::time::TimerMode::Once => quote::quote! { bevy::time::TimerMode::Once },
            bevy::time::TimerMode::Repeating => quote::quote! { bevy::time::TimerMode::Repeating },
        };
        let paused = self.paused;
        quote::quote! {{
            let mut __t = bevy::time::Timer::from_seconds(#d, #mode);
            if #paused { __t.pause(); }
            __t
        }}
    }
}
