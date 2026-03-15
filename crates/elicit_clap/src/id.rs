//! [`clap::Id`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Id, as Id);
elicit_newtype_traits!(Id, clap::Id, [cmp, display]);

#[reflect_methods]
impl Id {
    /// Returns the identifier as a string slice.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
