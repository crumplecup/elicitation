//! [`clap::parser::ValueSource`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::parser::ValueSource, as ValueSource);
elicit_newtype_traits!(ValueSource, clap::parser::ValueSource, [ord]);

impl serde::Serialize for ValueSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ValueSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "DefaultValue" => clap::parser::ValueSource::DefaultValue,
            "EnvVariable" => clap::parser::ValueSource::EnvVariable,
            "CommandLine" => clap::parser::ValueSource::CommandLine,
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &s,
                    &["DefaultValue", "EnvVariable", "CommandLine"],
                ));
            }
        };
        Ok(ValueSource(std::sync::Arc::new(inner)))
    }
}

#[reflect_methods]
impl ValueSource {
    /// Returns `true` if the value came from the default.
    #[tracing::instrument(skip(self))]
    pub fn is_default(&self) -> bool {
        matches!(*self.0, clap::parser::ValueSource::DefaultValue)
    }

    /// Returns `true` if the value came from an environment variable.
    #[tracing::instrument(skip(self))]
    pub fn is_env(&self) -> bool {
        matches!(*self.0, clap::parser::ValueSource::EnvVariable)
    }

    /// Returns `true` if the value came from the command line.
    #[tracing::instrument(skip(self))]
    pub fn is_cli(&self) -> bool {
        matches!(*self.0, clap::parser::ValueSource::CommandLine)
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            clap::parser::ValueSource::DefaultValue => "DefaultValue",
            clap::parser::ValueSource::EnvVariable => "EnvVariable",
            clap::parser::ValueSource::CommandLine => "CommandLine",
            _ => "Unknown",
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ValueSource;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ValueSource {
        fn to_code_literal(&self) -> TokenStream {
            let variant = self.as_str();
            let variant_ident = quote::format_ident!("{}", variant);
            quote::quote! {
                ::elicit_clap::ValueSource::from(::clap::parser::ValueSource::#variant_ident)
            }
        }
    }
}

impl elicitation::ElicitComplete for ValueSource {}
