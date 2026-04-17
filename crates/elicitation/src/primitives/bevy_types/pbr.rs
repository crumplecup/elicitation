//! Bevy 0.18 PBR material and light type elicitation.
//!
//! Covers:
//! - [`BevyStandardMaterial`] — scalar fields from `bevy::pbr::StandardMaterial`
//!   (texture handles are omitted; only numeric and boolean parameters are elicited).
//! - [`BevyDirectionalLight`] — trenchcoat for `bevy::light::DirectionalLight`.
//! - [`BevyPointLight`] — trenchcoat for `bevy::light::PointLight`.
//! - [`BevySpotLight`] — trenchcoat for `bevy::light::SpotLight`.
//! - [`BevyAmbientLight`] — trenchcoat for `bevy::light::AmbientLight`.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyStandardMaterial ─────────────────────────────────────────────────────

/// Elicitable trenchcoat for the scalar fields of
/// [`bevy::pbr::StandardMaterial`].
///
/// Texture handles and UV channel selectors are excluded; only the numeric and
/// boolean parameters that can be meaningfully elicited from an AI agent are
/// included.  Colors are represented as individual `f32` components to avoid
/// cross-module dependencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyStandardMaterial {
    /// Base color — red channel [0, 1].
    pub base_color_r: f32,
    /// Base color — green channel [0, 1].
    pub base_color_g: f32,
    /// Base color — blue channel [0, 1].
    pub base_color_b: f32,
    /// Base color — alpha channel [0, 1].
    pub base_color_a: f32,
    /// Emissive — red channel (HDR, can exceed 1.0).
    pub emissive_r: f32,
    /// Emissive — green channel (HDR).
    pub emissive_g: f32,
    /// Emissive — blue channel (HDR).
    pub emissive_b: f32,
    /// Emissive exposure weight [0, 1].
    pub emissive_exposure_weight: f32,
    /// Perceptual roughness [0, 1].
    pub perceptual_roughness: f32,
    /// Metallic factor [0, 1].
    pub metallic: f32,
    /// Reflectance (dielectric specular strength) [0, 1].
    pub reflectance: f32,
    /// Index of refraction (≥ 1.0).
    pub ior: f32,
    /// Whether the material is double-sided.
    pub double_sided: bool,
    /// Disables all lighting calculations.
    pub unlit: bool,
    /// Whether the material is affected by fog.
    pub fog_enabled: bool,
    /// Constant depth bias applied to the material.
    pub depth_bias: f32,
}

impl BevyStandardMaterial {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::pbr::StandardMaterial {
        self.into()
    }
}

impl From<&bevy::pbr::StandardMaterial> for BevyStandardMaterial {
    fn from(m: &bevy::pbr::StandardMaterial) -> Self {
        use bevy::color::ColorToComponents as _;
        let base = bevy::color::LinearRgba::from(m.base_color).to_f32_array();
        let emit = m.emissive.to_f32_array();
        Self {
            base_color_r: base[0],
            base_color_g: base[1],
            base_color_b: base[2],
            base_color_a: base[3],
            emissive_r: emit[0],
            emissive_g: emit[1],
            emissive_b: emit[2],
            emissive_exposure_weight: m.emissive_exposure_weight,
            perceptual_roughness: m.perceptual_roughness,
            metallic: m.metallic,
            reflectance: m.reflectance,
            ior: m.ior,
            double_sided: m.double_sided,
            unlit: m.unlit,
            fog_enabled: m.fog_enabled,
            depth_bias: m.depth_bias,
        }
    }
}

impl From<BevyStandardMaterial> for bevy::pbr::StandardMaterial {
    fn from(b: BevyStandardMaterial) -> Self {
        let mut m = bevy::pbr::StandardMaterial::default();
        m.base_color = bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
            red: b.base_color_r,
            green: b.base_color_g,
            blue: b.base_color_b,
            alpha: b.base_color_a,
        });
        m.emissive = bevy::color::LinearRgba {
            red: b.emissive_r,
            green: b.emissive_g,
            blue: b.emissive_b,
            alpha: 1.0,
        };
        m.emissive_exposure_weight = b.emissive_exposure_weight;
        m.perceptual_roughness = b.perceptual_roughness;
        m.metallic = b.metallic;
        m.reflectance = b.reflectance;
        m.ior = b.ior;
        m.double_sided = b.double_sided;
        m.unlit = b.unlit;
        m.fog_enabled = b.fog_enabled;
        m.depth_bias = b.depth_bias;
        m
    }
}

crate::default_style!(BevyStandardMaterial => BevyStandardMaterialStyle);

