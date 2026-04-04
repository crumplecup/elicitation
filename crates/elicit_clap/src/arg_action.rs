//! [`clap::ArgAction`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ArgAction, as ArgAction);
elicit_newtype_traits!(ArgAction, clap::ArgAction, []);

impl serde::Serialize for ArgAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ArgAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Set" => clap::ArgAction::Set,
            "Append" => clap::ArgAction::Append,
            "SetTrue" => clap::ArgAction::SetTrue,
            "SetFalse" => clap::ArgAction::SetFalse,
            "Count" => clap::ArgAction::Count,
            "Help" => clap::ArgAction::Help,
            "HelpShort" => clap::ArgAction::HelpShort,
            "Version" => clap::ArgAction::Version,
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &s,
                    &[
                        "Set",
                        "Append",
                        "SetTrue",
                        "SetFalse",
                        "Count",
                        "Help",
                        "HelpShort",
                        "Version",
                    ],
                ));
            }
        };
        Ok(ArgAction(std::sync::Arc::new(inner)))
    }
}

#[reflect_methods]
impl ArgAction {
    /// Returns `true` if this action stores a value (Set or Append).
    #[tracing::instrument(skip(self))]
    pub fn takes_value(&self) -> bool {
        matches!(*self.0, clap::ArgAction::Set | clap::ArgAction::Append)
    }

    /// Returns `true` if this is a flag action (SetTrue, SetFalse, Count).
    #[tracing::instrument(skip(self))]
    pub fn is_flag(&self) -> bool {
        matches!(
            *self.0,
            clap::ArgAction::SetTrue | clap::ArgAction::SetFalse | clap::ArgAction::Count
        )
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match &*self.0 {
            clap::ArgAction::Set => "Set",
            clap::ArgAction::Append => "Append",
            clap::ArgAction::SetTrue => "SetTrue",
            clap::ArgAction::SetFalse => "SetFalse",
            clap::ArgAction::Count => "Count",
            clap::ArgAction::Help => "Help",
            clap::ArgAction::HelpShort => "HelpShort",
            clap::ArgAction::Version => "Version",
            _ => "Unknown",
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ArgAction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ArgAction {
        fn to_code_literal(&self) -> TokenStream {
            let variant = self.as_str();
            let variant_ident = quote::format_ident!("{}", variant);
            quote::quote! {
                ::elicit_clap::ArgAction::from(::clap::ArgAction::#variant_ident)
            }
        }
    }
}

impl elicitation::ElicitComplete for ArgAction {}
