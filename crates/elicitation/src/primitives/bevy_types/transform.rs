//! Bevy transform type trenchcoats.
//!
//! Covers `bevy::transform::components::Transform` and
//! `bevy::transform::components::GlobalTransform`.
//! Both are flattened into 10 `f32` fields (translation xyz, rotation xyzw,
//! scale xyz) to avoid importing the sibling `BevyVec3`/`BevyQuat` wrappers,
//! which are not re-exported at the crate root when the `bevy-types` feature
//! is active.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyTransform ─────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::transform::components::Transform`].
///
/// All three sub-components (translation, rotation, scale) are flattened into
/// ten `f32` fields so that [`schemars::JsonSchema`] can be derived without
/// reaching for the sibling `BevyVec3`/`BevyQuat` wrappers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyTransform {
    /// Translation X.
    pub translation_x: f32,
    /// Translation Y.
    pub translation_y: f32,
    /// Translation Z.
    pub translation_z: f32,
    /// Rotation quaternion X (imaginary i).
    pub rotation_x: f32,
    /// Rotation quaternion Y (imaginary j).
    pub rotation_y: f32,
    /// Rotation quaternion Z (imaginary k).
    pub rotation_z: f32,
    /// Rotation quaternion W (real).
    pub rotation_w: f32,
    /// Scale X.
    pub scale_x: f32,
    /// Scale Y.
    pub scale_y: f32,
    /// Scale Z.
    pub scale_z: f32,
}

crate::default_style!(BevyTransform => BevyTransformStyle);

impl BevyTransform {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::transform::components::Transform {
        self.into()
    }
}

impl From<bevy::transform::components::Transform> for BevyTransform {
    fn from(t: bevy::transform::components::Transform) -> Self {
        Self {
            translation_x: t.translation.x,
            translation_y: t.translation.y,
            translation_z: t.translation.z,
            rotation_x: t.rotation.x,
            rotation_y: t.rotation.y,
            rotation_z: t.rotation.z,
            rotation_w: t.rotation.w,
            scale_x: t.scale.x,
            scale_y: t.scale.y,
            scale_z: t.scale.z,
        }
    }
}

impl From<BevyTransform> for bevy::transform::components::Transform {
    fn from(t: BevyTransform) -> Self {
        bevy::transform::components::Transform {
            translation: bevy::math::Vec3::new(t.translation_x, t.translation_y, t.translation_z),
            rotation: bevy::math::Quat::from_xyzw(
                t.rotation_x,
                t.rotation_y,
                t.rotation_z,
                t.rotation_w,
            ),
            scale: bevy::math::Vec3::new(t.scale_x, t.scale_y, t.scale_z),
        }
    }
}

impl Prompt for BevyTransform {
    fn prompt() -> Option<&'static str> {
        Some("Bevy transform: position, rotation, scale.")
    }
}

impl Elicitation for BevyTransform {
    type Style = BevyTransformStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::transform::components::Transform")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            translation_x: f32::elicit(communicator).await?,
            translation_y: f32::elicit(communicator).await?,
            translation_z: f32::elicit(communicator).await?,
            rotation_x: f32::elicit(communicator).await?,
            rotation_y: f32::elicit(communicator).await?,
            rotation_z: f32::elicit(communicator).await?,
            rotation_w: f32::elicit(communicator).await?,
            scale_x: f32::elicit(communicator).await?,
            scale_y: f32::elicit(communicator).await?,
            scale_z: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyTransform {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::transform::components::Transform",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "translation_x",
                        type_name: "f32",
                        prompt: Some("Translation X:"),
                    },
                    FieldInfo {
                        name: "translation_y",
                        type_name: "f32",
                        prompt: Some("Translation Y:"),
                    },
                    FieldInfo {
                        name: "translation_z",
                        type_name: "f32",
                        prompt: Some("Translation Z:"),
                    },
                    FieldInfo {
                        name: "rotation_x",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion X:"),
                    },
                    FieldInfo {
                        name: "rotation_y",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion Y:"),
                    },
                    FieldInfo {
                        name: "rotation_z",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion Z:"),
                    },
                    FieldInfo {
                        name: "rotation_w",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion W:"),
                    },
                    FieldInfo {
                        name: "scale_x",
                        type_name: "f32",
                        prompt: Some("Scale X:"),
                    },
                    FieldInfo {
                        name: "scale_y",
                        type_name: "f32",
                        prompt: Some("Scale Y:"),
                    },
                    FieldInfo {
                        name: "scale_z",
                        type_name: "f32",
                        prompt: Some("Scale Z:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyTransform {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::transform::components::Transform".to_string(),
            fields: vec![
                ("translation_x".to_string(), Box::new(f32::prompt_tree())),
                ("translation_y".to_string(), Box::new(f32::prompt_tree())),
                ("translation_z".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_x".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_y".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_z".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_w".to_string(), Box::new(f32::prompt_tree())),
                ("scale_x".to_string(), Box::new(f32::prompt_tree())),
                ("scale_y".to_string(), Box::new(f32::prompt_tree())),
                ("scale_z".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyTransform {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_x);
        let ty = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_y);
        let tz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_z);
        let rx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_x);
        let ry = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_y);
        let rz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_z);
        let rw = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_w);
        let sx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_x);
        let sy = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_y);
        let sz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_z);
        quote::quote! {
            bevy::transform::components::Transform {
                translation: bevy::math::Vec3::new(#tx, #ty, #tz),
                rotation: bevy::math::Quat::from_xyzw(#rx, #ry, #rz, #rw),
                scale: bevy::math::Vec3::new(#sx, #sy, #sz),
            }
        }
    }
}

// ── BevyGlobalTransform ───────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::transform::components::GlobalTransform`].
///
/// `GlobalTransform` is computed at runtime from the entity hierarchy and is
/// normally not authored directly. This wrapper uses the same 10-field layout
/// as [`BevyTransform`] and reconstructs the value via
/// `GlobalTransform::from(Transform { … })` in [`crate::emit_code::ToCodeLiteral`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyGlobalTransform {
    /// Translation X.
    pub translation_x: f32,
    /// Translation Y.
    pub translation_y: f32,
    /// Translation Z.
    pub translation_z: f32,
    /// Rotation quaternion X (imaginary i).
    pub rotation_x: f32,
    /// Rotation quaternion Y (imaginary j).
    pub rotation_y: f32,
    /// Rotation quaternion Z (imaginary k).
    pub rotation_z: f32,
    /// Rotation quaternion W (real).
    pub rotation_w: f32,
    /// Scale X.
    pub scale_x: f32,
    /// Scale Y.
    pub scale_y: f32,
    /// Scale Z.
    pub scale_z: f32,
}

