//! `Id` — GeoJSON feature identifier wrapper.

elicitation::elicit_newtype!(geojson::feature::Id, as Id);

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&*self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde_json::Value::deserialize(deserializer)? {
            serde_json::Value::String(value) => Ok(Self::string(value)),
            serde_json::Value::Number(value) => Ok(Self::number(value)),
            other => Err(serde::de::Error::custom(format!(
                "expected GeoJSON feature id string or number, got {}",
                crate::helpers::json_type_name(&other)
            ))),
        }
    }
}

impl From<Id> for geojson::feature::Id {
    fn from(value: Id) -> Self {
        (*value.0).clone()
    }
}

impl From<&Id> for geojson::feature::Id {
    fn from(value: &Id) -> Self {
        (*value.0).clone()
    }
}

impl Id {
    /// Creates a string feature identifier.
    #[tracing::instrument(skip(value))]
    pub fn string(value: impl Into<String>) -> Self {
        Self::from(geojson::feature::Id::String(value.into()))
    }

    /// Creates a numeric feature identifier.
    #[tracing::instrument]
    pub fn number(value: serde_json::Number) -> Self {
        Self::from(geojson::feature::Id::Number(value))
    }
}

mod emit_impls {
    use super::Id;

    impl elicitation::emit_code::ToCodeLiteral for Id {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("Id is serializable");
            quote::quote! {
                ::elicit_geojson::Id::from(
                    ::serde_json::from_str::<::geojson::feature::Id>(#json)
                        .expect("valid Id JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Id {}
