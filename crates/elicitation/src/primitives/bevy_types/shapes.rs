//! Bevy math primitive shape trenchcoats.
//!
//! Covers all 2D and 3D shape primitives in `bevy::math::primitives`.

use super::{
    ray::{BevyDir2, BevyDir3},
    vec::{BevyVec2, BevyVec3},
};
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── Arc2d (shared sub-type for circular arc shapes) ───────────────────────────

/// Elicitable trenchcoat for [`bevy::math::primitives::Arc2d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyArc2d {
    /// Arc radius.
    pub radius: f32,
    /// Half the arc angle in radians.
    pub half_angle: f32,
}

crate::default_style!(BevyArc2d => BevyArc2dStyle);

impl BevyArc2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Arc2d {
        self.into()
    }
}
impl From<bevy::math::primitives::Arc2d> for BevyArc2d {
    fn from(a: bevy::math::primitives::Arc2d) -> Self {
        Self {
            radius: a.radius,
            half_angle: a.half_angle,
        }
    }
}
impl From<BevyArc2d> for bevy::math::primitives::Arc2d {
    fn from(a: BevyArc2d) -> Self {
        bevy::math::primitives::Arc2d {
            radius: a.radius,
            half_angle: a.half_angle,
        }
    }
}
impl Prompt for BevyArc2d {
    fn prompt() -> Option<&'static str> {
        Some("2D arc (radius, half-angle in radians):")
    }
}
impl Elicitation for BevyArc2d {
    type Style = BevyArc2dStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Arc2d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            radius: f32::elicit(communicator).await?,
            half_angle: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyArc2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Arc2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "radius",
                        type_name: "f32",
                        prompt: Some("Radius:"),
                    },
                    FieldInfo {
                        name: "half_angle",
                        type_name: "f32",
                        prompt: Some("Half-angle (radians):"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyArc2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Arc2d".to_string(),
            fields: vec![
                ("radius".to_string(), Box::new(f32::prompt_tree())),
                ("half_angle".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyArc2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
        let ha = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_angle);
        quote::quote! { bevy::math::primitives::Arc2d { radius: #r, half_angle: #ha } }
    }
}

// ── Radius-only shapes ────────────────────────────────────────────────────────

macro_rules! bevy_radius_shape {
    (
        $name:ident,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat for [`", $type_name, "`]."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// Radius.
                pub radius: f32,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream { self.into() }
            }

            impl From<$upstream> for $name {
                fn from(s: $upstream) -> Self { Self { radius: s.radius } }
            }

            impl From<$name> for $upstream {
                fn from(s: $name) -> Self { $upstream { radius: s.radius } }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> { Some($prompt) }
            }

            impl Elicitation for $name {
                type Style = [< $name Style >];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    Ok(Self { radius: f32::elicit(communicator).await? })
                }

                fn kani_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::kani_proof() }
                fn verus_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::verus_proof() }
                fn creusot_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::creusot_proof() }
            }

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: "radius", type_name: "f32", prompt: Some("Radius:") },
                            ],
                        },
                    }
                }
            }

            impl crate::ElicitPromptTree for $name {
                fn prompt_tree() -> crate::PromptTree {
                    crate::PromptTree::Survey {
                        prompt: Self::prompt().map(str::to_string),
                        type_name: $type_name.to_string(),
                        fields: vec![("radius".to_string(), Box::new(f32::prompt_tree()))],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path { radius: #r } }
                }
            }
        }
    };
}

// ── Two-field radius+half_length shapes ───────────────────────────────────────

