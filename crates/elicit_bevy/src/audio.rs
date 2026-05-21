//! Audio wrappers.
//!
//! Covers [`PlaybackMode`], [`Volume`], [`PlaybackSettings`], and [`SpatialScale`].

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── PlaybackMode ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::audio::PlaybackMode, as PlaybackMode);
elicit_newtype_traits!(PlaybackMode, bevy::audio::PlaybackMode, []);

impl From<PlaybackMode> for bevy::audio::PlaybackMode {
    fn from(v: PlaybackMode) -> Self {
        *v.0
    }
}

impl serde::Serialize for PlaybackMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PlaybackMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Once" => bevy::audio::PlaybackMode::Once,
            "Loop" => bevy::audio::PlaybackMode::Loop,
            "Despawn" => bevy::audio::PlaybackMode::Despawn,
            "Remove" => bevy::audio::PlaybackMode::Remove,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Once", "Loop", "Despawn", "Remove"],
                ));
            }
        };
        Ok(PlaybackMode(Arc::new(inner)))
    }
}

#[reflect_methods]
impl PlaybackMode {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::audio::PlaybackMode::Once => "Once",
            bevy::audio::PlaybackMode::Loop => "Loop",
            bevy::audio::PlaybackMode::Despawn => "Despawn",
            bevy::audio::PlaybackMode::Remove => "Remove",
        }
    }

    /// Returns `true` if this is `PlaybackMode::Once`.
    #[tracing::instrument(skip(self))]
    pub fn is_once(&self) -> bool {
        matches!(*self.0, bevy::audio::PlaybackMode::Once)
    }

    /// Returns `true` if this is `PlaybackMode::Loop`.
    #[tracing::instrument(skip(self))]
    pub fn is_loop(&self) -> bool {
        matches!(*self.0, bevy::audio::PlaybackMode::Loop)
    }

    /// Returns `true` if this is `PlaybackMode::Despawn`.
    #[tracing::instrument(skip(self))]
    pub fn is_despawn(&self) -> bool {
        matches!(*self.0, bevy::audio::PlaybackMode::Despawn)
    }

    /// Returns `true` if this is `PlaybackMode::Remove`.
    #[tracing::instrument(skip(self))]
    pub fn is_remove(&self) -> bool {
        matches!(*self.0, bevy::audio::PlaybackMode::Remove)
    }
}

