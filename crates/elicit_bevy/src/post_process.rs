//! Bevy post-processing shadow types.
//!
//! Covers [`bevy::post_process::bloom::BloomCompositeMode`],
//! [`bevy::post_process::bloom::BloomPrefilter`],
//! [`bevy::post_process::bloom::Bloom`],
//! [`bevy::post_process::dof::DepthOfFieldMode`],
//! [`bevy::post_process::dof::DepthOfField`],
//! [`bevy::post_process::motion_blur::MotionBlur`],
//! [`bevy::post_process::auto_exposure::AutoExposure`], and
//! [`bevy::post_process::effect_stack::ChromaticAberration`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── BloomCompositeMode ────────────────────────────────────────────────────────

elicit_newtype!(
    bevy::post_process::bloom::BloomCompositeMode,
    as BloomCompositeMode
);
elicit_newtype_traits!(
    BloomCompositeMode,
    bevy::post_process::bloom::BloomCompositeMode,
    [eq]
);

impl serde::Serialize for BloomCompositeMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}
impl<'de> serde::Deserialize<'de> for BloomCompositeMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let mode = match value.as_str() {
            "EnergyConserving" => bevy::post_process::bloom::BloomCompositeMode::EnergyConserving,
            "Additive" => bevy::post_process::bloom::BloomCompositeMode::Additive,
            _ => {
                return Err(D::Error::unknown_variant(
                    &value,
                    &["EnergyConserving", "Additive"],
                ));
            }
        };
        Ok(BloomCompositeMode(std::sync::Arc::new(mode)))
    }
}
impl From<BloomCompositeMode> for bevy::post_process::bloom::BloomCompositeMode {
    fn from(v: BloomCompositeMode) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl BloomCompositeMode {
    /// Returns the variant name: `"EnergyConserving"` or `"Additive"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::post_process::bloom::BloomCompositeMode::EnergyConserving => "EnergyConserving",
            bevy::post_process::bloom::BloomCompositeMode::Additive => "Additive",
        }
    }
}

