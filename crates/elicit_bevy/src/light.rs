//! Light component wrappers.
//!
//! Covers [`AmbientLight`], [`DirectionalLight`], [`PointLight`],
//! [`SpotLight`], [`ShadowFilteringMethod`], [`LightProbe`],
//! [`GlobalAmbientLight`], [`EnvironmentMapLight`], and
//! [`GeneratedEnvironmentMapLight`].

use crate::Color;
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── ShadowFilteringMethod ─────────────────────────────────────────────────────

elicit_newtype!(bevy::light::ShadowFilteringMethod, as ShadowFilteringMethod);
elicit_newtype_traits!(
    ShadowFilteringMethod,
    bevy::light::ShadowFilteringMethod,
    [eq]
);

impl From<ShadowFilteringMethod> for bevy::light::ShadowFilteringMethod {
    fn from(v: ShadowFilteringMethod) -> Self {
        *v.0
    }
}

impl serde::Serialize for ShadowFilteringMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ShadowFilteringMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Hardware2x2" => bevy::light::ShadowFilteringMethod::Hardware2x2,
            "Gaussian" => bevy::light::ShadowFilteringMethod::Gaussian,
            "Temporal" => bevy::light::ShadowFilteringMethod::Temporal,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Hardware2x2", "Gaussian", "Temporal"],
                ));
            }
        };
        Ok(ShadowFilteringMethod(Arc::new(inner)))
    }
}

#[reflect_methods]
impl ShadowFilteringMethod {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::light::ShadowFilteringMethod::Hardware2x2 => "Hardware2x2",
            bevy::light::ShadowFilteringMethod::Gaussian => "Gaussian",
            bevy::light::ShadowFilteringMethod::Temporal => "Temporal",
        }
    }

    /// Returns `true` if this is `ShadowFilteringMethod::Hardware2x2`.
    #[tracing::instrument(skip(self))]
    pub fn is_hardware(&self) -> bool {
        matches!(*self.0, bevy::light::ShadowFilteringMethod::Hardware2x2)
    }

    /// Returns `true` if this is `ShadowFilteringMethod::Gaussian`.
    #[tracing::instrument(skip(self))]
    pub fn is_gaussian(&self) -> bool {
        matches!(*self.0, bevy::light::ShadowFilteringMethod::Gaussian)
    }

    /// Returns `true` if this is `ShadowFilteringMethod::Temporal`.
    #[tracing::instrument(skip(self))]
    pub fn is_temporal(&self) -> bool {
        matches!(*self.0, bevy::light::ShadowFilteringMethod::Temporal)
    }
}

