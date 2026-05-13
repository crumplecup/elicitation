//! Bevy affine transform type trenchcoats.
//!
//! Covers `Affine2`, `Affine3A`, `DAffine2`, and `DAffine3`.
//! The SIMD-aligned `Affine3A` columns are exposed as plain `BevyVec3`/`BevyMat3`.

use super::{
    mat::{BevyDMat2, BevyDMat3, BevyMat2, BevyMat3},
    vec::{BevyDVec2, BevyDVec3, BevyVec2, BevyVec3},
};
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Elicitable trenchcoat for [`bevy::math::Affine2`].
///
/// 2D affine transform: column-major 2×2 linear part + 2D translation.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyAffine2 {
    /// Linear (rotation/scale) part.
    pub matrix2: BevyMat2,
    /// Translation component.
    pub translation: BevyVec2,
}

crate::default_style!(BevyAffine2 => BevyAffine2Style);

impl BevyAffine2 {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Affine2 {
        self.into()
    }
}

impl From<bevy::math::Affine2> for BevyAffine2 {
    fn from(a: bevy::math::Affine2) -> Self {
        Self {
            matrix2: a.matrix2.into(),
            translation: a.translation.into(),
        }
    }
}

impl From<BevyAffine2> for bevy::math::Affine2 {
    fn from(a: BevyAffine2) -> Self {
        bevy::math::Affine2 {
            matrix2: a.matrix2.into(),
            translation: a.translation.into(),
        }
    }
}

impl Prompt for BevyAffine2 {
    fn prompt() -> Option<&'static str> {
        Some("2D affine transform (linear matrix + translation):")
    }
}

