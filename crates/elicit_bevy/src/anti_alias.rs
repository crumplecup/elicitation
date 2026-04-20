//! Bevy anti-aliasing shadow types.
//!
//! Covers:
//! - [`bevy::anti_alias::fxaa::Sensitivity`]
//! - [`bevy::anti_alias::fxaa::Fxaa`]
//! - [`bevy::anti_alias::smaa::SmaaPreset`]
//! - [`bevy::anti_alias::smaa::Smaa`]
//! - [`bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening`]

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Sensitivity (FXAA edge sensitivity) ──────────────────────────────────────

elicit_newtype!(bevy::anti_alias::fxaa::Sensitivity, as Sensitivity);
elicit_newtype_traits!(Sensitivity, bevy::anti_alias::fxaa::Sensitivity, [eq_hash]);

impl serde::Serialize for Sensitivity {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match *self.0 {
            bevy::anti_alias::fxaa::Sensitivity::Low => "Low",
            bevy::anti_alias::fxaa::Sensitivity::Medium => "Medium",
            bevy::anti_alias::fxaa::Sensitivity::High => "High",
            bevy::anti_alias::fxaa::Sensitivity::Ultra => "Ultra",
            bevy::anti_alias::fxaa::Sensitivity::Extreme => "Extreme",
        })
    }
}
impl<'de> serde::Deserialize<'de> for Sensitivity {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let inner = match s.as_str() {
            "Low" => bevy::anti_alias::fxaa::Sensitivity::Low,
            "Medium" => bevy::anti_alias::fxaa::Sensitivity::Medium,
            "High" => bevy::anti_alias::fxaa::Sensitivity::High,
            "Ultra" => bevy::anti_alias::fxaa::Sensitivity::Ultra,
            "Extreme" => bevy::anti_alias::fxaa::Sensitivity::Extreme,
            other => {
                return Err(serde::de::Error::unknown_variant(
                    other,
                    &["Low", "Medium", "High", "Ultra", "Extreme"],
                ));
            }
        };
        Ok(Sensitivity(std::sync::Arc::new(inner)))
    }
}

