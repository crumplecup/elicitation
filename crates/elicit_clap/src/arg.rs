//! [`clap::Arg`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Arg, as Arg);
elicit_newtype_traits!(Arg, clap::Arg, []);

impl serde::Serialize for Arg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let arg = &*self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &arg.get_id().to_string())?;
        if let Some(long) = arg.get_long() {
            map.serialize_entry("long", long)?;
        }
        if let Some(short) = arg.get_short() {
            map.serialize_entry("short", &short.to_string())?;
        }
        if let Some(help) = arg.get_help() {
            map.serialize_entry("help", &help.to_string())?;
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Arg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct ArgVisitor;
        impl<'de> Visitor<'de> for ArgVisitor {
            type Value = Arg;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an arg object with at least an "id" field"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Arg, A::Error> {
                let mut id: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "id" {
                        id = Some(map.next_value()?);
                    } else {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                Ok(Arg(std::sync::Arc::new(clap::Arg::new(id))))
            }
        }
        deserializer.deserialize_map(ArgVisitor)
    }
}

#[reflect_methods]
impl Arg {
    /// Returns the argument's identifier as a string.
    #[tracing::instrument(skip(self))]
    pub fn get_id(&self) -> String {
        self.0.get_id().to_string()
    }

    /// Returns the long flag name (e.g. `"output"` for `--output`), if set.
    #[tracing::instrument(skip(self))]
    pub fn get_long(&self) -> Option<String> {
        self.0.get_long().map(str::to_string)
    }

    /// Returns the short flag character (e.g. `'o'` for `-o`), if set.
    #[tracing::instrument(skip(self))]
    pub fn get_short(&self) -> Option<char> {
        self.0.get_short()
    }

    /// Returns the help text for this argument, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_help(&self) -> Option<String> {
        self.0.get_help().map(|s| s.to_string())
    }

    /// Returns the display order for this argument.
    #[tracing::instrument(skip(self))]
    pub fn get_display_order(&self) -> usize {
        self.0.get_display_order()
    }

    /// Returns the possible values for this argument.
    #[tracing::instrument(skip(self))]
    pub fn get_possible_values(&self) -> Vec<String> {
        self.0
            .get_possible_values()
            .into_iter()
            .map(|pv| pv.get_name().to_string())
            .collect()
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::Arg;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Arg {
        fn to_code_literal(&self) -> TokenStream {
            let id = self.0.get_id().to_string();
            quote::quote! {
                ::elicit_clap::Arg::from(::clap::Arg::new(#id))
            }
        }
    }
}

impl elicitation::ElicitComplete for Arg {}
