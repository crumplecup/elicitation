//! Bevy gizmo configuration shadow types.
//!
//! Covers [`bevy::gizmos::config::GizmoLineStyle`],
//! [`bevy::gizmos::config::GizmoLineJoint`],
//! [`bevy::gizmos::config::GizmoLineConfig`],
//! [`bevy::gizmos::config::GizmoConfig`],
//! [`bevy::gizmos::aabb::AabbGizmoConfigGroup`],
//! [`bevy::gizmos::aabb::ShowAabbGizmo`],
//! [`bevy::gizmos::light::LightGizmoColor`],
//! [`bevy::gizmos::light::ShowLightGizmo`], and
//! [`bevy::gizmos::light::LightGizmoConfigGroup`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── GizmoLineStyle ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::gizmos::config::GizmoLineStyle, as GizmoLineStyle);
elicit_newtype_traits!(GizmoLineStyle, bevy::gizmos::config::GizmoLineStyle, []);

impl serde::Serialize for GizmoLineStyle {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        match *self.0 {
            bevy::gizmos::config::GizmoLineStyle::Solid => s.serialize_str("Solid"),
            bevy::gizmos::config::GizmoLineStyle::Dotted => s.serialize_str("Dotted"),
            bevy::gizmos::config::GizmoLineStyle::Dashed {
                gap_scale,
                line_scale,
            } => {
                let mut m = s.serialize_map(Some(1))?;
                m.serialize_entry(
                    "Dashed",
                    &serde_json::json!({"gap_scale": gap_scale, "line_scale": line_scale}),
                )?;
                m.end()
            }
            _ => s.serialize_str("Solid"),
        }
    }
}
impl<'de> serde::Deserialize<'de> for GizmoLineStyle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = serde_json::Value::deserialize(d)?;
        let style = match &value {
            serde_json::Value::String(s) => match s.as_str() {
                "Solid" => bevy::gizmos::config::GizmoLineStyle::Solid,
                "Dotted" => bevy::gizmos::config::GizmoLineStyle::Dotted,
                _ => return Err(D::Error::unknown_variant(s, &["Solid", "Dotted", "Dashed"])),
            },
            serde_json::Value::Object(m) if m.contains_key("Dashed") => {
                let inner = &m["Dashed"];
                let gap_scale = inner["gap_scale"]
                    .as_f64()
                    .ok_or_else(|| D::Error::missing_field("gap_scale"))?
                    as f32;
                let line_scale = inner["line_scale"]
                    .as_f64()
                    .ok_or_else(|| D::Error::missing_field("line_scale"))?
                    as f32;
                bevy::gizmos::config::GizmoLineStyle::Dashed {
                    gap_scale,
                    line_scale,
                }
            }
            _ => return Err(D::Error::custom("invalid GizmoLineStyle")),
        };
        Ok(GizmoLineStyle(std::sync::Arc::new(style)))
    }
}
impl From<GizmoLineStyle> for bevy::gizmos::config::GizmoLineStyle {
    fn from(v: GizmoLineStyle) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl GizmoLineStyle {
    /// Returns the variant name: `"Solid"`, `"Dotted"`, or `"Dashed"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::gizmos::config::GizmoLineStyle::Solid => "Solid",
            bevy::gizmos::config::GizmoLineStyle::Dotted => "Dotted",
            bevy::gizmos::config::GizmoLineStyle::Dashed { .. } => "Dashed",
            _ => "Solid",
        }
    }
}

mod emit_impls_line_style {
    use super::GizmoLineStyle;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GizmoLineStyle {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::gizmos::config::GizmoLineStyle::Solid => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineStyle::Solid }
                }
                bevy::gizmos::config::GizmoLineStyle::Dotted => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineStyle::Dotted }
                }
                bevy::gizmos::config::GizmoLineStyle::Dashed {
                    gap_scale,
                    line_scale,
                } => {
                    quote::quote! {
                        ::bevy::gizmos::config::GizmoLineStyle::Dashed {
                            gap_scale: #gap_scale,
                            line_scale: #line_scale,
                        }
                    }
                }
                _ => quote::quote! { ::bevy::gizmos::config::GizmoLineStyle::Solid },
            }
        }
    }
}
impl elicitation::ElicitComplete for GizmoLineStyle {}

// ── GizmoLineJoint ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::gizmos::config::GizmoLineJoint, as GizmoLineJoint);
elicit_newtype_traits!(GizmoLineJoint, bevy::gizmos::config::GizmoLineJoint, [eq]);