impl Elicitation for BevyAffine2 {
    type Style = BevyAffine2Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Affine2"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            matrix2: BevyMat2::elicit(communicator).await?,
            translation: BevyVec2::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyMat2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyMat2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyMat2 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyAffine2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Affine2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "matrix2",
                        type_name: "BevyMat2",
                        prompt: Some("Linear part (2×2):"),
                    },
                    FieldInfo {
                        name: "translation",
                        type_name: "BevyVec2",
                        prompt: Some("Translation (x, y):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyAffine2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Affine2".to_string(),
            fields: vec![
                ("matrix2".to_string(), Box::new(BevyMat2::prompt_tree())),
                ("translation".to_string(), Box::new(BevyVec2::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAffine2 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let m = crate::emit_code::ToCodeLiteral::to_code_literal(&self.matrix2);
        let t = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation);
        quote::quote! { bevy::math::Affine2 { matrix2: #m, translation: #t } }
    }
}

// ── Affine3A ─────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Affine3A`] (SIMD-aligned 3D affine).
///
/// Stored as plain `BevyMat3`/`BevyVec3`; SIMD alignment is transparent.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyAffine3A {
    /// Linear (rotation/scale) part.
    pub matrix3: BevyMat3,
    /// Translation component.
    pub translation: BevyVec3,
}

crate::default_style!(BevyAffine3A => BevyAffine3AStyle);

impl BevyAffine3A {
    /// Converts this wrapper into the upstream SIMD type.
    pub fn into_inner(self) -> bevy::math::Affine3A {
        self.into()
    }
}

impl From<bevy::math::Affine3A> for BevyAffine3A {
    fn from(a: bevy::math::Affine3A) -> Self {
        use super::mat::BevyMat3A;
        let m: BevyMat3A = a.matrix3.into();
        Self {
            matrix3: BevyMat3 {
                x_axis: m.x_axis,
                y_axis: m.y_axis,
                z_axis: m.z_axis,
            },
            translation: bevy::math::Vec3::from(a.translation).into(),
        }
    }
}

impl From<BevyAffine3A> for bevy::math::Affine3A {
    fn from(a: BevyAffine3A) -> Self {
        let m3a: bevy::math::Mat3A = {
            let x: bevy::math::Vec3 = a.matrix3.x_axis.into();
            let y: bevy::math::Vec3 = a.matrix3.y_axis.into();
            let z: bevy::math::Vec3 = a.matrix3.z_axis.into();
            bevy::math::Mat3A::from_cols(x.into(), y.into(), z.into())
        };
        let t: bevy::math::Vec3 = a.translation.into();
        bevy::math::Affine3A {
            matrix3: m3a,
            translation: t.into(),
        }
    }
}

impl Prompt for BevyAffine3A {
    fn prompt() -> Option<&'static str> {
        Some("3D affine transform (linear matrix + translation):")
    }
}

impl Elicitation for BevyAffine3A {
    type Style = BevyAffine3AStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Affine3A"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            matrix3: BevyMat3::elicit(communicator).await?,
            translation: BevyVec3::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyMat3 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyMat3 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyMat3 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyAffine3A {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Affine3A",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "matrix3",
                        type_name: "BevyMat3",
                        prompt: Some("Linear part (3×3):"),
                    },
                    FieldInfo {
                        name: "translation",
                        type_name: "BevyVec3",
                        prompt: Some("Translation (x, y, z):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyAffine3A {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Affine3A".to_string(),
            fields: vec![
                ("matrix3".to_string(), Box::new(BevyMat3::prompt_tree())),
                ("translation".to_string(), Box::new(BevyVec3::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAffine3A {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let m = crate::emit_code::ToCodeLiteral::to_code_literal(&self.matrix3);
        let t = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation);
        quote::quote! {
            {
                let t: bevy::math::Vec3 = #t;
                bevy::math::Affine3A {
                    matrix3: bevy::math::Mat3A::from(#m),
                    translation: bevy::math::Vec3A::from(t),
                }
            }
        }
    }
}

// ── DAffine2 ──────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::DAffine2`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDAffine2 {
    /// Linear (rotation/scale) part.
    pub matrix2: BevyDMat2,
    /// Translation component.
    pub translation: BevyDVec2,
}

crate::default_style!(BevyDAffine2 => BevyDAffine2Style);

impl BevyDAffine2 {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::DAffine2 {
        self.into()
    }
}

impl From<bevy::math::DAffine2> for BevyDAffine2 {
    fn from(a: bevy::math::DAffine2) -> Self {
        Self {
            matrix2: a.matrix2.into(),
            translation: a.translation.into(),
        }
    }
}

impl From<BevyDAffine2> for bevy::math::DAffine2 {
    fn from(a: BevyDAffine2) -> Self {
        bevy::math::DAffine2 {
            matrix2: a.matrix2.into(),
            translation: a.translation.into(),
        }
    }
}

impl Prompt for BevyDAffine2 {
    fn prompt() -> Option<&'static str> {
        Some("2D f64 affine transform (linear matrix + translation):")
    }
}

impl Elicitation for BevyDAffine2 {
    type Style = BevyDAffine2Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::DAffine2"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            matrix2: BevyDMat2::elicit(communicator).await?,
            translation: BevyDVec2::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyDMat2 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyDMat2 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyDMat2 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyDAffine2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::DAffine2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "matrix2",
                        type_name: "BevyDMat2",
                        prompt: Some("Linear part (2×2 f64):"),
                    },
                    FieldInfo {
                        name: "translation",
                        type_name: "BevyDVec2",
                        prompt: Some("Translation (x, y):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDAffine2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::DAffine2".to_string(),
            fields: vec![
                ("matrix2".to_string(), Box::new(BevyDMat2::prompt_tree())),
                (
                    "translation".to_string(),
                    Box::new(BevyDVec2::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDAffine2 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let m = crate::emit_code::ToCodeLiteral::to_code_literal(&self.matrix2);
        let t = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation);
        quote::quote! { bevy::math::DAffine2 { matrix2: #m, translation: #t } }
    }
}

// ── DAffine3 ──────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::DAffine3`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDAffine3 {
    /// Linear (rotation/scale) part.
    pub matrix3: BevyDMat3,
    /// Translation component.
    pub translation: BevyDVec3,
}

crate::default_style!(BevyDAffine3 => BevyDAffine3Style);

impl BevyDAffine3 {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::DAffine3 {
        self.into()
    }
}

impl From<bevy::math::DAffine3> for BevyDAffine3 {
    fn from(a: bevy::math::DAffine3) -> Self {
        Self {
            matrix3: a.matrix3.into(),
            translation: a.translation.into(),
        }
    }
}

impl From<BevyDAffine3> for bevy::math::DAffine3 {
    fn from(a: BevyDAffine3) -> Self {
        bevy::math::DAffine3 {
            matrix3: a.matrix3.into(),
            translation: a.translation.into(),
        }
    }
}

impl Prompt for BevyDAffine3 {
    fn prompt() -> Option<&'static str> {
        Some("3D f64 affine transform (linear matrix + translation):")
    }
}

impl Elicitation for BevyDAffine3 {
    type Style = BevyDAffine3Style;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::DAffine3"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            matrix3: BevyDMat3::elicit(communicator).await?,
            translation: BevyDVec3::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyDMat3 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyDMat3 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyDMat3 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyDAffine3 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::DAffine3",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "matrix3",
                        type_name: "BevyDMat3",
                        prompt: Some("Linear part (3×3 f64):"),
                    },
                    FieldInfo {
                        name: "translation",
                        type_name: "BevyDVec3",
                        prompt: Some("Translation (x, y, z):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDAffine3 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::DAffine3".to_string(),
            fields: vec![
                ("matrix3".to_string(), Box::new(BevyDMat3::prompt_tree())),
                (
                    "translation".to_string(),
                    Box::new(BevyDVec3::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDAffine3 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let m = crate::emit_code::ToCodeLiteral::to_code_literal(&self.matrix3);
        let t = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation);
        quote::quote! { bevy::math::DAffine3 { matrix3: #m, translation: #t } }
    }
}
