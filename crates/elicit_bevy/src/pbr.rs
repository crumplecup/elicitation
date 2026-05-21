//! PBR material and rendering wrappers.
//!
//! Covers [`StandardMaterial`], [`AlphaMode`], [`Tonemapping`],
//! [`UvChannel`], [`ParallaxMappingMethod`], [`OpaqueRendererMethod`], [`FogFalloff`],
//! [`AtmosphereMode`], [`AtmosphereSettings`], [`ForwardDecal`], and [`ForwardDecalMaterialExt`].

use crate::{Color, LinearRgba};
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── AlphaMode ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::render::alpha::AlphaMode, as AlphaMode);
elicit_newtype_traits!(AlphaMode, bevy::render::alpha::AlphaMode, [eq]);

impl From<AlphaMode> for bevy::render::alpha::AlphaMode {
    fn from(v: AlphaMode) -> Self {
        *v.0
    }
}

impl serde::Serialize for AlphaMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(2))?;
        match *self.0 {
            bevy::render::alpha::AlphaMode::Opaque => {
                map.serialize_entry("variant", "Opaque")?;
            }
            bevy::render::alpha::AlphaMode::Mask(threshold) => {
                map.serialize_entry("variant", "Mask")?;
                map.serialize_entry("threshold", &threshold)?;
            }
            bevy::render::alpha::AlphaMode::Blend => {
                map.serialize_entry("variant", "Blend")?;
            }
            bevy::render::alpha::AlphaMode::Premultiplied => {
                map.serialize_entry("variant", "Premultiplied")?;
            }
            bevy::render::alpha::AlphaMode::AlphaToCoverage => {
                map.serialize_entry("variant", "AlphaToCoverage")?;
            }
            bevy::render::alpha::AlphaMode::Add => {
                map.serialize_entry("variant", "Add")?;
            }
            bevy::render::alpha::AlphaMode::Multiply => {
                map.serialize_entry("variant", "Multiply")?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AlphaMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};

        struct AlphaModeVisitor;

        impl<'de> Visitor<'de> for AlphaModeVisitor {
            type Value = AlphaMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    formatter,
                    r#"an object with "variant" and optional "threshold""#
                )
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<AlphaMode, A::Error> {
                let mut variant: Option<String> = None;
                let mut threshold: Option<f32> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        "threshold" => threshold = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }

                let inner = match variant.as_deref() {
                    Some("Opaque") => bevy::render::alpha::AlphaMode::Opaque,
                    Some("Mask") => bevy::render::alpha::AlphaMode::Mask(threshold.unwrap_or(0.5)),
                    Some("Blend") => bevy::render::alpha::AlphaMode::Blend,
                    Some("Premultiplied") => bevy::render::alpha::AlphaMode::Premultiplied,
                    Some("AlphaToCoverage") => bevy::render::alpha::AlphaMode::AlphaToCoverage,
                    Some("Add") => bevy::render::alpha::AlphaMode::Add,
                    Some("Multiply") => bevy::render::alpha::AlphaMode::Multiply,
                    Some(other) => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &[
                                "Opaque",
                                "Mask",
                                "Blend",
                                "Premultiplied",
                                "AlphaToCoverage",
                                "Add",
                                "Multiply",
                            ],
                        ));
                    }
                    None => return Err(de::Error::missing_field("variant")),
                };

                Ok(AlphaMode(Arc::new(inner)))
            }
        }

        deserializer.deserialize_map(AlphaModeVisitor)
    }
}

#[reflect_methods]
impl AlphaMode {
    /// Returns the variant name as a string.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match *self.0 {
            bevy::render::alpha::AlphaMode::Opaque => "Opaque".to_string(),
            bevy::render::alpha::AlphaMode::Mask(_) => "Mask".to_string(),
            bevy::render::alpha::AlphaMode::Blend => "Blend".to_string(),
            bevy::render::alpha::AlphaMode::Premultiplied => "Premultiplied".to_string(),
            bevy::render::alpha::AlphaMode::AlphaToCoverage => "AlphaToCoverage".to_string(),
            bevy::render::alpha::AlphaMode::Add => "Add".to_string(),
            bevy::render::alpha::AlphaMode::Multiply => "Multiply".to_string(),
        }
    }

    /// Returns `true` if this is `AlphaMode::Opaque`.
    #[tracing::instrument(skip(self))]
    pub fn is_opaque(&self) -> bool {
        matches!(*self.0, bevy::render::alpha::AlphaMode::Opaque)
    }

    /// Returns `true` if this is `AlphaMode::Mask`.
    #[tracing::instrument(skip(self))]
    pub fn is_mask(&self) -> bool {
        matches!(*self.0, bevy::render::alpha::AlphaMode::Mask(_))
    }

    /// Returns `true` if this is `AlphaMode::Blend`.
    #[tracing::instrument(skip(self))]
    pub fn is_blend(&self) -> bool {
        matches!(*self.0, bevy::render::alpha::AlphaMode::Blend)
    }

    /// Returns the mask threshold if this is `AlphaMode::Mask`.
    #[tracing::instrument(skip(self))]
    pub fn get_mask_threshold(&self) -> Option<f32> {
        if let bevy::render::alpha::AlphaMode::Mask(c) = *self.0 {
            Some(c)
        } else {
            None
        }
    }

    /// Constructs an `AlphaMode::Opaque`.
    #[tracing::instrument(skip(self))]
    pub fn opaque(&self) -> AlphaMode {
        AlphaMode::from(bevy::render::alpha::AlphaMode::Opaque)
    }

    /// Constructs an `AlphaMode::Mask` with the given threshold.
    #[tracing::instrument(skip(self))]
    pub fn mask(&self, threshold: f32) -> AlphaMode {
        AlphaMode::from(bevy::render::alpha::AlphaMode::Mask(threshold))
    }

    /// Constructs an `AlphaMode::Blend`.
    #[tracing::instrument(skip(self))]
    pub fn blend(&self) -> AlphaMode {
        AlphaMode::from(bevy::render::alpha::AlphaMode::Blend)
    }
}

