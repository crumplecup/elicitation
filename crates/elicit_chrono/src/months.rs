//! `Months` — shadow for `chrono::Months`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(elicitation::MonthsWrap, as Months, serde);
elicitation::elicit_newtype_traits!(Months, elicitation::MonthsWrap, [cmp]);

impl Months {
    /// Creates a new [`Months`] value from a `u32` count.
    #[instrument]
    pub fn new(num: u32) -> Self {
        elicitation::MonthsWrap::from(chrono::Months::new(num)).into()
    }
}

#[reflect_methods]
impl Months {
    /// Returns the month count as a `u32`.
    #[instrument(skip(self))]
    pub fn as_u32(&self) -> u32 {
        (*self.0).into_inner().as_u32()
    }
}
