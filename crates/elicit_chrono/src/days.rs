//! `Days` — shadow for `chrono::Days`.

use elicitation::elicit_newtype;
use tracing::instrument;

elicit_newtype!(elicitation::DaysWrap, as Days, serde);
elicitation::elicit_newtype_traits!(Days, elicitation::DaysWrap, [cmp]);

impl Days {
    /// Creates a new [`Days`] value from a `u64` day count.
    #[instrument]
    pub fn new(n: u64) -> Self {
        elicitation::DaysWrap::from(chrono::Days::new(n)).into()
    }
}