mod emit_impls_alpha_mode {
    use super::AlphaMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AlphaMode {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::render::alpha::AlphaMode::Opaque => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Opaque)
                },
                bevy::render::alpha::AlphaMode::Mask(c) => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Mask(#c))
                },
                bevy::render::alpha::AlphaMode::Blend => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Blend)
                },
                bevy::render::alpha::AlphaMode::Premultiplied => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Premultiplied)
                },
                bevy::render::alpha::AlphaMode::AlphaToCoverage => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::AlphaToCoverage)
                },
                bevy::render::alpha::AlphaMode::Add => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Add)
                },
                bevy::render::alpha::AlphaMode::Multiply => quote::quote! {
                    ::elicit_bevy::AlphaMode::from(::bevy::render::alpha::AlphaMode::Multiply)
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for AlphaMode {}

// ── Tonemapping ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::core_pipeline::tonemapping::Tonemapping, as Tonemapping);
elicit_newtype_traits!(
    Tonemapping,
    bevy::core_pipeline::tonemapping::Tonemapping,
    [eq]
);

impl From<Tonemapping> for bevy::core_pipeline::tonemapping::Tonemapping {
    fn from(v: Tonemapping) -> Self {
        *v.0
    }
}

impl serde::Serialize for Tonemapping {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Tonemapping {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "None" => bevy::core_pipeline::tonemapping::Tonemapping::None,
            "Reinhard" => bevy::core_pipeline::tonemapping::Tonemapping::Reinhard,
            "ReinhardLuminance" => bevy::core_pipeline::tonemapping::Tonemapping::ReinhardLuminance,
            "AcesFitted" => bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted,
            "AgX" => bevy::core_pipeline::tonemapping::Tonemapping::AgX,
            "SomewhatBoringDisplayTransform" => {
                bevy::core_pipeline::tonemapping::Tonemapping::SomewhatBoringDisplayTransform
            }
            "TonyMcMapface" => bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            "BlenderFilmic" => bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "None",
                        "Reinhard",
                        "ReinhardLuminance",
                        "AcesFitted",
                        "AgX",
                        "SomewhatBoringDisplayTransform",
                        "TonyMcMapface",
                        "BlenderFilmic",
                    ],
                ));
            }
        };
        Ok(Tonemapping(Arc::new(inner)))
    }
}

#[reflect_methods]
impl Tonemapping {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::core_pipeline::tonemapping::Tonemapping::None => "None",
            bevy::core_pipeline::tonemapping::Tonemapping::Reinhard => "Reinhard",
            bevy::core_pipeline::tonemapping::Tonemapping::ReinhardLuminance => "ReinhardLuminance",
            bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted => "AcesFitted",
            bevy::core_pipeline::tonemapping::Tonemapping::AgX => "AgX",
            bevy::core_pipeline::tonemapping::Tonemapping::SomewhatBoringDisplayTransform => {
                "SomewhatBoringDisplayTransform"
            }
            bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface => "TonyMcMapface",
            bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic => "BlenderFilmic",
        }
    }

    /// Returns `true` if this is `Tonemapping::None`.
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::core_pipeline::tonemapping::Tonemapping::None)
    }

    /// Returns `true` if this is `Tonemapping::AcesFitted`.
    #[tracing::instrument(skip(self))]
    pub fn is_aces(&self) -> bool {
        matches!(
            *self.0,
            bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted
        )
    }

    /// Returns `true` if this is `Tonemapping::AgX`.
    #[tracing::instrument(skip(self))]
    pub fn is_agx(&self) -> bool {
        matches!(*self.0, bevy::core_pipeline::tonemapping::Tonemapping::AgX)
    }
}

mod emit_impls_tonemapping {
    use super::Tonemapping;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Tonemapping {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::Tonemapping::from(
                    ::bevy::core_pipeline::tonemapping::Tonemapping::#variant
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Tonemapping {}

// ── UvChannel ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::UvChannel, as UvChannel);
elicit_newtype_traits!(UvChannel, bevy::pbr::UvChannel, [eq]);

impl From<UvChannel> for bevy::pbr::UvChannel {
    fn from(v: UvChannel) -> Self {
        v.0.as_ref().clone()
    }
}

impl serde::Serialize for UvChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for UvChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Uv0" => bevy::pbr::UvChannel::Uv0,
            "Uv1" => bevy::pbr::UvChannel::Uv1,
            _ => return Err(D::Error::unknown_variant(&s, &["Uv0", "Uv1"])),
        };
        Ok(UvChannel(Arc::new(inner)))
    }
}

