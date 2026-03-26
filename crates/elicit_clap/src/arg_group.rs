//! [`clap::ArgGroup`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ArgGroup, as ArgGroup);
elicit_newtype_traits!(ArgGroup, clap::ArgGroup, [eq]);

#[reflect_methods]
impl ArgGroup {
    /// Returns the group's identifier as a string.
    #[tracing::instrument(skip(self))]
    pub fn get_id(&self) -> String {
        self.0.get_id().to_string()
    }

    /// Returns `true` if at least one argument in this group is required.
    #[tracing::instrument(skip(self))]
    pub fn is_required(&self) -> bool {
        self.0.is_required_set()
    }

    /// Returns the member argument IDs as strings.
    #[tracing::instrument(skip(self))]
    pub fn get_args(&self) -> Vec<String> {
        self.0.get_args().map(|id| id.to_string()).collect()
    }
}