impl Prompt for BevyStandardMaterial {
    fn prompt() -> Option<&'static str> {
        Some("Configure PBR material properties:")
    }
}

impl Elicitation for BevyStandardMaterial {
    type Style = BevyStandardMaterialStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "bevy::pbr::StandardMaterial"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            base_color_r: f32::elicit(communicator).await?,
            base_color_g: f32::elicit(communicator).await?,
            base_color_b: f32::elicit(communicator).await?,
            base_color_a: f32::elicit(communicator).await?,
            emissive_r: f32::elicit(communicator).await?,
            emissive_g: f32::elicit(communicator).await?,
            emissive_b: f32::elicit(communicator).await?,
            emissive_exposure_weight: f32::elicit(communicator).await?,
            perceptual_roughness: f32::elicit(communicator).await?,
            metallic: f32::elicit(communicator).await?,
            reflectance: f32::elicit(communicator).await?,
            ior: f32::elicit(communicator).await?,
            double_sided: bool::elicit(communicator).await?,
            unlit: bool::elicit(communicator).await?,
            fog_enabled: bool::elicit(communicator).await?,
            depth_bias: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyStandardMaterial {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::pbr::StandardMaterial",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "base_color_r",
                        type_name: "f32",
                        prompt: Some("Base color red [0,1]:"),
                    },
                    FieldInfo {
                        name: "base_color_g",
                        type_name: "f32",
                        prompt: Some("Base color green [0,1]:"),
                    },
                    FieldInfo {
                        name: "base_color_b",
                        type_name: "f32",
                        prompt: Some("Base color blue [0,1]:"),
                    },
                    FieldInfo {
                        name: "base_color_a",
                        type_name: "f32",
                        prompt: Some("Base color alpha [0,1]:"),
                    },
                    FieldInfo {
                        name: "emissive_r",
                        type_name: "f32",
                        prompt: Some("Emissive red (HDR):"),
                    },
                    FieldInfo {
                        name: "emissive_g",
                        type_name: "f32",
                        prompt: Some("Emissive green (HDR):"),
                    },
                    FieldInfo {
                        name: "emissive_b",
                        type_name: "f32",
                        prompt: Some("Emissive blue (HDR):"),
                    },
                    FieldInfo {
                        name: "emissive_exposure_weight",
                        type_name: "f32",
                        prompt: Some("Emissive exposure weight [0,1]:"),
                    },
                    FieldInfo {
                        name: "perceptual_roughness",
                        type_name: "f32",
                        prompt: Some("Perceptual roughness [0,1]:"),
                    },
                    FieldInfo {
                        name: "metallic",
                        type_name: "f32",
                        prompt: Some("Metallic factor [0,1]:"),
                    },
                    FieldInfo {
                        name: "reflectance",
                        type_name: "f32",
                        prompt: Some("Reflectance [0,1]:"),
                    },
                    FieldInfo {
                        name: "ior",
                        type_name: "f32",
                        prompt: Some("Index of refraction (≥1.0):"),
                    },
                    FieldInfo {
                        name: "double_sided",
                        type_name: "bool",
                        prompt: Some("Double sided?"),
                    },
                    FieldInfo {
                        name: "unlit",
                        type_name: "bool",
                        prompt: Some("Unlit (disable lighting)?"),
                    },
                    FieldInfo {
                        name: "fog_enabled",
                        type_name: "bool",
                        prompt: Some("Affected by fog?"),
                    },
                    FieldInfo {
                        name: "depth_bias",
                        type_name: "f32",
                        prompt: Some("Depth bias:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyStandardMaterial {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::pbr::StandardMaterial".to_string(),
            fields: vec![
                ("base_color_r".to_string(), Box::new(f32::prompt_tree())),
                ("base_color_g".to_string(), Box::new(f32::prompt_tree())),
                ("base_color_b".to_string(), Box::new(f32::prompt_tree())),
                ("base_color_a".to_string(), Box::new(f32::prompt_tree())),
                ("emissive_r".to_string(), Box::new(f32::prompt_tree())),
                ("emissive_g".to_string(), Box::new(f32::prompt_tree())),
                ("emissive_b".to_string(), Box::new(f32::prompt_tree())),
                (
                    "emissive_exposure_weight".to_string(),
                    Box::new(f32::prompt_tree()),
                ),
                (
                    "perceptual_roughness".to_string(),
                    Box::new(f32::prompt_tree()),
                ),
                ("metallic".to_string(), Box::new(f32::prompt_tree())),
                ("reflectance".to_string(), Box::new(f32::prompt_tree())),
                ("ior".to_string(), Box::new(f32::prompt_tree())),
                ("double_sided".to_string(), Box::new(bool::prompt_tree())),
                ("unlit".to_string(), Box::new(bool::prompt_tree())),
                ("fog_enabled".to_string(), Box::new(bool::prompt_tree())),
                ("depth_bias".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyStandardMaterial {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let bcr = crate::emit_code::ToCodeLiteral::to_code_literal(&self.base_color_r);
        let bcg = crate::emit_code::ToCodeLiteral::to_code_literal(&self.base_color_g);
        let bcb = crate::emit_code::ToCodeLiteral::to_code_literal(&self.base_color_b);
        let bca = crate::emit_code::ToCodeLiteral::to_code_literal(&self.base_color_a);
        let er = crate::emit_code::ToCodeLiteral::to_code_literal(&self.emissive_r);
        let eg = crate::emit_code::ToCodeLiteral::to_code_literal(&self.emissive_g);
        let eb = crate::emit_code::ToCodeLiteral::to_code_literal(&self.emissive_b);
        let eew = crate::emit_code::ToCodeLiteral::to_code_literal(&self.emissive_exposure_weight);
        let pr = crate::emit_code::ToCodeLiteral::to_code_literal(&self.perceptual_roughness);
        let met = crate::emit_code::ToCodeLiteral::to_code_literal(&self.metallic);
        let ref_ = crate::emit_code::ToCodeLiteral::to_code_literal(&self.reflectance);
        let ior = crate::emit_code::ToCodeLiteral::to_code_literal(&self.ior);
        let ds = crate::emit_code::ToCodeLiteral::to_code_literal(&self.double_sided);
        let unlit = crate::emit_code::ToCodeLiteral::to_code_literal(&self.unlit);
        let fog = crate::emit_code::ToCodeLiteral::to_code_literal(&self.fog_enabled);
        let db = crate::emit_code::ToCodeLiteral::to_code_literal(&self.depth_bias);
        quote::quote! {
            {
                let mut mat = bevy::pbr::StandardMaterial::default();
                mat.base_color = bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                    red: #bcr, green: #bcg, blue: #bcb, alpha: #bca,
                });
                mat.emissive = bevy::color::LinearRgba {
                    red: #er, green: #eg, blue: #eb, alpha: 1.0,
                };
                mat.emissive_exposure_weight = #eew;
                mat.perceptual_roughness = #pr;
                mat.metallic = #met;
                mat.reflectance = #ref_;
                mat.ior = #ior;
                mat.double_sided = #ds;
                mat.unlit = #unlit;
                mat.fog_enabled = #fog;
                mat.depth_bias = #db;
                mat
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::pbr::StandardMaterial }
    }
}

// ── Shared helper macro for light struct trenchcoats ─────────────────────────

macro_rules! bevy_light_struct {
    (
        $name:ident,
        $upstream:path,
        $type_name:literal,
        $prompt:literal,
        $([$field:ident : $fty:ty, $fprompt:literal]),+ $(,)?
    ) => {
        paste::paste! {
            #[doc = concat!("Elicitable trenchcoat for [`", $type_name, "`].")]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
            pub struct $name {
                $(
                    #[doc = $fprompt]
                    pub $field: $fty,
                )+
            }

            crate::default_style!($name => [<$name Style>]);

            impl $name {
                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream { self.into() }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> { Some($prompt) }
            }

            impl Elicitation for $name {
                type Style = [<$name Style>];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(
                    communicator: &C,
                ) -> ElicitResult<Self> {
                    Ok(Self {
                        $(
                            $field: <$fty as Elicitation>::elicit(communicator).await?,
                        )+
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

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                $(
                                    FieldInfo {
                                        name: stringify!($field),
                                        type_name: stringify!($fty),
                                        prompt: Some($fprompt),
                                    },
                                )+
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
                            $(
                                (
                                    stringify!($field).to_string(),
                                    Box::new(<$fty as crate::ElicitPromptTree>::prompt_tree()),
                                ),
                            )+
                        ],
                    }
                }
            }
        }
    };
}

// ── BevyDirectionalLight ──────────────────────────────────────────────────────

bevy_light_struct!(
    BevyDirectionalLight,
    bevy::light::DirectionalLight,
    "bevy::light::DirectionalLight",
    "Configure a directional light (sun-like, infinite range):",
    [color_r: f32, "Color red [0,1]:"],
    [color_g: f32, "Color green [0,1]:"],
    [color_b: f32, "Color blue [0,1]:"],
    [illuminance: f32, "Illuminance in lux (e.g. 10000):"],
    [shadows_enabled: bool, "Cast shadows?"],
);

impl From<&bevy::light::DirectionalLight> for BevyDirectionalLight {
    fn from(l: &bevy::light::DirectionalLight) -> Self {
        use bevy::color::ColorToComponents as _;
        let rgb = bevy::color::LinearRgba::from(l.color).to_f32_array();
        Self {
            color_r: rgb[0],
            color_g: rgb[1],
            color_b: rgb[2],
            illuminance: l.illuminance,
            shadows_enabled: l.shadows_enabled,
        }
    }
}

impl From<BevyDirectionalLight> for bevy::light::DirectionalLight {
    fn from(b: BevyDirectionalLight) -> Self {
        let mut l = bevy::light::DirectionalLight::default();
        l.color = bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
            red: b.color_r,
            green: b.color_g,
            blue: b.color_b,
            alpha: 1.0,
        });
        l.illuminance = b.illuminance;
        l.shadows_enabled = b.shadows_enabled;
        l
    }
}

impl crate::emit_code::ToCodeLiteral for BevyDirectionalLight {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_r);
        let g = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_g);
        let b = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_b);
        let illum = crate::emit_code::ToCodeLiteral::to_code_literal(&self.illuminance);
        let shad = crate::emit_code::ToCodeLiteral::to_code_literal(&self.shadows_enabled);
        quote::quote! {
            bevy::light::DirectionalLight {
                color: bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                    red: #r, green: #g, blue: #b, alpha: 1.0,
                }),
                illuminance: #illum,
                shadows_enabled: #shad,
                ..bevy::light::DirectionalLight::default()
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::light::DirectionalLight }
    }
}