#[reflect_methods]
impl UvChannel {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::pbr::UvChannel::Uv0 => "Uv0",
            bevy::pbr::UvChannel::Uv1 => "Uv1",
        }
    }

    /// Returns `true` if this is `UvChannel::Uv0`.
    #[tracing::instrument(skip(self))]
    pub fn is_uv0(&self) -> bool {
        matches!(*self.0, bevy::pbr::UvChannel::Uv0)
    }

    /// Returns `true` if this is `UvChannel::Uv1`.
    #[tracing::instrument(skip(self))]
    pub fn is_uv1(&self) -> bool {
        matches!(*self.0, bevy::pbr::UvChannel::Uv1)
    }
}

mod emit_impls_uv_channel {
    use super::UvChannel;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for UvChannel {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::UvChannel::from(::bevy::pbr::UvChannel::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for UvChannel {}

// ── ParallaxMappingMethod ─────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::ParallaxMappingMethod, as ParallaxMappingMethod);
elicit_newtype_traits!(
    ParallaxMappingMethod,
    bevy::pbr::ParallaxMappingMethod,
    [eq]
);

impl From<ParallaxMappingMethod> for bevy::pbr::ParallaxMappingMethod {
    fn from(v: ParallaxMappingMethod) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| *arc)
    }
}

impl serde::Serialize for ParallaxMappingMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        match &*self.0 {
            bevy::pbr::ParallaxMappingMethod::Occlusion => {
                map.serialize_entry("variant", "Occlusion")?;
            }
            bevy::pbr::ParallaxMappingMethod::Relief { max_steps } => {
                map.serialize_entry("variant", "Relief")?;
                map.serialize_entry("max_steps", max_steps)?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for ParallaxMappingMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ParallaxMappingMethod;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "variant": "Occlusion" | "Relief""#)
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<ParallaxMappingMethod, A::Error> {
                let mut variant: Option<String> = None;
                let mut max_steps: Option<u32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        "max_steps" => max_steps = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let v = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match v.as_str() {
                    "Occlusion" => bevy::pbr::ParallaxMappingMethod::Occlusion,
                    "Relief" => bevy::pbr::ParallaxMappingMethod::Relief {
                        max_steps: max_steps.unwrap_or(8),
                    },
                    other => {
                        return Err(de::Error::unknown_variant(other, &["Occlusion", "Relief"]));
                    }
                };
                Ok(ParallaxMappingMethod(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl ParallaxMappingMethod {
    /// Returns the variant name as a string.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match &*self.0 {
            bevy::pbr::ParallaxMappingMethod::Occlusion => "Occlusion".to_string(),
            bevy::pbr::ParallaxMappingMethod::Relief { .. } => "Relief".to_string(),
        }
    }

    /// Returns `true` if this is `ParallaxMappingMethod::Occlusion`.
    #[tracing::instrument(skip(self))]
    pub fn is_occlusion(&self) -> bool {
        matches!(*self.0, bevy::pbr::ParallaxMappingMethod::Occlusion)
    }

    /// Returns `true` if this is `ParallaxMappingMethod::Relief`.
    #[tracing::instrument(skip(self))]
    pub fn is_relief(&self) -> bool {
        matches!(*self.0, bevy::pbr::ParallaxMappingMethod::Relief { .. })
    }

    /// Returns the `max_steps` value if this is `ParallaxMappingMethod::Relief`.
    #[tracing::instrument(skip(self))]
    pub fn get_max_steps(&self) -> Option<u32> {
        if let bevy::pbr::ParallaxMappingMethod::Relief { max_steps } = *self.0 {
            Some(max_steps)
        } else {
            None
        }
    }
}

mod emit_impls_parallax {
    use super::ParallaxMappingMethod;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ParallaxMappingMethod {
        fn to_code_literal(&self) -> TokenStream {
            match &*self.0 {
                bevy::pbr::ParallaxMappingMethod::Occlusion => quote::quote! {
                    ::elicit_bevy::ParallaxMappingMethod::from(
                        ::bevy::pbr::ParallaxMappingMethod::Occlusion
                    )
                },
                bevy::pbr::ParallaxMappingMethod::Relief { max_steps } => quote::quote! {
                    ::elicit_bevy::ParallaxMappingMethod::from(
                        ::bevy::pbr::ParallaxMappingMethod::Relief { max_steps: #max_steps }
                    )
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for ParallaxMappingMethod {}

// ── OpaqueRendererMethod ──────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::OpaqueRendererMethod, as OpaqueRendererMethod);
elicit_newtype_traits!(OpaqueRendererMethod, bevy::pbr::OpaqueRendererMethod, [eq]);

impl From<OpaqueRendererMethod> for bevy::pbr::OpaqueRendererMethod {
    fn from(v: OpaqueRendererMethod) -> Self {
        *v.0
    }
}

impl serde::Serialize for OpaqueRendererMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for OpaqueRendererMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Auto" => bevy::pbr::OpaqueRendererMethod::Auto,
            "Forward" => bevy::pbr::OpaqueRendererMethod::Forward,
            "Deferred" => bevy::pbr::OpaqueRendererMethod::Deferred,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Auto", "Forward", "Deferred"],
                ));
            }
        };
        Ok(OpaqueRendererMethod(Arc::new(inner)))
    }
}

#[reflect_methods]
impl OpaqueRendererMethod {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::pbr::OpaqueRendererMethod::Auto => "Auto",
            bevy::pbr::OpaqueRendererMethod::Forward => "Forward",
            bevy::pbr::OpaqueRendererMethod::Deferred => "Deferred",
        }
    }

    /// Returns `true` if this is `OpaqueRendererMethod::Forward`.
    #[tracing::instrument(skip(self))]
    pub fn is_forward(&self) -> bool {
        matches!(*self.0, bevy::pbr::OpaqueRendererMethod::Forward)
    }

    /// Returns `true` if this is `OpaqueRendererMethod::Deferred`.
    #[tracing::instrument(skip(self))]
    pub fn is_deferred(&self) -> bool {
        matches!(*self.0, bevy::pbr::OpaqueRendererMethod::Deferred)
    }
}

mod emit_impls_opaque {
    use super::OpaqueRendererMethod;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OpaqueRendererMethod {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::OpaqueRendererMethod::from(
                    ::bevy::pbr::OpaqueRendererMethod::#variant
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for OpaqueRendererMethod {}

// ── StandardMaterial ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::StandardMaterial, as StandardMaterial);
elicit_newtype_traits!(StandardMaterial, bevy::pbr::StandardMaterial, []);

impl From<StandardMaterial> for bevy::pbr::StandardMaterial {
    fn from(v: StandardMaterial) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for StandardMaterial {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let m = &*self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("base_color", &format!("{:?}", m.base_color))?;
        map.serialize_entry("emissive", &format!("{:?}", m.emissive))?;
        map.serialize_entry("perceptual_roughness", &m.perceptual_roughness)?;
        map.serialize_entry("metallic", &m.metallic)?;
        map.serialize_entry("reflectance", &m.reflectance)?;
        map.serialize_entry("double_sided", &m.double_sided)?;
        map.serialize_entry("unlit", &m.unlit)?;
        map.serialize_entry("fog_enabled", &m.fog_enabled)?;
        map.serialize_entry("depth_bias", &m.depth_bias)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for StandardMaterial {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = StandardMaterial;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a StandardMaterial JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<StandardMaterial, A::Error> {
                let mut roughness: Option<f32> = None;
                let mut metallic: Option<f32> = None;
                let mut unlit: Option<bool> = None;
                let mut double_sided: Option<bool> = None;
                let mut fog_enabled: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "perceptual_roughness" => roughness = Some(map.next_value()?),
                        "metallic" => metallic = Some(map.next_value()?),
                        "unlit" => unlit = Some(map.next_value()?),
                        "double_sided" => double_sided = Some(map.next_value()?),
                        "fog_enabled" => fog_enabled = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut m = bevy::pbr::StandardMaterial::default();
                if let Some(r) = roughness {
                    m.perceptual_roughness = r;
                }
                if let Some(met) = metallic {
                    m.metallic = met;
                }
                if let Some(u) = unlit {
                    m.unlit = u;
                }
                if let Some(ds) = double_sided {
                    m.double_sided = ds;
                }
                if let Some(fe) = fog_enabled {
                    m.fog_enabled = fe;
                }
                Ok(StandardMaterial(Arc::new(m)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl StandardMaterial {
    /// Returns the base color.
    #[tracing::instrument(skip(self))]
    pub fn base_color(&self) -> Color {
        Color::from(self.0.base_color)
    }

    /// Returns the emissive color.
    #[tracing::instrument(skip(self))]
    pub fn emissive(&self) -> LinearRgba {
        LinearRgba::from(self.0.emissive)
    }

    /// Returns the perceptual roughness.
    #[tracing::instrument(skip(self))]
    pub fn roughness(&self) -> f32 {
        self.0.perceptual_roughness
    }

    /// Returns the metallic factor.
    #[tracing::instrument(skip(self))]
    pub fn metallic(&self) -> f32 {
        self.0.metallic
    }

    /// Returns the reflectance.
    #[tracing::instrument(skip(self))]
    pub fn reflectance(&self) -> f32 {
        self.0.reflectance
    }

    /// Returns the alpha mode.
    #[tracing::instrument(skip(self))]
    pub fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::from(self.0.alpha_mode)
    }

    /// Returns `true` if the material is double-sided.
    #[tracing::instrument(skip(self))]
    pub fn double_sided(&self) -> bool {
        self.0.double_sided
    }

    /// Returns `true` if the material is unlit.
    #[tracing::instrument(skip(self))]
    pub fn unlit(&self) -> bool {
        self.0.unlit
    }

    /// Returns `true` if fog is enabled on the material.
    #[tracing::instrument(skip(self))]
    pub fn fog_enabled(&self) -> bool {
        self.0.fog_enabled
    }

    /// Returns the depth bias.
    #[tracing::instrument(skip(self))]
    pub fn depth_bias(&self) -> f32 {
        self.0.depth_bias
    }

    /// Creates a new default [`StandardMaterial`].
    #[tracing::instrument(skip(self))]
    pub fn new_standard_material(&self) -> StandardMaterial {
        StandardMaterial::from(bevy::pbr::StandardMaterial::default())
    }

    /// Returns a copy of this material with the given base color.
    #[tracing::instrument(skip(self))]
    pub fn with_base_color(&self, color: Color) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.base_color = *color.0;
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the given perceptual roughness.
    #[tracing::instrument(skip(self))]
    pub fn with_roughness(&self, roughness: f32) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.perceptual_roughness = roughness;
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the given metallic factor.
    #[tracing::instrument(skip(self))]
    pub fn with_metallic(&self, metallic: f32) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.metallic = metallic;
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the given emissive color (r, g, b).
    #[tracing::instrument(skip(self))]
    pub fn with_emissive(&self, r: f32, g: f32, b: f32) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.emissive = bevy::color::LinearRgba::new(r, g, b, 1.0);
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the given alpha mode.
    #[tracing::instrument(skip(self))]
    pub fn with_alpha_mode(&self, mode: AlphaMode) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.alpha_mode = *mode.0;
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the unlit flag set.
    #[tracing::instrument(skip(self))]
    pub fn with_unlit(&self, unlit: bool) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.unlit = unlit;
        StandardMaterial::from(m)
    }

    /// Returns a copy of this material with the double-sided flag set.
    #[tracing::instrument(skip(self))]
    pub fn with_double_sided(&self, double_sided: bool) -> StandardMaterial {
        let mut m = (*self.0).clone();
        m.double_sided = double_sided;
        StandardMaterial::from(m)
    }
}

mod emit_impls_standard_material {
    use super::StandardMaterial;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for StandardMaterial {
        fn to_code_literal(&self) -> TokenStream {
            let roughness = self.0.perceptual_roughness;
            let metallic = self.0.metallic;
            let unlit = self.0.unlit;
            quote::quote! {
                ::elicit_bevy::StandardMaterial::from({
                    let mut m = ::bevy::pbr::StandardMaterial::default();
                    m.perceptual_roughness = #roughness;
                    m.metallic = #metallic;
                    m.unlit = #unlit;
                    m
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for StandardMaterial {}

// ── FogFalloff ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::FogFalloff, as FogFalloff);
elicit_newtype_traits!(FogFalloff, bevy::pbr::FogFalloff, []);

impl From<FogFalloff> for bevy::pbr::FogFalloff {
    fn from(v: FogFalloff) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for FogFalloff {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match &*self.0 {
            bevy::pbr::FogFalloff::Linear { start, end } => {
                let mut map = s.serialize_map(Some(3))?;
                map.serialize_entry("variant", "Linear")?;
                map.serialize_entry("start", start)?;
                map.serialize_entry("end", end)?;
                map.end()
            }
            bevy::pbr::FogFalloff::Exponential { density } => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("variant", "Exponential")?;
                map.serialize_entry("density", density)?;
                map.end()
            }
            bevy::pbr::FogFalloff::ExponentialSquared { density } => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("variant", "ExponentialSquared")?;
                map.serialize_entry("density", density)?;
                map.end()
            }
            bevy::pbr::FogFalloff::Atmospheric {
                extinction,
                inscattering,
            } => {
                let mut map = s.serialize_map(Some(3))?;
                map.serialize_entry("variant", "Atmospheric")?;
                map.serialize_entry("extinction", extinction)?;
                map.serialize_entry("inscattering", inscattering)?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for FogFalloff {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        use std::sync::Arc;

        struct FogFalloffVisitor;
        impl<'de> Visitor<'de> for FogFalloffVisitor {
            type Value = FogFalloff;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"FogFalloff object with "variant" field (Linear, Exponential, ExponentialSquared, Atmospheric)"#
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<FogFalloff, A::Error> {
                let mut variant: Option<String> = None;
                let mut start: Option<f32> = None;
                let mut end: Option<f32> = None;
                let mut density: Option<f32> = None;
                let mut extinction: Option<bevy::math::Vec3> = None;
                let mut inscattering: Option<bevy::math::Vec3> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        "start" => start = Some(map.next_value()?),
                        "end" => end = Some(map.next_value()?),
                        "density" => density = Some(map.next_value()?),
                        "extinction" => extinction = Some(map.next_value()?),
                        "inscattering" => inscattering = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let variant = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match variant.as_str() {
                    "Linear" => bevy::pbr::FogFalloff::Linear {
                        start: start.ok_or_else(|| de::Error::missing_field("start"))?,
                        end: end.ok_or_else(|| de::Error::missing_field("end"))?,
                    },
                    "Exponential" => bevy::pbr::FogFalloff::Exponential {
                        density: density.ok_or_else(|| de::Error::missing_field("density"))?,
                    },
                    "ExponentialSquared" => bevy::pbr::FogFalloff::ExponentialSquared {
                        density: density.ok_or_else(|| de::Error::missing_field("density"))?,
                    },
                    "Atmospheric" => bevy::pbr::FogFalloff::Atmospheric {
                        extinction: extinction
                            .ok_or_else(|| de::Error::missing_field("extinction"))?,
                        inscattering: inscattering
                            .ok_or_else(|| de::Error::missing_field("inscattering"))?,
                    },
                    other => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &["Linear", "Exponential", "ExponentialSquared", "Atmospheric"],
                        ));
                    }
                };
                Ok(FogFalloff(Arc::new(inner)))
            }
        }
        d.deserialize_map(FogFalloffVisitor)
    }
}

#[reflect_methods]
impl FogFalloff {
    /// Returns a string name for this variant.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match &*self.0 {
            bevy::pbr::FogFalloff::Linear { .. } => "Linear".to_string(),
            bevy::pbr::FogFalloff::Exponential { .. } => "Exponential".to_string(),
            bevy::pbr::FogFalloff::ExponentialSquared { .. } => "ExponentialSquared".to_string(),
            bevy::pbr::FogFalloff::Atmospheric { .. } => "Atmospheric".to_string(),
        }
    }

    /// Returns the density if this is `Exponential` or `ExponentialSquared`.
    #[tracing::instrument(skip(self))]
    pub fn get_density(&self) -> Option<f32> {
        match &*self.0 {
            bevy::pbr::FogFalloff::Exponential { density }
            | bevy::pbr::FogFalloff::ExponentialSquared { density } => Some(*density),
            _ => None,
        }
    }
}

mod emit_impls_fog_falloff {
    use super::FogFalloff;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FogFalloff {
        fn to_code_literal(&self) -> TokenStream {
            match &*self.0 {
                bevy::pbr::FogFalloff::Linear { start, end } => {
                    let s = *start;
                    let e = *end;
                    quote::quote! {
                        ::elicit_bevy::FogFalloff::from(::bevy::pbr::FogFalloff::Linear {
                            start: #s,
                            end: #e,
                        })
                    }
                }
                bevy::pbr::FogFalloff::Exponential { density } => {
                    let d = *density;
                    quote::quote! {
                        ::elicit_bevy::FogFalloff::from(
                            ::bevy::pbr::FogFalloff::Exponential { density: #d }
                        )
                    }
                }
                bevy::pbr::FogFalloff::ExponentialSquared { density } => {
                    let d = *density;
                    quote::quote! {
                        ::elicit_bevy::FogFalloff::from(
                            ::bevy::pbr::FogFalloff::ExponentialSquared { density: #d }
                        )
                    }
                }
                bevy::pbr::FogFalloff::Atmospheric {
                    extinction,
                    inscattering,
                } => {
                    let (ex, ey, ez) = (extinction.x, extinction.y, extinction.z);
                    let (ix, iy, iz) = (inscattering.x, inscattering.y, inscattering.z);
                    quote::quote! {
                        ::elicit_bevy::FogFalloff::from(
                            ::bevy::pbr::FogFalloff::Atmospheric {
                                extinction: ::bevy::math::Vec3::new(#ex, #ey, #ez),
                                inscattering: ::bevy::math::Vec3::new(#ix, #iy, #iz),
                            }
                        )
                    }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for FogFalloff {}

// ── shadow_elicitation + unit_elicitation macros ──────────────────────────────

macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }
            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }
            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }
            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }
        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }
            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }
        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }
        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
            }
            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }
            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }
            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }
        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }
            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }
        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }
        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── ForwardDecal ──────────────────────────────────────────────────────────────

/// Shadow of [`bevy::pbr::decal::ForwardDecal`].
///
/// Marker component that makes a mesh entity act as a forward decal.
/// Requires [`Mesh3d`](bevy::prelude::Mesh3d) to also be present on the entity.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct ForwardDecal;

impl From<ForwardDecal> for bevy::pbr::decal::ForwardDecal {
    fn from(_: ForwardDecal) -> Self {
        bevy::pbr::decal::ForwardDecal
    }
}

mod emit_impls_forward_decal {
    use super::ForwardDecal;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ForwardDecal {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::ForwardDecal }
        }
    }
}

unit_elicitation!(ForwardDecal, bevy::pbr::decal::ForwardDecal);

// ── ForwardDecalMaterialExt ───────────────────────────────────────────────────

/// Shadow of [`bevy::pbr::decal::ForwardDecalMaterialExt`].
///
/// Material extension that enables forward decal blending behavior.
/// Controls the distance threshold for decal blending with surfaces.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ForwardDecalMaterialExt {
    /// Distance threshold for decal blending with surfaces, in meters.
    ///
    /// Lower values cause the decal to only blend with close surfaces;
    /// higher values allow blending with more distant surfaces.
    pub depth_fade_factor: f32,
}

impl Default for ForwardDecalMaterialExt {
    fn default() -> Self {
        Self {
            depth_fade_factor: 0.5,
        }
    }
}

impl From<ForwardDecalMaterialExt> for bevy::pbr::decal::ForwardDecalMaterialExt {
    fn from(v: ForwardDecalMaterialExt) -> Self {
        bevy::pbr::decal::ForwardDecalMaterialExt {
            depth_fade_factor: v.depth_fade_factor,
        }
    }
}

mod emit_impls_forward_decal_material_ext {
    use super::ForwardDecalMaterialExt;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ForwardDecalMaterialExt {
        fn to_code_literal(&self) -> TokenStream {
            let depth_fade_factor = self.depth_fade_factor;
            quote::quote! {
                ::elicit_bevy::ForwardDecalMaterialExt {
                    depth_fade_factor: #depth_fade_factor,
                }
            }
        }
    }
}

shadow_elicitation!(ForwardDecalMaterialExt);

// ── AtmosphereMode ────────────────────────────────────────────────────────────

/// Shadow of [`bevy::pbr::AtmosphereMode`].
///
/// Selects the rendering algorithm used for atmosphere simulation.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum AtmosphereMode {
    /// High-performance lookup-texture method. Best for scenes mostly inside the atmosphere.
    #[default]
    LookupTexture,
    /// Slower raymarching method. More accurate for space views and sharp volumetric shadows.
    Raymarched,
}

impl From<AtmosphereMode> for bevy::pbr::AtmosphereMode {
    fn from(v: AtmosphereMode) -> Self {
        match v {
            AtmosphereMode::LookupTexture => bevy::pbr::AtmosphereMode::LookupTexture,
            AtmosphereMode::Raymarched => bevy::pbr::AtmosphereMode::Raymarched,
        }
    }
}

impl From<bevy::pbr::AtmosphereMode> for AtmosphereMode {
    fn from(v: bevy::pbr::AtmosphereMode) -> Self {
        match v {
            bevy::pbr::AtmosphereMode::LookupTexture => AtmosphereMode::LookupTexture,
            bevy::pbr::AtmosphereMode::Raymarched => AtmosphereMode::Raymarched,
        }
    }
}

mod emit_impls_atmosphere_mode {
    use super::AtmosphereMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AtmosphereMode {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                AtmosphereMode::LookupTexture => {
                    quote::quote! { ::elicit_bevy::AtmosphereMode::LookupTexture }
                }
                AtmosphereMode::Raymarched => {
                    quote::quote! { ::elicit_bevy::AtmosphereMode::Raymarched }
                }
            }
        }
    }
}

