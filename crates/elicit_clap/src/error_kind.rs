//! [`clap::error::ErrorKind`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::error::ErrorKind, as ErrorKind);
elicit_newtype_traits!(ErrorKind, clap::error::ErrorKind, [eq]);

impl serde::Serialize for ErrorKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let name = match *self.0 {
            clap::error::ErrorKind::InvalidValue => "InvalidValue",
            clap::error::ErrorKind::UnknownArgument => "UnknownArgument",
            clap::error::ErrorKind::InvalidSubcommand => "InvalidSubcommand",
            clap::error::ErrorKind::NoEquals => "NoEquals",
            clap::error::ErrorKind::ValueValidation => "ValueValidation",
            clap::error::ErrorKind::TooManyValues => "TooManyValues",
            clap::error::ErrorKind::TooFewValues => "TooFewValues",
            clap::error::ErrorKind::WrongNumberOfValues => "WrongNumberOfValues",
            clap::error::ErrorKind::ArgumentConflict => "ArgumentConflict",
            clap::error::ErrorKind::MissingRequiredArgument => "MissingRequiredArgument",
            clap::error::ErrorKind::MissingSubcommand => "MissingSubcommand",
            clap::error::ErrorKind::InvalidUtf8 => "InvalidUtf8",
            clap::error::ErrorKind::DisplayHelp => "DisplayHelp",
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                "DisplayHelpOnMissingArgumentOrSubcommand"
            }
            clap::error::ErrorKind::DisplayVersion => "DisplayVersion",
            clap::error::ErrorKind::Io => "Io",
            clap::error::ErrorKind::Format => "Format",
            _ => "Unknown",
        };
        serializer.serialize_str(name)
    }
}

impl<'de> serde::Deserialize<'de> for ErrorKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "InvalidValue" => clap::error::ErrorKind::InvalidValue,
            "UnknownArgument" => clap::error::ErrorKind::UnknownArgument,
            "InvalidSubcommand" => clap::error::ErrorKind::InvalidSubcommand,
            "NoEquals" => clap::error::ErrorKind::NoEquals,
            "ValueValidation" => clap::error::ErrorKind::ValueValidation,
            "TooManyValues" => clap::error::ErrorKind::TooManyValues,
            "TooFewValues" => clap::error::ErrorKind::TooFewValues,
            "WrongNumberOfValues" => clap::error::ErrorKind::WrongNumberOfValues,
            "ArgumentConflict" => clap::error::ErrorKind::ArgumentConflict,
            "MissingRequiredArgument" => clap::error::ErrorKind::MissingRequiredArgument,
            "MissingSubcommand" => clap::error::ErrorKind::MissingSubcommand,
            "InvalidUtf8" => clap::error::ErrorKind::InvalidUtf8,
            "DisplayHelp" => clap::error::ErrorKind::DisplayHelp,
            "DisplayHelpOnMissingArgumentOrSubcommand" => {
                clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            }
            "DisplayVersion" => clap::error::ErrorKind::DisplayVersion,
            "Io" => clap::error::ErrorKind::Io,
            "Format" => clap::error::ErrorKind::Format,
            _ => {
                return Err(serde::de::Error::unknown_variant(
                    &s,
                    &[
                        "InvalidValue",
                        "UnknownArgument",
                        "InvalidSubcommand",
                        "NoEquals",
                        "ValueValidation",
                        "TooManyValues",
                        "TooFewValues",
                        "WrongNumberOfValues",
                        "ArgumentConflict",
                        "MissingRequiredArgument",
                        "MissingSubcommand",
                        "InvalidUtf8",
                        "DisplayHelp",
                        "DisplayHelpOnMissingArgumentOrSubcommand",
                        "DisplayVersion",
                        "Io",
                        "Format",
                    ],
                ));
            }
        };
        Ok(ErrorKind(std::sync::Arc::new(inner)))
    }
}

#[reflect_methods]
impl ErrorKind {
    /// Returns `true` if this error indicates the process should display help and exit.
    #[tracing::instrument(skip(self))]
    pub fn is_display(&self) -> bool {
        matches!(
            *self.0,
            clap::error::ErrorKind::DisplayHelp
                | clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
                | clap::error::ErrorKind::DisplayVersion
        )
    }

    /// Returns `true` if this error indicates invalid user input.
    #[tracing::instrument(skip(self))]
    pub fn is_user_error(&self) -> bool {
        matches!(
            *self.0,
            clap::error::ErrorKind::InvalidValue
                | clap::error::ErrorKind::UnknownArgument
                | clap::error::ErrorKind::InvalidSubcommand
                | clap::error::ErrorKind::MissingRequiredArgument
                | clap::error::ErrorKind::MissingSubcommand
                | clap::error::ErrorKind::ArgumentConflict
                | clap::error::ErrorKind::TooManyValues
                | clap::error::ErrorKind::TooFewValues
                | clap::error::ErrorKind::WrongNumberOfValues
        )
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            clap::error::ErrorKind::InvalidValue => "InvalidValue",
            clap::error::ErrorKind::UnknownArgument => "UnknownArgument",
            clap::error::ErrorKind::InvalidSubcommand => "InvalidSubcommand",
            clap::error::ErrorKind::NoEquals => "NoEquals",
            clap::error::ErrorKind::ValueValidation => "ValueValidation",
            clap::error::ErrorKind::TooManyValues => "TooManyValues",
            clap::error::ErrorKind::TooFewValues => "TooFewValues",
            clap::error::ErrorKind::WrongNumberOfValues => "WrongNumberOfValues",
            clap::error::ErrorKind::ArgumentConflict => "ArgumentConflict",
            clap::error::ErrorKind::MissingRequiredArgument => "MissingRequiredArgument",
            clap::error::ErrorKind::MissingSubcommand => "MissingSubcommand",
            clap::error::ErrorKind::InvalidUtf8 => "InvalidUtf8",
            clap::error::ErrorKind::DisplayHelp => "DisplayHelp",
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                "DisplayHelpOnMissingArgumentOrSubcommand"
            }
            clap::error::ErrorKind::DisplayVersion => "DisplayVersion",
            clap::error::ErrorKind::Io => "Io",
            clap::error::ErrorKind::Format => "Format",
            _ => "Unknown",
        }
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::ErrorKind;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ErrorKind {
        fn to_code_literal(&self) -> TokenStream {
            let variant = self.as_str();
            let variant_ident = quote::format_ident!("{}", variant);
            quote::quote! {
                ::elicit_clap::ErrorKind::from(::clap::error::ErrorKind::#variant_ident)
            }
        }
    }
}

impl elicitation::ElicitComplete for ErrorKind {}
