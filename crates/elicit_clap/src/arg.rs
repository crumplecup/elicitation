//! [`clap::Arg`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Arg, as Arg);
elicit_newtype_traits!(Arg, clap::Arg, []);

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
