//! [`clap::builder::ValueRange`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::builder::ValueRange, as ValueRange);
elicit_newtype_traits!(ValueRange, clap::builder::ValueRange, [eq]);

#[reflect_methods]
impl ValueRange {
    /// Returns the minimum number of values.
    #[tracing::instrument(skip(self))]
    pub fn min_values(&self) -> usize {
        self.0.min_values()
    }

    /// Returns the maximum number of values.
    #[tracing::instrument(skip(self))]
    pub fn max_values(&self) -> usize {
        self.0.max_values()
    }

    /// Returns `true` if this range accepts any values at all.
    #[tracing::instrument(skip(self))]
    pub fn takes_values(&self) -> bool {
        self.0.takes_values()
    }

    /// Returns `true` if this is exactly one value (the common case).
    #[tracing::instrument(skip(self))]
    pub fn is_exactly_one(&self) -> bool {
        self.0.min_values() == 1 && self.0.max_values() == 1
    }
}
