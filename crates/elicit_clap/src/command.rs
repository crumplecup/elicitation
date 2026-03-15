//! [`clap::Command`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Command, as Command);
elicit_newtype_traits!(Command, clap::Command, []);

#[reflect_methods]
impl Command {
    /// Returns the command's name.
    #[tracing::instrument(skip(self))]
    pub fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    /// Returns the short about string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_about(&self) -> Option<String> {
        self.0.get_about().map(|s| s.to_string())
    }

    /// Returns the version string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_version(&self) -> Option<String> {
        self.0.get_version().map(str::to_string)
    }

    /// Returns the author string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_author(&self) -> Option<String> {
        self.0.get_author().map(str::to_string)
    }

    /// Returns the display name (used in help), if set.
    #[tracing::instrument(skip(self))]
    pub fn get_display_name(&self) -> Option<String> {
        self.0.get_display_name().map(str::to_string)
    }

    /// Returns the binary name, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_bin_name(&self) -> Option<String> {
        self.0.get_bin_name().map(str::to_string)
    }

    /// Returns the display order for this command.
    #[tracing::instrument(skip(self))]
    pub fn get_display_order(&self) -> usize {
        self.0.get_display_order()
    }
}