shadow_elicitation!(AtmosphereMode);

// ── AtmosphereSettings ────────────────────────────────────────────────────────

/// Shadow of [`bevy::pbr::AtmosphereSettings`].
///
/// Performance-tuning component for atmosphere LUT sizes and sample counts.
/// Usually left at defaults; attach to a camera entity alongside [`Atmosphere`](bevy::pbr::Atmosphere).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AtmosphereSettings {
    /// Transmittance LUT dimensions (width, height).
    pub transmittance_lut_size: crate::UVec2,
    /// Multiscattering LUT dimensions.
    pub multiscattering_lut_size: crate::UVec2,
    /// Sky-view LUT dimensions.
    pub sky_view_lut_size: crate::UVec2,
    /// Aerial-view LUT dimensions (width, height, depth).
    pub aerial_view_lut_size: crate::UVec3,
    /// Sample count along each ray for the transmittance LUT.
    pub transmittance_lut_samples: u32,
    /// Ray count per pixel for the multiscattering LUT.
    pub multiscattering_lut_dirs: u32,
    /// Sample count per ray for the multiscattering LUT.
    pub multiscattering_lut_samples: u32,
    /// Sample count per ray for the sky-view LUT.
    pub sky_view_lut_samples: u32,
    /// Sample count per z-slice for the aerial-view LUT.
    pub aerial_view_lut_samples: u32,
    /// Maximum distance (meters) for aerial-view LUT evaluation.
    pub aerial_view_lut_max_distance: f32,
    /// Scene-unit to meters conversion factor.
    pub scene_units_to_m: f32,
    /// Sample count per fragment for raymarched sky rendering.
    pub sky_max_samples: u32,
    /// Atmosphere rendering algorithm.
    pub rendering_method: AtmosphereMode,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        let d = bevy::pbr::AtmosphereSettings::default();
        Self {
            transmittance_lut_size: crate::UVec2::from(d.transmittance_lut_size),
            multiscattering_lut_size: crate::UVec2::from(d.multiscattering_lut_size),
            sky_view_lut_size: crate::UVec2::from(d.sky_view_lut_size),
            aerial_view_lut_size: crate::UVec3::from(d.aerial_view_lut_size),
            transmittance_lut_samples: d.transmittance_lut_samples,
            multiscattering_lut_dirs: d.multiscattering_lut_dirs,
            multiscattering_lut_samples: d.multiscattering_lut_samples,
            sky_view_lut_samples: d.sky_view_lut_samples,
            aerial_view_lut_samples: d.aerial_view_lut_samples,
            aerial_view_lut_max_distance: d.aerial_view_lut_max_distance,
            scene_units_to_m: d.scene_units_to_m,
            sky_max_samples: d.sky_max_samples,
            rendering_method: AtmosphereMode::from(d.rendering_method),
        }
    }
}