// ── BevyPointLight ────────────────────────────────────────────────────────────

bevy_light_struct!(
    BevyPointLight,
    bevy::light::PointLight,
    "bevy::light::PointLight",
    "Configure a point light (omnidirectional):",
    [color_r: f32, "Color red [0,1]:"],
    [color_g: f32, "Color green [0,1]:"],
    [color_b: f32, "Color blue [0,1]:"],
    [intensity: f32, "Luminous power in lumens (e.g. 800):"],
    [range: f32, "Range in world units:"],
    [radius: f32, "Source radius (affects specular highlights):"],
    [shadows_enabled: bool, "Cast shadows?"],
);

impl From<&bevy::light::PointLight> for BevyPointLight {
    fn from(l: &bevy::light::PointLight) -> Self {
        use bevy::color::ColorToComponents as _;
        let rgb = bevy::color::LinearRgba::from(l.color).to_f32_array();
        Self {
            color_r: rgb[0],
            color_g: rgb[1],
            color_b: rgb[2],
            intensity: l.intensity,
            range: l.range,
            radius: l.radius,
            shadows_enabled: l.shadows_enabled,
        }
    }
}

impl From<BevyPointLight> for bevy::light::PointLight {
    fn from(b: BevyPointLight) -> Self {
        let mut l = bevy::light::PointLight::default();
        l.color = bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
            red: b.color_r,
            green: b.color_g,
            blue: b.color_b,
            alpha: 1.0,
        });
        l.intensity = b.intensity;
        l.range = b.range;
        l.radius = b.radius;
        l.shadows_enabled = b.shadows_enabled;
        l
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPointLight {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_r);
        let g = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_g);
        let b = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_b);
        let intensity = crate::emit_code::ToCodeLiteral::to_code_literal(&self.intensity);
        let range = crate::emit_code::ToCodeLiteral::to_code_literal(&self.range);
        let radius = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
        let shad = crate::emit_code::ToCodeLiteral::to_code_literal(&self.shadows_enabled);
        quote::quote! {
            bevy::light::PointLight {
                color: bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                    red: #r, green: #g, blue: #b, alpha: 1.0,
                }),
                intensity: #intensity,
                range: #range,
                radius: #radius,
                shadows_enabled: #shad,
                ..bevy::light::PointLight::default()
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::light::PointLight }
    }
}

