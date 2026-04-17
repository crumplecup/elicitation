//! Bevy window type elicitation trenchcoats.
//!
//! Covers:
//! - [`BevyPresentMode`] — select-trenchcoat for `bevy::window::PresentMode`
//! - [`BevyWindowLevel`] — select-trenchcoat for `bevy::window::WindowLevel`
//! - [`BevyWindowTheme`] — select-trenchcoat for `bevy::window::WindowTheme`
//! - [`BevyWindowMode`] — local mirror enum (data variants, Entity excluded)
//! - [`BevyMonitorSelection`] — local mirror enum (Entity variant excluded)
//! - [`BevyWindowResolution`] — survey trenchcoat for `bevy::window::WindowResolution`

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, Survey, TypeMetadata,
    VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Shared helper macro for unit-enum window selects ─────────────────────────

macro_rules! impl_window_select {
    (
        type       = $ty:ty,
        style      = $style:ident,
        prompt     = $prompt:literal,
        kani_var   = $kani_var:literal,
        variants   = [ $($v:expr),+ $(,)? ]
    ) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> { Some($prompt) }
        }

        impl Select for $ty {
            fn options() -> Vec<Self> { vec![$($v),+] }

            fn labels() -> Vec<String> {
                Self::options()
                    .iter()
                    .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
                    .collect()
            }

            fn from_label(label: &str) -> Option<Self> {
                serde_json::from_str(&format!("\"{}\"", label)).ok()
            }
        }

        crate::default_style!($ty => $style);

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                let params = mcp::select_params(
                    Self::prompt().unwrap_or("Choose a value:"),
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
                        "Invalid {}: {label}", stringify!($ty)
                    )))
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_select_wrapper(stringify!($ty), $kani_var)
            }
            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_select_wrapper(stringify!($ty), $kani_var)
            }
            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_select_wrapper(stringify!($ty), $kani_var)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($ty),
                    description: None,
                    details: PatternDetails::Select {
                        variants: Self::labels()
                            .into_iter()
                            .map(|label| VariantMetadata { label, fields: vec![] })
                            .collect(),
                    },
                }
            }
        }
    };
}

// ── PresentMode ───────────────────────────────────────────────────────────────

impl_window_select!(
    type     = bevy::window::PresentMode,
    style    = BevyPresentModeStyle,
    prompt   = "Choose a window presentation mode (VSync, etc.):",
    kani_var = "bevy::window::PresentMode::Fifo",
    variants = [
        Self::AutoVsync,
        Self::AutoNoVsync,
        Self::Fifo,
        Self::FifoRelaxed,
        Self::Immediate,
        Self::Mailbox,
    ]
);

crate::select_trenchcoat!(bevy::window::PresentMode, as BevyPresentMode, serde);
crate::select_trenchcoat_traits!(BevyPresentMode, bevy::window::PresentMode, [copy, eq, hash]);

// ── WindowLevel ───────────────────────────────────────────────────────────────

impl_window_select!(
    type     = bevy::window::WindowLevel,
    style    = BevyWindowLevelStyle,
    prompt   = "Choose a window stacking level:",
    kani_var = "bevy::window::WindowLevel::Normal",
    variants = [
        Self::AlwaysOnBottom,
        Self::Normal,
        Self::AlwaysOnTop,
    ]
);

crate::select_trenchcoat!(bevy::window::WindowLevel, as BevyWindowLevel, serde);
crate::select_trenchcoat_traits!(BevyWindowLevel, bevy::window::WindowLevel, [copy, eq]);

// ── WindowTheme ───────────────────────────────────────────────────────────────

impl_window_select!(
    type     = bevy::window::WindowTheme,
    style    = BevyWindowThemeStyle,
    prompt   = "Choose a window theme:",
    kani_var = "bevy::window::WindowTheme::Light",
    variants = [Self::Light, Self::Dark]
);

crate::select_trenchcoat!(bevy::window::WindowTheme, as BevyWindowTheme, serde);
crate::select_trenchcoat_traits!(BevyWindowTheme, bevy::window::WindowTheme, [copy, eq]);

