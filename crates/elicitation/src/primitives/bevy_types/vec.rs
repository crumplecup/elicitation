//! Bevy vector type trenchcoats.
//!
//! Owned struct wrappers for glam-based vector types re-exported from
//! `bevy::math`. Covers `Vec2/3/4`, `DVec2/3/4`, `IVec2/3/4`, `UVec2/3/4`,
//! `BVec2/3/4`, and the SIMD-aligned `Vec3A`.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Generate a 2-component vector trenchcoat struct.
macro_rules! bevy_vec2 {
    (
        $name:ident,
        $upstream:path,
        $prim:ty,
        $type_name:literal,
        $prompt:literal,
        $new:expr,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat wrapper for [`", $type_name, "`]."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// X component.
                pub x: $prim,
                /// Y component.
                pub y: $prim,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(v: $upstream) -> Self {
                    Self { x: v.x, y: v.y }
                }
            }

            impl From<$name> for $upstream {
                fn from(v: $name) -> Self {
                    ($new)(v.x, v.y)
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
                        x: <$prim>::elicit(communicator).await?,
                        y: <$prim>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::creusot_proof()
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
                                FieldInfo { name: "x", type_name: stringify!($prim), prompt: Some("X component:") },
                                FieldInfo { name: "y", type_name: stringify!($prim), prompt: Some("Y component:") },
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
                            ("x".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("y".to_string(), Box::new(<$prim>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::new(#x, #y) }
                }
            }
        }
    };
}

/// Generate a 3-component vector trenchcoat struct.
macro_rules! bevy_vec3 {
    (
        $name:ident,
        $upstream:path,
        $prim:ty,
        $type_name:literal,
        $prompt:literal,
        $new:expr,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat wrapper for [`", $type_name, "`]."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// X component.
                pub x: $prim,
                /// Y component.
                pub y: $prim,
                /// Z component.
                pub z: $prim,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(v: $upstream) -> Self {
                    Self { x: v.x, y: v.y, z: v.z }
                }
            }

            impl From<$name> for $upstream {
                fn from(v: $name) -> Self {
                    ($new)(v.x, v.y, v.z)
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
                        x: <$prim>::elicit(communicator).await?,
                        y: <$prim>::elicit(communicator).await?,
                        z: <$prim>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::creusot_proof()
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
                                FieldInfo { name: "x", type_name: stringify!($prim), prompt: Some("X component:") },
                                FieldInfo { name: "y", type_name: stringify!($prim), prompt: Some("Y component:") },
                                FieldInfo { name: "z", type_name: stringify!($prim), prompt: Some("Z component:") },
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
                            ("x".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("y".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("z".to_string(), Box::new(<$prim>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
                    let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::new(#x, #y, #z) }
                }
            }
        }
    };
}

/// Generate a 4-component vector trenchcoat struct.
macro_rules! bevy_vec4 {
    (
        $name:ident,
        $upstream:path,
        $prim:ty,
        $type_name:literal,
        $prompt:literal,
        $new:expr,
        $code_path:literal
    ) => {
        paste::paste! {
            #[doc = concat!(
                "Elicitable trenchcoat wrapper for [`", $type_name, "`]."
            )]
            #[derive(
                Debug, Clone, Copy, PartialEq,
                serde::Serialize, serde::Deserialize, schemars::JsonSchema,
            )]
            pub struct $name {
                /// X component.
                pub x: $prim,
                /// Y component.
                pub y: $prim,
                /// Z component.
                pub z: $prim,
                /// W component.
                pub w: $prim,
            }

            crate::default_style!($name => [< $name Style >]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream {
                    self.into()
                }
            }

            impl From<$upstream> for $name {
                fn from(v: $upstream) -> Self {
                    Self { x: v.x, y: v.y, z: v.z, w: v.w }
                }
            }

            impl From<$name> for $upstream {
                fn from(v: $name) -> Self {
                    ($new)(v.x, v.y, v.z, v.w)
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
                        x: <$prim>::elicit(communicator).await?,
                        y: <$prim>::elicit(communicator).await?,
                        z: <$prim>::elicit(communicator).await?,
                        w: <$prim>::elicit(communicator).await?,
                    })
                }

                fn kani_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::kani_proof()
                }
                fn verus_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::verus_proof()
                }
                fn creusot_proof() -> proc_macro2::TokenStream {
                    <$prim as Elicitation>::creusot_proof()
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
                                FieldInfo { name: "x", type_name: stringify!($prim), prompt: Some("X component:") },
                                FieldInfo { name: "y", type_name: stringify!($prim), prompt: Some("Y component:") },
                                FieldInfo { name: "z", type_name: stringify!($prim), prompt: Some("Z component:") },
                                FieldInfo { name: "w", type_name: stringify!($prim), prompt: Some("W component:") },
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
                            ("x".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("y".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("z".to_string(), Box::new(<$prim>::prompt_tree())),
                            ("w".to_string(), Box::new(<$prim>::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
                    let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
                    let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
                    let w = crate::emit_code::ToCodeLiteral::to_code_literal(&self.w);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path::new(#x, #y, #z, #w) }
                }
            }
        }
    };
}

// ── f32 vectors ──────────────────────────────────────────────────────────────

bevy_vec2!(
    BevyVec2,
    bevy::math::Vec2,
    f32,
    "bevy::math::Vec2",
    "2D f32 vector (x, y):",
    bevy::math::Vec2::new,
    "bevy::math::Vec2"
);

bevy_vec3!(
    BevyVec3,
    bevy::math::Vec3,
    f32,
    "bevy::math::Vec3",
    "3D f32 vector (x, y, z):",
    bevy::math::Vec3::new,
    "bevy::math::Vec3"
);

bevy_vec4!(
    BevyVec4,
    bevy::math::Vec4,
    f32,
    "bevy::math::Vec4",
    "4D f32 vector (x, y, z, w):",
    bevy::math::Vec4::new,
    "bevy::math::Vec4"
);

// ── f64 vectors ──────────────────────────────────────────────────────────────

bevy_vec2!(
    BevyDVec2,
    bevy::math::DVec2,
    f64,
    "bevy::math::DVec2",
    "2D f64 vector (x, y):",
    bevy::math::DVec2::new,
    "bevy::math::DVec2"
);

bevy_vec3!(
    BevyDVec3,
    bevy::math::DVec3,
    f64,
    "bevy::math::DVec3",
    "3D f64 vector (x, y, z):",
    bevy::math::DVec3::new,
    "bevy::math::DVec3"
);

bevy_vec4!(
    BevyDVec4,
    bevy::math::DVec4,
    f64,
    "bevy::math::DVec4",
    "4D f64 vector (x, y, z, w):",
    bevy::math::DVec4::new,
    "bevy::math::DVec4"
);

// ── i32 vectors ──────────────────────────────────────────────────────────────

bevy_vec2!(
    BevyIVec2,
    bevy::math::IVec2,
    i32,
    "bevy::math::IVec2",
    "2D i32 vector (x, y):",
    bevy::math::IVec2::new,
    "bevy::math::IVec2"
);

bevy_vec3!(
    BevyIVec3,
    bevy::math::IVec3,
    i32,
    "bevy::math::IVec3",
    "3D i32 vector (x, y, z):",
    bevy::math::IVec3::new,
    "bevy::math::IVec3"
);

bevy_vec4!(
    BevyIVec4,
    bevy::math::IVec4,
    i32,
    "bevy::math::IVec4",
    "4D i32 vector (x, y, z, w):",
    bevy::math::IVec4::new,
    "bevy::math::IVec4"
);

// ── u32 vectors ──────────────────────────────────────────────────────────────

bevy_vec2!(
    BevyUVec2,
    bevy::math::UVec2,
    u32,
    "bevy::math::UVec2",
    "2D u32 vector (x, y):",
    bevy::math::UVec2::new,
    "bevy::math::UVec2"
);

bevy_vec3!(
    BevyUVec3,
    bevy::math::UVec3,
    u32,
    "bevy::math::UVec3",
    "3D u32 vector (x, y, z):",
    bevy::math::UVec3::new,
    "bevy::math::UVec3"
);

bevy_vec4!(
    BevyUVec4,
    bevy::math::UVec4,
    u32,
    "bevy::math::UVec4",
    "4D u32 vector (x, y, z, w):",
    bevy::math::UVec4::new,
    "bevy::math::UVec4"
);

// ── bool vectors ─────────────────────────────────────────────────────────────

bevy_vec2!(
    BevyBVec2,
    bevy::math::BVec2,
    bool,
    "bevy::math::BVec2",
    "2D bool vector (x, y):",
    bevy::math::BVec2::new,
    "bevy::math::BVec2"
);

bevy_vec3!(
    BevyBVec3,
    bevy::math::BVec3,
    bool,
    "bevy::math::BVec3",
    "3D bool vector (x, y, z):",
    bevy::math::BVec3::new,
    "bevy::math::BVec3"
);

bevy_vec4!(
    BevyBVec4,
    bevy::math::BVec4,
    bool,
    "bevy::math::BVec4",
    "4D bool vector (x, y, z, w):",
    bevy::math::BVec4::new,
    "bevy::math::BVec4"
);

// ── SIMD-aligned f32 vec3 ─────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::math::Vec3A`] (SIMD-aligned 3D f32 vector).
///
/// Stores as plain `f32` fields; converts to/from the SIMD-aligned upstream type.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct BevyVec3A {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
}

crate::default_style!(BevyVec3A => BevyVec3AStyle);

impl BevyVec3A {
    /// Converts this wrapper into the upstream SIMD type.
    pub fn into_inner(self) -> bevy::math::Vec3A {
        self.into()
    }
}

impl From<bevy::math::Vec3A> for BevyVec3A {
    fn from(v: bevy::math::Vec3A) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<BevyVec3A> for bevy::math::Vec3A {
    fn from(v: BevyVec3A) -> Self {
        bevy::math::Vec3A::new(v.x, v.y, v.z)
    }
}

impl Prompt for BevyVec3A {
    fn prompt() -> Option<&'static str> {
        Some("SIMD-aligned 3D f32 vector (x, y, z):")
    }
}

impl Elicitation for BevyVec3A {
    type Style = BevyVec3AStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::math::Vec3A"))]
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

impl ElicitIntrospect for BevyVec3A {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::math::Vec3A",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X component:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y component:"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "f32",
                        prompt: Some("Z component:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyVec3A {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::math::Vec3A".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
                ("z".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyVec3A {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = crate::emit_code::ToCodeLiteral::to_code_literal(&self.x);
        let y = crate::emit_code::ToCodeLiteral::to_code_literal(&self.y);
        let z = crate::emit_code::ToCodeLiteral::to_code_literal(&self.z);
        quote::quote! { bevy::math::Vec3A::new(#x, #y, #z) }
    }
}
