//! Bevy audio type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyPlaybackMode ──────────────────────────────────────────────────────────
//
// bevy::audio::PlaybackMode derives neither PartialEq nor serde, so we use a
// local mirror enum and provide From conversions.

/// Mirror of [`bevy::audio::PlaybackMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyPlaybackMode {
    /// Play once then stop.
    Once,
    /// Loop forever.
    Loop,
    /// Play once then despawn the entity.
    Despawn,
    /// Play once then remove the AudioPlayer component.
    Remove,
}

impl From<bevy::audio::PlaybackMode> for BevyPlaybackMode {
    fn from(m: bevy::audio::PlaybackMode) -> Self {
        match m {
            bevy::audio::PlaybackMode::Once => Self::Once,
            bevy::audio::PlaybackMode::Loop => Self::Loop,
            bevy::audio::PlaybackMode::Despawn => Self::Despawn,
            bevy::audio::PlaybackMode::Remove => Self::Remove,
        }
    }
}

impl From<BevyPlaybackMode> for bevy::audio::PlaybackMode {
    fn from(m: BevyPlaybackMode) -> Self {
        match m {
            BevyPlaybackMode::Once => Self::Once,
            BevyPlaybackMode::Loop => Self::Loop,
            BevyPlaybackMode::Despawn => Self::Despawn,
            BevyPlaybackMode::Remove => Self::Remove,
        }
    }
}

impl Prompt for BevyPlaybackMode {
    fn prompt() -> Option<&'static str> {
        Some("Audio playback mode:")
    }
}

impl Select for BevyPlaybackMode {
    fn options() -> Vec<Self> {
        vec![Self::Once, Self::Loop, Self::Despawn, Self::Remove]
    }

    fn labels() -> Vec<String> {
        vec![
            "Once".into(),
            "Loop".into(),
            "Despawn".into(),
            "Remove".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Once" => Some(Self::Once),
            "Loop" => Some(Self::Loop),
            "Despawn" => Some(Self::Despawn),
            "Remove" => Some(Self::Remove),
            _ => None,
        }
    }
}

crate::default_style!(BevyPlaybackMode => BevyPlaybackModeStyle);

impl Elicitation for BevyPlaybackMode {
    type Style = BevyPlaybackModeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyPlaybackMode"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose playback mode:"),
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
                "Invalid BevyPlaybackMode: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("BevyPlaybackMode", "Once")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("BevyPlaybackMode", "Once")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("BevyPlaybackMode", "Once")
    }
}

impl ElicitIntrospect for BevyPlaybackMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyPlaybackMode",
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

// ── BevyVolume ────────────────────────────────────────────────────────────────

impl crate::ElicitPromptTree for BevyPlaybackMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Playback mode:").to_string(),
            type_name: "BevyPlaybackMode".to_string(),
            options: Self::labels(),
            branches: vec![None, None, None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPlaybackMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Once => quote::quote! { bevy::audio::PlaybackMode::Once },
            Self::Loop => quote::quote! { bevy::audio::PlaybackMode::Loop },
            Self::Despawn => quote::quote! { bevy::audio::PlaybackMode::Despawn },
            Self::Remove => quote::quote! { bevy::audio::PlaybackMode::Remove },
        }
    }
}

/// Elicitable trenchcoat for [`bevy::audio::Volume`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum BevyVolume {
    /// Linear volume scale (1.0 = normal).
    Linear(f32),
    /// Volume in decibels (0.0 = normal, negative = quieter).
    Decibels(f32),
}

impl From<bevy::audio::Volume> for BevyVolume {
    fn from(v: bevy::audio::Volume) -> Self {
        match v {
            bevy::audio::Volume::Linear(x) => Self::Linear(x),
            bevy::audio::Volume::Decibels(x) => Self::Decibels(x),
        }
    }
}

impl From<BevyVolume> for bevy::audio::Volume {
    fn from(v: BevyVolume) -> Self {
        match v {
            BevyVolume::Linear(x) => Self::Linear(x),
            BevyVolume::Decibels(x) => Self::Decibels(x),
        }
    }
}

impl Prompt for BevyVolume {
    fn prompt() -> Option<&'static str> {
        Some("Audio volume (Linear or Decibels):")
    }
}

crate::default_style!(BevyVolume => BevyVolumeStyle);

