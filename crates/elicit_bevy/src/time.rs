//! Bevy time shadow types.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── TimerMode ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::time::TimerMode, as TimerMode);
elicit_newtype_traits!(TimerMode, bevy::time::TimerMode, []);

impl serde::Serialize for TimerMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.variant_name())
    }
}
impl<'de> serde::Deserialize<'de> for TimerMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let value = String::deserialize(d)?;
        let mode = match value.as_str() {
            "Once" => bevy::time::TimerMode::Once,
            "Repeating" => bevy::time::TimerMode::Repeating,
            _ => return Err(D::Error::unknown_variant(&value, &["Once", "Repeating"])),
        };
        Ok(TimerMode(Arc::new(mode)))
    }
}
impl From<TimerMode> for bevy::time::TimerMode {
    fn from(v: TimerMode) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl TimerMode {
    /// Returns `"Once"` or `"Repeating"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::time::TimerMode::Once => "Once",
            bevy::time::TimerMode::Repeating => "Repeating",
        }
    }
}

mod emit_timermode {
    use super::TimerMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for TimerMode {
        fn to_code_literal(&self) -> TokenStream {
            let v = proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! { ::bevy::time::TimerMode::#v }
        }
    }
}
impl elicitation::ElicitComplete for TimerMode {}

// ── Timer ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::time::Timer, as Timer);
elicit_newtype_traits!(Timer, bevy::time::Timer, []);

impl serde::Serialize for Timer {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(3))?;
        map.serialize_entry("duration_secs", &self.0.duration().as_secs_f32())?;
        map.serialize_entry("mode", &TimerMode::from(self.0.mode()))?;
        map.serialize_entry("elapsed_secs", &self.0.elapsed_secs())?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Timer {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Timer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Timer object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Timer, A::Error> {
                let mut dur = 1.0f32;
                let mut mode = bevy::time::TimerMode::Once;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "duration_secs" => dur = map.next_value()?,
                        "mode" => {
                            mode = bevy::time::TimerMode::from(map.next_value::<TimerMode>()?)
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Timer(Arc::new(bevy::time::Timer::new(
                    std::time::Duration::from_secs_f32(dur),
                    mode,
                ))))
            }
        }
        d.deserialize_map(V)
    }
}
impl From<Timer> for bevy::time::Timer {
    fn from(v: Timer) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Timer {
    /// Duration in seconds.
    #[tracing::instrument(skip(self))]
    pub fn duration_secs(&self) -> f32 {
        self.0.duration().as_secs_f32()
    }
    /// Elapsed time in seconds.
    #[tracing::instrument(skip(self))]
    pub fn elapsed_secs(&self) -> f32 {
        self.0.elapsed_secs()
    }
    /// Remaining time in seconds.
    #[tracing::instrument(skip(self))]
    pub fn remaining_secs(&self) -> f32 {
        self.0.remaining_secs()
    }
    /// Fraction of the timer completed [0.0, 1.0].
    #[tracing::instrument(skip(self))]
    pub fn fraction(&self) -> f32 {
        self.0.fraction()
    }
    /// Returns `true` if the timer finished in the last tick.
    #[tracing::instrument(skip(self))]
    pub fn just_finished(&self) -> bool {
        self.0.just_finished()
    }
    /// Returns `true` if the timer has finished.
    #[tracing::instrument(skip(self))]
    pub fn finished(&self) -> bool {
        self.0.is_finished()
    }
    /// Returns `true` if the timer is paused.
    #[tracing::instrument(skip(self))]
    pub fn paused(&self) -> bool {
        self.0.is_paused()
    }
    /// Timer mode.
    #[tracing::instrument(skip(self))]
    pub fn mode(&self) -> TimerMode {
        TimerMode::from(self.0.mode())
    }
}

mod emit_timer {
    use super::Timer;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Timer {
        fn to_code_literal(&self) -> TokenStream {
            let secs = self.0.duration().as_secs_f32();
            let mode = proc_macro2::Ident::new(
                super::TimerMode::from(self.0.mode()).variant_name(),
                proc_macro2::Span::call_site(),
            );
            quote::quote! {
                ::bevy::time::Timer::new(
                    ::std::time::Duration::from_secs_f32(#secs),
                    ::bevy::time::TimerMode::#mode,
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Timer {}

// ── Stopwatch ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::time::Stopwatch, as Stopwatch);
elicit_newtype_traits!(Stopwatch, bevy::time::Stopwatch, []);

impl serde::Serialize for Stopwatch {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(2))?;
        map.serialize_entry("elapsed_secs", &self.0.elapsed_secs())?;
        map.serialize_entry("paused", &self.0.is_paused())?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Stopwatch {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Stopwatch;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Stopwatch object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut _map: A) -> Result<Stopwatch, A::Error> {
                while let Some(key) = _map.next_key::<String>()? {
                    _map.next_value::<serde::de::IgnoredAny>()?;
                    let _ = key;
                }
                Ok(Stopwatch(Arc::new(bevy::time::Stopwatch::new())))
            }
        }
        d.deserialize_map(V)
    }
}
impl From<Stopwatch> for bevy::time::Stopwatch {
    fn from(v: Stopwatch) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Stopwatch {
    /// Elapsed time in seconds.
    #[tracing::instrument(skip(self))]
    pub fn elapsed_secs(&self) -> f32 {
        self.0.elapsed_secs()
    }
    /// Returns `true` if the stopwatch is paused.
    #[tracing::instrument(skip(self))]
    pub fn is_paused(&self) -> bool {
        self.0.is_paused()
    }
}

mod emit_stopwatch {
    use super::Stopwatch;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Stopwatch {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::time::Stopwatch::new() }
        }
    }
}
impl elicitation::ElicitComplete for Stopwatch {}