mod emit_impls_shadow_filtering {
    use super::ShadowFilteringMethod;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ShadowFilteringMethod {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::ShadowFilteringMethod::from(
                    ::bevy::light::ShadowFilteringMethod::#variant
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ShadowFilteringMethod {}

// ── AmbientLight ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::light::AmbientLight, as AmbientLight);
elicit_newtype_traits!(AmbientLight, bevy::light::AmbientLight, []);

impl From<AmbientLight> for bevy::light::AmbientLight {
    fn from(v: AmbientLight) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for AmbientLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let l = &*self.0;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("color", &format!("{:?}", l.color))?;
        map.serialize_entry("brightness", &l.brightness)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AmbientLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = AmbientLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "an AmbientLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<AmbientLight, A::Error> {
                let mut brightness: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "brightness" => brightness = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut l = bevy::light::AmbientLight::default();
                if let Some(b) = brightness {
                    l.brightness = b;
                }
                Ok(AmbientLight(Arc::new(l)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl AmbientLight {
    /// Returns the light color.
    #[tracing::instrument(skip(self))]
    pub fn ambient_light_color(&self) -> Color {
        Color::from(self.0.color)
    }

    /// Returns the brightness multiplier.
    #[tracing::instrument(skip(self))]
    pub fn brightness(&self) -> f32 {
        self.0.brightness
    }

    /// Creates a new ambient light with the given color and brightness.
    #[tracing::instrument(skip(self))]
    pub fn new_ambient(&self, r: f32, g: f32, b: f32, brightness: f32) -> AmbientLight {
        AmbientLight(Arc::new(bevy::light::AmbientLight {
            color: bevy::color::Color::srgb(r, g, b),
            brightness,
            ..Default::default()
        }))
    }

    /// Returns a copy with the given brightness.
    #[tracing::instrument(skip(self))]
    pub fn with_brightness(&self, brightness: f32) -> AmbientLight {
        let mut l = (*self.0).clone();
        l.brightness = brightness;
        AmbientLight::from(l)
    }
}

mod emit_impls_ambient {
    use super::AmbientLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AmbientLight {
        fn to_code_literal(&self) -> TokenStream {
            let brightness = self.0.brightness;
            quote::quote! {
                ::elicit_bevy::AmbientLight::from({
                    let mut l = ::bevy::light::AmbientLight::default();
                    l.brightness = #brightness;
                    l
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for AmbientLight {}

// ── DirectionalLight ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::light::DirectionalLight, as DirectionalLight);
elicit_newtype_traits!(DirectionalLight, bevy::light::DirectionalLight, []);

impl From<DirectionalLight> for bevy::light::DirectionalLight {
    fn from(v: DirectionalLight) -> Self {
        *v.0
    }
}

impl serde::Serialize for DirectionalLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let l = &*self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("color", &format!("{:?}", l.color))?;
        map.serialize_entry("illuminance", &l.illuminance)?;
        map.serialize_entry("shadows_enabled", &l.shadows_enabled)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for DirectionalLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = DirectionalLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a DirectionalLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<DirectionalLight, A::Error> {
                let mut illuminance: Option<f32> = None;
                let mut shadows_enabled: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "illuminance" => illuminance = Some(map.next_value()?),
                        "shadows_enabled" => shadows_enabled = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut l = bevy::light::DirectionalLight::default();
                if let Some(i) = illuminance {
                    l.illuminance = i;
                }
                if let Some(s) = shadows_enabled {
                    l.shadows_enabled = s;
                }
                Ok(DirectionalLight(Arc::new(l)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl DirectionalLight {
    /// Returns the light color.
    #[tracing::instrument(skip(self))]
    pub fn directional_light_color(&self) -> Color {
        Color::from(self.0.color)
    }

    /// Returns the illuminance in lux.
    #[tracing::instrument(skip(self))]
    pub fn illuminance(&self) -> f32 {
        self.0.illuminance
    }

    /// Returns `true` if shadows are enabled.
    #[tracing::instrument(skip(self))]
    pub fn directional_light_shadows_enabled(&self) -> bool {
        self.0.shadows_enabled
    }

    /// Creates a default directional light with the given illuminance.
    #[tracing::instrument(skip(self))]
    pub fn new_directional(&self, illuminance: f32) -> DirectionalLight {
        DirectionalLight(Arc::new(bevy::light::DirectionalLight {
            illuminance,
            ..Default::default()
        }))
    }

    /// Returns a copy with shadows enabled or disabled.
    #[tracing::instrument(skip(self))]
    pub fn directional_light_with_shadows(&self, enabled: bool) -> DirectionalLight {
        let mut l = *self.0;
        l.shadows_enabled = enabled;
        DirectionalLight::from(l)
    }
}

mod emit_impls_directional {
    use super::DirectionalLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DirectionalLight {
        fn to_code_literal(&self) -> TokenStream {
            let illuminance = self.0.illuminance;
            let shadows = self.0.shadows_enabled;
            quote::quote! {
                ::elicit_bevy::DirectionalLight::from(::bevy::light::DirectionalLight {
                    illuminance: #illuminance,
                    shadows_enabled: #shadows,
                    ..Default::default()
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for DirectionalLight {}

// ── PointLight ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::light::PointLight, as PointLight);
elicit_newtype_traits!(PointLight, bevy::light::PointLight, []);

impl From<PointLight> for bevy::light::PointLight {
    fn from(v: PointLight) -> Self {
        *v.0
    }
}

impl serde::Serialize for PointLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let l = &*self.0;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("color", &format!("{:?}", l.color))?;
        map.serialize_entry("intensity", &l.intensity)?;
        map.serialize_entry("range", &l.range)?;
        map.serialize_entry("radius", &l.radius)?;
        map.serialize_entry("shadows_enabled", &l.shadows_enabled)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for PointLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PointLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a PointLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<PointLight, A::Error> {
                let mut intensity: Option<f32> = None;
                let mut range: Option<f32> = None;
                let mut shadows_enabled: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "intensity" => intensity = Some(map.next_value()?),
                        "range" => range = Some(map.next_value()?),
                        "shadows_enabled" => shadows_enabled = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut l = bevy::light::PointLight::default();
                if let Some(i) = intensity {
                    l.intensity = i;
                }
                if let Some(r) = range {
                    l.range = r;
                }
                if let Some(s) = shadows_enabled {
                    l.shadows_enabled = s;
                }
                Ok(PointLight(Arc::new(l)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl PointLight {
    /// Returns the light color.
    #[tracing::instrument(skip(self))]
    pub fn point_light_color(&self) -> Color {
        Color::from(self.0.color)
    }

    /// Returns the luminous intensity in lumens.
    #[tracing::instrument(skip(self))]
    pub fn point_light_intensity(&self) -> f32 {
        self.0.intensity
    }

    /// Returns the effective range of this light.
    #[tracing::instrument(skip(self))]
    pub fn point_light_range(&self) -> f32 {
        self.0.range
    }

    /// Returns the radius of the light source sphere.
    #[tracing::instrument(skip(self))]
    pub fn point_light_radius(&self) -> f32 {
        self.0.radius
    }

    /// Returns `true` if shadows are enabled.
    #[tracing::instrument(skip(self))]
    pub fn point_light_shadows_enabled(&self) -> bool {
        self.0.shadows_enabled
    }

    /// Creates a point light with the given intensity and range.
    #[tracing::instrument(skip(self))]
    pub fn new_point(&self, intensity: f32, range: f32) -> PointLight {
        PointLight(Arc::new(bevy::light::PointLight {
            intensity,
            range,
            ..Default::default()
        }))
    }

    /// Returns a copy with shadows enabled or disabled.
    #[tracing::instrument(skip(self))]
    pub fn point_light_with_shadows(&self, enabled: bool) -> PointLight {
        let mut l = *self.0;
        l.shadows_enabled = enabled;
        PointLight::from(l)
    }
}

mod emit_impls_point {
    use super::PointLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PointLight {
        fn to_code_literal(&self) -> TokenStream {
            let intensity = self.0.intensity;
            let range = self.0.range;
            let shadows = self.0.shadows_enabled;
            quote::quote! {
                ::elicit_bevy::PointLight::from(::bevy::light::PointLight {
                    intensity: #intensity,
                    range: #range,
                    shadows_enabled: #shadows,
                    ..Default::default()
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for PointLight {}

// ── SpotLight ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::light::SpotLight, as SpotLight);
elicit_newtype_traits!(SpotLight, bevy::light::SpotLight, []);

impl From<SpotLight> for bevy::light::SpotLight {
    fn from(v: SpotLight) -> Self {
        *v.0
    }
}

impl serde::Serialize for SpotLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let l = &*self.0;
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("color", &format!("{:?}", l.color))?;
        map.serialize_entry("intensity", &l.intensity)?;
        map.serialize_entry("range", &l.range)?;
        map.serialize_entry("radius", &l.radius)?;
        map.serialize_entry("shadows_enabled", &l.shadows_enabled)?;
        map.serialize_entry("inner_angle", &l.inner_angle)?;
        map.serialize_entry("outer_angle", &l.outer_angle)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for SpotLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SpotLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a SpotLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<SpotLight, A::Error> {
                let mut intensity: Option<f32> = None;
                let mut range: Option<f32> = None;
                let mut shadows_enabled: Option<bool> = None;
                let mut inner_angle: Option<f32> = None;
                let mut outer_angle: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "intensity" => intensity = Some(map.next_value()?),
                        "range" => range = Some(map.next_value()?),
                        "shadows_enabled" => shadows_enabled = Some(map.next_value()?),
                        "inner_angle" => inner_angle = Some(map.next_value()?),
                        "outer_angle" => outer_angle = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut l = bevy::light::SpotLight::default();
                if let Some(i) = intensity {
                    l.intensity = i;
                }
                if let Some(r) = range {
                    l.range = r;
                }
                if let Some(s) = shadows_enabled {
                    l.shadows_enabled = s;
                }
                if let Some(a) = inner_angle {
                    l.inner_angle = a;
                }
                if let Some(a) = outer_angle {
                    l.outer_angle = a;
                }
                Ok(SpotLight(Arc::new(l)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl SpotLight {
    /// Returns the light color.
    #[tracing::instrument(skip(self))]
    pub fn spot_light_color(&self) -> Color {
        Color::from(self.0.color)
    }

    /// Returns the luminous intensity in lumens.
    #[tracing::instrument(skip(self))]
    pub fn spot_light_intensity(&self) -> f32 {
        self.0.intensity
    }

    /// Returns the effective range of this light.
    #[tracing::instrument(skip(self))]
    pub fn spot_light_range(&self) -> f32 {
        self.0.range
    }

    /// Returns the radius of the light source sphere.
    #[tracing::instrument(skip(self))]
    pub fn spot_light_radius(&self) -> f32 {
        self.0.radius
    }

    /// Returns `true` if shadows are enabled.
    #[tracing::instrument(skip(self))]
    pub fn spot_light_shadows_enabled(&self) -> bool {
        self.0.shadows_enabled
    }

    /// Returns the inner cone half-angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn inner_angle(&self) -> f32 {
        self.0.inner_angle
    }

    /// Returns the outer cone half-angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn outer_angle(&self) -> f32 {
        self.0.outer_angle
    }
}

mod emit_impls_spot {
    use super::SpotLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpotLight {
        fn to_code_literal(&self) -> TokenStream {
            let intensity = self.0.intensity;
            let range = self.0.range;
            let inner = self.0.inner_angle;
            let outer = self.0.outer_angle;
            quote::quote! {
                ::elicit_bevy::SpotLight::from(::bevy::light::SpotLight {
                    intensity: #intensity,
                    range: #range,
                    inner_angle: #inner,
                    outer_angle: #outer,
                    ..Default::default()
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for SpotLight {}

// ── unit_elicitation helper macro ─────────────────────────────────────────────

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

// ── LightProbe ────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::light::LightProbe`].
///
/// Marker component indicating that an entity contributes to a light probe
/// (environment map or irradiance volume) in the Bevy render world.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct LightProbe;

impl From<LightProbe> for bevy::light::LightProbe {
    fn from(_: LightProbe) -> Self {
        bevy::light::LightProbe
    }
}

mod emit_impls_light_probe {
    use super::LightProbe;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for LightProbe {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::light::LightProbe }
        }
    }
}

unit_elicitation!(LightProbe, bevy::light::LightProbe);

// ── GlobalAmbientLight ────────────────────────────────────────────────────────

elicit_newtype!(bevy::light::GlobalAmbientLight, as GlobalAmbientLight);
elicit_newtype_traits!(GlobalAmbientLight, bevy::light::GlobalAmbientLight, []);

impl From<GlobalAmbientLight> for bevy::light::GlobalAmbientLight {
    fn from(v: GlobalAmbientLight) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for GlobalAmbientLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let l = &*self.0;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("brightness", &l.brightness)?;
        map.serialize_entry("affects_lightmapped_meshes", &l.affects_lightmapped_meshes)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for GlobalAmbientLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = GlobalAmbientLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a GlobalAmbientLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<GlobalAmbientLight, A::Error> {
                let mut brightness: Option<f32> = None;
                let mut affects: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "brightness" => brightness = Some(map.next_value()?),
                        "affects_lightmapped_meshes" => affects = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut l = bevy::light::GlobalAmbientLight::default();
                if let Some(b) = brightness {
                    l.brightness = b;
                }
                if let Some(a) = affects {
                    l.affects_lightmapped_meshes = a;
                }
                Ok(GlobalAmbientLight(Arc::new(l)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl GlobalAmbientLight {
    /// Returns the brightness multiplier.
    #[tracing::instrument(skip(self))]
    pub fn brightness(&self) -> f32 {
        self.0.brightness
    }

    /// Returns whether this light affects lightmapped meshes.
    #[tracing::instrument(skip(self))]
    pub fn affects_lightmapped_meshes(&self) -> bool {
        self.0.affects_lightmapped_meshes
    }

    /// Returns a copy with the given brightness.
    #[tracing::instrument(skip(self))]
    pub fn with_global_brightness(&self, brightness: f32) -> GlobalAmbientLight {
        let mut l = (*self.0).clone();
        l.brightness = brightness;
        GlobalAmbientLight(Arc::new(l))
    }
}

mod emit_impls_global_ambient_light {
    use super::GlobalAmbientLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GlobalAmbientLight {
        fn to_code_literal(&self) -> TokenStream {
            let brightness = self.0.brightness;
            let affects = self.0.affects_lightmapped_meshes;
            quote::quote! {
                ::elicit_bevy::GlobalAmbientLight::from({
                    let mut l = ::bevy::light::GlobalAmbientLight::default();
                    l.brightness = #brightness;
                    l.affects_lightmapped_meshes = #affects;
                    l
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for GlobalAmbientLight {}

// ── EnvironmentMapLight ───────────────────────────────────────────────────────

elicit_newtype!(bevy::light::EnvironmentMapLight, as EnvironmentMapLight, nodebug);
elicit_newtype_traits!(EnvironmentMapLight, bevy::light::EnvironmentMapLight, []);

impl From<EnvironmentMapLight> for bevy::light::EnvironmentMapLight {
    fn from(v: EnvironmentMapLight) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for EnvironmentMapLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let e = &*self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("intensity", &e.intensity)?;
        map.serialize_entry(
            "rotation",
            &[e.rotation.x, e.rotation.y, e.rotation.z, e.rotation.w],
        )?;
        map.serialize_entry(
            "affects_lightmapped_mesh_diffuse",
            &e.affects_lightmapped_mesh_diffuse,
        )?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for EnvironmentMapLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = EnvironmentMapLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "an EnvironmentMapLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<EnvironmentMapLight, A::Error> {
                let mut intensity: Option<f32> = None;
                let mut rotation: Option<[f32; 4]> = None;
                let mut affects: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "intensity" => intensity = Some(map.next_value()?),
                        "rotation" => rotation = Some(map.next_value()?),
                        "affects_lightmapped_mesh_diffuse" => affects = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut e = bevy::light::EnvironmentMapLight::default();
                if let Some(i) = intensity {
                    e.intensity = i;
                }
                if let Some([x, y, z, w]) = rotation {
                    e.rotation = bevy::math::Quat::from_xyzw(x, y, z, w);
                }
                if let Some(a) = affects {
                    e.affects_lightmapped_mesh_diffuse = a;
                }
                Ok(EnvironmentMapLight(Arc::new(e)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl EnvironmentMapLight {
    /// Returns the radiance scale factor.
    #[tracing::instrument(skip(self))]
    pub fn intensity(&self) -> f32 {
        self.0.intensity
    }

    /// Returns whether this light affects lightmapped mesh diffuse.
    #[tracing::instrument(skip(self))]
    pub fn affects_lightmapped_mesh_diffuse(&self) -> bool {
        self.0.affects_lightmapped_mesh_diffuse
    }

    /// Returns a copy with the given intensity.
    #[tracing::instrument(skip(self))]
    pub fn with_intensity(&self, intensity: f32) -> EnvironmentMapLight {
        let mut e = (*self.0).clone();
        e.intensity = intensity;
        EnvironmentMapLight(Arc::new(e))
    }
}

mod emit_impls_environment_map_light {
    use super::EnvironmentMapLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for EnvironmentMapLight {
        fn to_code_literal(&self) -> TokenStream {
            let intensity = self.0.intensity;
            let affects = self.0.affects_lightmapped_mesh_diffuse;
            quote::quote! {
                ::elicit_bevy::EnvironmentMapLight::from({
                    let mut e = ::bevy::light::EnvironmentMapLight::default();
                    e.intensity = #intensity;
                    e.affects_lightmapped_mesh_diffuse = #affects;
                    e
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for EnvironmentMapLight {}

// ── GeneratedEnvironmentMapLight ──────────────────────────────────────────────

elicit_newtype!(
    bevy::light::GeneratedEnvironmentMapLight,
    as GeneratedEnvironmentMapLight,
    nodebug
);
elicit_newtype_traits!(
    GeneratedEnvironmentMapLight,
    bevy::light::GeneratedEnvironmentMapLight,
    []
);

impl From<GeneratedEnvironmentMapLight> for bevy::light::GeneratedEnvironmentMapLight {
    fn from(v: GeneratedEnvironmentMapLight) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for GeneratedEnvironmentMapLight {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let g = &*self.0;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("intensity", &g.intensity)?;
        map.serialize_entry(
            "rotation",
            &[g.rotation.x, g.rotation.y, g.rotation.z, g.rotation.w],
        )?;
        map.serialize_entry(
            "affects_lightmapped_mesh_diffuse",
            &g.affects_lightmapped_mesh_diffuse,
        )?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for GeneratedEnvironmentMapLight {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = GeneratedEnvironmentMapLight;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a GeneratedEnvironmentMapLight JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<GeneratedEnvironmentMapLight, A::Error> {
                let mut intensity: Option<f32> = None;
                let mut rotation: Option<[f32; 4]> = None;
                let mut affects: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "intensity" => intensity = Some(map.next_value()?),
                        "rotation" => rotation = Some(map.next_value()?),
                        "affects_lightmapped_mesh_diffuse" => affects = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut g = bevy::light::GeneratedEnvironmentMapLight::default();
                if let Some(i) = intensity {
                    g.intensity = i;
                }
                if let Some([x, y, z, w]) = rotation {
                    g.rotation = bevy::math::Quat::from_xyzw(x, y, z, w);
                }
                if let Some(a) = affects {
                    g.affects_lightmapped_mesh_diffuse = a;
                }
                Ok(GeneratedEnvironmentMapLight(Arc::new(g)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl GeneratedEnvironmentMapLight {
    /// Returns the radiance scale factor.
    #[tracing::instrument(skip(self))]
    pub fn intensity(&self) -> f32 {
        self.0.intensity
    }

    /// Returns whether this light affects lightmapped mesh diffuse.
    #[tracing::instrument(skip(self))]
    pub fn affects_lightmapped_mesh_diffuse(&self) -> bool {
        self.0.affects_lightmapped_mesh_diffuse
    }

    /// Returns a copy with the given intensity.
    #[tracing::instrument(skip(self))]
    pub fn with_generated_intensity(&self, intensity: f32) -> GeneratedEnvironmentMapLight {
        let mut g = (*self.0).clone();
        g.intensity = intensity;
        GeneratedEnvironmentMapLight(Arc::new(g))
    }
}

mod emit_impls_generated_environment_map_light {
    use super::GeneratedEnvironmentMapLight;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GeneratedEnvironmentMapLight {
        fn to_code_literal(&self) -> TokenStream {
            let intensity = self.0.intensity;
            let affects = self.0.affects_lightmapped_mesh_diffuse;
            quote::quote! {
                ::elicit_bevy::GeneratedEnvironmentMapLight::from({
                    let mut g = ::bevy::light::GeneratedEnvironmentMapLight::default();
                    g.intensity = #intensity;
                    g.affects_lightmapped_mesh_diffuse = #affects;
                    g
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for GeneratedEnvironmentMapLight {}
