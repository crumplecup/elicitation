//! [`clap::builder::ValueRange`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::builder::ValueRange, as ValueRange);
elicit_newtype_traits!(ValueRange, clap::builder::ValueRange, [eq]);

impl serde::Serialize for ValueRange {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("min", &self.0.min_values())?;
        map.serialize_entry("max", &self.0.max_values())?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for ValueRange {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct ValueRangeVisitor;
        impl<'de> Visitor<'de> for ValueRangeVisitor {
            type Value = ValueRange;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"a value range object with "min" and "max" fields"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<ValueRange, A::Error> {
                let mut min: Option<usize> = None;
                let mut max: Option<usize> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "min" => min = Some(map.next_value()?),
                        "max" => max = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let min = min.ok_or_else(|| de::Error::missing_field("min"))?;
                let max = max.ok_or_else(|| de::Error::missing_field("max"))?;
                Ok(ValueRange(std::sync::Arc::new(
                    clap::builder::ValueRange::new(min..=max),
                )))
            }
        }
        deserializer.deserialize_map(ValueRangeVisitor)
    }
}

#[reflect_methods]
impl ValueRange {
    /// Returns the minimum number of values.
    #[tracing::instrument(skip(self))]
    pub fn min_values(&self) -> usize {
        self.0.min_values()
    }

    /// Returns the maximum number of values.
    #[tracing::instrument(skip(self))]
    pub fn max_values(&self) -> usize {
        self.0.max_values()
    }

    /// Returns `true` if this range accepts any values at all.
    #[tracing::instrument(skip(self))]
    pub fn takes_values(&self) -> bool {
        self.0.takes_values()
    }

    /// Returns `true` if this is exactly one value (the common case).
    #[tracing::instrument(skip(self))]
    pub fn is_exactly_one(&self) -> bool {
        self.0.min_values() == 1 && self.0.max_values() == 1
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ValueRange;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ValueRange {
        fn to_code_literal(&self) -> TokenStream {
            let min = self.0.min_values();
            let max = self.0.max_values();
            quote::quote! {
                ::elicit_clap::ValueRange::from(
                    ::clap::builder::ValueRange::new(#min..=#max)
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ValueRange {}
