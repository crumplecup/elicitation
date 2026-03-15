//! [`clap::builder::PossibleValue`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::builder::PossibleValue, as PossibleValue);
elicit_newtype_traits!(PossibleValue, clap::builder::PossibleValue, [eq]);

#[reflect_methods]
impl PossibleValue {
    /// Returns the name of this possible value.
    #[tracing::instrument(skip(self))]
    pub fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    /// Returns `true` if this value is hidden from help output.
    #[tracing::instrument(skip(self))]
    pub fn is_hidden(&self) -> bool {
        self.0.is_hide_set()
    }

    /// Returns `true` if the given string matches this value (case-sensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches(&self, value: String) -> bool {
        self.0.matches(&value, false)
    }

    /// Returns `true` if the given string matches this value (case-insensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches_ignore_case(&self, value: String) -> bool {
        self.0.matches(&value, true)
    }
}