crate::default_style!(BevyGlobalTransform => BevyGlobalTransformStyle);

impl BevyGlobalTransform {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::transform::components::GlobalTransform {
        self.into()
    }
}

impl From<bevy::transform::components::GlobalTransform> for BevyGlobalTransform {
    fn from(g: bevy::transform::components::GlobalTransform) -> Self {
        let (scale, rotation, translation) = g.to_scale_rotation_translation();
        Self {
            translation_x: translation.x,
            translation_y: translation.y,
            translation_z: translation.z,
            rotation_x: rotation.x,
            rotation_y: rotation.y,
            rotation_z: rotation.z,
            rotation_w: rotation.w,
            scale_x: scale.x,
            scale_y: scale.y,
            scale_z: scale.z,
        }
    }
}

impl From<BevyGlobalTransform> for bevy::transform::components::GlobalTransform {
    fn from(g: BevyGlobalTransform) -> Self {
        let t = bevy::transform::components::Transform {
            translation: bevy::math::Vec3::new(g.translation_x, g.translation_y, g.translation_z),
            rotation: bevy::math::Quat::from_xyzw(
                g.rotation_x,
                g.rotation_y,
                g.rotation_z,
                g.rotation_w,
            ),
            scale: bevy::math::Vec3::new(g.scale_x, g.scale_y, g.scale_z),
        };
        bevy::transform::components::GlobalTransform::from(t)
    }
}

impl Prompt for BevyGlobalTransform {
    fn prompt() -> Option<&'static str> {
        Some("Bevy global transform: world-space position, rotation, scale.")
    }
}

impl Elicitation for BevyGlobalTransform {
    type Style = BevyGlobalTransformStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::transform::components::GlobalTransform")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            translation_x: f32::elicit(communicator).await?,
            translation_y: f32::elicit(communicator).await?,
            translation_z: f32::elicit(communicator).await?,
            rotation_x: f32::elicit(communicator).await?,
            rotation_y: f32::elicit(communicator).await?,
            rotation_z: f32::elicit(communicator).await?,
            rotation_w: f32::elicit(communicator).await?,
            scale_x: f32::elicit(communicator).await?,
            scale_y: f32::elicit(communicator).await?,
            scale_z: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyGlobalTransform {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::transform::components::GlobalTransform",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "translation_x",
                        type_name: "f32",
                        prompt: Some("Translation X:"),
                    },
                    FieldInfo {
                        name: "translation_y",
                        type_name: "f32",
                        prompt: Some("Translation Y:"),
                    },
                    FieldInfo {
                        name: "translation_z",
                        type_name: "f32",
                        prompt: Some("Translation Z:"),
                    },
                    FieldInfo {
                        name: "rotation_x",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion X:"),
                    },
                    FieldInfo {
                        name: "rotation_y",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion Y:"),
                    },
                    FieldInfo {
                        name: "rotation_z",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion Z:"),
                    },
                    FieldInfo {
                        name: "rotation_w",
                        type_name: "f32",
                        prompt: Some("Rotation quaternion W:"),
                    },
                    FieldInfo {
                        name: "scale_x",
                        type_name: "f32",
                        prompt: Some("Scale X:"),
                    },
                    FieldInfo {
                        name: "scale_y",
                        type_name: "f32",
                        prompt: Some("Scale Y:"),
                    },
                    FieldInfo {
                        name: "scale_z",
                        type_name: "f32",
                        prompt: Some("Scale Z:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGlobalTransform {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::transform::components::GlobalTransform".to_string(),
            fields: vec![
                ("translation_x".to_string(), Box::new(f32::prompt_tree())),
                ("translation_y".to_string(), Box::new(f32::prompt_tree())),
                ("translation_z".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_x".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_y".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_z".to_string(), Box::new(f32::prompt_tree())),
                ("rotation_w".to_string(), Box::new(f32::prompt_tree())),
                ("scale_x".to_string(), Box::new(f32::prompt_tree())),
                ("scale_y".to_string(), Box::new(f32::prompt_tree())),
                ("scale_z".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGlobalTransform {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_x);
        let ty = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_y);
        let tz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.translation_z);
        let rx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_x);
        let ry = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_y);
        let rz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_z);
        let rw = crate::emit_code::ToCodeLiteral::to_code_literal(&self.rotation_w);
        let sx = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_x);
        let sy = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_y);
        let sz = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scale_z);
        quote::quote! {
            bevy::transform::components::GlobalTransform::from(
                bevy::transform::components::Transform {
                    translation: bevy::math::Vec3::new(#tx, #ty, #tz),
                    rotation: bevy::math::Quat::from_xyzw(#rx, #ry, #rz, #rw),
                    scale: bevy::math::Vec3::new(#sx, #sy, #sz),
                }
            )
        }
    }
}
