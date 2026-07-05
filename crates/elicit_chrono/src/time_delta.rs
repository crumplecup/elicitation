//! `TimeDelta` — shadow for `chrono::TimeDelta` (the primary name for `chrono::Duration`).

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(chrono::TimeDelta, as TimeDelta, serde);
elicitation::elicit_newtype_traits!(TimeDelta, chrono::TimeDelta, [cmp]);

impl Default for TimeDelta {
    fn default() -> Self {
        chrono::TimeDelta::zero().into()
    }
}

impl std::ops::Add for TimeDelta {
    type Output = TimeDelta;
    fn add(self, rhs: TimeDelta) -> TimeDelta {
        (*self.0 + *rhs.0).into()
    }
}

impl std::ops::AddAssign for TimeDelta {
    fn add_assign(&mut self, rhs: TimeDelta) {
        self.0 = std::sync::Arc::new(*self.0 + *rhs.0);
    }
}

impl std::ops::Sub for TimeDelta {
    type Output = TimeDelta;
    fn sub(self, rhs: TimeDelta) -> TimeDelta {
        (*self.0 - *rhs.0).into()
    }
}

impl std::ops::SubAssign for TimeDelta {
    fn sub_assign(&mut self, rhs: TimeDelta) {
        self.0 = std::sync::Arc::new(*self.0 - *rhs.0);
    }
}

impl std::ops::Mul<i32> for TimeDelta {
    type Output = TimeDelta;
    fn mul(self, rhs: i32) -> TimeDelta {
        (*self.0 * rhs).into()
    }
}

impl std::ops::Div<i32> for TimeDelta {
    type Output = TimeDelta;
    fn div(self, rhs: i32) -> TimeDelta {
        (*self.0 / rhs).into()
    }
}

impl std::ops::Neg for TimeDelta {
    type Output = TimeDelta;
    fn neg(self) -> TimeDelta {
        (-*self.0).into()
    }
}

impl std::iter::Sum for TimeDelta {
    fn sum<I: Iterator<Item = TimeDelta>>(iter: I) -> TimeDelta {
        iter.fold(TimeDelta::zero(), |a, b| a + b)
    }
}

mod emit_impls {
    use super::TimeDelta;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TimeDelta {
        fn to_code_literal(&self) -> TokenStream {
            let secs = self.0.num_seconds();
            quote::quote! {
                match ::chrono::TimeDelta::try_seconds(#secs) {
                    Some(d) => ::elicit_chrono::TimeDelta::from(d),
                    None => return Err("try_seconds: seconds value out of TimeDelta range".into()),
                }
            }
        }

        fn type_tokens() -> TokenStream {
            quote::quote! { ::elicit_chrono::TimeDelta }
        }
    }
}

impl elicitation::ElicitComplete for TimeDelta {}

// ── Static constructors ───────────────────────────────────────────────────────

impl TimeDelta {
    /// The zero duration.
    #[instrument]
    pub fn zero() -> TimeDelta {
        chrono::TimeDelta::zero().into()
    }

    /// Constructs a `TimeDelta` from seconds and nanoseconds.
    #[instrument]
    pub fn new(secs: i64, nanos: u32) -> Option<TimeDelta> {
        chrono::TimeDelta::new(secs, nanos).map(Into::into)
    }

    /// Constructs from whole weeks. Returns `None` on overflow.
    #[instrument]
    pub fn try_weeks(weeks: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_weeks(weeks).map(Into::into)
    }

    /// Constructs from whole days. Returns `None` on overflow.
    #[instrument]
    pub fn try_days(days: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_days(days).map(Into::into)
    }

    /// Constructs from whole hours. Returns `None` on overflow.
    #[instrument]
    pub fn try_hours(hours: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_hours(hours).map(Into::into)
    }

    /// Constructs from whole minutes. Returns `None` on overflow.
    #[instrument]
    pub fn try_minutes(mins: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_minutes(mins).map(Into::into)
    }

    /// Constructs from whole seconds. Returns `None` on overflow.
    #[instrument]
    pub fn try_seconds(secs: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_seconds(secs).map(Into::into)
    }

    /// Constructs from whole milliseconds. Returns `None` on overflow.
    #[instrument]
    pub fn try_milliseconds(millis: i64) -> Option<TimeDelta> {
        chrono::TimeDelta::try_milliseconds(millis).map(Into::into)
    }

    /// Constructs from whole microseconds.
    #[instrument]
    pub fn microseconds(micros: i64) -> TimeDelta {
        chrono::TimeDelta::microseconds(micros).into()
    }

    /// Constructs from whole nanoseconds.
    #[instrument]
    pub fn nanoseconds(nanos: i64) -> TimeDelta {
        chrono::TimeDelta::nanoseconds(nanos).into()
    }

    /// The maximum representable `TimeDelta`.
    #[instrument]
    pub fn max_value() -> TimeDelta {
        chrono::TimeDelta::MAX.into()
    }

    /// The minimum representable `TimeDelta`.
    #[instrument]
    pub fn min_value() -> TimeDelta {
        chrono::TimeDelta::MIN.into()
    }

    /// Converts a `std::time::Duration` (whole seconds as `u64`) to `TimeDelta`.
    #[instrument]
    pub fn from_std(d: u64) -> Option<TimeDelta> {
        let std_d = std::time::Duration::from_secs(d);
        chrono::TimeDelta::from_std(std_d).ok().map(Into::into)
    }

