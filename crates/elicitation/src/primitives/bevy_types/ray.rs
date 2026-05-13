//! Bevy direction, ray, and line type trenchcoats.
//!
//! Covers `Dir2`, `Dir3`, `Dir3A`, `Ray2d`, `Ray3d`, `Line2d`, `Line3d`.
//! Direction types store normalized components; normalization is re-applied on
//! conversion back, falling back to a canonical axis if the user provides a zero vector.

use super::vec::{BevyVec2, BevyVec3};
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── Dir2 ─────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Dir2`].
///
/// Stores the XY components of the unit direction; the vector is re-normalized on
/// conversion. Falls back to `Dir2::X` if the user-supplied vector is zero.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDir2 {
    /// X component (will be normalized).
    pub x: f32,
    /// Y component (will be normalized).
    pub y: f32,
}

crate::default_style!(BevyDir2 => BevyDir2Style);

impl BevyDir2 {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Dir2 {
        self.into()
    }
}

impl From<bevy::math::Dir2> for BevyDir2 {
    fn from(d: bevy::math::Dir2) -> Self {
        Self { x: d.x, y: d.y }
    }
}

impl From<BevyDir2> for bevy::math::Dir2 {
    fn from(d: BevyDir2) -> Self {
        bevy::math::Dir2::new(bevy::math::Vec2::new(d.x, d.y)).unwrap_or(bevy::math::Dir2::X)
    }
}

impl Prompt for BevyDir2 {
    fn prompt() -> Option<&'static str> {
        Some("2D unit direction (x, y — will be normalized):")
    }
}

impl Elicitation for BevyDir2 {
    type Style = BevyDir2Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Dir2"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f32::elicit(communicator).await?,
            y: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyDir2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Dir2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDir2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Dir2".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDir2 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        quote::quote! {
            bevy::math::Dir2::new(bevy::math::Vec2::new(#x, #y))
                .unwrap_or(bevy::math::Dir2::X)
        }
    }
}

// ── Dir3 ─────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Dir3`].
///
/// Falls back to `Dir3::X` if the provided vector is zero.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDir3 {
    /// X component (will be normalized).
    pub x: f32,
    /// Y component (will be normalized).
    pub y: f32,
    /// Z component (will be normalized).
    pub z: f32,
}

crate::default_style!(BevyDir3 => BevyDir3Style);

impl BevyDir3 {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Dir3 {
        self.into()
    }
}

impl From<bevy::math::Dir3> for BevyDir3 {
    fn from(d: bevy::math::Dir3) -> Self {
        Self {
            x: d.x,
            y: d.y,
            z: d.z,
        }
    }
}

impl From<BevyDir3> for bevy::math::Dir3 {
    fn from(d: BevyDir3) -> Self {
        bevy::math::Dir3::new(bevy::math::Vec3::new(d.x, d.y, d.z)).unwrap_or(bevy::math::Dir3::X)
    }
}

impl Prompt for BevyDir3 {
    fn prompt() -> Option<&'static str> {
        Some("3D unit direction (x, y, z — will be normalized):")
    }
}

