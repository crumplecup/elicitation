//! Bevy 0.18 camera projection type elicitation.
//!
//! Covers:
//! - [`BevyScalingMode`] — owned trenchcoat for `bevy::camera::ScalingMode`
//!   (several variants carry data fields, so the full owned-enum pattern is used).
//! - [`BevyOrthographicProjection`] — struct trenchcoat for
//!   `bevy::camera::OrthographicProjection` (scalar fields only).
//! - [`BevyPerspectiveProjection`] — struct trenchcoat for
//!   `bevy::camera::PerspectiveProjection` (fov, aspect_ratio, near, far).

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyScalingMode ───────────────────────────────────────────────────────────

/// Internal variant-selection enum for [`BevyScalingMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BevyScalingModeKind {
    WindowSize,
    Fixed,
    AutoMin,
    AutoMax,
    FixedVertical,
    FixedHorizontal,
}

impl Prompt for BevyScalingModeKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose an orthographic scaling mode:")
    }
}

impl Select for BevyScalingModeKind {
    fn options() -> Vec<Self> {
        vec![
            Self::WindowSize,
            Self::Fixed,
            Self::AutoMin,
            Self::AutoMax,
            Self::FixedVertical,
            Self::FixedHorizontal,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "WindowSize".to_string(),
            "Fixed".to_string(),
            "AutoMin".to_string(),
            "AutoMax".to_string(),
            "FixedVertical".to_string(),
            "FixedHorizontal".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "WindowSize" => Some(Self::WindowSize),
            "Fixed" => Some(Self::Fixed),
            "AutoMin" => Some(Self::AutoMin),
            "AutoMax" => Some(Self::AutoMax),
            "FixedVertical" => Some(Self::FixedVertical),
            "FixedHorizontal" => Some(Self::FixedHorizontal),
            _ => None,
        }
    }
}

/// Owned trenchcoat for [`bevy::camera::ScalingMode`].
///
/// Several variants carry numeric data fields, requiring the full owned-enum
/// pattern.  Serde uses `#[serde(tag = "mode")]` for clean JSON representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode")]
pub enum BevyScalingMode {
    /// Match the viewport size (1 world unit = 1 pixel at scale 1.0).
    WindowSize,
    /// Fixed world-space size regardless of viewport; stretches to fit.
    Fixed {
        /// World-space width of the viewport.
        width: f32,
        /// World-space height of the viewport.
        height: f32,
    },
    /// Preserve aspect ratio; neither axis shrinks below the minimum.
    AutoMin {
        /// Minimum world-space width.
        min_width: f32,
        /// Minimum world-space height.
        min_height: f32,
    },
    /// Preserve aspect ratio; neither axis grows above the maximum.
    AutoMax {
        /// Maximum world-space width.
        max_width: f32,
        /// Maximum world-space height.
        max_height: f32,
    },
    /// Keep height constant; width adjusts to match aspect ratio.
    FixedVertical {
        /// Desired world-space viewport height.
        viewport_height: f32,
    },
    /// Keep width constant; height adjusts to match aspect ratio.
    FixedHorizontal {
        /// Desired world-space viewport width.
        viewport_width: f32,
    },
}

impl From<bevy::camera::ScalingMode> for BevyScalingMode {
    fn from(s: bevy::camera::ScalingMode) -> Self {
        match s {
            bevy::camera::ScalingMode::WindowSize => Self::WindowSize,
            bevy::camera::ScalingMode::Fixed { width, height } => Self::Fixed { width, height },
            bevy::camera::ScalingMode::AutoMin {
                min_width,
                min_height,
            } => Self::AutoMin {
                min_width,
                min_height,
            },
            bevy::camera::ScalingMode::AutoMax {
                max_width,
                max_height,
            } => Self::AutoMax {
                max_width,
                max_height,
            },
            bevy::camera::ScalingMode::FixedVertical { viewport_height } => {
                Self::FixedVertical { viewport_height }
            }
            bevy::camera::ScalingMode::FixedHorizontal { viewport_width } => {
                Self::FixedHorizontal { viewport_width }
            }
        }
    }
}

