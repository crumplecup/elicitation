//! [`clap::ArgGroup`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ArgGroup, as ArgGroup);
elicit_newtype_traits!(ArgGroup, clap::ArgGroup, [eq]);

impl serde::Serialize for ArgGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let grp = &*self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &grp.get_id().to_string())?;
        map.serialize_entry("required", &grp.is_required_set())?;
        let args: Vec<String> = grp.get_args().map(|id| id.to_string()).collect();
        map.serialize_entry("args", &args)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for ArgGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct ArgGroupVisitor;
        impl<'de> Visitor<'de> for ArgGroupVisitor {
            type Value = ArgGroup;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an arg group object with at least an "id" field"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<ArgGroup, A::Error> {
                let mut id: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "id" {
                        id = Some(map.next_value()?);
                    } else {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(ArgGroup(std::sync::Arc::new(clap::ArgGroup::new(id))))
            }
        }
        deserializer.deserialize_map(ArgGroupVisitor)
    }
}

#[reflect_methods]
impl ArgGroup {
    /// Returns the group's identifier as a string.
    #[tracing::instrument(skip(self))]
    pub fn get_id(&self) -> String {
        self.0.get_id().to_string()
    }

    /// Returns `true` if at least one argument in this group is required.
    #[tracing::instrument(skip(self))]
    pub fn is_required(&self) -> bool {
        self.0.is_required_set()
    }

    /// Returns the member argument IDs as strings.
    #[tracing::instrument(skip(self))]
    pub fn get_args(&self) -> Vec<String> {
        self.0.get_args().map(|id| id.to_string()).collect()
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ArgGroup;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ArgGroup {
        fn to_code_literal(&self) -> TokenStream {
            let id = self.0.get_id().to_string();
            quote::quote! {
                ::elicit_clap::ArgGroup::from(::clap::ArgGroup::new(#id))
            }
        }
    }
}

impl elicitation::ElicitComplete for ArgGroup {}
