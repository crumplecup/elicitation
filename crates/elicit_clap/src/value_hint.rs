//! [`clap::ValueHint`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ValueHint, as ValueHint);
elicit_newtype_traits!(ValueHint, clap::ValueHint, [eq]);

impl serde::Serialize for ValueHint {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for ValueHint {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Unknown" => clap::ValueHint::Unknown,
            "Other" => clap::ValueHint::Other,
            "AnyPath" => clap::ValueHint::AnyPath,
            "FilePath" => clap::ValueHint::FilePath,
            "DirPath" => clap::ValueHint::DirPath,
            "ExecutablePath" => clap::ValueHint::ExecutablePath,
            "CommandName" => clap::ValueHint::CommandName,
            "CommandString" => clap::ValueHint::CommandString,
            "CommandWithArguments" => clap::ValueHint::CommandWithArguments,
            "Username" => clap::ValueHint::Username,
            "Hostname" => clap::ValueHint::Hostname,
            "Url" => clap::ValueHint::Url,
            "EmailAddress" => clap::ValueHint::EmailAddress,
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &s,
                    &[
                        "Unknown",
                        "Other",
                        "AnyPath",
                        "FilePath",
                        "DirPath",
                        "ExecutablePath",
                        "CommandName",
                        "CommandString",
                        "CommandWithArguments",
                        "Username",
                        "Hostname",
                        "Url",
                        "EmailAddress",
                    ],
                ));
            }
        };
        Ok(ValueHint(std::sync::Arc::new(inner)))
    }
}

#[reflect_methods]
impl ValueHint {
    /// Returns `true` if this hint is path-related.
    #[tracing::instrument(skip(self))]
    pub fn is_path(&self) -> bool {
        matches!(
            *self.0,
            clap::ValueHint::AnyPath
                | clap::ValueHint::FilePath
                | clap::ValueHint::DirPath
                | clap::ValueHint::ExecutablePath
        )
    }

    /// Returns `true` if this hint is command-related.
    #[tracing::instrument(skip(self))]
    pub fn is_command(&self) -> bool {
        matches!(
            *self.0,
            clap::ValueHint::CommandName
                | clap::ValueHint::CommandString
                | clap::ValueHint::CommandWithArguments
        )
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            clap::ValueHint::Unknown => "Unknown",
            clap::ValueHint::Other => "Other",
            clap::ValueHint::AnyPath => "AnyPath",
            clap::ValueHint::FilePath => "FilePath",
            clap::ValueHint::DirPath => "DirPath",
            clap::ValueHint::ExecutablePath => "ExecutablePath",
            clap::ValueHint::CommandName => "CommandName",
            clap::ValueHint::CommandString => "CommandString",
            clap::ValueHint::CommandWithArguments => "CommandWithArguments",
            clap::ValueHint::Username => "Username",
            clap::ValueHint::Hostname => "Hostname",
            clap::ValueHint::Url => "Url",
            clap::ValueHint::EmailAddress => "EmailAddress",
            _ => "Unknown",
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ValueHint;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ValueHint {
        fn to_code_literal(&self) -> TokenStream {
            let variant = self.as_str();
            let variant_ident = quote::format_ident!("{}", variant);
            quote::quote! {
                ::elicit_clap::ValueHint::from(::clap::ValueHint::#variant_ident)
            }
        }
    }
}

impl elicitation::ElicitComplete for ValueHint {}