mod emit_impls_bloom_composite_mode {
    use super::BloomCompositeMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for BloomCompositeMode {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::post_process::bloom::BloomCompositeMode::EnergyConserving => {
                    quote::quote! {
                        ::bevy::post_process::bloom::BloomCompositeMode::EnergyConserving
                    }
                }
                bevy::post_process::bloom::BloomCompositeMode::Additive => {
                    quote::quote! { ::bevy::post_process::bloom::BloomCompositeMode::Additive }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for BloomCompositeMode {}

// ── BloomPrefilter ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::post_process::bloom::BloomPrefilter, as BloomPrefilter, nodebug);
elicit_newtype_traits!(
    BloomPrefilter,
    bevy::post_process::bloom::BloomPrefilter,
    []
);

impl serde::Serialize for BloomPrefilter {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct as _;
        let mut st = s.serialize_struct("BloomPrefilter", 2)?;
        st.serialize_field("threshold", &self.0.threshold)?;
        st.serialize_field("threshold_softness", &self.0.threshold_softness)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for BloomPrefilter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let threshold = v["threshold"].as_f64().unwrap_or(0.0) as f32;
        let threshold_softness = v["threshold_softness"].as_f64().unwrap_or(0.0) as f32;
        Ok(BloomPrefilter(std::sync::Arc::new(
            bevy::post_process::bloom::BloomPrefilter {
                threshold,
                threshold_softness,
            },
        )))
    }
}
impl From<BloomPrefilter> for bevy::post_process::bloom::BloomPrefilter {
    fn from(v: BloomPrefilter) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl BloomPrefilter {
    /// Returns the threshold value.
    #[tracing::instrument(skip(self))]
    pub fn threshold(&self) -> f32 {
        self.0.threshold
    }

    /// Returns the threshold softness value.
    #[tracing::instrument(skip(self))]
    pub fn threshold_softness(&self) -> f32 {
        self.0.threshold_softness
    }
}

mod emit_impls_bloom_prefilter {
    use super::BloomPrefilter;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for BloomPrefilter {
        fn to_code_literal(&self) -> TokenStream {
            let threshold = self.0.threshold;
            let threshold_softness = self.0.threshold_softness;
            quote::quote! {
                ::bevy::post_process::bloom::BloomPrefilter {
                    threshold: #threshold,
                    threshold_softness: #threshold_softness,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for BloomPrefilter {}

// ── Bloom ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::post_process::bloom::Bloom, as Bloom, nodebug);
elicit_newtype_traits!(Bloom, bevy::post_process::bloom::Bloom, []);

impl serde::Serialize for Bloom {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct as _;
        let inner = &*self.0;
        let prefilter = BloomPrefilter(std::sync::Arc::new(inner.prefilter.clone()));
        let composite_mode = BloomCompositeMode(std::sync::Arc::new(inner.composite_mode));
        let mut st = s.serialize_struct("Bloom", 8)?;
        st.serialize_field("intensity", &inner.intensity)?;
        st.serialize_field("low_frequency_boost", &inner.low_frequency_boost)?;
        st.serialize_field(
            "low_frequency_boost_curvature",
            &inner.low_frequency_boost_curvature,
        )?;
        st.serialize_field("high_pass_frequency", &inner.high_pass_frequency)?;
        st.serialize_field("prefilter", &prefilter)?;
        st.serialize_field("composite_mode", &composite_mode)?;
        st.serialize_field("max_mip_dimension", &inner.max_mip_dimension)?;
        st.serialize_field("scale_x", &inner.scale.x)?;
        st.serialize_field("scale_y", &inner.scale.y)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bloom {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let def = bevy::post_process::bloom::Bloom::default();
        let v = serde_json::Value::deserialize(d)?;
        let intensity = v["intensity"].as_f64().unwrap_or(def.intensity as f64) as f32;
        let low_frequency_boost = v["low_frequency_boost"]
            .as_f64()
            .unwrap_or(def.low_frequency_boost as f64) as f32;
        let low_frequency_boost_curvature = v["low_frequency_boost_curvature"]
            .as_f64()
            .unwrap_or(def.low_frequency_boost_curvature as f64)
            as f32;
        let high_pass_frequency = v["high_pass_frequency"]
            .as_f64()
            .unwrap_or(def.high_pass_frequency as f64) as f32;
        // Parse BloomPrefilter fields manually (it has no serde support)
        let prefilter = bevy::post_process::bloom::BloomPrefilter {
            threshold: v["prefilter"]["threshold"]
                .as_f64()
                .unwrap_or(def.prefilter.threshold as f64) as f32,
            threshold_softness: v["prefilter"]["threshold_softness"]
                .as_f64()
                .unwrap_or(def.prefilter.threshold_softness as f64)
                as f32,
        };
        let composite_mode: bevy::post_process::bloom::BloomCompositeMode =
            serde_json::from_value(v["composite_mode"].clone())
                .map(|cm: BloomCompositeMode| cm.into())
                .unwrap_or(def.composite_mode);
        let max_mip_dimension = v["max_mip_dimension"]
            .as_u64()
            .unwrap_or(def.max_mip_dimension as u64) as u32;
        let scale_x = v["scale_x"].as_f64().unwrap_or(def.scale.x as f64) as f32;
        let scale_y = v["scale_y"].as_f64().unwrap_or(def.scale.y as f64) as f32;
        Ok(Bloom(std::sync::Arc::new(
            bevy::post_process::bloom::Bloom {
                intensity,
                low_frequency_boost,
                low_frequency_boost_curvature,
                high_pass_frequency,
                prefilter,
                composite_mode,
                max_mip_dimension,
                scale: bevy::math::Vec2::new(scale_x, scale_y),
            },
        )))
    }
}
impl From<Bloom> for bevy::post_process::bloom::Bloom {
    fn from(v: Bloom) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl Bloom {
    /// Returns the bloom intensity.
    #[tracing::instrument(skip(self))]
    pub fn intensity(&self) -> f32 {
        self.0.intensity
    }

    /// Returns the composite mode variant name.
    #[tracing::instrument(skip(self))]
    pub fn composite_mode_name(&self) -> &'static str {
        match self.0.composite_mode {
            bevy::post_process::bloom::BloomCompositeMode::EnergyConserving => "EnergyConserving",
            bevy::post_process::bloom::BloomCompositeMode::Additive => "Additive",
        }
    }
}

mod emit_impls_bloom {
    use super::{Bloom, BloomCompositeMode, BloomPrefilter};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Bloom {
        fn to_code_literal(&self) -> TokenStream {
            let inner = &*self.0;
            let intensity = inner.intensity;
            let lfb = inner.low_frequency_boost;
            let lfbc = inner.low_frequency_boost_curvature;
            let hpf = inner.high_pass_frequency;
            let prefilter =
                BloomPrefilter(std::sync::Arc::new(inner.prefilter.clone())).to_code_literal();
            let composite_mode =
                BloomCompositeMode(std::sync::Arc::new(inner.composite_mode)).to_code_literal();
            let max_mip = inner.max_mip_dimension;
            let sx = inner.scale.x;
            let sy = inner.scale.y;
            quote::quote! {
                ::bevy::post_process::bloom::Bloom {
                    intensity: #intensity,
                    low_frequency_boost: #lfb,
                    low_frequency_boost_curvature: #lfbc,
                    high_pass_frequency: #hpf,
                    prefilter: #prefilter,
                    composite_mode: #composite_mode,
                    max_mip_dimension: #max_mip,
                    scale: ::bevy::math::Vec2::new(#sx, #sy),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Bloom {}

// ── DepthOfFieldMode ──────────────────────────────────────────────────────────

elicit_newtype!(
    bevy::post_process::dof::DepthOfFieldMode,
    as DepthOfFieldMode
);
elicit_newtype_traits!(
    DepthOfFieldMode,
    bevy::post_process::dof::DepthOfFieldMode,
    [eq]
);

impl serde::Serialize for DepthOfFieldMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}
impl<'de> serde::Deserialize<'de> for DepthOfFieldMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let mode = match value.as_str() {
            "Bokeh" => bevy::post_process::dof::DepthOfFieldMode::Bokeh,
            "Gaussian" => bevy::post_process::dof::DepthOfFieldMode::Gaussian,
            _ => return Err(D::Error::unknown_variant(&value, &["Bokeh", "Gaussian"])),
        };
        Ok(DepthOfFieldMode(std::sync::Arc::new(mode)))
    }
}
impl From<DepthOfFieldMode> for bevy::post_process::dof::DepthOfFieldMode {
    fn from(v: DepthOfFieldMode) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DepthOfFieldMode {
    /// Returns the variant name: `"Bokeh"` or `"Gaussian"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::post_process::dof::DepthOfFieldMode::Bokeh => "Bokeh",
            bevy::post_process::dof::DepthOfFieldMode::Gaussian => "Gaussian",
        }
    }
}

mod emit_impls_dof_mode {
    use super::DepthOfFieldMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DepthOfFieldMode {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::post_process::dof::DepthOfFieldMode::Bokeh => {
                    quote::quote! { ::bevy::post_process::dof::DepthOfFieldMode::Bokeh }
                }
                bevy::post_process::dof::DepthOfFieldMode::Gaussian => {
                    quote::quote! { ::bevy::post_process::dof::DepthOfFieldMode::Gaussian }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for DepthOfFieldMode {}

// ── DepthOfField ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::post_process::dof::DepthOfField, as DepthOfField, nodebug);
elicit_newtype_traits!(DepthOfField, bevy::post_process::dof::DepthOfField, []);

impl serde::Serialize for DepthOfField {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct as _;
        let inner = &*self.0;
        let mode = DepthOfFieldMode(std::sync::Arc::new(inner.mode));
        let mut st = s.serialize_struct("DepthOfField", 6)?;
        st.serialize_field("mode", &mode)?;
        st.serialize_field("focal_distance", &inner.focal_distance)?;
        st.serialize_field("sensor_height", &inner.sensor_height)?;
        st.serialize_field("aperture_f_stops", &inner.aperture_f_stops)?;
        st.serialize_field(
            "max_circle_of_confusion_diameter",
            &inner.max_circle_of_confusion_diameter,
        )?;
        st.serialize_field("max_depth", &inner.max_depth)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for DepthOfField {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let def = bevy::post_process::dof::DepthOfField::default();
        let v = serde_json::Value::deserialize(d)?;
        let mode: bevy::post_process::dof::DepthOfFieldMode =
            serde_json::from_value(v["mode"].clone())
                .map(|m: DepthOfFieldMode| m.into())
                .unwrap_or(def.mode);
        let focal_distance = v["focal_distance"].as_f64().unwrap_or(0.0) as f32;
        let sensor_height = v["sensor_height"]
            .as_f64()
            .unwrap_or(def.sensor_height as f64) as f32;
        let aperture_f_stops = v["aperture_f_stops"]
            .as_f64()
            .unwrap_or(def.aperture_f_stops as f64) as f32;
        let max_circle_of_confusion_diameter = v["max_circle_of_confusion_diameter"]
            .as_f64()
            .unwrap_or(def.max_circle_of_confusion_diameter as f64)
            as f32;
        let max_depth = v["max_depth"].as_f64().unwrap_or(def.max_depth as f64) as f32;
        Ok(DepthOfField(std::sync::Arc::new(
            bevy::post_process::dof::DepthOfField {
                mode,
                focal_distance,
                sensor_height,
                aperture_f_stops,
                max_circle_of_confusion_diameter,
                max_depth,
            },
        )))
    }
}
impl From<DepthOfField> for bevy::post_process::dof::DepthOfField {
    fn from(v: DepthOfField) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DepthOfField {
    /// Returns the focal distance in meters.
    #[tracing::instrument(skip(self))]
    pub fn focal_distance(&self) -> f32 {
        self.0.focal_distance
    }

    /// Returns the depth of field mode name.
    #[tracing::instrument(skip(self))]
    pub fn mode_name(&self) -> &'static str {
        match self.0.mode {
            bevy::post_process::dof::DepthOfFieldMode::Bokeh => "Bokeh",
            bevy::post_process::dof::DepthOfFieldMode::Gaussian => "Gaussian",
        }
    }
}

mod emit_impls_dof {
    use super::{DepthOfField, DepthOfFieldMode};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DepthOfField {
        fn to_code_literal(&self) -> TokenStream {
            let inner = &*self.0;
            let mode = DepthOfFieldMode(std::sync::Arc::new(inner.mode)).to_code_literal();
            let focal_distance = inner.focal_distance;
            let sensor_height = inner.sensor_height;
            let aperture_f_stops = inner.aperture_f_stops;
            let max_coc = inner.max_circle_of_confusion_diameter;
            let max_depth = inner.max_depth;
            quote::quote! {
                ::bevy::post_process::dof::DepthOfField {
                    mode: #mode,
                    focal_distance: #focal_distance,
                    sensor_height: #sensor_height,
                    aperture_f_stops: #aperture_f_stops,
                    max_circle_of_confusion_diameter: #max_coc,
                    max_depth: #max_depth,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for DepthOfField {}

// ── MotionBlur ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::post_process::motion_blur::MotionBlur, as MotionBlur, nodebug);
elicit_newtype_traits!(MotionBlur, bevy::post_process::motion_blur::MotionBlur, []);

impl serde::Serialize for MotionBlur {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("MotionBlur", 2)?;
        st.serialize_field("shutter_angle", &self.0.shutter_angle)?;
        st.serialize_field("samples", &self.0.samples)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for MotionBlur {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let shutter_angle = v["shutter_angle"].as_f64().unwrap_or(0.5) as f32;
        let samples = v["samples"].as_u64().unwrap_or(1) as u32;
        Ok(MotionBlur(std::sync::Arc::new(
            bevy::post_process::motion_blur::MotionBlur {
                shutter_angle,
                samples,
            },
        )))
    }
}

#[reflect_methods]
impl MotionBlur {
    /// Returns the shutter angle.
    #[tracing::instrument(skip(self))]
    pub fn shutter_angle(&self) -> f32 {
        self.0.shutter_angle
    }
    /// Returns the sample count.
    #[tracing::instrument(skip(self))]
    pub fn samples(&self) -> u32 {
        self.0.samples
    }
}

mod emit_impls_motion_blur {
    use super::MotionBlur;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for MotionBlur {
        fn to_code_literal(&self) -> TokenStream {
            let shutter_angle = self.0.shutter_angle;
            let samples = self.0.samples;
            quote::quote! {
                ::bevy::post_process::motion_blur::MotionBlur {
                    shutter_angle: #shutter_angle,
                    samples: #samples,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for MotionBlur {}

// ── AutoExposure ──────────────────────────────────────────────────────────────
//
// Fields `metering_mask` (Handle<Image>) and `compensation_curve`
// (Handle<AutoExposureCompensationCurve>) are asset handles that cannot be
// meaningfully serialised as values; they are skipped.

elicit_newtype!(bevy::post_process::auto_exposure::AutoExposure, as AutoExposure, nodebug);
elicit_newtype_traits!(
    AutoExposure,
    bevy::post_process::auto_exposure::AutoExposure,
    []
);

impl serde::Serialize for AutoExposure {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("AutoExposure", 5)?;
        st.serialize_field("range_start", self.0.range.start())?;
        st.serialize_field("range_end", self.0.range.end())?;
        st.serialize_field("filter_start", self.0.filter.start())?;
        st.serialize_field("filter_end", self.0.filter.end())?;
        st.serialize_field("speed_brighten", &self.0.speed_brighten)?;
        st.serialize_field("speed_darken", &self.0.speed_darken)?;
        st.serialize_field(
            "exponential_transition_distance",
            &self.0.exponential_transition_distance,
        )?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for AutoExposure {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let range_start = v["range_start"].as_f64().unwrap_or(-8.0) as f32;
        let range_end = v["range_end"].as_f64().unwrap_or(8.0) as f32;
        let filter_start = v["filter_start"].as_f64().unwrap_or(0.10) as f32;
        let filter_end = v["filter_end"].as_f64().unwrap_or(0.90) as f32;
        let speed_brighten = v["speed_brighten"].as_f64().unwrap_or(3.0) as f32;
        let speed_darken = v["speed_darken"].as_f64().unwrap_or(1.0) as f32;
        let exponential_transition_distance =
            v["exponential_transition_distance"].as_f64().unwrap_or(1.5) as f32;
        Ok(AutoExposure(std::sync::Arc::new(
            bevy::post_process::auto_exposure::AutoExposure {
                range: range_start..=range_end,
                filter: filter_start..=filter_end,
                speed_brighten,
                speed_darken,
                exponential_transition_distance,
                ..Default::default()
            },
        )))
    }
}

#[reflect_methods]
impl AutoExposure {
    /// Returns the exposure range start.
    #[tracing::instrument(skip(self))]
    pub fn range_start(&self) -> f32 {
        *self.0.range.start()
    }
    /// Returns the exposure range end.
    #[tracing::instrument(skip(self))]
    pub fn range_end(&self) -> f32 {
        *self.0.range.end()
    }
}

mod emit_impls_auto_exposure {
    use super::AutoExposure;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for AutoExposure {
        fn to_code_literal(&self) -> TokenStream {
            let range_start = *self.0.range.start();
            let range_end = *self.0.range.end();
            let filter_start = *self.0.filter.start();
            let filter_end = *self.0.filter.end();
            let speed_brighten = self.0.speed_brighten;
            let speed_darken = self.0.speed_darken;
            let etd = self.0.exponential_transition_distance;
            quote::quote! {
                ::bevy::post_process::auto_exposure::AutoExposure {
                    range: #range_start..=#range_end,
                    filter: #filter_start..=#filter_end,
                    speed_brighten: #speed_brighten,
                    speed_darken: #speed_darken,
                    exponential_transition_distance: #etd,
                    ..::std::default::Default::default()
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for AutoExposure {}

// ── ChromaticAberration ───────────────────────────────────────────────────────
//
// The `color_lut` field is `Option<Handle<Image>>` and is skipped.

elicit_newtype!(
    bevy::post_process::effect_stack::ChromaticAberration,
    as ChromaticAberration,
    nodebug
);
elicit_newtype_traits!(
    ChromaticAberration,
    bevy::post_process::effect_stack::ChromaticAberration,
    []
);

impl serde::Serialize for ChromaticAberration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("ChromaticAberration", 2)?;
        st.serialize_field("intensity", &self.0.intensity)?;
        st.serialize_field("max_samples", &self.0.max_samples)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChromaticAberration {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let intensity = v["intensity"].as_f64().unwrap_or(0.02) as f32;
        let max_samples = v["max_samples"].as_u64().unwrap_or(8) as u32;
        Ok(ChromaticAberration(std::sync::Arc::new(
            bevy::post_process::effect_stack::ChromaticAberration {
                intensity,
                max_samples,
                ..Default::default()
            },
        )))
    }
}

#[reflect_methods]
impl ChromaticAberration {
    /// Returns the intensity.
    #[tracing::instrument(skip(self))]
    pub fn intensity(&self) -> f32 {
        self.0.intensity
    }
    /// Returns the max sample count.
    #[tracing::instrument(skip(self))]
    pub fn max_samples(&self) -> u32 {
        self.0.max_samples
    }
}

mod emit_impls_chromatic_aberration {
    use super::ChromaticAberration;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ChromaticAberration {
        fn to_code_literal(&self) -> TokenStream {
            let intensity = self.0.intensity;
            let max_samples = self.0.max_samples;
            quote::quote! {
                ::bevy::post_process::effect_stack::ChromaticAberration {
                    intensity: #intensity,
                    max_samples: #max_samples,
                    ..::std::default::Default::default()
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for ChromaticAberration {}