impl serde::Serialize for GizmoLineJoint {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        match *self.0 {
            bevy::gizmos::config::GizmoLineJoint::None => s.serialize_str("None"),
            bevy::gizmos::config::GizmoLineJoint::Miter => s.serialize_str("Miter"),
            bevy::gizmos::config::GizmoLineJoint::Bevel => s.serialize_str("Bevel"),
            bevy::gizmos::config::GizmoLineJoint::Round(res) => {
                let mut m = s.serialize_map(Some(1))?;
                m.serialize_entry("Round", &res)?;
                m.end()
            }
        }
    }
}
impl<'de> serde::Deserialize<'de> for GizmoLineJoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = serde_json::Value::deserialize(d)?;
        let joint = match &value {
            serde_json::Value::String(s) => match s.as_str() {
                "None" => bevy::gizmos::config::GizmoLineJoint::None,
                "Miter" => bevy::gizmos::config::GizmoLineJoint::Miter,
                "Bevel" => bevy::gizmos::config::GizmoLineJoint::Bevel,
                _ => {
                    return Err(D::Error::unknown_variant(
                        s,
                        &["None", "Miter", "Bevel", "Round"],
                    ));
                }
            },
            serde_json::Value::Object(m) if m.contains_key("Round") => {
                let res = m["Round"]
                    .as_u64()
                    .ok_or_else(|| D::Error::custom("Round resolution must be u64"))?
                    as u32;
                bevy::gizmos::config::GizmoLineJoint::Round(res)
            }
            _ => return Err(D::Error::custom("invalid GizmoLineJoint")),
        };
        Ok(GizmoLineJoint(std::sync::Arc::new(joint)))
    }
}
impl From<GizmoLineJoint> for bevy::gizmos::config::GizmoLineJoint {
    fn from(v: GizmoLineJoint) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl GizmoLineJoint {
    /// Returns the variant name: `"None"`, `"Miter"`, `"Bevel"`, or `"Round"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::gizmos::config::GizmoLineJoint::None => "None",
            bevy::gizmos::config::GizmoLineJoint::Miter => "Miter",
            bevy::gizmos::config::GizmoLineJoint::Bevel => "Bevel",
            bevy::gizmos::config::GizmoLineJoint::Round(_) => "Round",
        }
    }

    /// Returns the round resolution if this is `Round`, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn round_resolution(&self) -> Option<u32> {
        match *self.0 {
            bevy::gizmos::config::GizmoLineJoint::Round(r) => Some(r),
            _ => None,
        }
    }
}