impl From<BevyScalingMode> for bevy::camera::ScalingMode {
    fn from(b: BevyScalingMode) -> Self {
        use bevy::camera::ScalingMode as S;
        match b {
            BevyScalingMode::WindowSize => S::WindowSize,
            BevyScalingMode::Fixed { width, height } => S::Fixed { width, height },
            BevyScalingMode::AutoMin {
                min_width,
                min_height,
            } => S::AutoMin {
                min_width,
                min_height,
            },
            BevyScalingMode::AutoMax {
                max_width,
                max_height,
            } => S::AutoMax {
                max_width,
                max_height,
            },
            BevyScalingMode::FixedVertical { viewport_height } => {
                S::FixedVertical { viewport_height }
            }
            BevyScalingMode::FixedHorizontal { viewport_width } => {
                S::FixedHorizontal { viewport_width }
            }
        }
    }
}

impl BevyScalingMode {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::camera::ScalingMode {
        self.into()
    }
}

crate::default_style!(BevyScalingMode => BevyScalingModeStyle);

impl Prompt for BevyScalingMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose an orthographic scaling mode:")
    }
}

impl Elicitation for BevyScalingMode {
    type Style = BevyScalingModeStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::camera::ScalingMode"))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(mcp::select_params(
                            BevyScalingModeKind::prompt().unwrap_or("Choose a scaling mode:"),
                            &BevyScalingModeKind::labels(),
                        )),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            match BevyScalingModeKind::from_label(&label) {
                Some(BevyScalingModeKind::WindowSize) => Ok(Self::WindowSize),
                Some(BevyScalingModeKind::Fixed) => Ok(Self::Fixed {
                    width: f32::elicit(communicator).await?,
                    height: f32::elicit(communicator).await?,
                }),
                Some(BevyScalingModeKind::AutoMin) => Ok(Self::AutoMin {
                    min_width: f32::elicit(communicator).await?,
                    min_height: f32::elicit(communicator).await?,
                }),
                Some(BevyScalingModeKind::AutoMax) => Ok(Self::AutoMax {
                    max_width: f32::elicit(communicator).await?,
                    max_height: f32::elicit(communicator).await?,
                }),
                Some(BevyScalingModeKind::FixedVertical) => Ok(Self::FixedVertical {
                    viewport_height: f32::elicit(communicator).await?,
                }),
                Some(BevyScalingModeKind::FixedHorizontal) => Ok(Self::FixedHorizontal {
                    viewport_width: f32::elicit(communicator).await?,
                }),
                None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
            }
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

