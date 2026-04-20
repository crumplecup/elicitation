//! Bevy asset system wrappers.
//!
//! ## Note on `Handle<T>`
//!
//! `Handle<T>` is generic over the asset type `T` and cannot be directly
//! wrapped as a single MCP type. Use the `AssetTools` factory from
//! `trait_factories.rs` to generate typed handle tools per asset type.
//! Agents interact with assets via their string path or UUID-based `AssetId`.
//!
//! ## Note on `DependencyLoadState`
//!
//! [`bevy::asset::DependencyLoadState`] tracks the direct dependencies of an
//! asset. [`RecursiveDependencyLoadState`] tracks the full transitive dependency
//! tree. Both are wrapped here with the same method surface.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── LoadState ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::asset::LoadState, as LoadState);
elicit_newtype_traits!(LoadState, bevy::asset::LoadState, []);

impl serde::Serialize for LoadState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        match &*self.0 {
            bevy::asset::LoadState::NotLoaded => {
                map.serialize_entry("variant", "NotLoaded")?;
            }
            bevy::asset::LoadState::Loading => {
                map.serialize_entry("variant", "Loading")?;
            }
            bevy::asset::LoadState::Loaded => {
                map.serialize_entry("variant", "Loaded")?;
            }
            bevy::asset::LoadState::Failed(err) => {
                map.serialize_entry("variant", "Failed")?;
                map.serialize_entry("error", &err.to_string())?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for LoadState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct LoadStateVisitor;
        impl<'de> Visitor<'de> for LoadStateVisitor {
            type Value = LoadState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an object with "variant": "NotLoaded" | "Loading" | "Loaded" | "Failed""#
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<LoadState, A::Error> {
                let mut variant: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let variant = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match variant.as_str() {
                    "NotLoaded" => bevy::asset::LoadState::NotLoaded,
                    "Loading" => bevy::asset::LoadState::Loading,
                    "Loaded" => bevy::asset::LoadState::Loaded,
                    other => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &["NotLoaded", "Loading", "Loaded", "Failed"],
                        ));
                    }
                };
                Ok(LoadState(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(LoadStateVisitor)
    }
}

impl From<LoadState> for bevy::asset::LoadState {
    fn from(v: LoadState) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl LoadState {
    /// Returns `true` if the asset has finished loading successfully.
    #[tracing::instrument(skip(self))]
    pub fn is_loaded(&self) -> bool {
        self.0.is_loaded()
    }

    /// Returns `true` if the asset is currently being loaded.
    #[tracing::instrument(skip(self))]
    pub fn is_loading(&self) -> bool {
        self.0.is_loading()
    }

    /// Returns `true` if the asset failed to load.
    #[tracing::instrument(skip(self))]
    pub fn is_failed(&self) -> bool {
        self.0.is_failed()
    }

    /// Returns the variant name as a string: `"NotLoaded"`, `"Loading"`,
    /// `"Loaded"`, or `"Failed"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::asset::LoadState::NotLoaded => "NotLoaded",
            bevy::asset::LoadState::Loading => "Loading",
            bevy::asset::LoadState::Loaded => "Loaded",
            bevy::asset::LoadState::Failed(_) => "Failed",
        }
    }

    /// Returns the error message if the state is [`LoadState::Failed`],
    /// otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn error_message(&self) -> Option<String> {
        match &*self.0 {
            bevy::asset::LoadState::Failed(err) => Some(err.to_string()),
            _ => None,
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod load_state_emit {
    use super::LoadState;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for LoadState {
        fn to_code_literal(&self) -> TokenStream {
            let variant =
                proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! {
                ::elicit_bevy::LoadState::from(::bevy::asset::LoadState::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for LoadState {}

// ── RecursiveDependencyLoadState ─────────────────────────────────────────────

elicit_newtype!(
    bevy::asset::RecursiveDependencyLoadState,
    as RecursiveDependencyLoadState
);
elicit_newtype_traits!(
    RecursiveDependencyLoadState,
    bevy::asset::RecursiveDependencyLoadState,
    []
);

impl serde::Serialize for RecursiveDependencyLoadState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        match &*self.0 {
            bevy::asset::RecursiveDependencyLoadState::NotLoaded => {
                map.serialize_entry("variant", "NotLoaded")?;
            }
            bevy::asset::RecursiveDependencyLoadState::Loading => {
                map.serialize_entry("variant", "Loading")?;
            }
            bevy::asset::RecursiveDependencyLoadState::Loaded => {
                map.serialize_entry("variant", "Loaded")?;
            }
            bevy::asset::RecursiveDependencyLoadState::Failed(err) => {
                map.serialize_entry("variant", "Failed")?;
                map.serialize_entry("error", &err.to_string())?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for RecursiveDependencyLoadState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct Vis;
        impl<'de> Visitor<'de> for Vis {
            type Value = RecursiveDependencyLoadState;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an object with "variant": "NotLoaded" | "Loading" | "Loaded" | "Failed""#
                )
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<RecursiveDependencyLoadState, A::Error> {
                let mut variant: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let variant = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match variant.as_str() {
                    "NotLoaded" => bevy::asset::RecursiveDependencyLoadState::NotLoaded,
                    "Loading" => bevy::asset::RecursiveDependencyLoadState::Loading,
                    "Loaded" => bevy::asset::RecursiveDependencyLoadState::Loaded,
                    other => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &["NotLoaded", "Loading", "Loaded", "Failed"],
                        ));
                    }
                };
                Ok(RecursiveDependencyLoadState(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(Vis)
    }
}

impl From<RecursiveDependencyLoadState> for bevy::asset::RecursiveDependencyLoadState {
    fn from(v: RecursiveDependencyLoadState) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl RecursiveDependencyLoadState {
    /// Returns `true` if all transitive dependencies have finished loading.
    #[tracing::instrument(skip(self))]
    pub fn is_loaded(&self) -> bool {
        self.0.is_loaded()
    }

    /// Returns `true` if transitive dependencies are still loading.
    #[tracing::instrument(skip(self))]
    pub fn is_loading(&self) -> bool {
        self.0.is_loading()
    }

    /// Returns `true` if any transitive dependency failed to load.
    #[tracing::instrument(skip(self))]
    pub fn is_failed(&self) -> bool {
        self.0.is_failed()
    }

    /// Returns the variant name: `"NotLoaded"`, `"Loading"`, `"Loaded"`, or
    /// `"Failed"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::asset::RecursiveDependencyLoadState::NotLoaded => "NotLoaded",
            bevy::asset::RecursiveDependencyLoadState::Loading => "Loading",
            bevy::asset::RecursiveDependencyLoadState::Loaded => "Loaded",
            bevy::asset::RecursiveDependencyLoadState::Failed(_) => "Failed",
        }
    }

    /// Returns the error message if the state is `Failed`, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn error_message(&self) -> Option<String> {
        match &*self.0 {
            bevy::asset::RecursiveDependencyLoadState::Failed(err) => Some(err.to_string()),
            _ => None,
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod rdls_emit {
    use super::RecursiveDependencyLoadState;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RecursiveDependencyLoadState {
        fn to_code_literal(&self) -> TokenStream {
            let variant =
                proc_macro2::Ident::new(self.variant_name(), proc_macro2::Span::call_site());
            quote::quote! {
                ::elicit_bevy::RecursiveDependencyLoadState::from(
                    ::bevy::asset::RecursiveDependencyLoadState::#variant
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for RecursiveDependencyLoadState {}