// ── BevySpotLight ─────────────────────────────────────────────────────────────

bevy_light_struct!(
    BevySpotLight,
    bevy::light::SpotLight,
    "bevy::light::SpotLight",
    "Configure a spot light (cone-shaped):",
    [color_r: f32, "Color red [0,1]:"],
    [color_g: f32, "Color green [0,1]:"],
    [color_b: f32, "Color blue [0,1]:"],
    [intensity: f32, "Luminous power in lumens:"],
    [range: f32, "Range in world units:"],
    [radius: f32, "Source radius:"],
    [outer_angle: f32, "Outer cone half-angle in radians (0 < angle ≤ π/2):"],
    [inner_angle: f32, "Inner cone half-angle in radians (0 ≤ angle < outer_angle):"],
    [shadows_enabled: bool, "Cast shadows?"],
);

impl From<&bevy::light::SpotLight> for BevySpotLight {
    fn from(l: &bevy::light::SpotLight) -> Self {
        use bevy::color::ColorToComponents as _;
        let rgb = bevy::color::LinearRgba::from(l.color).to_f32_array();
        Self {
            color_r: rgb[0],
            color_g: rgb[1],
            color_b: rgb[2],
            intensity: l.intensity,
            range: l.range,
            radius: l.radius,
            outer_angle: l.outer_angle,
            inner_angle: l.inner_angle,
            shadows_enabled: l.shadows_enabled,
        }
    }
}