impl ElicitIntrospect for BevyScalingMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::camera::ScalingMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "WindowSize".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Fixed".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "width",
                                type_name: "f32",
                                prompt: Some("World-space viewport width:"),
                            },
                            FieldInfo {
                                name: "height",
                                type_name: "f32",
                                prompt: Some("World-space viewport height:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "AutoMin".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "min_width",
                                type_name: "f32",
                                prompt: Some("Minimum width:"),
                            },
                            FieldInfo {
                                name: "min_height",
                                type_name: "f32",
                                prompt: Some("Minimum height:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "AutoMax".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "max_width",
                                type_name: "f32",
                                prompt: Some("Maximum width:"),
                            },
                            FieldInfo {
                                name: "max_height",
                                type_name: "f32",
                                prompt: Some("Maximum height:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "FixedVertical".to_string(),
                        fields: vec![FieldInfo {
                            name: "viewport_height",
                            type_name: "f32",
                            prompt: Some("Viewport height (world units):"),
                        }],
                    },
                    VariantMetadata {
                        label: "FixedHorizontal".to_string(),
                        fields: vec![FieldInfo {
                            name: "viewport_width",
                            type_name: "f32",
                            prompt: Some("Viewport width (world units):"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyScalingMode {
    fn prompt_tree() -> crate::PromptTree {
        let two_f32 = || {
            Box::new(crate::PromptTree::Survey {
                prompt: None,
                type_name: "fields".to_string(),
                fields: vec![
                    ("width_or_min".to_string(), Box::new(f32::prompt_tree())),
                    ("height_or_min".to_string(), Box::new(f32::prompt_tree())),
                ],
            })
        };
        let one_f32 = |name: &str| {
            Box::new(crate::PromptTree::Survey {
                prompt: None,
                type_name: name.to_string(),
                fields: vec![(name.to_string(), Box::new(f32::prompt_tree()))],
            })
        };
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a scaling mode:")
                .to_string(),
            type_name: "bevy::camera::ScalingMode".to_string(),
            options: BevyScalingModeKind::labels(),
            branches: vec![
                None,
                Some(two_f32()),
                Some(two_f32()),
                Some(two_f32()),
                Some(one_f32("viewport_height")),
                Some(one_f32("viewport_width")),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyScalingMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            BevyScalingMode::WindowSize => {
                quote::quote! { bevy::camera::ScalingMode::WindowSize }
            }
            BevyScalingMode::Fixed { width, height } => {
                let w = crate::emit_code::ToCodeLiteral::to_code_literal(width);
                let h = crate::emit_code::ToCodeLiteral::to_code_literal(height);
                quote::quote! { bevy::camera::ScalingMode::Fixed { width: #w, height: #h } }
            }
            BevyScalingMode::AutoMin {
                min_width,
                min_height,
            } => {
                let w = crate::emit_code::ToCodeLiteral::to_code_literal(min_width);
                let h = crate::emit_code::ToCodeLiteral::to_code_literal(min_height);
                quote::quote! {
                    bevy::camera::ScalingMode::AutoMin { min_width: #w, min_height: #h }
                }
            }
            BevyScalingMode::AutoMax {
                max_width,
                max_height,
            } => {
                let w = crate::emit_code::ToCodeLiteral::to_code_literal(max_width);
                let h = crate::emit_code::ToCodeLiteral::to_code_literal(max_height);
                quote::quote! {
                    bevy::camera::ScalingMode::AutoMax { max_width: #w, max_height: #h }
                }
            }
            BevyScalingMode::FixedVertical { viewport_height } => {
                let h = crate::emit_code::ToCodeLiteral::to_code_literal(viewport_height);
                quote::quote! {
                    bevy::camera::ScalingMode::FixedVertical { viewport_height: #h }
                }
            }
            BevyScalingMode::FixedHorizontal { viewport_width } => {
                let w = crate::emit_code::ToCodeLiteral::to_code_literal(viewport_width);
                quote::quote! {
                    bevy::camera::ScalingMode::FixedHorizontal { viewport_width: #w }
                }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::camera::ScalingMode }
    }
}

// ── BevyPerspectiveProjection ─────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::camera::PerspectiveProjection`].
///
/// Only the core numeric fields are included. The `near_clip_plane` (`Vec4`)
/// field is excluded to avoid pulling in vector type dependencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyPerspectiveProjection {
    /// Vertical field of view in radians (e.g. π/4 ≈ 0.785 for 45°).
    pub fov: f32,
    /// Aspect ratio (width / height).
    pub aspect_ratio: f32,
    /// Near clipping plane distance in world units.
    pub near: f32,
    /// Far clipping plane distance in world units.
    pub far: f32,
}

impl From<&bevy::camera::PerspectiveProjection> for BevyPerspectiveProjection {
    fn from(p: &bevy::camera::PerspectiveProjection) -> Self {
        Self {
            fov: p.fov,
            aspect_ratio: p.aspect_ratio,
            near: p.near,
            far: p.far,
        }
    }
}

impl From<BevyPerspectiveProjection> for bevy::camera::PerspectiveProjection {
    fn from(b: BevyPerspectiveProjection) -> Self {
        let mut p = bevy::camera::PerspectiveProjection::default();
        p.fov = b.fov;
        p.aspect_ratio = b.aspect_ratio;
        p.near = b.near;
        p.far = b.far;
        p
    }
}

impl BevyPerspectiveProjection {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::camera::PerspectiveProjection {
        self.into()
    }
}

crate::default_style!(BevyPerspectiveProjection => BevyPerspectiveProjectionStyle);

impl Prompt for BevyPerspectiveProjection {
    fn prompt() -> Option<&'static str> {
        Some("Configure perspective camera projection:")
    }
}

impl Elicitation for BevyPerspectiveProjection {
    type Style = BevyPerspectiveProjectionStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::camera::PerspectiveProjection")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            fov: f32::elicit(communicator).await?,
            aspect_ratio: f32::elicit(communicator).await?,
            near: f32::elicit(communicator).await?,
            far: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyPerspectiveProjection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::camera::PerspectiveProjection",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "fov",
                        type_name: "f32",
                        prompt: Some("Vertical FOV in radians (e.g. 0.785 for 45°):"),
                    },
                    FieldInfo {
                        name: "aspect_ratio",
                        type_name: "f32",
                        prompt: Some("Aspect ratio (width / height):"),
                    },
                    FieldInfo {
                        name: "near",
                        type_name: "f32",
                        prompt: Some("Near clipping plane distance:"),
                    },
                    FieldInfo {
                        name: "far",
                        type_name: "f32",
                        prompt: Some("Far clipping plane distance:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyPerspectiveProjection {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::camera::PerspectiveProjection".to_string(),
            fields: vec![
                ("fov".to_string(), Box::new(f32::prompt_tree())),
                ("aspect_ratio".to_string(), Box::new(f32::prompt_tree())),
                ("near".to_string(), Box::new(f32::prompt_tree())),
                ("far".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPerspectiveProjection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let fov = crate::emit_code::ToCodeLiteral::to_code_literal(&self.fov);
        let ar = crate::emit_code::ToCodeLiteral::to_code_literal(&self.aspect_ratio);
        let near = crate::emit_code::ToCodeLiteral::to_code_literal(&self.near);
        let far = crate::emit_code::ToCodeLiteral::to_code_literal(&self.far);
        quote::quote! {
            bevy::camera::PerspectiveProjection {
                fov: #fov,
                aspect_ratio: #ar,
                near: #near,
                far: #far,
                ..bevy::camera::PerspectiveProjection::default()
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::camera::PerspectiveProjection }
    }
}

// ── BevyOrthographicProjection ────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::camera::OrthographicProjection`].
///
/// Includes the numeric scalar fields.  The `area` (`Rect`) and
/// `viewport_origin` (`Vec2`) fields are excluded to avoid pulling in
/// geometry type dependencies.  `scaling_mode` is represented as
/// [`BevyScalingMode`] for full fidelity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyOrthographicProjection {
    /// Near clipping plane distance in world units.
    pub near: f32,
    /// Far clipping plane distance in world units.
    pub far: f32,
    /// Scale factor applied on top of the scaling mode.
    pub scale: f32,
    /// How the projection scales with viewport size changes.
    pub scaling_mode: BevyScalingMode,
}

impl From<&bevy::camera::OrthographicProjection> for BevyOrthographicProjection {
    fn from(p: &bevy::camera::OrthographicProjection) -> Self {
        Self {
            near: p.near,
            far: p.far,
            scale: p.scale,
            scaling_mode: BevyScalingMode::from(p.scaling_mode.clone()),
        }
    }
}

impl From<BevyOrthographicProjection> for bevy::camera::OrthographicProjection {
    fn from(b: BevyOrthographicProjection) -> Self {
        let mut p = bevy::camera::OrthographicProjection::default_2d();
        p.near = b.near;
        p.far = b.far;
        p.scale = b.scale;
        p.scaling_mode = b.scaling_mode.into();
        p
    }
}

impl BevyOrthographicProjection {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::camera::OrthographicProjection {
        self.into()
    }
}

crate::default_style!(BevyOrthographicProjection => BevyOrthographicProjectionStyle);

impl Prompt for BevyOrthographicProjection {
    fn prompt() -> Option<&'static str> {
        Some("Configure orthographic camera projection:")
    }
}

impl Elicitation for BevyOrthographicProjection {
    type Style = BevyOrthographicProjectionStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::camera::OrthographicProjection")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            near: f32::elicit(communicator).await?,
            far: f32::elicit(communicator).await?,
            scale: f32::elicit(communicator).await?,
            scaling_mode: BevyScalingMode::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyOrthographicProjection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::camera::OrthographicProjection",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "near",
                        type_name: "f32",
                        prompt: Some("Near clipping plane distance:"),
                    },
                    FieldInfo {
                        name: "far",
                        type_name: "f32",
                        prompt: Some("Far clipping plane distance:"),
                    },
                    FieldInfo {
                        name: "scale",
                        type_name: "f32",
                        prompt: Some("Scale factor (higher = smaller apparent objects):"),
                    },
                    FieldInfo {
                        name: "scaling_mode",
                        type_name: "BevyScalingMode",
                        prompt: Some("Scaling mode:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyOrthographicProjection {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::camera::OrthographicProjection".to_string(),
            fields: vec![
                ("near".to_string(), Box::new(f32::prompt_tree())),
                ("far".to_string(), Box::new(f32::prompt_tree())),
                ("scale".to_string(), Box::new(f32::prompt_tree())),
                (
                    "scaling_mode".to_string(),
                    Box::new(BevyScalingMode::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyOrthographicProjection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let near = crate::emit_code::ToCodeLiteral::to_code_literal(&self.near);
        let far = crate::emit_code::ToCodeLiteral::to_code_literal(&self.far);
        let scale = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale);
        let sm = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scaling_mode);
        quote::quote! {
            {
                let mut proj = bevy::camera::OrthographicProjection::default_2d();
                proj.near = #near;
                proj.far = #far;
                proj.scale = #scale;
                proj.scaling_mode = #sm;
                proj
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::camera::OrthographicProjection }
    }
}