mod emit_sensitivity {
    use super::Sensitivity;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Sensitivity {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::anti_alias::fxaa::Sensitivity::Low => {
                    quote::quote! { ::bevy::anti_alias::fxaa::Sensitivity::Low }
                }
                bevy::anti_alias::fxaa::Sensitivity::Medium => {
                    quote::quote! { ::bevy::anti_alias::fxaa::Sensitivity::Medium }
                }
                bevy::anti_alias::fxaa::Sensitivity::High => {
                    quote::quote! { ::bevy::anti_alias::fxaa::Sensitivity::High }
                }
                bevy::anti_alias::fxaa::Sensitivity::Ultra => {
                    quote::quote! { ::bevy::anti_alias::fxaa::Sensitivity::Ultra }
                }
                bevy::anti_alias::fxaa::Sensitivity::Extreme => {
                    quote::quote! { ::bevy::anti_alias::fxaa::Sensitivity::Extreme }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Sensitivity {}

// ── Fxaa ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::anti_alias::fxaa::Fxaa, as Fxaa, nodebug);
elicit_newtype_traits!(Fxaa, bevy::anti_alias::fxaa::Fxaa, []);

impl serde::Serialize for Fxaa {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("Fxaa", 3)?;
        st.serialize_field("enabled", &self.0.enabled)?;
        st.serialize_field(
            "edge_threshold",
            &Sensitivity(std::sync::Arc::new(self.0.edge_threshold)),
        )?;
        st.serialize_field(
            "edge_threshold_min",
            &Sensitivity(std::sync::Arc::new(self.0.edge_threshold_min)),
        )?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fxaa {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let enabled = v["enabled"].as_bool().unwrap_or(true);
        let edge_threshold = parse_sensitivity(&v["edge_threshold"]);
        let edge_threshold_min = parse_sensitivity(&v["edge_threshold_min"]);
        Ok(Fxaa(std::sync::Arc::new(bevy::anti_alias::fxaa::Fxaa {
            enabled,
            edge_threshold,
            edge_threshold_min,
        })))
    }
}

fn parse_sensitivity(v: &serde_json::Value) -> bevy::anti_alias::fxaa::Sensitivity {
    match v.as_str().unwrap_or("High") {
        "Low" => bevy::anti_alias::fxaa::Sensitivity::Low,
        "Medium" => bevy::anti_alias::fxaa::Sensitivity::Medium,
        "Ultra" => bevy::anti_alias::fxaa::Sensitivity::Ultra,
        "Extreme" => bevy::anti_alias::fxaa::Sensitivity::Extreme,
        _ => bevy::anti_alias::fxaa::Sensitivity::High,
    }
}

#[reflect_methods]
impl Fxaa {
    /// Returns whether FXAA is enabled.
    #[tracing::instrument(skip(self))]
    pub fn enabled(&self) -> bool {
        self.0.enabled
    }
}

mod emit_fxaa {
    use super::{Fxaa, Sensitivity};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Fxaa {
        fn to_code_literal(&self) -> TokenStream {
            let enabled = self.0.enabled;
            let et = Sensitivity(std::sync::Arc::new(self.0.edge_threshold)).to_code_literal();
            let etm = Sensitivity(std::sync::Arc::new(self.0.edge_threshold_min)).to_code_literal();
            quote::quote! {
                ::bevy::anti_alias::fxaa::Fxaa {
                    enabled: #enabled,
                    edge_threshold: #et,
                    edge_threshold_min: #etm,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Fxaa {}

// ── SmaaPreset ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::anti_alias::smaa::SmaaPreset, as SmaaPreset, nodebug);
elicit_newtype_traits!(SmaaPreset, bevy::anti_alias::smaa::SmaaPreset, [eq_hash]);

impl serde::Serialize for SmaaPreset {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(match *self.0 {
            bevy::anti_alias::smaa::SmaaPreset::Low => "Low",
            bevy::anti_alias::smaa::SmaaPreset::Medium => "Medium",
            bevy::anti_alias::smaa::SmaaPreset::High => "High",
            bevy::anti_alias::smaa::SmaaPreset::Ultra => "Ultra",
        })
    }
}
impl<'de> serde::Deserialize<'de> for SmaaPreset {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let inner = match s.as_str() {
            "Low" => bevy::anti_alias::smaa::SmaaPreset::Low,
            "Medium" => bevy::anti_alias::smaa::SmaaPreset::Medium,
            "High" => bevy::anti_alias::smaa::SmaaPreset::High,
            "Ultra" => bevy::anti_alias::smaa::SmaaPreset::Ultra,
            other => {
                return Err(serde::de::Error::unknown_variant(
                    other,
                    &["Low", "Medium", "High", "Ultra"],
                ));
            }
        };
        Ok(SmaaPreset(std::sync::Arc::new(inner)))
    }
}

mod emit_smaa_preset {
    use super::SmaaPreset;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for SmaaPreset {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::anti_alias::smaa::SmaaPreset::Low => {
                    quote::quote! { ::bevy::anti_alias::smaa::SmaaPreset::Low }
                }
                bevy::anti_alias::smaa::SmaaPreset::Medium => {
                    quote::quote! { ::bevy::anti_alias::smaa::SmaaPreset::Medium }
                }
                bevy::anti_alias::smaa::SmaaPreset::High => {
                    quote::quote! { ::bevy::anti_alias::smaa::SmaaPreset::High }
                }
                bevy::anti_alias::smaa::SmaaPreset::Ultra => {
                    quote::quote! { ::bevy::anti_alias::smaa::SmaaPreset::Ultra }
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for SmaaPreset {}

// ── Smaa ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::anti_alias::smaa::Smaa, as Smaa, nodebug);
elicit_newtype_traits!(Smaa, bevy::anti_alias::smaa::Smaa, []);

impl serde::Serialize for Smaa {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("Smaa", 1)?;
        st.serialize_field("preset", &SmaaPreset(std::sync::Arc::new(self.0.preset)))?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for Smaa {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let preset = match v["preset"].as_str().unwrap_or("High") {
            "Low" => bevy::anti_alias::smaa::SmaaPreset::Low,
            "Medium" => bevy::anti_alias::smaa::SmaaPreset::Medium,
            "Ultra" => bevy::anti_alias::smaa::SmaaPreset::Ultra,
            _ => bevy::anti_alias::smaa::SmaaPreset::High,
        };
        Ok(Smaa(std::sync::Arc::new(bevy::anti_alias::smaa::Smaa {
            preset,
        })))
    }
}

#[reflect_methods]
impl Smaa {
    /// Returns the SMAA quality preset name.
    #[tracing::instrument(skip(self))]
    pub fn preset_name(&self) -> &'static str {
        match self.0.preset {
            bevy::anti_alias::smaa::SmaaPreset::Low => "Low",
            bevy::anti_alias::smaa::SmaaPreset::Medium => "Medium",
            bevy::anti_alias::smaa::SmaaPreset::High => "High",
            bevy::anti_alias::smaa::SmaaPreset::Ultra => "Ultra",
        }
    }
}

mod emit_smaa {
    use super::{Smaa, SmaaPreset};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Smaa {
        fn to_code_literal(&self) -> TokenStream {
            let preset = SmaaPreset(std::sync::Arc::new(self.0.preset)).to_code_literal();
            quote::quote! {
                ::bevy::anti_alias::smaa::Smaa { preset: #preset }
            }
        }
    }
}
impl elicitation::ElicitComplete for Smaa {}

// ── ContrastAdaptiveSharpening ────────────────────────────────────────────────

elicit_newtype!(
    bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
    as ContrastAdaptiveSharpening,
    nodebug
);
elicit_newtype_traits!(
    ContrastAdaptiveSharpening,
    bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening,
    []
);

impl serde::Serialize for ContrastAdaptiveSharpening {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("ContrastAdaptiveSharpening", 3)?;
        st.serialize_field("enabled", &self.0.enabled)?;
        st.serialize_field("sharpening_strength", &self.0.sharpening_strength)?;
        st.serialize_field("denoise", &self.0.denoise)?;
        st.end()
    }
}
impl<'de> serde::Deserialize<'de> for ContrastAdaptiveSharpening {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v: serde_json::Value = serde_json::Value::deserialize(d)?;
        let enabled = v["enabled"].as_bool().unwrap_or(true);
        let sharpening_strength = v["sharpening_strength"].as_f64().unwrap_or(0.6) as f32;
        let denoise = v["denoise"].as_bool().unwrap_or(false);
        Ok(ContrastAdaptiveSharpening(std::sync::Arc::new(
            bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening {
                enabled,
                sharpening_strength,
                denoise,
            },
        )))
    }
}

#[reflect_methods]
impl ContrastAdaptiveSharpening {
    /// Returns whether CAS is enabled.
    #[tracing::instrument(skip(self))]
    pub fn enabled(&self) -> bool {
        self.0.enabled
    }
    /// Returns the sharpening strength.
    #[tracing::instrument(skip(self))]
    pub fn sharpening_strength(&self) -> f32 {
        self.0.sharpening_strength
    }
}

mod emit_cas {
    use super::ContrastAdaptiveSharpening;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ContrastAdaptiveSharpening {
        fn to_code_literal(&self) -> TokenStream {
            let enabled = self.0.enabled;
            let strength = self.0.sharpening_strength;
            let denoise = self.0.denoise;
            quote::quote! {
                ::bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening {
                    enabled: #enabled,
                    sharpening_strength: #strength,
                    denoise: #denoise,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for ContrastAdaptiveSharpening {}