impl From<AtmosphereSettings> for bevy::pbr::AtmosphereSettings {
    fn from(v: AtmosphereSettings) -> Self {
        bevy::pbr::AtmosphereSettings {
            transmittance_lut_size: bevy::math::UVec2::from(v.transmittance_lut_size),
            multiscattering_lut_size: bevy::math::UVec2::from(v.multiscattering_lut_size),
            sky_view_lut_size: bevy::math::UVec2::from(v.sky_view_lut_size),
            aerial_view_lut_size: bevy::math::UVec3::from(v.aerial_view_lut_size),
            transmittance_lut_samples: v.transmittance_lut_samples,
            multiscattering_lut_dirs: v.multiscattering_lut_dirs,
            multiscattering_lut_samples: v.multiscattering_lut_samples,
            sky_view_lut_samples: v.sky_view_lut_samples,
            aerial_view_lut_samples: v.aerial_view_lut_samples,
            aerial_view_lut_max_distance: v.aerial_view_lut_max_distance,
            scene_units_to_m: v.scene_units_to_m,
            sky_max_samples: v.sky_max_samples,
            rendering_method: v.rendering_method.into(),
        }
    }
}

mod emit_impls_atmosphere_settings {
    use super::AtmosphereSettings;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AtmosphereSettings {
        fn to_code_literal(&self) -> TokenStream {
            let tls = self.transmittance_lut_size.to_code_literal();
            let mls = self.multiscattering_lut_size.to_code_literal();
            let svls = self.sky_view_lut_size.to_code_literal();
            let avls = self.aerial_view_lut_size.to_code_literal();
            let tlsamp = self.transmittance_lut_samples;
            let mldirs = self.multiscattering_lut_dirs;
            let mlsamp = self.multiscattering_lut_samples;
            let svlsamp = self.sky_view_lut_samples;
            let avlsamp = self.aerial_view_lut_samples;
            let avmax = self.aerial_view_lut_max_distance;
            let s2m = self.scene_units_to_m;
            let skymax = self.sky_max_samples;
            let method = self.rendering_method.to_code_literal();
            quote::quote! {
                ::elicit_bevy::AtmosphereSettings {
                    transmittance_lut_size: ::elicit_bevy::UVec2::from(#tls),
                    multiscattering_lut_size: ::elicit_bevy::UVec2::from(#mls),
                    sky_view_lut_size: ::elicit_bevy::UVec2::from(#svls),
                    aerial_view_lut_size: ::elicit_bevy::UVec3::from(#avls),
                    transmittance_lut_samples: #tlsamp,
                    multiscattering_lut_dirs: #mldirs,
                    multiscattering_lut_samples: #mlsamp,
                    sky_view_lut_samples: #svlsamp,
                    aerial_view_lut_samples: #avlsamp,
                    aerial_view_lut_max_distance: #avmax,
                    scene_units_to_m: #s2m,
                    sky_max_samples: #skymax,
                    rendering_method: #method,
                }
            }
        }
    }
}

shadow_elicitation!(AtmosphereSettings);

// ── DistanceFog ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::pbr::DistanceFog, as DistanceFog);
elicit_newtype_traits!(DistanceFog, bevy::pbr::DistanceFog, []);

impl From<DistanceFog> for bevy::pbr::DistanceFog {
    fn from(v: DistanceFog) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for DistanceFog {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let d = &*self.0;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("color", &crate::Color::from(d.color))?;
        map.serialize_entry(
            "directional_light_color",
            &crate::Color::from(d.directional_light_color),
        )?;
        map.serialize_entry("directional_light_exponent", &d.directional_light_exponent)?;
        map.serialize_entry("falloff", &crate::FogFalloff(Arc::new(d.falloff.clone())))?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for DistanceFog {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = DistanceFog;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a DistanceFog JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<DistanceFog, A::Error> {
                let mut directional_light_exponent: Option<f32> = None;
                let mut falloff: Option<crate::FogFalloff> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "directional_light_exponent" => {
                            directional_light_exponent = Some(map.next_value()?)
                        }
                        "falloff" => falloff = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut d = bevy::pbr::DistanceFog::default();
                if let Some(exp) = directional_light_exponent {
                    d.directional_light_exponent = exp;
                }
                if let Some(f) = falloff {
                    d.falloff = bevy::pbr::FogFalloff::from(f);
                }
                Ok(DistanceFog(Arc::new(d)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl DistanceFog {
    /// Returns the fog color.
    #[tracing::instrument(skip(self))]
    pub fn fog_color(&self) -> crate::Color {
        crate::Color::from(self.0.color)
    }

    /// Returns the directional light scatter exponent.
    #[tracing::instrument(skip(self))]
    pub fn directional_light_exponent(&self) -> f32 {
        self.0.directional_light_exponent
    }

    /// Returns the fog falloff mode.
    #[tracing::instrument(skip(self))]
    pub fn falloff(&self) -> crate::FogFalloff {
        crate::FogFalloff(Arc::new(self.0.falloff.clone()))
    }

    /// Returns a copy with the given falloff.
    #[tracing::instrument(skip(self))]
    pub fn with_falloff(&self, falloff: crate::FogFalloff) -> DistanceFog {
        let mut d = (*self.0).clone();
        d.falloff = bevy::pbr::FogFalloff::from(falloff);
        DistanceFog(Arc::new(d))
    }
}

mod emit_impls_distance_fog {
    use super::DistanceFog;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DistanceFog {
        fn to_code_literal(&self) -> TokenStream {
            let exp = self.0.directional_light_exponent;
            let falloff =
                crate::FogFalloff(std::sync::Arc::new(self.0.falloff.clone())).to_code_literal();
            quote::quote! {
                ::elicit_bevy::DistanceFog::from({
                    let mut d = ::bevy::pbr::DistanceFog::default();
                    d.directional_light_exponent = #exp;
                    d.falloff = ::bevy::pbr::FogFalloff::from(#falloff);
                    d
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for DistanceFog {}