mod emit_impls_mode {
    use super::PlaybackMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PlaybackMode {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::PlaybackMode::from(::bevy::audio::PlaybackMode::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for PlaybackMode {}

// ── Volume ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::audio::Volume, as Volume);
elicit_newtype_traits!(Volume, bevy::audio::Volume, [eq]);

impl From<Volume> for bevy::audio::Volume {
    fn from(v: Volume) -> Self {
        *v.0
    }
}

impl serde::Serialize for Volume {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        match *self.0 {
            bevy::audio::Volume::Linear(v) => {
                map.serialize_entry("variant", "Linear")?;
                map.serialize_entry("value", &v)?;
            }
            bevy::audio::Volume::Decibels(v) => {
                map.serialize_entry("variant", "Decibels")?;
                map.serialize_entry("value", &v)?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Volume {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Volume;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an object with "variant": "Linear" | "Decibels" and "value": f32"#
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Volume, A::Error> {
                let mut variant: Option<String> = None;
                let mut value: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        "value" => value = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let v = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let val = value.unwrap_or(1.0);
                let inner = match v.as_str() {
                    "Linear" => bevy::audio::Volume::Linear(val),
                    "Decibels" => bevy::audio::Volume::Decibels(val),
                    other => {
                        return Err(de::Error::unknown_variant(other, &["Linear", "Decibels"]));
                    }
                };
                Ok(Volume(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Volume {
    /// Converts this volume to a linear scale value.
    #[tracing::instrument(skip(self))]
    pub fn to_linear(&self) -> f32 {
        self.0.to_linear()
    }

    /// Converts this volume to decibels.
    #[tracing::instrument(skip(self))]
    pub fn to_decibels(&self) -> f32 {
        self.0.to_decibels()
    }

    /// Returns `true` if this is `Volume::Linear`.
    #[tracing::instrument(skip(self))]
    pub fn is_linear(&self) -> bool {
        matches!(*self.0, bevy::audio::Volume::Linear(_))
    }

    /// Returns `true` if this is `Volume::Decibels`.
    #[tracing::instrument(skip(self))]
    pub fn is_decibels(&self) -> bool {
        matches!(*self.0, bevy::audio::Volume::Decibels(_))
    }

    /// Constructs a `Volume::Linear` from a linear scale value (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_linear_volume(&self, v: f32) -> Volume {
        Volume::from(bevy::audio::Volume::Linear(v))
    }

    /// Constructs a `Volume::Decibels` from a decibel value (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_db_volume(&self, db: f32) -> Volume {
        Volume::from(bevy::audio::Volume::Decibels(db))
    }

    /// Returns a new volume increased by the given percentage.
    #[tracing::instrument(skip(self))]
    pub fn increase_by_percentage(&self, percentage: f32) -> Volume {
        Volume::from(self.0.increase_by_percentage(percentage))
    }

    /// Returns a new volume decreased by the given percentage.
    #[tracing::instrument(skip(self))]
    pub fn decrease_by_percentage(&self, percentage: f32) -> Volume {
        Volume::from(self.0.decrease_by_percentage(percentage))
    }

    /// Returns a new volume scaled by the given factor.
    #[tracing::instrument(skip(self))]
    pub fn scale_to_factor(&self, factor: f32) -> Volume {
        Volume::from(self.0.scale_to_factor(factor))
    }

    /// Interpolates toward `target` by `factor` (0.0 = self, 1.0 = target).
    #[tracing::instrument(skip(self))]
    pub fn fade_towards(&self, target: Volume, factor: f32) -> Volume {
        Volume::from(self.0.fade_towards(*target.0, factor))
    }
}

mod emit_impls_volume {
    use super::Volume;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Volume {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::audio::Volume::Linear(v) => quote::quote! {
                    ::elicit_bevy::Volume::from(::bevy::audio::Volume::Linear(#v))
                },
                bevy::audio::Volume::Decibels(v) => quote::quote! {
                    ::elicit_bevy::Volume::from(::bevy::audio::Volume::Decibels(#v))
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for Volume {}

// ── PlaybackSettings ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::audio::PlaybackSettings, as PlaybackSettings);
elicit_newtype_traits!(PlaybackSettings, bevy::audio::PlaybackSettings, []);

impl From<PlaybackSettings> for bevy::audio::PlaybackSettings {
    fn from(v: PlaybackSettings) -> Self {
        *v.0
    }
}

impl serde::Serialize for PlaybackSettings {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let s = &*self.0;
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("mode", &PlaybackMode::from(s.mode))?;
        map.serialize_entry("volume", &Volume::from(s.volume))?;
        map.serialize_entry("speed", &s.speed)?;
        map.serialize_entry("paused", &s.paused)?;
        map.serialize_entry("spatial", &s.spatial)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for PlaybackSettings {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = PlaybackSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a PlaybackSettings JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<PlaybackSettings, A::Error> {
                let mut speed: Option<f32> = None;
                let mut paused: Option<bool> = None;
                let mut spatial: Option<bool> = None;
                let mut volume: Option<Volume> = None;
                let mut mode: Option<PlaybackMode> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "speed" => speed = Some(map.next_value()?),
                        "paused" => paused = Some(map.next_value()?),
                        "spatial" => spatial = Some(map.next_value()?),
                        "volume" => volume = Some(map.next_value()?),
                        "mode" => mode = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut s = bevy::audio::PlaybackSettings::default();
                if let Some(sp) = speed {
                    s.speed = sp;
                }
                if let Some(p) = paused {
                    s.paused = p;
                }
                if let Some(sp) = spatial {
                    s.spatial = sp;
                }
                if let Some(v) = volume {
                    s.volume = *v.0;
                }
                if let Some(m) = mode {
                    s.mode = *m.0;
                }
                Ok(PlaybackSettings(Arc::new(s)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl PlaybackSettings {
    /// Returns the playback speed multiplier.
    #[tracing::instrument(skip(self))]
    pub fn speed(&self) -> f32 {
        self.0.speed
    }

    /// Returns `true` if playback starts paused.
    #[tracing::instrument(skip(self))]
    pub fn paused(&self) -> bool {
        self.0.paused
    }

    /// Returns `true` if this is a spatial audio source.
    #[tracing::instrument(skip(self))]
    pub fn spatial(&self) -> bool {
        self.0.spatial
    }

    /// Returns the volume setting.
    #[tracing::instrument(skip(self))]
    pub fn volume(&self) -> Volume {
        Volume::from(self.0.volume)
    }

    /// Returns the playback mode.
    #[tracing::instrument(skip(self))]
    pub fn mode(&self) -> PlaybackMode {
        PlaybackMode::from(self.0.mode)
    }

    /// Returns a copy with the given playback speed.
    #[tracing::instrument(skip(self))]
    pub fn with_speed(&self, speed: f32) -> PlaybackSettings {
        let mut s = *self.0;
        s.speed = speed;
        PlaybackSettings::from(s)
    }

    /// Returns a copy with the given volume.
    #[tracing::instrument(skip(self))]
    pub fn with_volume(&self, v: Volume) -> PlaybackSettings {
        let mut s = *self.0;
        s.volume = *v.0;
        PlaybackSettings::from(s)
    }

    /// Returns a copy with the given playback mode.
    #[tracing::instrument(skip(self))]
    pub fn with_mode(&self, mode: PlaybackMode) -> PlaybackSettings {
        let mut s = *self.0;
        s.mode = *mode.0;
        PlaybackSettings::from(s)
    }

    /// Returns a copy configured to start paused.
    #[tracing::instrument(skip(self))]
    pub fn paused_settings(&self) -> PlaybackSettings {
        let mut s = *self.0;
        s.paused = true;
        PlaybackSettings::from(s)
    }
}

mod emit_impls_settings {
    use super::{PlaybackMode, PlaybackSettings, Volume};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PlaybackSettings {
        fn to_code_literal(&self) -> TokenStream {
            let mode = PlaybackMode::from(self.0.mode).to_code_literal();
            let vol = Volume::from(self.0.volume).to_code_literal();
            let speed = self.0.speed;
            let paused = self.0.paused;
            quote::quote! {
                ::elicit_bevy::PlaybackSettings::from(::bevy::audio::PlaybackSettings {
                    mode: ::bevy::audio::PlaybackMode::from(#mode),
                    volume: ::bevy::audio::Volume::from(#vol),
                    speed: #speed,
                    paused: #paused,
                    ..::std::default::Default::default()
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for PlaybackSettings {}

// ── SpatialScale ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::audio::SpatialScale, as SpatialScale);
elicit_newtype_traits!(SpatialScale, bevy::audio::SpatialScale, []);

impl From<SpatialScale> for bevy::audio::SpatialScale {
    fn from(v: SpatialScale) -> Self {
        *v.0
    }
}

impl serde::Serialize for SpatialScale {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("x", &self.0.0.x)?;
        map.serialize_entry("y", &self.0.0.y)?;
        map.serialize_entry("z", &self.0.0.z)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for SpatialScale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SpatialScale;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "x", "y", "z" f32 fields"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<SpatialScale, A::Error> {
                let mut x: Option<f32> = None;
                let mut y: Option<f32> = None;
                let mut z: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        "z" => z = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let inner = bevy::audio::SpatialScale(bevy::math::Vec3::new(
                    x.unwrap_or(1.0),
                    y.unwrap_or(1.0),
                    z.unwrap_or(1.0),
                ));
                Ok(SpatialScale(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl SpatialScale {
    /// Returns the x component of the spatial scale.
    #[tracing::instrument(skip(self))]
    pub fn x(&self) -> f32 {
        self.0.0.x
    }

    /// Returns the y component of the spatial scale.
    #[tracing::instrument(skip(self))]
    pub fn y(&self) -> f32 {
        self.0.0.y
    }

    /// Returns the z component of the spatial scale.
    #[tracing::instrument(skip(self))]
    pub fn z(&self) -> f32 {
        self.0.0.z
    }
}

mod emit_impls_spatial_scale {
    use super::SpatialScale;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpatialScale {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.0.x;
            let y = self.0.0.y;
            let z = self.0.0.z;
            quote::quote! {
                ::elicit_bevy::SpatialScale::from(::bevy::audio::SpatialScale(
                    ::bevy::math::Vec3::new(#x, #y, #z)
                ))
            }
        }
    }
}

impl elicitation::ElicitComplete for SpatialScale {}

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

// ── SpatialListener ───────────────────────────────────────────────────────────

/// Shadow of [`bevy::audio::SpatialListener`].
///
/// Ear offsets are stored as `[f32; 3]` arrays `[x, y, z]` to enable serde
/// without pulling in a math-type dependency.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpatialListener {
    /// Left ear position relative to the entity's `GlobalTransform`.
    pub left_ear_offset: [f32; 3],
    /// Right ear position relative to the entity's `GlobalTransform`.
    pub right_ear_offset: [f32; 3],
}

impl Default for SpatialListener {
    fn default() -> Self {
        Self::from(bevy::audio::SpatialListener::default())
    }
}

impl From<bevy::audio::SpatialListener> for SpatialListener {
    fn from(v: bevy::audio::SpatialListener) -> Self {
        Self {
            left_ear_offset: v.left_ear_offset.to_array(),
            right_ear_offset: v.right_ear_offset.to_array(),
        }
    }
}

impl From<SpatialListener> for bevy::audio::SpatialListener {
    fn from(v: SpatialListener) -> Self {
        Self {
            left_ear_offset: bevy::math::Vec3::from_array(v.left_ear_offset),
            right_ear_offset: bevy::math::Vec3::from_array(v.right_ear_offset),
        }
    }
}

mod emit_impls_spatial_listener {
    use super::SpatialListener;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpatialListener {
        fn to_code_literal(&self) -> TokenStream {
            let [lx, ly, lz] = self.left_ear_offset;
            let [rx, ry, rz] = self.right_ear_offset;
            quote::quote! {
                ::bevy::audio::SpatialListener {
                    left_ear_offset: ::bevy::math::Vec3::new(#lx, #ly, #lz),
                    right_ear_offset: ::bevy::math::Vec3::new(#rx, #ry, #rz),
                }
            }
        }
    }
}

shadow_elicitation!(SpatialListener);

// ── GlobalVolume ──────────────────────────────────────────────────────────────

/// Shadow of [`bevy::audio::GlobalVolume`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GlobalVolume {
    /// The global volume applied to all audio.
    pub volume: Volume,
}

impl From<bevy::audio::GlobalVolume> for GlobalVolume {
    fn from(v: bevy::audio::GlobalVolume) -> Self {
        Self {
            volume: Volume::from(v.volume),
        }
    }
}

impl From<GlobalVolume> for bevy::audio::GlobalVolume {
    fn from(v: GlobalVolume) -> Self {
        Self {
            volume: *v.volume.0,
        }
    }
}

mod emit_impls_global_volume {
    use super::GlobalVolume;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GlobalVolume {
        fn to_code_literal(&self) -> TokenStream {
            let vol_lit = self.volume.to_code_literal();
            quote::quote! {
                ::bevy::audio::GlobalVolume {
                    volume: #vol_lit,
                }
            }
        }
    }
}

shadow_elicitation!(GlobalVolume);

// ── DefaultSpatialScale ───────────────────────────────────────────────────────

/// Shadow of [`bevy::audio::DefaultSpatialScale`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct DefaultSpatialScale(pub SpatialScale);

impl From<bevy::audio::DefaultSpatialScale> for DefaultSpatialScale {
    fn from(v: bevy::audio::DefaultSpatialScale) -> Self {
        Self(SpatialScale::from(v.0))
    }
}

impl From<DefaultSpatialScale> for bevy::audio::DefaultSpatialScale {
    fn from(v: DefaultSpatialScale) -> Self {
        Self(*v.0.0)
    }
}

mod emit_impls_default_spatial_scale {
    use super::DefaultSpatialScale;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DefaultSpatialScale {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! {
                ::bevy::audio::DefaultSpatialScale(#inner)
            }
        }
    }
}

shadow_elicitation!(DefaultSpatialScale);