impl Elicitation for BevyVolume {
    type Style = BevyVolumeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyVolume"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Volume type:"),
                &["Linear", "Decibels"],
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
                "Linear" => Ok(Self::Linear(f32::elicit(communicator).await?)),
                "Decibels" => Ok(Self::Decibels(f32::elicit(communicator).await?)),
                _ => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
            }
        })
        .await
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyVolume {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyVolume",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Linear".into(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "f32",
                            prompt: Some("Linear volume (1.0 = normal):"),
                        }],
                    },
                    VariantMetadata {
                        label: "Decibels".into(),
                        fields: vec![FieldInfo {
                            name: "value",
                            type_name: "f32",
                            prompt: Some("Volume in dB (0.0 = normal):"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyVolume {
    fn prompt_tree() -> crate::PromptTree {
        let leaf = crate::PromptTree::Leaf {
            prompt: "Volume value:".to_string(),
            type_name: "f32".to_string(),
        };
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Volume type:").to_string(),
            type_name: "BevyVolume".to_string(),
            options: vec!["Linear".into(), "Decibels".into()],
            branches: vec![Some(Box::new(leaf.clone())), Some(Box::new(leaf))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyVolume {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Linear(x) => quote::quote! { bevy::audio::Volume::Linear(#x) },
            Self::Decibels(x) => quote::quote! { bevy::audio::Volume::Decibels(#x) },
        }
    }
}

// ── BevyPlaybackSettings ──────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::audio::PlaybackSettings`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyPlaybackSettings {
    /// Playback mode.
    pub mode: BevyPlaybackMode,
    /// Volume.
    pub volume: BevyVolume,
    /// Playback speed (1.0 = normal).
    pub speed: f32,
    /// Start paused.
    pub paused: bool,
    /// Start muted.
    pub muted: bool,
    /// Enable spatial audio.
    pub spatial: bool,
}

crate::default_style!(BevyPlaybackSettings => BevyPlaybackSettingsStyle);

impl From<BevyPlaybackSettings> for bevy::audio::PlaybackSettings {
    fn from(s: BevyPlaybackSettings) -> Self {
        bevy::audio::PlaybackSettings {
            mode: s.mode.into(),
            volume: s.volume.into(),
            speed: s.speed,
            paused: s.paused,
            muted: s.muted,
            spatial: s.spatial,
            spatial_scale: None,
            start_position: None,
            duration: None,
        }
    }
}

impl Prompt for BevyPlaybackSettings {
    fn prompt() -> Option<&'static str> {
        Some("Audio playback settings:")
    }
}

impl Elicitation for BevyPlaybackSettings {
    type Style = BevyPlaybackSettingsStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyPlaybackSettings"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            mode: BevyPlaybackMode::elicit(communicator).await?,
            volume: BevyVolume::elicit(communicator).await?,
            speed: f32::elicit(communicator).await?,
            paused: bool::elicit(communicator).await?,
            muted: bool::elicit(communicator).await?,
            spatial: bool::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyPlaybackSettings {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyPlaybackSettings",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "mode",
                        type_name: "bevy::audio::PlaybackMode",
                        prompt: Some("Mode:"),
                    },
                    FieldInfo {
                        name: "volume",
                        type_name: "BevyVolume",
                        prompt: Some("Volume:"),
                    },
                    FieldInfo {
                        name: "speed",
                        type_name: "f32",
                        prompt: Some("Speed (1.0 = normal):"),
                    },
                    FieldInfo {
                        name: "paused",
                        type_name: "bool",
                        prompt: Some("Start paused?"),
                    },
                    FieldInfo {
                        name: "muted",
                        type_name: "bool",
                        prompt: Some("Start muted?"),
                    },
                    FieldInfo {
                        name: "spatial",
                        type_name: "bool",
                        prompt: Some("Spatial audio?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyPlaybackSettings {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyPlaybackSettings".to_string(),
            fields: vec![
                (
                    "mode".to_string(),
                    Box::new(BevyPlaybackMode::prompt_tree()),
                ),
                ("volume".to_string(), Box::new(BevyVolume::prompt_tree())),
                ("speed".to_string(), Box::new(f32::prompt_tree())),
                ("paused".to_string(), Box::new(bool::prompt_tree())),
                ("muted".to_string(), Box::new(bool::prompt_tree())),
                ("spatial".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPlaybackSettings {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let mode = crate::emit_code::ToCodeLiteral::to_code_literal(&self.mode);
        let volume = crate::emit_code::ToCodeLiteral::to_code_literal(&self.volume);
        let speed = self.speed;
        let paused = self.paused;
        let muted = self.muted;
        let spatial = self.spatial;
        quote::quote! {
            bevy::audio::PlaybackSettings {
                mode: #mode,
                volume: #volume,
                speed: #speed,
                paused: #paused,
                muted: #muted,
                spatial: #spatial,
                spatial_scale: None,
                start_position: None,
                duration: None,
            }
        }
    }
}