    // ── Deprecated constructors (kept for method-name coverage) ──────────────

    /// Constructs from whole weeks (deprecated; use `try_weeks`).
    #[instrument]
    pub fn weeks(weeks: i64) -> Option<TimeDelta> {
        TimeDelta::try_weeks(weeks)
    }

    /// Constructs from whole days (deprecated; use `try_days`).
    #[instrument]
    pub fn days(days: i64) -> Option<TimeDelta> {
        TimeDelta::try_days(days)
    }

    /// Constructs from whole hours (deprecated; use `try_hours`).
    #[instrument]
    pub fn hours(hours: i64) -> Option<TimeDelta> {
        TimeDelta::try_hours(hours)
    }

    /// Constructs from whole minutes (deprecated; use `try_minutes`).
    #[instrument]
    pub fn minutes(mins: i64) -> Option<TimeDelta> {
        TimeDelta::try_minutes(mins)
    }

    /// Constructs from whole seconds (deprecated; use `try_seconds`).
    #[instrument]
    pub fn seconds(secs: i64) -> Option<TimeDelta> {
        TimeDelta::try_seconds(secs)
    }

    /// Constructs from whole milliseconds (deprecated; use `try_milliseconds`).
    #[instrument]
    pub fn milliseconds(millis: i64) -> Option<TimeDelta> {
        TimeDelta::try_milliseconds(millis)
    }
}

// ── Instance methods via reflect_methods ──────────────────────────────────────

#[reflect_methods]
impl TimeDelta {
    /// Returns `true` if this duration is zero.
    #[instrument(skip(self))]
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Returns the absolute value of this duration.
    #[instrument(skip(self))]
    pub fn abs(&self) -> TimeDelta {
        self.0.abs().into()
    }

    /// Returns `self + rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_add(&self, rhs: TimeDelta) -> Option<TimeDelta> {
        self.0.checked_add(&rhs.0).map(Into::into)
    }

    /// Returns `self - rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_sub(&self, rhs: TimeDelta) -> Option<TimeDelta> {
        self.0.checked_sub(&rhs.0).map(Into::into)
    }

    /// Returns `self * rhs`, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn checked_mul(&self, rhs: i32) -> Option<TimeDelta> {
        self.0.checked_mul(rhs).map(Into::into)
    }

    /// Returns `self / rhs`, or `None` on overflow or zero divisor.
    #[instrument(skip(self))]
    pub fn checked_div(&self, rhs: i32) -> Option<TimeDelta> {
        self.0.checked_div(rhs).map(Into::into)
    }

    /// Returns the number of whole weeks in this duration.
    #[instrument(skip(self))]
    pub fn num_weeks(&self) -> i64 {
        self.0.num_weeks()
    }

    /// Returns the number of whole days in this duration.
    #[instrument(skip(self))]
    pub fn num_days(&self) -> i64 {
        self.0.num_days()
    }

    /// Returns the number of whole hours in this duration.
    #[instrument(skip(self))]
    pub fn num_hours(&self) -> i64 {
        self.0.num_hours()
    }

    /// Returns the number of whole minutes in this duration.
    #[instrument(skip(self))]
    pub fn num_minutes(&self) -> i64 {
        self.0.num_minutes()
    }

    /// Returns the number of whole seconds in this duration.
    #[instrument(skip(self))]
    pub fn num_seconds(&self) -> i64 {
        self.0.num_seconds()
    }

    /// Returns the number of whole milliseconds in this duration.
    #[instrument(skip(self))]
    pub fn num_milliseconds(&self) -> i64 {
        self.0.num_milliseconds()
    }

    /// Returns the number of whole microseconds, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn num_microseconds(&self) -> Option<i64> {
        self.0.num_microseconds()
    }

    /// Returns the number of whole nanoseconds, or `None` on overflow.
    #[instrument(skip(self))]
    pub fn num_nanoseconds(&self) -> Option<i64> {
        self.0.num_nanoseconds()
    }

    /// Returns the sub-second nanosecond component (0–999_999_999).
    #[instrument(skip(self))]
    pub fn subsec_nanos(&self) -> i32 {
        self.0.subsec_nanos()
    }

    /// Returns the sub-second millisecond component (0–999).
    #[instrument(skip(self))]
    pub fn subsec_millis(&self) -> i32 {
        self.0.subsec_nanos() / 1_000_000
    }

    /// Returns the sub-second microsecond component (0–999_999).
    #[instrument(skip(self))]
    pub fn subsec_micros(&self) -> i32 {
        self.0.subsec_nanos() / 1_000
    }

    /// Returns the duration as `f32` seconds.
    #[instrument(skip(self))]
    pub fn as_seconds_f32(&self) -> f32 {
        self.0.num_seconds() as f32 + self.0.subsec_nanos() as f32 / 1_000_000_000.0
    }

    /// Returns the duration as `f64` seconds.
    #[instrument(skip(self))]
    pub fn as_seconds_f64(&self) -> f64 {
        self.0.num_seconds() as f64 + self.0.subsec_nanos() as f64 / 1_000_000_000.0
    }

    /// Converts this `TimeDelta` to a `std::time::Duration` (whole seconds only).
    #[instrument(skip(self))]
    pub fn to_std(&self) -> Option<u64> {
        if self.0.num_seconds() < 0 {
            return None;
        }
        Some(self.0.num_seconds() as u64)
    }
}
