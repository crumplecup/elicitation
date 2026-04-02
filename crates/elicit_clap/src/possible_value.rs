//! [`clap::builder::PossibleValue`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::builder::PossibleValue, as PossibleValue);
elicit_newtype_traits!(PossibleValue, clap::builder::PossibleValue, [eq]);

/// Unwrap the Arc back to an owned `clap::builder::PossibleValue`.
impl From<PossibleValue> for clap::builder::PossibleValue {
    fn from(val: PossibleValue) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for PossibleValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let pv = &*self.0;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("name", pv.get_name())?;
        if let Some(help) = pv.get_help() {
            map.serialize_entry("help", &help.to_string())?;
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for PossibleValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct PossibleValueVisitor;
        impl<'de> Visitor<'de> for PossibleValueVisitor {
            type Value = PossibleValue;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"a possible value object with at least a "name" field"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<PossibleValue, A::Error> {
                let mut name: Option<String> = None;
                let mut help: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => name = Some(map.next_value()?),
                        "help" => help = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let mut pv = clap::builder::PossibleValue::new(name);
                if let Some(h) = help {
                    pv = pv.help(h);
                }
                Ok(PossibleValue(std::sync::Arc::new(pv)))
            }
        }
        deserializer.deserialize_map(PossibleValueVisitor)
    }
}

#[reflect_methods]
impl PossibleValue {
    /// Returns the name of this possible value.
    #[tracing::instrument(skip(self))]
    pub fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    /// Returns `true` if this value is hidden from help output.
    #[tracing::instrument(skip(self))]
    pub fn is_hidden(&self) -> bool {
        self.0.is_hide_set()
    }

    /// Returns `true` if the given string matches this value (case-sensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches(&self, value: String) -> bool {
        self.0.matches(&value, false)
    }

    /// Returns `true` if the given string matches this value (case-insensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches_ignore_case(&self, value: String) -> bool {
        self.0.matches(&value, true)
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::PossibleValue;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PossibleValue {
        fn to_code_literal(&self) -> TokenStream {
            let name = self.0.get_name().to_string();
            if let Some(help) = self.0.get_help() {
                let help_str = help.to_string();
                quote::quote! {
                    ::elicit_clap::PossibleValue::from(
                        ::clap::builder::PossibleValue::new(#name).help(#help_str)
                    )
                }
            } else {
                quote::quote! {
                    ::elicit_clap::PossibleValue::from(
                        ::clap::builder::PossibleValue::new(#name)
                    )
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for PossibleValue {}