impl From<BevySpotLight> for bevy::light::SpotLight {
    fn from(b: BevySpotLight) -> Self {
        let mut l = bevy::light::SpotLight::default();
        l.color = bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
            red: b.color_r,
            green: b.color_g,
            blue: b.color_b,
            alpha: 1.0,
        });
        l.intensity = b.intensity;
        l.range = b.range;
        l.radius = b.radius;
        l.outer_angle = b.outer_angle;
        l.inner_angle = b.inner_angle;
        l.shadows_enabled = b.shadows_enabled;
        l
    }
}

impl crate::emit_code::ToCodeLiteral for BevySpotLight {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_r);
        let g = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_g);
        let b = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_b);
        let intensity = crate::emit_code::ToCodeLiteral::to_code_literal(&self.intensity);
        let range = crate::emit_code::ToCodeLiteral::to_code_literal(&self.range);
        let radius = crate::emit_code::ToCodeLiteral::to_code_literal(&self.radius);
        let outer = crate::emit_code::ToCodeLiteral::to_code_literal(&self.outer_angle);
        let inner = crate::emit_code::ToCodeLiteral::to_code_literal(&self.inner_angle);
        let shad = crate::emit_code::ToCodeLiteral::to_code_literal(&self.shadows_enabled);
        quote::quote! {
            bevy::light::SpotLight {
                color: bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                    red: #r, green: #g, blue: #b, alpha: 1.0,
                }),
                intensity: #intensity,
                range: #range,
                radius: #radius,
                outer_angle: #outer,
                inner_angle: #inner,
                shadows_enabled: #shad,
                ..bevy::light::SpotLight::default()
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::light::SpotLight }
    }
}

// ── BevyAmbientLight ──────────────────────────────────────────────────────────

bevy_light_struct!(
    BevyAmbientLight,
    bevy::light::AmbientLight,
    "bevy::light::AmbientLight",
    "Configure ambient (scene-wide) lighting:",
    [color_r: f32, "Color red [0,1]:"],
    [color_g: f32, "Color green [0,1]:"],
    [color_b: f32, "Color blue [0,1]:"],
    [brightness: f32, "Brightness in cd/m² (e.g. 80):"],
    [affects_lightmapped_meshes: bool, "Affect lightmapped meshes?"],
);

impl From<&bevy::light::AmbientLight> for BevyAmbientLight {
    fn from(l: &bevy::light::AmbientLight) -> Self {
        use bevy::color::ColorToComponents as _;
        let rgb = bevy::color::LinearRgba::from(l.color).to_f32_array();
        Self {
            color_r: rgb[0],
            color_g: rgb[1],
            color_b: rgb[2],
            brightness: l.brightness,
            affects_lightmapped_meshes: l.affects_lightmapped_meshes,
        }
    }
}

impl From<BevyAmbientLight> for bevy::light::AmbientLight {
    fn from(b: BevyAmbientLight) -> Self {
        bevy::light::AmbientLight {
            color: bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                red: b.color_r,
                green: b.color_g,
                blue: b.color_b,
                alpha: 1.0,
            }),
            brightness: b.brightness,
            affects_lightmapped_meshes: b.affects_lightmapped_meshes,
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAmbientLight {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_r);
        let g = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_g);
        let b = crate::emit_code::ToCodeLiteral::to_code_literal(&self.color_b);
        let bright = crate::emit_code::ToCodeLiteral::to_code_literal(&self.brightness);
        let alm =
            crate::emit_code::ToCodeLiteral::to_code_literal(&self.affects_lightmapped_meshes);
        quote::quote! {
            bevy::light::AmbientLight {
                color: bevy::color::Color::LinearRgba(bevy::color::LinearRgba {
                    red: #r, green: #g, blue: #b, alpha: 1.0,
                }),
                brightness: #bright,
                affects_lightmapped_meshes: #alm,
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::light::AmbientLight }
    }
}
