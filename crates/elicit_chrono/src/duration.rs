//! `Duration` — shadow for `chrono::Duration` (= `chrono::TimeDelta`).

use std::sync::Arc;

use elicitation_derive::reflect_methods;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::instrument;

/// Shadow for [`chrono::Duration`] (`chrono::TimeDelta`).
///
/// Serializes and deserializes as a whole-second integer (matching the core
/// `chrono::TimeDelta` elicitor), making it natural to pass between MCP tools.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration(pub Arc<chrono::TimeDelta>);

impl Serialize for Duration {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(self.0.num_seconds())
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let secs = i64::deserialize(d)?;
        chrono::TimeDelta::try_seconds(secs)
            .ok_or_else(|| {
                serde::de::Error::custom(format!("seconds {secs} out of TimeDelta range"))
            })
            .map(Into::into)
    }
}

impl JsonSchema for Duration {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Duration".into()
    }
    fn json_schema(schema_gen: &mut SchemaGenerator) -> Schema {
        schema_gen.subschema_for::<i64>()
    }
}

impl std::ops::Deref for Duration {
    type Target = chrono::TimeDelta;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<chrono::TimeDelta> for Duration {
    fn as_ref(&self) -> &chrono::TimeDelta {
        &self.0
    }
}

impl From<chrono::TimeDelta> for Duration {
    fn from(inner: chrono::TimeDelta) -> Self {
        Self(Arc::new(inner))
    }
}

impl From<Arc<chrono::TimeDelta>> for Duration {
    fn from(arc: Arc<chrono::TimeDelta>) -> Self {
        Self(arc)
    }
}

#[reflect_methods]
impl Duration {
    /// Returns the number of whole weeks.
    #[instrument(skip(self))]
    pub fn num_weeks(&self) -> i64 {
        self.0.num_weeks()
    }

    /// Returns the number of whole days.
    #[instrument(skip(self))]
    pub fn num_days(&self) -> i64 {
        self.0.num_days()
    }

    /// Returns the number of whole hours.
    #[instrument(skip(self))]
    pub fn num_hours(&self) -> i64 {
        self.0.num_hours()
    }

    /// Returns the number of whole minutes.
    #[instrument(skip(self))]
    pub fn num_minutes(&self) -> i64 {
        self.0.num_minutes()
    }

    /// Returns the number of whole seconds.
    #[instrument(skip(self))]
    pub fn num_seconds(&self) -> i64 {
        self.0.num_seconds()
    }

    /// Returns the number of whole milliseconds.
    #[instrument(skip(self))]
    pub fn num_milliseconds(&self) -> i64 {
        self.0.num_milliseconds()
    }

    /// Returns the number of whole microseconds, or `None` if the value overflows `i64`.
    #[instrument(skip(self))]
    pub fn num_microseconds(&self) -> Option<i64> {
        self.0.num_microseconds()
    }

    /// Returns the number of whole nanoseconds, or `None` if the value overflows `i64`.
    #[instrument(skip(self))]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        self.0.num_nanoseconds()
    }

    /// Returns the nanosecond component of the fractional part (0–999_999_999).
    #[instrument(skip(self))]
    pub fn subsec_nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    /// Returns `true` if the duration is zero.
    #[instrument(skip(self))]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns the absolute value of this duration.
    #[instrument(skip(self))]
    pub fn abs(&self) -> Duration {
        (*self.0).abs().into()
    }

    /// Returns `self + rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add(&self, rhs: Duration) -> Option<Duration> {
        self.0.checked_add(&rhs.0).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub(&self, rhs: Duration) -> Option<Duration> {
        self.0.checked_sub(&rhs.0).map(Into::into)
    }

    /// Returns `self * rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_mul(&self, rhs: i32) -> Option<Duration> {
        self.0.checked_mul(rhs).map(Into::into)
    }

    /// Returns `self / rhs`, or `None` on overflow or division by zero.
    #[instrument(skip(self))]
    pub fn checked_div(&self, rhs: i32) -> Option<Duration> {
        self.0.checked_div(rhs).map(Into::into)
    }
}