// ── BevyMonitorSelection ──────────────────────────────────────────────────────
//
// bevy::window::MonitorSelection has an Entity variant that cannot be elicited.
// We mirror the three portable variants (Current, Primary, Index).

/// Mirror of [`bevy::window::MonitorSelection`] without the `Entity` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub enum BevyMonitorSelection {
    /// The monitor the window is currently on.
    Current,
    /// The primary monitor.
    Primary,
    /// A monitor by zero-based index.
    #[serde(rename_all = "camelCase")]
    Index(u32),
}

impl From<BevyMonitorSelection> for bevy::window::MonitorSelection {
    fn from(m: BevyMonitorSelection) -> Self {
        match m {
            BevyMonitorSelection::Current => Self::Current,
            BevyMonitorSelection::Primary => Self::Primary,
            BevyMonitorSelection::Index(i) => Self::Index(i as usize),
        }
    }
}

impl Prompt for BevyMonitorSelection {
    fn prompt() -> Option<&'static str> {
        Some("Choose a monitor:")
    }
}

impl Select for BevyMonitorSelection {
    fn options() -> Vec<Self> {
        vec![Self::Current, Self::Primary]
    }

    fn labels() -> Vec<String> {
        vec!["Current".into(), "Primary".into()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Current" => Some(Self::Current),
            "Primary" => Some(Self::Primary),
            _ => None,
        }
    }
}

crate::default_style!(BevyMonitorSelection => BevyMonitorSelectionStyle);

impl Elicitation for BevyMonitorSelection {
    type Style = BevyMonitorSelectionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose a monitor:"),
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
                "Invalid BevyMonitorSelection: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyMonitorSelection",
            "BevyMonitorSelection::Current",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyMonitorSelection",
            "BevyMonitorSelection::Current",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyMonitorSelection",
            "BevyMonitorSelection::Current",
        )
    }
}

impl ElicitIntrospect for BevyMonitorSelection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyMonitorSelection",
            description: Some("Monitor selection (Entity variant excluded)"),
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

// ── BevyWindowMode ────────────────────────────────────────────────────────────
//
// bevy::window::WindowMode has data variants referencing MonitorSelection and
// VideoModeSelection. We mirror the three modes with BevyMonitorSelection for
// the screen-targeting variants. VideoModeSelection::Specific(VideoMode) is too
// complex to elicit and is omitted — Fullscreen always uses Current mode.

/// Mirror of [`bevy::window::WindowMode`] with simplified monitor/video targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum BevyWindowMode {
    /// Run in a window.
    Windowed,
    /// Borderless fullscreen on the chosen monitor.
    BorderlessFullscreen {
        /// Which monitor to target.
        monitor: BevyMonitorSelection,
    },
    /// Exclusive fullscreen on the chosen monitor (uses current video mode).
    Fullscreen {
        /// Which monitor to target.
        monitor: BevyMonitorSelection,
    },
}

impl From<BevyWindowMode> for bevy::window::WindowMode {
    fn from(m: BevyWindowMode) -> Self {
        match m {
            BevyWindowMode::Windowed => Self::Windowed,
            BevyWindowMode::BorderlessFullscreen { monitor } => {
                Self::BorderlessFullscreen(monitor.into())
            }
            BevyWindowMode::Fullscreen { monitor } => {
                Self::Fullscreen(monitor.into(), bevy::window::VideoModeSelection::Current)
            }
        }
    }
}

impl Prompt for BevyWindowMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose a window mode:")
    }
}

// ── Internal kind enum for BevyWindowMode selection ──────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BevyWindowModeKind {
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

impl Prompt for BevyWindowModeKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a window mode:")
    }
}

impl Select for BevyWindowModeKind {
    fn options() -> Vec<Self> {
        vec![Self::Windowed, Self::BorderlessFullscreen, Self::Fullscreen]
    }

    fn labels() -> Vec<String> {
        vec![
            "Windowed".into(),
            "BorderlessFullscreen".into(),
            "Fullscreen".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Windowed" => Some(Self::Windowed),
            "BorderlessFullscreen" => Some(Self::BorderlessFullscreen),
            "Fullscreen" => Some(Self::Fullscreen),
            _ => None,
        }
    }
}

