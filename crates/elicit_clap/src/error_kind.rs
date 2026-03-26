//! [`clap::error::ErrorKind`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::error::ErrorKind, as ErrorKind);
elicit_newtype_traits!(ErrorKind, clap::error::ErrorKind, [eq]);

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
