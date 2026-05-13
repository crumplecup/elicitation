//! Bevy application exit status wrapper.
//!
//! The [`App`] struct itself is stateful and non-instantiable over MCP.
//! Only [`AppExit`] is wrapped here. Full `App` access should be handled
//! through a custom Phase 3C workflow plugin.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

elicit_newtype!(bevy::app::AppExit, as AppExit);
elicit_newtype_traits!(AppExit, bevy::app::AppExit, [eq]);

impl serde::Serialize for AppExit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        match *self.0 {
            bevy::app::AppExit::Success => {
                map.serialize_entry("type", "Success")?;
            }
            bevy::app::AppExit::Error(code) => {
                map.serialize_entry("type", "Error")?;
                map.serialize_entry("code", &code.get())?;
            }
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for AppExit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct AppExitVisitor;
        impl<'de> Visitor<'de> for AppExitVisitor {
            type Value = AppExit;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "type": "Success" or "type": "Error""#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<AppExit, A::Error> {
                let mut ty: Option<String> = None;
                let mut code: Option<u8> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => ty = Some(map.next_value()?),
                        "code" => code = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let ty = ty.ok_or_else(|| de::Error::missing_field("type"))?;
                let inner = match ty.as_str() {
                    "Success" => bevy::app::AppExit::Success,
                    "Error" => {
                        let c = code.unwrap_or(1);
                        bevy::app::AppExit::from_code(c)
                    }
                    other => {
                        return Err(de::Error::unknown_variant(other, &["Success", "Error"]));
                    }
                };
                Ok(AppExit(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(AppExitVisitor)
    }
}

impl From<AppExit> for bevy::app::AppExit {
    fn from(v: AppExit) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl AppExit {
    /// Returns `true` if this is [`AppExit::Success`].
    #[tracing::instrument(skip(self))]
    pub fn is_success(&self) -> bool {
        self.0.is_success()
    }

    /// Returns `true` if this is [`AppExit::Error`].
    #[tracing::instrument(skip(self))]
    pub fn is_error(&self) -> bool {
        self.0.is_error()
    }

    /// Returns the error code if this is [`AppExit::Error`], otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn error_code(&self) -> Option<u8> {
        match *self.0 {
            bevy::app::AppExit::Error(code) => Some(code.get()),
            bevy::app::AppExit::Success => None,
        }
    }

    /// Construct an [`AppExit::Success`] value.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn success_constructor(&self) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::Success))
    }

    /// Construct an [`AppExit::Error`] with exit code 1.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn error_constructor(&self) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::error()))
    }

    /// Construct an [`AppExit`] from a numeric exit code.
    ///
    /// Code 0 maps to [`AppExit::Success`]; non-zero maps to
    /// [`AppExit::Error`] carrying that code.
    ///
    /// The `&self` receiver is ignored; this is a factory constructor.
    #[tracing::instrument(skip(self))]
    pub fn from_code_constructor(&self, code: u8) -> AppExit {
        AppExit(Arc::new(bevy::app::AppExit::from_code(code)))
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::AppExit;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AppExit {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::app::AppExit::Success => quote::quote! {
                    ::elicit_bevy::AppExit::from(::bevy::app::AppExit::Success)
                },
                bevy::app::AppExit::Error(code) => {
                    let c = code.get();
                    quote::quote! {
                        ::elicit_bevy::AppExit::from(
                            ::bevy::app::AppExit::from_code(#c)
                        )
                    }
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for AppExit {}
