//! [`clap::parser::ValueSource`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::parser::ValueSource, as ValueSource);
elicit_newtype_traits!(ValueSource, clap::parser::ValueSource, [ord]);

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