impl Duration {
    /// The zero duration.
    pub fn zero() -> Duration {
        chrono::TimeDelta::zero().into()
    }

    /// Construct from a number of whole weeks. Returns `None` on overflow.
    pub fn try_weeks(weeks: i64) -> Option<Duration> {
        chrono::TimeDelta::try_weeks(weeks).map(Into::into)
    }

    /// Construct from a number of whole days. Returns `None` on overflow.
    pub fn try_days(days: i64) -> Option<Duration> {
        chrono::TimeDelta::try_days(days).map(Into::into)
    }

    /// Construct from a number of whole hours. Returns `None` on overflow.
    pub fn try_hours(hours: i64) -> Option<Duration> {
        chrono::TimeDelta::try_hours(hours).map(Into::into)
    }

    /// Construct from a number of whole minutes. Returns `None` on overflow.
    pub fn try_minutes(minutes: i64) -> Option<Duration> {
        chrono::TimeDelta::try_minutes(minutes).map(Into::into)
    }

    /// Construct from a number of whole seconds. Returns `None` on overflow.
    pub fn try_seconds(secs: i64) -> Option<Duration> {
        chrono::TimeDelta::try_seconds(secs).map(Into::into)
    }

    /// Construct from a number of milliseconds. Returns `None` on overflow.
    pub fn try_milliseconds(millis: i64) -> Option<Duration> {
        chrono::TimeDelta::try_milliseconds(millis).map(Into::into)
    }

    /// Construct from a number of microseconds. Panics on overflow.
    pub fn microseconds(micros: i64) -> Duration {
        chrono::TimeDelta::microseconds(micros).into()
    }

    /// Construct from a number of nanoseconds. Panics on overflow.
    pub fn nanoseconds(nanos: i64) -> Duration {
        chrono::TimeDelta::nanoseconds(nanos).into()
    }

    /// The maximum representable duration.
    pub fn max_value() -> Duration {
        chrono::TimeDelta::MAX.into()
    }

    /// The minimum representable duration.
    pub fn min_value() -> Duration {
        chrono::TimeDelta::MIN.into()
    }
}

// ── Elicitation framework traits — delegate to `chrono::TimeDelta` core impls ─

impl elicitation::Prompt for Duration {
    fn prompt() -> Option<&'static str> {
        <chrono::TimeDelta as elicitation::Prompt>::prompt()
    }
}

impl elicitation::Elicitation for Duration {
    type Style = <chrono::TimeDelta as elicitation::Elicitation>::Style;

    async fn elicit<C: elicitation::ElicitCommunicator>(
        communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        chrono::TimeDelta::elicit(communicator)
            .await
            .map(Into::into)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <chrono::TimeDelta as elicitation::Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <chrono::TimeDelta as elicitation::Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <chrono::TimeDelta as elicitation::Elicitation>::creusot_proof()
    }
}

impl elicitation::ElicitIntrospect for Duration {
    fn pattern() -> elicitation::ElicitationPattern {
        <chrono::TimeDelta as elicitation::ElicitIntrospect>::pattern()
    }
    fn metadata() -> elicitation::TypeMetadata {
        <chrono::TimeDelta as elicitation::ElicitIntrospect>::metadata()
    }
}

impl elicitation::ElicitPromptTree for Duration {
    fn prompt_tree() -> elicitation::PromptTree {
        <chrono::TimeDelta as elicitation::ElicitPromptTree>::prompt_tree()
    }
}

impl elicitation::ElicitSpec for Duration {
    fn type_spec() -> elicitation::TypeSpec {
        <chrono::TimeDelta as elicitation::ElicitSpec>::type_spec()
    }
}

mod emit_impls {
    use super::Duration;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Duration {
        fn to_code_literal(&self) -> TokenStream {
            let secs = self.0.num_seconds();
            quote::quote! {
                ::elicit_chrono::Duration::try_seconds(#secs)
                    .expect("valid Duration seconds")
            }
        }
    }
}

impl elicitation::ElicitComplete for Duration {}
