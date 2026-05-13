//! Bevy matrix type trenchcoats.
//!
//! Covers `Mat2/3/4`, `DMat2/3/4`, and the SIMD-aligned `Mat3A`.
//! All matrices are stored column-major via their axis fields.

use super::vec::{BevyDVec2, BevyDVec3, BevyDVec4, BevyVec2, BevyVec3, BevyVec4};
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Generate a 2-column matrix trenchcoat (column-major).
macro_rules! bevy_mat2x2 {
    (
        $name:ident,
        $col:ty,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat for [`", $type_name, "`] (column-major 2×2 matrix)."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// First (X) column.
                pub x_axis: $col,
                /// Second (Y) column.
                pub y_axis: $col,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(m: $upstream) -> Self {
                    Self {
                        x_axis: m.x_axis.into(),
                        y_axis: m.y_axis.into(),
                    }
                }
            }

            impl From<$name> for $upstream {
                fn from(m: $name) -> Self {
                    $upstream::from_cols(m.x_axis.into(), m.y_axis.into())
                }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> {
                    Some($prompt)
                }
            }

            impl Elicitation for $name {
                type Style = [< $name Style >];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(
                    communicator: &C,
                ) -> ElicitResult<Self> {
                    Ok(Self {
                        x_axis: <$col>::elicit(communicator).await?,
                        y_axis: <$col>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::creusot_proof()
                }
            }

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern {
                    ElicitationPattern::Survey
                }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: "x_axis", type_name: stringify!($col), prompt: Some("Column 0 (x-axis):") },
                                FieldInfo { name: "y_axis", type_name: stringify!($col), prompt: Some("Column 1 (y-axis):") },
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
                            ("x_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("y_axis".to_string(), Box::new(<$col>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x_axis);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y_axis);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::from_cols(#x, #y) }
                }
            }
        }
    };
}

/// Generate a 3-column matrix trenchcoat (column-major).
macro_rules! bevy_mat3x3 {
    (
        $name:ident,
        $col:ty,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat for [`", $type_name, "`] (column-major 3×3 matrix)."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// First (X) column.
                pub x_axis: $col,
                /// Second (Y) column.
                pub y_axis: $col,
                /// Third (Z) column.
                pub z_axis: $col,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(m: $upstream) -> Self {
                    Self {
                        x_axis: m.x_axis.into(),
                        y_axis: m.y_axis.into(),
                        z_axis: m.z_axis.into(),
                    }
                }
            }

            impl From<$name> for $upstream {
                fn from(m: $name) -> Self {
                    $upstream::from_cols(m.x_axis.into(), m.y_axis.into(), m.z_axis.into())
                }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> {
                    Some($prompt)
                }
            }

            impl Elicitation for $name {
                type Style = [< $name Style >];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(
                    communicator: &C,
                ) -> ElicitResult<Self> {
                    Ok(Self {
                        x_axis: <$col>::elicit(communicator).await?,
                        y_axis: <$col>::elicit(communicator).await?,
                        z_axis: <$col>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::creusot_proof()
                }
            }

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern {
                    ElicitationPattern::Survey
                }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: "x_axis", type_name: stringify!($col), prompt: Some("Column 0 (x-axis):") },
                                FieldInfo { name: "y_axis", type_name: stringify!($col), prompt: Some("Column 1 (y-axis):") },
                                FieldInfo { name: "z_axis", type_name: stringify!($col), prompt: Some("Column 2 (z-axis):") },
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
                            ("x_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("y_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("z_axis".to_string(), Box::new(<$col>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x_axis);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y_axis);
                    let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z_axis);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::from_cols(#x, #y, #z) }
                }
            }
        }
    };
}

/// Generate a 4-column matrix trenchcoat (column-major).
macro_rules! bevy_mat4x4 {
    (
        $name:ident,
        $col:ty,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat for [`", $type_name, "`] (column-major 4×4 matrix)."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// First (X) column.
                pub x_axis: $col,
                /// Second (Y) column.
                pub y_axis: $col,
                /// Third (Z) column.
                pub z_axis: $col,
                /// Fourth (W) column.
                pub w_axis: $col,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(m: $upstream) -> Self {
                    Self {
                        x_axis: m.x_axis.into(),
                        y_axis: m.y_axis.into(),
                        z_axis: m.z_axis.into(),
                        w_axis: m.w_axis.into(),
                    }
                }
            }

            impl From<$name> for $upstream {
                fn from(m: $name) -> Self {
                    $upstream::from_cols(
                        m.x_axis.into(),
                        m.y_axis.into(),
                        m.z_axis.into(),
                        m.w_axis.into(),
                    )
                }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> {
                    Some($prompt)
                }
            }

            impl Elicitation for $name {
                type Style = [< $name Style >];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(
                    communicator: &C,
                ) -> ElicitResult<Self> {
                    Ok(Self {
                        x_axis: <$col>::elicit(communicator).await?,
                        y_axis: <$col>::elicit(communicator).await?,
                        z_axis: <$col>::elicit(communicator).await?,
                        w_axis: <$col>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$col as Elicitation>::creusot_proof()
                }
            }

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern {
                    ElicitationPattern::Survey
                }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: "x_axis", type_name: stringify!($col), prompt: Some("Column 0 (x-axis):") },
                                FieldInfo { name: "y_axis", type_name: stringify!($col), prompt: Some("Column 1 (y-axis):") },
                                FieldInfo { name: "z_axis", type_name: stringify!($col), prompt: Some("Column 2 (z-axis):") },
                                FieldInfo { name: "w_axis", type_name: stringify!($col), prompt: Some("Column 3 (w-axis):") },
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
                            ("x_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("y_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("z_axis".to_string(), Box::new(<$col>::prompt_tree())),
                            ("w_axis".to_string(), Box::new(<$col>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x_axis);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y_axis);
                    let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z_axis);
                    let w = crate::emit_code::ToCodeLiteral::to_code_literal(&self.w_axis);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::from_cols(#x, #y, #z, #w) }
                }
            }
        }
    };
}