mod emit_impls_line_joint {
    use super::GizmoLineJoint;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GizmoLineJoint {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::gizmos::config::GizmoLineJoint::None => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineJoint::None }
                }
                bevy::gizmos::config::GizmoLineJoint::Miter => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineJoint::Miter }
                }
                bevy::gizmos::config::GizmoLineJoint::Bevel => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineJoint::Bevel }
                }
                bevy::gizmos::config::GizmoLineJoint::Round(r) => {
                    quote::quote! { ::bevy::gizmos::config::GizmoLineJoint::Round(#r) }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for GizmoLineJoint {}

// ── GizmoLineConfig ───────────────────────────────────────────────────────────

elicit_newtype!(bevy::gizmos::config::GizmoLineConfig, as GizmoLineConfig);
elicit_newtype_traits!(GizmoLineConfig, bevy::gizmos::config::GizmoLineConfig, []);

impl serde::Serialize for GizmoLineConfig {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct as _;
        let inner = &*self.0;
        let style = GizmoLineStyle(std::sync::Arc::new(inner.style.clone()));
        let joints = GizmoLineJoint(std::sync::Arc::new(inner.joints));
        let mut st = s.serialize_struct("GizmoLineConfig", 4)?;
        st.serialize_field("width", &inner.width)?;
        st.serialize_field("perspective", &inner.perspective)?;
        st.serialize_field("style", &style)?;
        st.serialize_field("joints", &joints)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for GizmoLineConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let width = v["width"].as_f64().unwrap_or(2.0) as f32;
        let perspective = v["perspective"].as_bool().unwrap_or(false);
        let style: GizmoLineStyle =
            serde_json::from_value(v["style"].clone()).unwrap_or_else(|_| {
                GizmoLineStyle(std::sync::Arc::new(
                    bevy::gizmos::config::GizmoLineStyle::Solid,
                ))
            });
        let joints: GizmoLineJoint =
            serde_json::from_value(v["joints"].clone()).unwrap_or_else(|_| {
                GizmoLineJoint(std::sync::Arc::new(
                    bevy::gizmos::config::GizmoLineJoint::None,
                ))
            });
        Ok(GizmoLineConfig(std::sync::Arc::new(
            bevy::gizmos::config::GizmoLineConfig {
                width,
                perspective,
                style: bevy::gizmos::config::GizmoLineStyle::from(style),
                joints: bevy::gizmos::config::GizmoLineJoint::from(joints),
            },
        )))
    }
}
impl From<GizmoLineConfig> for bevy::gizmos::config::GizmoLineConfig {
    fn from(v: GizmoLineConfig) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl GizmoLineConfig {
    /// Returns the line width in pixels.
    #[tracing::instrument(skip(self))]
    pub fn width(&self) -> f32 {
        self.0.width
    }

    /// Returns `true` if perspective scaling is enabled.
    #[tracing::instrument(skip(self))]
    pub fn perspective(&self) -> bool {
        self.0.perspective
    }
}

mod emit_impls_line_config {
    use super::{GizmoLineConfig, GizmoLineJoint, GizmoLineStyle};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GizmoLineConfig {
        fn to_code_literal(&self) -> TokenStream {
            let style = GizmoLineStyle(std::sync::Arc::new(self.0.style.clone())).to_code_literal();
            let joints = GizmoLineJoint(std::sync::Arc::new(self.0.joints)).to_code_literal();
            let width = self.0.width;
            let perspective = self.0.perspective;
            quote::quote! {
                ::bevy::gizmos::config::GizmoLineConfig {
                    width: #width,
                    perspective: #perspective,
                    style: #style,
                    joints: #joints,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for GizmoLineConfig {}

// ── GizmoConfig ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::gizmos::config::GizmoConfig, as GizmoConfig);
elicit_newtype_traits!(GizmoConfig, bevy::gizmos::config::GizmoConfig, []);

impl serde::Serialize for GizmoConfig {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct as _;
        let inner = &*self.0;
        let line = GizmoLineConfig(std::sync::Arc::new(inner.line.clone()));
        let mut st = s.serialize_struct("GizmoConfig", 3)?;
        st.serialize_field("enabled", &inner.enabled)?;
        st.serialize_field("line", &line)?;
        st.serialize_field("depth_bias", &inner.depth_bias)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for GizmoConfig {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let enabled = v["enabled"].as_bool().unwrap_or(true);
        let depth_bias = v["depth_bias"].as_f64().unwrap_or(0.0) as f32;
        let line: GizmoLineConfig =
            serde_json::from_value(v["line"].clone()).unwrap_or_else(|_| {
                GizmoLineConfig(std::sync::Arc::new(
                    bevy::gizmos::config::GizmoLineConfig::default(),
                ))
            });
        Ok(GizmoConfig(std::sync::Arc::new(
            bevy::gizmos::config::GizmoConfig {
                enabled,
                line: bevy::gizmos::config::GizmoLineConfig::from(line),
                depth_bias,
                render_layers: Default::default(),
            },
        )))
    }
}
impl From<GizmoConfig> for bevy::gizmos::config::GizmoConfig {
    fn from(v: GizmoConfig) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl GizmoConfig {
    /// Returns `true` if gizmo rendering is enabled.
    #[tracing::instrument(skip(self))]
    pub fn enabled(&self) -> bool {
        self.0.enabled
    }

    /// Returns the depth bias.
    #[tracing::instrument(skip(self))]
    pub fn depth_bias(&self) -> f32 {
        self.0.depth_bias
    }
}

mod emit_impls_gizmo_config {
    use super::{GizmoConfig, GizmoLineConfig};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GizmoConfig {
        fn to_code_literal(&self) -> TokenStream {
            let enabled = self.0.enabled;
            let depth_bias = self.0.depth_bias;
            let line = GizmoLineConfig(std::sync::Arc::new(self.0.line.clone())).to_code_literal();
            quote::quote! {
                ::bevy::gizmos::config::GizmoConfig {
                    enabled: #enabled,
                    line: #line,
                    depth_bias: #depth_bias,
                    render_layers: ::bevy::render::view::RenderLayers::default(),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for GizmoConfig {}

// ── shadow_elicitation macro ──────────────────────────────────────────────────

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

// ── LightGizmoColor ───────────────────────────────────────────────────────────

/// Shadow for [`bevy::gizmos::light::LightGizmoColor`].
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum LightGizmoColor {
    /// User-specified color.
    Manual(crate::Color),
    /// Random color derived from the light's entity.
    Varied,
    /// Take the color of the represented light.
    #[default]
    MatchLightColor,
    /// Use the per-light-type colors from [`LightGizmoConfigGroup`].
    ByLightType,
}

impl From<LightGizmoColor> for bevy::gizmos::light::LightGizmoColor {
    fn from(v: LightGizmoColor) -> Self {
        match v {
            LightGizmoColor::Manual(c) => Self::Manual(bevy::color::Color::from(c)),
            LightGizmoColor::Varied => Self::Varied,
            LightGizmoColor::MatchLightColor => Self::MatchLightColor,
            LightGizmoColor::ByLightType => Self::ByLightType,
        }
    }
}

impl From<bevy::gizmos::light::LightGizmoColor> for LightGizmoColor {
    fn from(v: bevy::gizmos::light::LightGizmoColor) -> Self {
        match v {
            bevy::gizmos::light::LightGizmoColor::Manual(c) => {
                Self::Manual(crate::Color(std::sync::Arc::new(c)))
            }
            bevy::gizmos::light::LightGizmoColor::Varied => Self::Varied,
            bevy::gizmos::light::LightGizmoColor::MatchLightColor => Self::MatchLightColor,
            bevy::gizmos::light::LightGizmoColor::ByLightType => Self::ByLightType,
        }
    }
}

mod emit_light_gizmo_color {
    use super::LightGizmoColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for LightGizmoColor {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                LightGizmoColor::Manual(c) => {
                    let inner = c.to_code_literal();
                    quote::quote! { ::bevy::gizmos::light::LightGizmoColor::Manual(#inner) }
                }
                LightGizmoColor::Varied => {
                    quote::quote! { ::bevy::gizmos::light::LightGizmoColor::Varied }
                }
                LightGizmoColor::MatchLightColor => {
                    quote::quote! { ::bevy::gizmos::light::LightGizmoColor::MatchLightColor }
                }
                LightGizmoColor::ByLightType => {
                    quote::quote! { ::bevy::gizmos::light::LightGizmoColor::ByLightType }
                }
            }
        }
    }
}

shadow_elicitation!(LightGizmoColor);

// ── ShowAabbGizmo ─────────────────────────────────────────────────────────────

/// Shadow for [`bevy::gizmos::aabb::ShowAabbGizmo`].
///
/// Add to an entity to render its AABB as a gizmo wireframe.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ShowAabbGizmo {
    /// Optional color override; falls back to the [`AabbGizmoConfigGroup`] color.
    pub color: Option<crate::Color>,
}

impl From<ShowAabbGizmo> for bevy::gizmos::aabb::ShowAabbGizmo {
    fn from(v: ShowAabbGizmo) -> Self {
        Self {
            color: v.color.map(bevy::color::Color::from),
        }
    }
}

mod emit_show_aabb_gizmo {
    use super::ShowAabbGizmo;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ShowAabbGizmo {
        fn to_code_literal(&self) -> TokenStream {
            match &self.color {
                None => {
                    quote::quote! { ::bevy::gizmos::aabb::ShowAabbGizmo { color: None } }
                }
                Some(c) => {
                    let color = c.to_code_literal();
                    quote::quote! {
                        ::bevy::gizmos::aabb::ShowAabbGizmo { color: Some(#color) }
                    }
                }
            }
        }
    }
}

shadow_elicitation!(ShowAabbGizmo);

// ── ShowLightGizmo ────────────────────────────────────────────────────────────

/// Shadow for [`bevy::gizmos::light::ShowLightGizmo`].
///
/// Add to a light entity to visualize it with a debug gizmo.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ShowLightGizmo {
    /// Optional color strategy override; falls back to [`LightGizmoConfigGroup`].
    pub color: Option<LightGizmoColor>,
}

impl From<ShowLightGizmo> for bevy::gizmos::light::ShowLightGizmo {
    fn from(v: ShowLightGizmo) -> Self {
        Self {
            color: v.color.map(bevy::gizmos::light::LightGizmoColor::from),
        }
    }
}

mod emit_show_light_gizmo {
    use super::ShowLightGizmo;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ShowLightGizmo {
        fn to_code_literal(&self) -> TokenStream {
            match &self.color {
                None => {
                    quote::quote! { ::bevy::gizmos::light::ShowLightGizmo { color: None } }
                }
                Some(c) => {
                    let color = c.to_code_literal();
                    quote::quote! {
                        ::bevy::gizmos::light::ShowLightGizmo { color: Some(#color) }
                    }
                }
            }
        }
    }
}

shadow_elicitation!(ShowLightGizmo);

// ── LightGizmoConfigGroup ─────────────────────────────────────────────────────

/// Shadow for [`bevy::gizmos::light::LightGizmoConfigGroup`].
///
/// Resource configuring how all light gizmos appear by default.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct LightGizmoConfigGroup {
    /// Draw a gizmo for all lights when `true`.
    pub draw_all: bool,
    /// Default color strategy for all light gizmos.
    pub color: LightGizmoColor,
    /// Color for point light gizmos when `color` is [`LightGizmoColor::ByLightType`].
    pub point_light_color: crate::Color,
    /// Color for spot light gizmos when `color` is [`LightGizmoColor::ByLightType`].
    pub spot_light_color: crate::Color,
    /// Color for directional light gizmos when `color` is [`LightGizmoColor::ByLightType`].
    pub directional_light_color: crate::Color,
}

impl Default for LightGizmoConfigGroup {
    fn default() -> Self {
        let upstream = bevy::gizmos::light::LightGizmoConfigGroup::default();
        Self {
            draw_all: upstream.draw_all,
            color: LightGizmoColor::from(upstream.color),
            point_light_color: crate::Color(std::sync::Arc::new(upstream.point_light_color)),
            spot_light_color: crate::Color(std::sync::Arc::new(upstream.spot_light_color)),
            directional_light_color: crate::Color(std::sync::Arc::new(
                upstream.directional_light_color,
            )),
        }
    }
}

impl From<LightGizmoConfigGroup> for bevy::gizmos::light::LightGizmoConfigGroup {
    fn from(v: LightGizmoConfigGroup) -> Self {
        Self {
            draw_all: v.draw_all,
            color: bevy::gizmos::light::LightGizmoColor::from(v.color),
            point_light_color: bevy::color::Color::from(v.point_light_color),
            spot_light_color: bevy::color::Color::from(v.spot_light_color),
            directional_light_color: bevy::color::Color::from(v.directional_light_color),
        }
    }
}

mod emit_light_gizmo_config_group {
    use super::LightGizmoConfigGroup;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for LightGizmoConfigGroup {
        fn to_code_literal(&self) -> TokenStream {
            let draw_all = self.draw_all;
            let color = self.color.to_code_literal();
            let point = self.point_light_color.to_code_literal();
            let spot = self.spot_light_color.to_code_literal();
            let dir = self.directional_light_color.to_code_literal();
            quote::quote! {
                ::bevy::gizmos::light::LightGizmoConfigGroup {
                    draw_all: #draw_all,
                    color: #color,
                    point_light_color: #point,
                    spot_light_color: #spot,
                    directional_light_color: #dir,
                }
            }
        }
    }
}

shadow_elicitation!(LightGizmoConfigGroup);

// ── AabbGizmoConfigGroup ──────────────────────────────────────────────────────

/// Shadow of [`bevy::gizmos::aabb::AabbGizmoConfigGroup`].
///
/// Resource configuring how AABB debug gizmos appear.  Registered via
/// `GizmoConfigStore`; can be retrieved with `app.world.resource::<GizmoConfigStore>()`.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct AabbGizmoConfigGroup {
    /// When `true`, draws bounding boxes for all entities in the scene.
    ///
    /// To show only a specific entity's box, add [`ShowAabbGizmo`] to it instead.
    pub draw_all: bool,
    /// Default color for all bounding-box gizmos.  `None` picks a random color per box.
    pub default_color: Option<crate::Color>,
}

impl From<AabbGizmoConfigGroup> for bevy::gizmos::aabb::AabbGizmoConfigGroup {
    fn from(v: AabbGizmoConfigGroup) -> Self {
        Self {
            draw_all: v.draw_all,
            default_color: v.default_color.map(bevy::color::Color::from),
        }
    }
}

impl From<bevy::gizmos::aabb::AabbGizmoConfigGroup> for AabbGizmoConfigGroup {
    fn from(v: bevy::gizmos::aabb::AabbGizmoConfigGroup) -> Self {
        Self {
            draw_all: v.draw_all,
            default_color: v.default_color.map(crate::Color::from),
        }
    }
}

mod emit_aabb_gizmo_config_group {
    use super::AabbGizmoConfigGroup;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AabbGizmoConfigGroup {
        fn to_code_literal(&self) -> TokenStream {
            let draw_all = self.draw_all;
            let color = match &self.default_color {
                None => quote::quote! { None },
                Some(c) => {
                    let lit = c.to_code_literal();
                    quote::quote! { Some(#lit) }
                }
            };
            quote::quote! {
                ::bevy::gizmos::aabb::AabbGizmoConfigGroup {
                    draw_all: #draw_all,
                    default_color: #color,
                }
            }
        }
    }
}

shadow_elicitation!(AabbGizmoConfigGroup);