crate::default_style!(BevyWindowMode => BevyWindowModeStyle);

impl Elicitation for BevyWindowMode {
    type Style = BevyWindowModeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let kind_params = mcp::select_params(
            Self::prompt().unwrap_or("Choose window mode:"),
            &BevyWindowModeKind::labels(),
        );
        let kind_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(kind_params),
            )
            .await?;
        let kind_value = mcp::extract_value(kind_result)?;
        let kind_label = mcp::parse_string(kind_value)?;
        let kind = BevyWindowModeKind::from_label(&kind_label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyWindowMode kind: {kind_label}"
            )))
        })?;

        match kind {
            BevyWindowModeKind::Windowed => Ok(Self::Windowed),
            BevyWindowModeKind::BorderlessFullscreen => {
                let monitor = BevyMonitorSelection::elicit(communicator).await?;
                Ok(Self::BorderlessFullscreen { monitor })
            }
            BevyWindowModeKind::Fullscreen => {
                let monitor = BevyMonitorSelection::elicit(communicator).await?;
                Ok(Self::Fullscreen { monitor })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyWindowMode",
            "BevyWindowMode::Windowed",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyWindowMode",
            "BevyWindowMode::Windowed",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyWindowMode",
            "BevyWindowMode::Windowed",
        )
    }
}

impl ElicitIntrospect for BevyWindowMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyWindowMode",
            description: Some(
                "Window display mode (Windowed, BorderlessFullscreen, or Fullscreen)",
            ),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Windowed".into(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "BorderlessFullscreen".into(),
                        fields: vec![FieldInfo {
                            name: "monitor",
                            prompt: Some("Choose a monitor:"),
                            type_name: "BevyMonitorSelection",
                        }],
                    },
                    VariantMetadata {
                        label: "Fullscreen".into(),
                        fields: vec![FieldInfo {
                            name: "monitor",
                            prompt: Some("Choose a monitor:"),
                            type_name: "BevyMonitorSelection",
                        }],
                    },
                ],
            },
        }
    }
}

// ── BevyWindowResolution ──────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::window::WindowResolution`].
///
/// Stores physical pixel dimensions. Use `width` × `height` in physical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyWindowResolution {
    /// Physical width in pixels.
    pub width: u32,
    /// Physical height in pixels.
    pub height: u32,
}

crate::default_style!(BevyWindowResolution => BevyWindowResolutionStyle);

impl BevyWindowResolution {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::window::WindowResolution {
        self.into()
    }
}

impl From<bevy::window::WindowResolution> for BevyWindowResolution {
    fn from(r: bevy::window::WindowResolution) -> Self {
        Self {
            width: r.physical_width(),
            height: r.physical_height(),
        }
    }
}

impl From<BevyWindowResolution> for bevy::window::WindowResolution {
    fn from(r: BevyWindowResolution) -> Self {
        bevy::window::WindowResolution::new(r.width, r.height)
    }
}

impl Prompt for BevyWindowResolution {
    fn prompt() -> Option<&'static str> {
        Some("Enter window resolution (width × height in pixels):")
    }
}

impl Survey for BevyWindowResolution {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "width",
                prompt: Some("Window width (physical pixels):"),
                type_name: "u32",
            },
            FieldInfo {
                name: "height",
                prompt: Some("Window height (physical pixels):"),
                type_name: "u32",
            },
        ]
    }
}

impl Elicitation for BevyWindowResolution {
    type Style = BevyWindowResolutionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let width = u32::elicit(communicator).await?;
        let height = u32::elicit(communicator).await?;
        Ok(Self { width, height })
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

impl ElicitIntrospect for BevyWindowResolution {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyWindowResolution",
            description: Some("Window resolution in physical pixels"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        prompt: Some("Window width (physical pixels):"),
                        type_name: "u32",
                    },
                    FieldInfo {
                        name: "height",
                        prompt: Some("Window height (physical pixels):"),
                        type_name: "u32",
                    },
                ],
            },
        }
    }
}