// ── f32 matrices ──────────────────────────────────────────────────────────────

bevy_mat2x2!(
    BevyMat2,
    BevyVec2,
    bevy::math::Mat2,
    "bevy::math::Mat2",
    "2×2 f32 matrix (column-major, provide x_axis and y_axis):",
    "bevy::math::Mat2"
);

bevy_mat3x3!(
    BevyMat3,
    BevyVec3,
    bevy::math::Mat3,
    "bevy::math::Mat3",
    "3×3 f32 matrix (column-major, provide x_axis, y_axis, z_axis):",
    "bevy::math::Mat3"
);

bevy_mat4x4!(
    BevyMat4,
    BevyVec4,
    bevy::math::Mat4,
    "bevy::math::Mat4",
    "4×4 f32 matrix (column-major, provide x_axis, y_axis, z_axis, w_axis):",
    "bevy::math::Mat4"
);

// ── f64 matrices ──────────────────────────────────────────────────────────────

bevy_mat2x2!(
    BevyDMat2,
    BevyDVec2,
    bevy::math::DMat2,
    "bevy::math::DMat2",
    "2×2 f64 matrix (column-major):",
    "bevy::math::DMat2"
);

bevy_mat3x3!(
    BevyDMat3,
    BevyDVec3,
    bevy::math::DMat3,
    "bevy::math::DMat3",
    "3×3 f64 matrix (column-major):",
    "bevy::math::DMat3"
);

bevy_mat4x4!(
    BevyDMat4,
    BevyDVec4,
    bevy::math::DMat4,
    "bevy::math::DMat4",
    "4×4 f64 matrix (column-major):",
    "bevy::math::DMat4"
);

// ── SIMD-aligned 3×3 matrix ───────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Mat3A`] (SIMD-aligned 3×3 f32 matrix).
///
/// Each axis is exposed as a plain `BevyVec3` (i.e., `[f32; 3]`); the SIMD
/// alignment is transparent to the elicitation layer.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyMat3A {
    /// First (X) column.
    pub x_axis: BevyVec3,
    /// Second (Y) column.
    pub y_axis: BevyVec3,
    /// Third (Z) column.
    pub z_axis: BevyVec3,
}

crate::default_style!(BevyMat3A => BevyMat3AStyle);

impl BevyMat3A {
    /// Converts this wrapper into the upstream SIMD type.
    pub fn into_inner(self) -> bevy::math::Mat3A {
        self.into()
    }
}

impl From<bevy::math::Mat3A> for BevyMat3A {
    fn from(m: bevy::math::Mat3A) -> Self {
        Self {
            x_axis: bevy::math::Vec3::from(m.x_axis).into(),
            y_axis: bevy::math::Vec3::from(m.y_axis).into(),
            z_axis: bevy::math::Vec3::from(m.z_axis).into(),
        }
    }
}

impl From<BevyMat3A> for bevy::math::Mat3A {
    fn from(m: BevyMat3A) -> Self {
        let x: bevy::math::Vec3 = m.x_axis.into();
        let y: bevy::math::Vec3 = m.y_axis.into();
        let z: bevy::math::Vec3 = m.z_axis.into();
        bevy::math::Mat3A::from_cols(x.into(), y.into(), z.into())
    }
}

impl Prompt for BevyMat3A {
    fn prompt() -> Option<&'static str> {
        Some("SIMD-aligned 3×3 f32 matrix (column-major):")
    }
}

impl Elicitation for BevyMat3A {
    type Style = BevyMat3AStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Mat3A"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x_axis: BevyVec3::elicit(communicator).await?,
            y_axis: BevyVec3::elicit(communicator).await?,
            z_axis: BevyVec3::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyMat3A {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Mat3A",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x_axis",
                        type_name: "BevyVec3",
                        prompt: Some("Column 0 (x-axis):"),
                    },
                    FieldInfo {
                        name: "y_axis",
                        type_name: "BevyVec3",
                        prompt: Some("Column 1 (y-axis):"),
                    },
                    FieldInfo {
                        name: "z_axis",
                        type_name: "BevyVec3",
                        prompt: Some("Column 2 (z-axis):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyMat3A {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Mat3A".to_string(),
            fields: vec![
                ("x_axis".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("y_axis".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("z_axis".to_string(), Box::new(BevyVec3::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyMat3A {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x_axis);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y_axis);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z_axis);
        quote::quote! {
            bevy::math::Mat3A::from_cols(
                bevy::math::Vec3A::from(#x),
                bevy::math::Vec3A::from(#y),
                bevy::math::Vec3A::from(#z),
            )
        }
    }
}
