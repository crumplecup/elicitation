//! [`clap::ValueHint`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ValueHint, as ValueHint);
elicit_newtype_traits!(ValueHint, clap::ValueHint, [eq]);

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