macro_rules! bevy_capsule_shape {
    (
        $name:ident,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat for [`", $type_name, "`]."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// Capsule end-cap radius.
                pub radius: f32,
                /// Half the length of the cylindrical body.
                pub half_length: f32,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream { self.into() }
            }

            impl From<$upstream> for $name {
                fn from(s: $upstream) -> Self {
                    Self { radius: s.radius, half_length: s.half_length }
                }
            }

            impl From<$name> for $upstream {
                fn from(s: $name) -> Self {
                    $upstream { radius: s.radius, half_length: s.half_length }
                }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> { Some($prompt) }
            }

            impl Elicitation for $name {
                type Style = [< $name Style >];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    Ok(Self {
                        radius: f32::elicit(communicator).await?,
                        half_length: f32::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::kani_proof() }
                fn verus_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::verus_proof() }
                fn creusot_proof() -> proc_macro2::TokenStream { <f32 as Elicitation>::creusot_proof() }
            }

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: "radius", type_name: "f32", prompt: Some("Radius:") },
                                FieldInfo { name: "half_length", type_name: "f32", prompt: Some("Half-length of cylindrical body:") },
                            ],
                        },
                    }
                }
            }

            impl crate::ElicitPromptTree for $name {
                fn prompt_tree() -> crate::PromptTree {
                    crate::PromptTree::Survey {
                        prompt: Self::prompt().map(str::to_string),
                        type_name: $type_name.to_string(),
                        fields: vec![
                            ("radius".to_string(), Box::new(f32::prompt_tree())),
                            ("half_length".to_string(), Box::new(f32::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
                    let hl = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_length);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path { radius: #r, half_length: #hl } }
                }
            }
        }
    };
}

// ── 2D shapes ─────────────────────────────────────────────────────────────────

bevy_radius_shape!(
    BevyCircle,
    bevy::math::primitives::Circle,
    "bevy::math::primitives::Circle",
    "Circle (radius):",
    "bevy::math::primitives::Circle"
);

bevy_capsule_shape!(
    BevyCapsule2d,
    bevy::math::primitives::Capsule2d,
    "bevy::math::primitives::Capsule2d",
    "2D capsule (radius + half-length):",
    "bevy::math::primitives::Capsule2d"
);

/// Elicitable trenchcoat for [`bevy::math::primitives::Ellipse`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyEllipse {
    /// Half-extents along each axis.
    pub half_size: BevyVec2,
}

crate::default_style!(BevyEllipse => BevyEllipseStyle);

