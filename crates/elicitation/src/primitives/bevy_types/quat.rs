//! Bevy quaternion type trenchcoats.
//!
//! Covers `bevy::math::Quat` (f32) and `bevy::math::DQuat` (f64).

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

// ── Quat (f32) ────────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Quat`].
///
/// Exposes the four XYZW components directly. The user is expected to provide
/// a unit quaternion; `bevy::math::Quat::from_xyzw` is used for reconstruction.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyQuat {
    /// X (imaginary i) component.
    pub x: f32,
    /// Y (imaginary j) component.
    pub y: f32,
    /// Z (imaginary k) component.
    pub z: f32,
    /// W (real) component.
    pub w: f32,
}

crate::default_style!(BevyQuat => BevyQuatStyle);

impl BevyQuat {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::Quat {
        self.into()
    }
}

impl From<bevy::math::Quat> for BevyQuat {
    fn from(q: bevy::math::Quat) -> Self {
        Self {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }
}

impl From<BevyQuat> for bevy::math::Quat {
    fn from(q: BevyQuat) -> Self {
        bevy::math::Quat::from_xyzw(q.x, q.y, q.z, q.w)
    }
}

impl Prompt for BevyQuat {
    fn prompt() -> Option<&'static str> {
        Some("Quaternion (x, y, z, w — provide a unit quaternion):")
    }
}

impl Elicitation for BevyQuat {
    type Style = BevyQuatStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Quat"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f32::elicit(communicator).await?,
            y: f32::elicit(communicator).await?,
            z: f32::elicit(communicator).await?,
            w: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyQuat {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Quat",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X (imaginary i):"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y (imaginary j):"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "f32",
                        prompt: Some("Z (imaginary k):"),
                    },
                    FieldInfo {
                        name: "w",
                        type_name: "f32",
                        prompt: Some("W (real):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyQuat {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Quat".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
                ("z".to_string(), Box::new(f32::prompt_tree())),
                ("w".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyQuat {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
        let w = crate::emit_code::ToCodeLiteral::to_code_literal(&self.w);
        quote::quote! { bevy::math::Quat::from_xyzw(#x, #y, #z, #w) }
    }
}

// ── DQuat (f64) ───────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::DQuat`].
///
/// F64 double-precision quaternion.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyDQuat {
    /// X (imaginary i) component.
    pub x: f64,
    /// Y (imaginary j) component.
    pub y: f64,
    /// Z (imaginary k) component.
    pub z: f64,
    /// W (real) component.
    pub w: f64,
}

crate::default_style!(BevyDQuat => BevyDQuatStyle);

impl BevyDQuat {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::math::DQuat {
        self.into()
    }
}

impl From<bevy::math::DQuat> for BevyDQuat {
    fn from(q: bevy::math::DQuat) -> Self {
        Self {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }
}

impl From<BevyDQuat> for bevy::math::DQuat {
    fn from(q: BevyDQuat) -> Self {
        bevy::math::DQuat::from_xyzw(q.x, q.y, q.z, q.w)
    }
}

impl Prompt for BevyDQuat {
    fn prompt() -> Option<&'static str> {
        Some("Double-precision quaternion (x, y, z, w — provide a unit quaternion):")
    }
}

impl Elicitation for BevyDQuat {
    type Style = BevyDQuatStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::DQuat"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f64::elicit(communicator).await?,
            y: f64::elicit(communicator).await?,
            z: f64::elicit(communicator).await?,
            w: f64::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyDQuat {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::DQuat",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f64",
                        prompt: Some("X (imaginary i):"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f64",
                        prompt: Some("Y (imaginary j):"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "f64",
                        prompt: Some("Z (imaginary k):"),
                    },
                    FieldInfo {
                        name: "w",
                        type_name: "f64",
                        prompt: Some("W (real):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyDQuat {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::DQuat".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
                ("z".to_string(), Box::new(f64::prompt_tree())),
                ("w".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDQuat {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
        let w = crate::emit_code::ToCodeLiteral::to_code_literal(&self.w);
        quote::quote! { bevy::math::DQuat::from_xyzw(#x, #y, #z, #w) }
    }
}
