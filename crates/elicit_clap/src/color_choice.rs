//! [`clap::ColorChoice`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ColorChoice, as ColorChoice);
elicit_newtype_traits!(ColorChoice, clap::ColorChoice, [eq, display]);

impl serde::Serialize for ColorChoice {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ColorChoice {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Auto" => clap::ColorChoice::Auto,
            "Always" => clap::ColorChoice::Always,
            "Never" => clap::ColorChoice::Never,
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &s,
                    &["Auto", "Always", "Never"],
                ));
            }
        };
        Ok(ColorChoice(std::sync::Arc::new(inner)))
    }
}

#[reflect_methods]
impl ColorChoice {
    /// Returns `true` if this is `ColorChoice::Auto`.
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Auto)
    }

    /// Returns `true` if this is `ColorChoice::Always`.
    #[tracing::instrument(skip(self))]
    pub fn is_always(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Always)
    }

    /// Returns `true` if this is `ColorChoice::Never`.
    #[tracing::instrument(skip(self))]
    pub fn is_never(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Never)
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            clap::ColorChoice::Auto => "Auto",
            clap::ColorChoice::Always => "Always",
            clap::ColorChoice::Never => "Never",
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ColorChoice;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ColorChoice {
        fn to_code_literal(&self) -> TokenStream {
            let variant = self.as_str();
            let variant_ident = quote::format_ident!("{}", variant);
            quote::quote! {
                ::elicit_clap::ColorChoice::from(::clap::ColorChoice::#variant_ident)
            }
        }
    }
}

impl elicitation::ElicitComplete for ColorChoice {}