impl BevyEllipse {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Ellipse {
        self.into()
    }
}
impl From<bevy::math::primitives::Ellipse> for BevyEllipse {
    fn from(s: bevy::math::primitives::Ellipse) -> Self {
        Self {
            half_size: s.half_size.into(),
        }
    }
}
impl From<BevyEllipse> for bevy::math::primitives::Ellipse {
    fn from(s: BevyEllipse) -> Self {
        bevy::math::primitives::Ellipse {
            half_size: s.half_size.into(),
        }
    }
}
impl Prompt for BevyEllipse {
    fn prompt() -> Option<&'static str> {
        Some("Ellipse (half-size x, y):")
    }
}
impl Elicitation for BevyEllipse {
    type Style = BevyEllipseStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Ellipse")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            half_size: BevyVec2::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyEllipse {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Ellipse",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "half_size",
                    type_name: "BevyVec2",
                    prompt: Some("Half-size:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyEllipse {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Ellipse".to_string(),
            fields: vec![("half_size".to_string(), Box::new(BevyVec2::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyEllipse {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hs = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_size);
        quote::quote! { bevy::math::primitives::Ellipse { half_size: #hs } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Rectangle`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyRectangle {
    /// Half-extents along each axis.
    pub half_size: BevyVec2,
}

crate::default_style!(BevyRectangle => BevyRectangleStyle);

impl BevyRectangle {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Rectangle {
        self.into()
    }
}
impl From<bevy::math::primitives::Rectangle> for BevyRectangle {
    fn from(s: bevy::math::primitives::Rectangle) -> Self {
        Self {
            half_size: s.half_size.into(),
        }
    }
}
impl From<BevyRectangle> for bevy::math::primitives::Rectangle {
    fn from(s: BevyRectangle) -> Self {
        bevy::math::primitives::Rectangle {
            half_size: s.half_size.into(),
        }
    }
}
impl Prompt for BevyRectangle {
    fn prompt() -> Option<&'static str> {
        Some("Rectangle (half-size x, y):")
    }
}
impl Elicitation for BevyRectangle {
    type Style = BevyRectangleStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Rectangle")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            half_size: BevyVec2::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyRectangle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Rectangle",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "half_size",
                    type_name: "BevyVec2",
                    prompt: Some("Half-size:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyRectangle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Rectangle".to_string(),
            fields: vec![("half_size".to_string(), Box::new(BevyVec2::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyRectangle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hs = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_size);
        quote::quote! { bevy::math::primitives::Rectangle { half_size: #hs } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Rhombus`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyRhombus {
    /// Half-lengths of the two diagonals.
    pub half_diagonals: BevyVec2,
}

crate::default_style!(BevyRhombus => BevyRhombusStyle);

impl BevyRhombus {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Rhombus {
        self.into()
    }
}
impl From<bevy::math::primitives::Rhombus> for BevyRhombus {
    fn from(s: bevy::math::primitives::Rhombus) -> Self {
        Self {
            half_diagonals: s.half_diagonals.into(),
        }
    }
}
impl From<BevyRhombus> for bevy::math::primitives::Rhombus {
    fn from(s: BevyRhombus) -> Self {
        bevy::math::primitives::Rhombus {
            half_diagonals: s.half_diagonals.into(),
        }
    }
}
impl Prompt for BevyRhombus {
    fn prompt() -> Option<&'static str> {
        Some("Rhombus (half-diagonals x, y):")
    }
}
impl Elicitation for BevyRhombus {
    type Style = BevyRhombusStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Rhombus")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            half_diagonals: BevyVec2::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyRhombus {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Rhombus",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "half_diagonals",
                    type_name: "BevyVec2",
                    prompt: Some("Half-diagonals:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyRhombus {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Rhombus".to_string(),
            fields: vec![(
                "half_diagonals".to_string(),
                Box::new(BevyVec2::prompt_tree()),
            )],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyRhombus {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hd = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_diagonals);
        quote::quote! { bevy::math::primitives::Rhombus { half_diagonals: #hd } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Annulus`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyAnnulus {
    /// Inner (hole) radius.
    pub inner_radius: f32,
    /// Outer radius.
    pub outer_radius: f32,
}

crate::default_style!(BevyAnnulus => BevyAnnulusStyle);

impl BevyAnnulus {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Annulus {
        self.into()
    }
}
impl From<bevy::math::primitives::Annulus> for BevyAnnulus {
    fn from(s: bevy::math::primitives::Annulus) -> Self {
        Self {
            inner_radius: s.inner_circle.radius,
            outer_radius: s.outer_circle.radius,
        }
    }
}
impl From<BevyAnnulus> for bevy::math::primitives::Annulus {
    fn from(s: BevyAnnulus) -> Self {
        bevy::math::primitives::Annulus::new(s.inner_radius, s.outer_radius)
    }
}
impl Prompt for BevyAnnulus {
    fn prompt() -> Option<&'static str> {
        Some("Annulus (inner radius, outer radius):")
    }
}
impl Elicitation for BevyAnnulus {
    type Style = BevyAnnulusStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Annulus")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            inner_radius: f32::elicit(communicator).await?,
            outer_radius: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyAnnulus {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Annulus",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "inner_radius",
                        type_name: "f32",
                        prompt: Some("Inner radius:"),
                    },
                    FieldInfo {
                        name: "outer_radius",
                        type_name: "f32",
                        prompt: Some("Outer radius:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyAnnulus {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Annulus".to_string(),
            fields: vec![
                ("inner_radius".to_string(), Box::new(f32::prompt_tree())),
                ("outer_radius".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyAnnulus {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let ir = crate::emit_code::ToCodeLiteral::to_code_literal(&self.inner_radius);
        let or_ = crate::emit_code::ToCodeLiteral::to_code_literal(&self.outer_radius);
        quote::quote! { bevy::math::primitives::Annulus { inner_radius: #ir, outer_radius: #or_ } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::RegularPolygon`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyRegularPolygon {
    /// Circumscribed circle radius.
    pub circumradius: f32,
    /// Number of sides (≥ 3).
    pub sides: usize,
}

crate::default_style!(BevyRegularPolygon => BevyRegularPolygonStyle);

impl BevyRegularPolygon {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::RegularPolygon {
        self.into()
    }
}
impl From<bevy::math::primitives::RegularPolygon> for BevyRegularPolygon {
    fn from(s: bevy::math::primitives::RegularPolygon) -> Self {
        Self {
            circumradius: s.circumradius(),
            sides: s.sides as usize,
        }
    }
}
impl From<BevyRegularPolygon> for bevy::math::primitives::RegularPolygon {
    fn from(s: BevyRegularPolygon) -> Self {
        bevy::math::primitives::RegularPolygon::new(s.circumradius, s.sides as u32)
    }
}
impl Prompt for BevyRegularPolygon {
    fn prompt() -> Option<&'static str> {
        Some("Regular polygon (circumradius, sides ≥ 3):")
    }
}
impl Elicitation for BevyRegularPolygon {
    type Style = BevyRegularPolygonStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::RegularPolygon")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            circumradius: f32::elicit(communicator).await?,
            sides: usize::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyRegularPolygon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::RegularPolygon",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "circumradius",
                        type_name: "f32",
                        prompt: Some("Circumradius:"),
                    },
                    FieldInfo {
                        name: "sides",
                        type_name: "usize",
                        prompt: Some("Number of sides:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyRegularPolygon {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::RegularPolygon".to_string(),
            fields: vec![
                ("circumradius".to_string(), Box::new(f32::prompt_tree())),
                ("sides".to_string(), Box::new(usize::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyRegularPolygon {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let cr = crate::emit_code::ToCodeLiteral::to_code_literal(&self.circumradius);
        let s = self.sides;
        quote::quote! {
            bevy::math::primitives::RegularPolygon { circumradius: #cr, sides: #s }
        }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Segment2d`].
///
/// Internally, `Segment2d` stores two vertices; direction and half_length are
/// derived methods.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevySegment2d {
    /// The two endpoints of the segment.
    pub vertices: [BevyVec2; 2],
}

crate::default_style!(BevySegment2d => BevySegment2dStyle);

impl BevySegment2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Segment2d {
        self.into()
    }
}
impl From<bevy::math::primitives::Segment2d> for BevySegment2d {
    fn from(s: bevy::math::primitives::Segment2d) -> Self {
        Self {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl From<BevySegment2d> for bevy::math::primitives::Segment2d {
    fn from(s: BevySegment2d) -> Self {
        bevy::math::primitives::Segment2d {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl Prompt for BevySegment2d {
    fn prompt() -> Option<&'static str> {
        Some("2D segment (two endpoint vertices):")
    }
}
impl Elicitation for BevySegment2d {
    type Style = BevySegment2dStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Segment2d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let v0 = BevyVec2::elicit(communicator).await?;
        let v1 = BevyVec2::elicit(communicator).await?;
        Ok(Self { vertices: [v0, v1] })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevySegment2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Segment2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "vertices[0]",
                        type_name: "BevyVec2",
                        prompt: Some("Vertex 0:"),
                    },
                    FieldInfo {
                        name: "vertices[1]",
                        type_name: "BevyVec2",
                        prompt: Some("Vertex 1:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevySegment2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Segment2d".to_string(),
            fields: vec![
                ("vertices[0]".to_string(), Box::new(BevyVec2::prompt_tree())),
                ("vertices[1]".to_string(), Box::new(BevyVec2::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevySegment2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v0 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[0]);
        let v1 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[1]);
        quote::quote! { bevy::math::primitives::Segment2d { vertices: [#v0, #v1] } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Plane2d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyPlane2d {
    /// The plane normal (unit direction).
    pub normal: BevyDir2,
}

crate::default_style!(BevyPlane2d => BevyPlane2dStyle);

impl BevyPlane2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Plane2d {
        self.into()
    }
}
impl From<bevy::math::primitives::Plane2d> for BevyPlane2d {
    fn from(s: bevy::math::primitives::Plane2d) -> Self {
        Self {
            normal: s.normal.into(),
        }
    }
}
impl From<BevyPlane2d> for bevy::math::primitives::Plane2d {
    fn from(s: BevyPlane2d) -> Self {
        bevy::math::primitives::Plane2d {
            normal: s.normal.into(),
        }
    }
}
impl Prompt for BevyPlane2d {
    fn prompt() -> Option<&'static str> {
        Some("2D plane (normal direction):")
    }
}
impl Elicitation for BevyPlane2d {
    type Style = BevyPlane2dStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Plane2d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            normal: BevyDir2::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyDir2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyDir2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyDir2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyPlane2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Plane2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "normal",
                    type_name: "BevyDir2",
                    prompt: Some("Normal:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyPlane2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Plane2d".to_string(),
            fields: vec![("normal".to_string(), Box::new(BevyDir2::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyPlane2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = crate::emit_code::ToCodeLiteral::to_code_literal(&self.normal);
        quote::quote! { bevy::math::primitives::Plane2d { normal: #n } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Triangle2d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyTriangle2d {
    /// Triangle vertices (CCW winding).
    pub vertices: [BevyVec2; 3],
}

crate::default_style!(BevyTriangle2d => BevyTriangle2dStyle);

impl BevyTriangle2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Triangle2d {
        self.into()
    }
}
impl From<bevy::math::primitives::Triangle2d> for BevyTriangle2d {
    fn from(s: bevy::math::primitives::Triangle2d) -> Self {
        Self {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl From<BevyTriangle2d> for bevy::math::primitives::Triangle2d {
    fn from(s: BevyTriangle2d) -> Self {
        bevy::math::primitives::Triangle2d {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl Prompt for BevyTriangle2d {
    fn prompt() -> Option<&'static str> {
        Some("2D triangle (3 vertices, CCW winding):")
    }
}
impl Elicitation for BevyTriangle2d {
    type Style = BevyTriangle2dStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Triangle2d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let v0 = BevyVec2::elicit(communicator).await?;
        let v1 = BevyVec2::elicit(communicator).await?;
        let v2 = BevyVec2::elicit(communicator).await?;
        Ok(Self {
            vertices: [v0, v1, v2],
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec2 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyTriangle2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Triangle2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "vertices[0]",
                        type_name: "BevyVec2",
                        prompt: Some("Vertex 0:"),
                    },
                    FieldInfo {
                        name: "vertices[1]",
                        type_name: "BevyVec2",
                        prompt: Some("Vertex 1:"),
                    },
                    FieldInfo {
                        name: "vertices[2]",
                        type_name: "BevyVec2",
                        prompt: Some("Vertex 2:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyTriangle2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Triangle2d".to_string(),
            fields: vec![
                ("vertices[0]".to_string(), Box::new(BevyVec2::prompt_tree())),
                ("vertices[1]".to_string(), Box::new(BevyVec2::prompt_tree())),
                ("vertices[2]".to_string(), Box::new(BevyVec2::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyTriangle2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v0 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[0]);
        let v1 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[1]);
        let v2 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[2]);
        quote::quote! {
            bevy::math::primitives::Triangle2d { vertices: [#v0, #v1, #v2] }
        }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::CircularSector`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyCircularSector {
    /// Underlying arc definition.
    pub arc: BevyArc2d,
}

crate::default_style!(BevyCircularSector => BevyCircularSectorStyle);

impl BevyCircularSector {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::CircularSector {
        self.into()
    }
}
impl From<bevy::math::primitives::CircularSector> for BevyCircularSector {
    fn from(s: bevy::math::primitives::CircularSector) -> Self {
        Self { arc: s.arc.into() }
    }
}
impl From<BevyCircularSector> for bevy::math::primitives::CircularSector {
    fn from(s: BevyCircularSector) -> Self {
        bevy::math::primitives::CircularSector { arc: s.arc.into() }
    }
}
impl Prompt for BevyCircularSector {
    fn prompt() -> Option<&'static str> {
        Some("Circular sector (arc definition):")
    }
}
impl Elicitation for BevyCircularSector {
    type Style = BevyCircularSectorStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::CircularSector")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            arc: BevyArc2d::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyCircularSector {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::CircularSector",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "arc",
                    type_name: "BevyArc2d",
                    prompt: Some("Arc:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyCircularSector {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::CircularSector".to_string(),
            fields: vec![("arc".to_string(), Box::new(BevyArc2d::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyCircularSector {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let arc = crate::emit_code::ToCodeLiteral::to_code_literal(&self.arc);
        quote::quote! { bevy::math::primitives::CircularSector { arc: #arc } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::CircularSegment`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyCircularSegment {
    /// Underlying arc definition.
    pub arc: BevyArc2d,
}

crate::default_style!(BevyCircularSegment => BevyCircularSegmentStyle);

impl BevyCircularSegment {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::CircularSegment {
        self.into()
    }
}
impl From<bevy::math::primitives::CircularSegment> for BevyCircularSegment {
    fn from(s: bevy::math::primitives::CircularSegment) -> Self {
        Self { arc: s.arc.into() }
    }
}
impl From<BevyCircularSegment> for bevy::math::primitives::CircularSegment {
    fn from(s: BevyCircularSegment) -> Self {
        bevy::math::primitives::CircularSegment { arc: s.arc.into() }
    }
}
impl Prompt for BevyCircularSegment {
    fn prompt() -> Option<&'static str> {
        Some("Circular segment (arc definition):")
    }
}
impl Elicitation for BevyCircularSegment {
    type Style = BevyCircularSegmentStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::CircularSegment")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            arc: BevyArc2d::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyArc2d as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyCircularSegment {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::CircularSegment",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "arc",
                    type_name: "BevyArc2d",
                    prompt: Some("Arc:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyCircularSegment {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::CircularSegment".to_string(),
            fields: vec![("arc".to_string(), Box::new(BevyArc2d::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyCircularSegment {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let arc = crate::emit_code::ToCodeLiteral::to_code_literal(&self.arc);
        quote::quote! { bevy::math::primitives::CircularSegment { arc: #arc } }
    }
}

// ── 3D shapes ─────────────────────────────────────────────────────────────────

bevy_radius_shape!(
    BevySphere,
    bevy::math::primitives::Sphere,
    "bevy::math::primitives::Sphere",
    "Sphere (radius):",
    "bevy::math::primitives::Sphere"
);

bevy_capsule_shape!(
    BevyCapsule3d,
    bevy::math::primitives::Capsule3d,
    "bevy::math::primitives::Capsule3d",
    "3D capsule (radius + half-length):",
    "bevy::math::primitives::Capsule3d"
);

/// Elicitable trenchcoat for [`bevy::math::primitives::Cuboid`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyCuboid {
    /// Half-extents along each axis.
    pub half_size: BevyVec3,
}

crate::default_style!(BevyCuboid => BevyCuboidStyle);

impl BevyCuboid {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Cuboid {
        self.into()
    }
}
impl From<bevy::math::primitives::Cuboid> for BevyCuboid {
    fn from(s: bevy::math::primitives::Cuboid) -> Self {
        Self {
            half_size: s.half_size.into(),
        }
    }
}
impl From<BevyCuboid> for bevy::math::primitives::Cuboid {
    fn from(s: BevyCuboid) -> Self {
        bevy::math::primitives::Cuboid {
            half_size: s.half_size.into(),
        }
    }
}
impl Prompt for BevyCuboid {
    fn prompt() -> Option<&'static str> {
        Some("Cuboid (half-size x, y, z):")
    }
}
impl Elicitation for BevyCuboid {
    type Style = BevyCuboidStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Cuboid")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            half_size: BevyVec3::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyCuboid {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Cuboid",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "half_size",
                    type_name: "BevyVec3",
                    prompt: Some("Half-size:"),
                }],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyCuboid {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Cuboid".to_string(),
            fields: vec![("half_size".to_string(), Box::new(BevyVec3::prompt_tree()))],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyCuboid {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hs = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_size);
        quote::quote! { bevy::math::primitives::Cuboid { half_size: #hs } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Cylinder`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyCylinder {
    /// Cylinder radius.
    pub radius: f32,
    /// Half the cylinder height.
    pub half_height: f32,
}

crate::default_style!(BevyCylinder => BevyCylinderStyle);

impl BevyCylinder {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Cylinder {
        self.into()
    }
}
impl From<bevy::math::primitives::Cylinder> for BevyCylinder {
    fn from(s: bevy::math::primitives::Cylinder) -> Self {
        Self {
            radius: s.radius,
            half_height: s.half_height,
        }
    }
}
impl From<BevyCylinder> for bevy::math::primitives::Cylinder {
    fn from(s: BevyCylinder) -> Self {
        bevy::math::primitives::Cylinder {
            radius: s.radius,
            half_height: s.half_height,
        }
    }
}
impl Prompt for BevyCylinder {
    fn prompt() -> Option<&'static str> {
        Some("Cylinder (radius, half-height):")
    }
}
impl Elicitation for BevyCylinder {
    type Style = BevyCylinderStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Cylinder")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            radius: f32::elicit(communicator).await?,
            half_height: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyCylinder {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Cylinder",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "radius",
                        type_name: "f32",
                        prompt: Some("Radius:"),
                    },
                    FieldInfo {
                        name: "half_height",
                        type_name: "f32",
                        prompt: Some("Half-height:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyCylinder {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Cylinder".to_string(),
            fields: vec![
                ("radius".to_string(), Box::new(f32::prompt_tree())),
                ("half_height".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyCylinder {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
        let hh = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_height);
        quote::quote! { bevy::math::primitives::Cylinder { radius: #r, half_height: #hh } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Cone`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyCone {
    /// Base radius.
    pub radius: f32,
    /// Cone height.
    pub height: f32,
}

crate::default_style!(BevyCone => BevyConeStyle);

impl BevyCone {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Cone {
        self.into()
    }
}
impl From<bevy::math::primitives::Cone> for BevyCone {
    fn from(s: bevy::math::primitives::Cone) -> Self {
        Self {
            radius: s.radius,
            height: s.height,
        }
    }
}
impl From<BevyCone> for bevy::math::primitives::Cone {
    fn from(s: BevyCone) -> Self {
        bevy::math::primitives::Cone {
            radius: s.radius,
            height: s.height,
        }
    }
}
impl Prompt for BevyCone {
    fn prompt() -> Option<&'static str> {
        Some("Cone (base radius, height):")
    }
}
impl Elicitation for BevyCone {
    type Style = BevyConeStyle;
    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::primitives::Cone"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            radius: f32::elicit(communicator).await?,
            height: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyCone {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Cone",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "radius",
                        type_name: "f32",
                        prompt: Some("Base radius:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "f32",
                        prompt: Some("Height:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyCone {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Cone".to_string(),
            fields: vec![
                ("radius".to_string(), Box::new(f32::prompt_tree())),
                ("height".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyCone {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
        let h = crate::emit_code::ToCodeLiteral::to_code_literal(&self.height);
        quote::quote! { bevy::math::primitives::Cone { radius: #r, height: #h } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::ConicalFrustum`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyConicalFrustum {
    /// Top cap radius.
    pub radius_top: f32,
    /// Bottom cap radius.
    pub radius_bottom: f32,
    /// Height of the frustum.
    pub height: f32,
}

crate::default_style!(BevyConicalFrustum => BevyConicalFrustumStyle);

impl BevyConicalFrustum {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::ConicalFrustum {
        self.into()
    }
}
impl From<bevy::math::primitives::ConicalFrustum> for BevyConicalFrustum {
    fn from(s: bevy::math::primitives::ConicalFrustum) -> Self {
        Self {
            radius_top: s.radius_top,
            radius_bottom: s.radius_bottom,
            height: s.height,
        }
    }
}
impl From<BevyConicalFrustum> for bevy::math::primitives::ConicalFrustum {
    fn from(s: BevyConicalFrustum) -> Self {
        bevy::math::primitives::ConicalFrustum {
            radius_top: s.radius_top,
            radius_bottom: s.radius_bottom,
            height: s.height,
        }
    }
}
impl Prompt for BevyConicalFrustum {
    fn prompt() -> Option<&'static str> {
        Some("Conical frustum (radius_top, radius_bottom, height):")
    }
}
impl Elicitation for BevyConicalFrustum {
    type Style = BevyConicalFrustumStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::ConicalFrustum")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            radius_top: f32::elicit(communicator).await?,
            radius_bottom: f32::elicit(communicator).await?,
            height: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyConicalFrustum {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::ConicalFrustum",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "radius_top",
                        type_name: "f32",
                        prompt: Some("Top radius:"),
                    },
                    FieldInfo {
                        name: "radius_bottom",
                        type_name: "f32",
                        prompt: Some("Bottom radius:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "f32",
                        prompt: Some("Height:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyConicalFrustum {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::ConicalFrustum".to_string(),
            fields: vec![
                ("radius_top".to_string(), Box::new(f32::prompt_tree())),
                ("radius_bottom".to_string(), Box::new(f32::prompt_tree())),
                ("height".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyConicalFrustum {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let rt = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius_top);
        let rb = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius_bottom);
        let h = crate::emit_code::ToCodeLiteral::to_code_literal(&self.height);
        quote::quote! {
            bevy::math::primitives::ConicalFrustum {
                radius_top: #rt, radius_bottom: #rb, height: #h
            }
        }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Torus`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyTorus {
    /// Minor radius (tube cross-section).
    pub minor_radius: f32,
    /// Major radius (distance from center to tube center).
    pub major_radius: f32,
}

crate::default_style!(BevyTorus => BevyTorusStyle);

impl BevyTorus {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Torus {
        self.into()
    }
}
impl From<bevy::math::primitives::Torus> for BevyTorus {
    fn from(s: bevy::math::primitives::Torus) -> Self {
        Self {
            minor_radius: s.minor_radius,
            major_radius: s.major_radius,
        }
    }
}
impl From<BevyTorus> for bevy::math::primitives::Torus {
    fn from(s: BevyTorus) -> Self {
        bevy::math::primitives::Torus {
            minor_radius: s.minor_radius,
            major_radius: s.major_radius,
        }
    }
}
impl Prompt for BevyTorus {
    fn prompt() -> Option<&'static str> {
        Some("Torus (minor radius, major radius):")
    }
}
impl Elicitation for BevyTorus {
    type Style = BevyTorusStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Torus")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            minor_radius: f32::elicit(communicator).await?,
            major_radius: f32::elicit(communicator).await?,
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
impl ElicitIntrospect for BevyTorus {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Torus",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "minor_radius",
                        type_name: "f32",
                        prompt: Some("Minor radius (tube):"),
                    },
                    FieldInfo {
                        name: "major_radius",
                        type_name: "f32",
                        prompt: Some("Major radius (ring):"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyTorus {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Torus".to_string(),
            fields: vec![
                ("minor_radius".to_string(), Box::new(f32::prompt_tree())),
                ("major_radius".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyTorus {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let min = crate::emit_code::ToCodeLiteral::to_code_literal(&self.minor_radius);
        let maj = crate::emit_code::ToCodeLiteral::to_code_literal(&self.major_radius);
        quote::quote! {
            bevy::math::primitives::Torus { minor_radius: #min, major_radius: #maj }
        }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Plane3d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyPlane3d {
    /// The plane normal (unit direction).
    pub normal: BevyDir3,
    /// Half-extents of the finite plane representation.
    pub half_size: BevyVec2,
}

crate::default_style!(BevyPlane3d => BevyPlane3dStyle);

impl BevyPlane3d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Plane3d {
        self.into()
    }
}
impl From<bevy::math::primitives::Plane3d> for BevyPlane3d {
    fn from(s: bevy::math::primitives::Plane3d) -> Self {
        Self {
            normal: s.normal.into(),
            half_size: s.half_size.into(),
        }
    }
}
impl From<BevyPlane3d> for bevy::math::primitives::Plane3d {
    fn from(s: BevyPlane3d) -> Self {
        bevy::math::primitives::Plane3d {
            normal: s.normal.into(),
            half_size: s.half_size.into(),
        }
    }
}
impl Prompt for BevyPlane3d {
    fn prompt() -> Option<&'static str> {
        Some("3D plane (normal + half-size):")
    }
}
impl Elicitation for BevyPlane3d {
    type Style = BevyPlane3dStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Plane3d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            normal: BevyDir3::elicit(communicator).await?,
            half_size: BevyVec2::elicit(communicator).await?,
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyDir3 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyDir3 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyDir3 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyPlane3d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Plane3d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "normal",
                        type_name: "BevyDir3",
                        prompt: Some("Normal:"),
                    },
                    FieldInfo {
                        name: "half_size",
                        type_name: "BevyVec2",
                        prompt: Some("Half-size:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyPlane3d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Plane3d".to_string(),
            fields: vec![
                ("normal".to_string(), Box::new(BevyDir3::prompt_tree())),
                ("half_size".to_string(), Box::new(BevyVec2::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyPlane3d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = crate::emit_code::ToCodeLiteral::to_code_literal(&self.normal);
        let hs = crate::emit_code::ToCodeLiteral::to_code_literal(&self.half_size);
        quote::quote! { bevy::math::primitives::Plane3d { normal: #n, half_size: #hs } }
    }
}

/// Elicitable trenchcoat for [`bevy::math::primitives::Tetrahedron`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyTetrahedron {
    /// The four vertices of the tetrahedron.
    pub vertices: [BevyVec3; 4],
}

crate::default_style!(BevyTetrahedron => BevyTetrahedronStyle);

impl BevyTetrahedron {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Tetrahedron {
        self.into()
    }
}
impl From<bevy::math::primitives::Tetrahedron> for BevyTetrahedron {
    fn from(s: bevy::math::primitives::Tetrahedron) -> Self {
        Self {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl From<BevyTetrahedron> for bevy::math::primitives::Tetrahedron {
    fn from(s: BevyTetrahedron) -> Self {
        bevy::math::primitives::Tetrahedron {
            vertices: s.vertices.map(Into::into),
        }
    }
}
impl Prompt for BevyTetrahedron {
    fn prompt() -> Option<&'static str> {
        Some("Tetrahedron (4 vertices):")
    }
}
impl Elicitation for BevyTetrahedron {
    type Style = BevyTetrahedronStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Tetrahedron")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let v0 = BevyVec3::elicit(communicator).await?;
        let v1 = BevyVec3::elicit(communicator).await?;
        let v2 = BevyVec3::elicit(communicator).await?;
        let v3 = BevyVec3::elicit(communicator).await?;
        Ok(Self {
            vertices: [v0, v1, v2, v3],
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::creusot_proof()
    }
}
impl ElicitIntrospect for BevyTetrahedron {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Tetrahedron",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "vertices[0]",
                        type_name: "BevyVec3",
                        prompt: Some("Vertex 0:"),
                    },
                    FieldInfo {
                        name: "vertices[1]",
                        type_name: "BevyVec3",
                        prompt: Some("Vertex 1:"),
                    },
                    FieldInfo {
                        name: "vertices[2]",
                        type_name: "BevyVec3",
                        prompt: Some("Vertex 2:"),
                    },
                    FieldInfo {
                        name: "vertices[3]",
                        type_name: "BevyVec3",
                        prompt: Some("Vertex 3:"),
                    },
                ],
            },
        }
    }
}
impl crate::ElicitPromptTree for BevyTetrahedron {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Tetrahedron".to_string(),
            fields: vec![
                ("vertices[0]".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("vertices[1]".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("vertices[2]".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("vertices[3]".to_string(), Box::new(BevyVec3::prompt_tree())),
            ],
        }
    }
}
impl crate::emit_code::ToCodeLiteral for BevyTetrahedron {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v0 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[0]);
        let v1 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[1]);
        let v2 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[2]);
        let v3 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.vertices[3]);
        quote::quote! {
            bevy::math::primitives::Tetrahedron { vertices: [#v0, #v1, #v2, #v3] }
        }
    }
}