impl Elicitation for BevyDir3 {
    type Style = BevyDir3Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Dir3"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f32::elicit(communicator).await?,
            y: f32::elicit(communicator).await?,
            z: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyDir3 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Dir3",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y:"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "f32",
                        prompt: Some("Z:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDir3 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Dir3".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
                ("z".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDir3 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
        quote::quote! {
            bevy::math::Dir3::new(bevy::math::Vec3::new(#x, #y, #z))
                .unwrap_or(bevy::math::Dir3::X)
        }
    }
}

// ── Dir3A (SIMD-aligned) ──────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Dir3A`] (SIMD-aligned unit direction).
///
/// Stored identically to `BevyDir3`; SIMD alignment is transparent.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDir3A {
    /// X component (will be normalized).
    pub x: f32,
    /// Y component (will be normalized).
    pub y: f32,
    /// Z component (will be normalized).
    pub z: f32,
}

crate::default_style!(BevyDir3A => BevyDir3AStyle);

impl BevyDir3A {
    /// Converts this wrapper into the upstream SIMD type.
    pub fn into_inner(self) -> bevy::math::Dir3A {
        self.into()
    }
}

impl From<bevy::math::Dir3A> for BevyDir3A {
    fn from(d: bevy::math::Dir3A) -> Self {
        Self {
            x: d.x,
            y: d.y,
            z: d.z,
        }
    }
}

impl From<BevyDir3A> for bevy::math::Dir3A {
    fn from(d: BevyDir3A) -> Self {
        bevy::math::Dir3A::new(bevy::math::Vec3A::new(d.x, d.y, d.z))
            .unwrap_or(bevy::math::Dir3A::X)
    }
}

impl Prompt for BevyDir3A {
    fn prompt() -> Option<&'static str> {
        Some("3D SIMD-aligned unit direction (x, y, z — will be normalized):")
    }
}

impl Elicitation for BevyDir3A {
    type Style = BevyDir3AStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Dir3A"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f32::elicit(communicator).await?,
            y: f32::elicit(communicator).await?,
            z: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyDir3A {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Dir3A",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y:"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "f32",
                        prompt: Some("Z:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDir3A {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Dir3A".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
                ("z".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDir3A {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
        quote::quote! {
            bevy::math::Dir3A::new(bevy::math::Vec3A::new(#x, #y, #z))
                .unwrap_or(bevy::math::Dir3A::X)
        }
    }
}

// ── Ray2d ─────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Ray2d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyRay2d {
    /// Ray origin.
    pub origin: BevyVec2,
    /// Ray direction (unit vector).
    pub direction: BevyDir2,
}

crate::default_style!(BevyRay2d => BevyRay2dStyle);

impl BevyRay2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Ray2d {
        self.into()
    }
}

impl From<bevy::math::Ray2d> for BevyRay2d {
    fn from(r: bevy::math::Ray2d) -> Self {
        Self {
            origin: r.origin.into(),
            direction: r.direction.into(),
        }
    }
}

impl From<BevyRay2d> for bevy::math::Ray2d {
    fn from(r: BevyRay2d) -> Self {
        bevy::math::Ray2d {
            origin: r.origin.into(),
            direction: r.direction.into(),
        }
    }
}

impl Prompt for BevyRay2d {
    fn prompt() -> Option<&'static str> {
        Some("2D ray (origin + unit direction):")
    }
}

impl Elicitation for BevyRay2d {
    type Style = BevyRay2dStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Ray2d"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            origin: BevyVec2::elicit(communicator).await?,
            direction: BevyDir2::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyRay2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Ray2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "origin",
                        type_name: "BevyVec2",
                        prompt: Some("Origin:"),
                    },
                    FieldInfo {
                        name: "direction",
                        type_name: "BevyDir2",
                        prompt: Some("Direction (unit):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyRay2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Ray2d".to_string(),
            fields: vec![
                ("origin".to_string(), Box::new(BevyVec2::prompt_tree())),
                ("direction".to_string(), Box::new(BevyDir2::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyRay2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let o = crate::emit_code::ToCodeLiteral::to_code_literal(&self.origin);
        let d = crate::emit_code::ToCodeLiteral::to_code_literal(&self.direction);
        quote::quote! { bevy::math::Ray2d { origin: #o, direction: #d } }
    }
}

// ── Ray3d ─────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Ray3d`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyRay3d {
    /// Ray origin.
    pub origin: BevyVec3,
    /// Ray direction (unit vector).
    pub direction: BevyDir3,
}

crate::default_style!(BevyRay3d => BevyRay3dStyle);

impl BevyRay3d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Ray3d {
        self.into()
    }
}

impl From<bevy::math::Ray3d> for BevyRay3d {
    fn from(r: bevy::math::Ray3d) -> Self {
        Self {
            origin: r.origin.into(),
            direction: r.direction.into(),
        }
    }
}

impl From<BevyRay3d> for bevy::math::Ray3d {
    fn from(r: BevyRay3d) -> Self {
        bevy::math::Ray3d {
            origin: r.origin.into(),
            direction: r.direction.into(),
        }
    }
}

impl Prompt for BevyRay3d {
    fn prompt() -> Option<&'static str> {
        Some("3D ray (origin + unit direction):")
    }
}

impl Elicitation for BevyRay3d {
    type Style = BevyRay3dStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Ray3d"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            origin: BevyVec3::elicit(communicator).await?,
            direction: BevyDir3::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyRay3d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Ray3d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "origin",
                        type_name: "BevyVec3",
                        prompt: Some("Origin:"),
                    },
                    FieldInfo {
                        name: "direction",
                        type_name: "BevyDir3",
                        prompt: Some("Direction (unit):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyRay3d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Ray3d".to_string(),
            fields: vec![
                ("origin".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("direction".to_string(), Box::new(BevyDir3::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyRay3d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let o = crate::emit_code::ToCodeLiteral::to_code_literal(&self.origin);
        let d = crate::emit_code::ToCodeLiteral::to_code_literal(&self.direction);
        quote::quote! { bevy::math::Ray3d { origin: #o, direction: #d } }
    }
}

// ── Line2d ────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::primitives::Line2d`].
///
/// An infinite line passing through the origin with a unit direction.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyLine2d {
    /// Line direction (unit vector).
    pub direction: BevyDir2,
}

crate::default_style!(BevyLine2d => BevyLine2dStyle);

impl BevyLine2d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Line2d {
        self.into()
    }
}

impl From<bevy::math::primitives::Line2d> for BevyLine2d {
    fn from(l: bevy::math::primitives::Line2d) -> Self {
        Self {
            direction: l.direction.into(),
        }
    }
}

impl From<BevyLine2d> for bevy::math::primitives::Line2d {
    fn from(l: BevyLine2d) -> Self {
        bevy::math::primitives::Line2d {
            direction: l.direction.into(),
        }
    }
}

impl Prompt for BevyLine2d {
    fn prompt() -> Option<&'static str> {
        Some("2D infinite line (unit direction through origin):")
    }
}

impl Elicitation for BevyLine2d {
    type Style = BevyLine2dStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Line2d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            direction: BevyDir2::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyLine2d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Line2d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "direction",
                    type_name: "BevyDir2",
                    prompt: Some("Direction:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyLine2d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Line2d".to_string(),
            fields: vec![("direction".to_string(), Box::new(BevyDir2::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyLine2d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let d = crate::emit_code::ToCodeLiteral::to_code_literal(&self.direction);
        quote::quote! { bevy::math::primitives::Line2d { direction: #d } }
    }
}

// ── Line3d ────────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::primitives::Line3d`].
///
/// An infinite line passing through the origin with a unit direction.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyLine3d {
    /// Line direction (unit vector).
    pub direction: BevyDir3,
}

crate::default_style!(BevyLine3d => BevyLine3dStyle);

impl BevyLine3d {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::primitives::Line3d {
        self.into()
    }
}

impl From<bevy::math::primitives::Line3d> for BevyLine3d {
    fn from(l: bevy::math::primitives::Line3d) -> Self {
        Self {
            direction: l.direction.into(),
        }
    }
}

impl From<BevyLine3d> for bevy::math::primitives::Line3d {
    fn from(l: BevyLine3d) -> Self {
        bevy::math::primitives::Line3d {
            direction: l.direction.into(),
        }
    }
}

impl Prompt for BevyLine3d {
    fn prompt() -> Option<&'static str> {
        Some("3D infinite line (unit direction through origin):")
    }
}

impl Elicitation for BevyLine3d {
    type Style = BevyLine3dStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::math::primitives::Line3d")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            direction: BevyDir3::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyLine3d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::primitives::Line3d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "direction",
                    type_name: "BevyDir3",
                    prompt: Some("Direction:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyLine3d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::primitives::Line3d".to_string(),
            fields: vec![("direction".to_string(), Box::new(BevyDir3::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyLine3d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let d = crate::emit_code::ToCodeLiteral::to_code_literal(&self.direction);
        quote::quote! { bevy::math::primitives::Line3d { direction: #d } }
    }
}
