//! [`clap::ColorChoice`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ColorChoice, as ColorChoice);
elicit_newtype_traits!(ColorChoice, clap::ColorChoice, [eq, display]);

#[reflect_methods]
impl ColorChoice {
    /// Returns `true` if this is [`ColorChoice::Auto`].
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Auto)
    }

    /// Returns `true` if this is [`ColorChoice::Always`].
    #[tracing::instrument(skip(self))]
    pub fn is_always(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Always)
    }

    /// Returns `true` if this is [`ColorChoice::Never`].
    #[tracing::instrument(skip(self))]
    pub fn is_never(&self) -> bool {
        matches!(*self.0, clap::ColorChoice::Never)
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            clap::ColorChoice::Auto => "Auto",
            clap::ColorChoice::Always => "Always",
            clap::ColorChoice::Never => "Never",
        }
    }
}
