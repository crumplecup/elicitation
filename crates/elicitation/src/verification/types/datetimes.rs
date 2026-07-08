//! DateTime contract types for chrono, time, and jiff libraries.
//!
//! Available with the `chrono`, `time`, or `jiff` features.

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use super::ValidationError;
#[cfg(all(not(kani), any(feature = "chrono", feature = "time")))]
use anodized::spec;

#[cfg(all(not(kani), any(feature = "chrono", feature = "jiff", feature = "time")))]
use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

#[cfg(all(not(kani), any(feature = "chrono", feature = "jiff", feature = "time")))]
use elicitation_derive::instrumented_impl;

#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};

// DateTimeUtcAfter - DateTime<Utc> after a threshold
/// A `DateTime<Utc>` that is guaranteed to be after a threshold time.
///
/// Available with the `chrono` feature.
#[cfg(feature = "chrono")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[cfg(not(kani))]
pub struct DateTimeUtcAfter {
    value: DateTime<Utc>,
    threshold: DateTime<Utc>,
}

#[cfg(all(feature = "chrono", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeUtcAfter(std::marker::PhantomData<()>);

#[cfg(feature = "chrono")]
#[cfg_attr(not(kani), instrumented_impl)]
impl DateTimeUtcAfter {
    /// Create a new DateTimeUtcAfter, validating value > threshold.
    #[cfg(not(kani))]
    #[spec(requires: [value > threshold, value < threshold])]
    pub fn new(value: DateTime<Utc>, threshold: DateTime<Utc>) -> Result<Self, ValidationError> {
        if value > threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust chrono, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: DateTime<Utc>, _threshold: DateTime<Utc>) -> Result<Self, ValidationError> {
        let is_after: bool = kani::any();
        if is_after {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the datetime value.
    #[cfg(not(kani))]
    pub fn get(&self) -> DateTime<Utc> {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> DateTime<Utc> {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> DateTime<Utc> {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> DateTime<Utc> {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner `DateTime<Utc>`.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> DateTime<Utc> {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for DateTimeUtcAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (UTC):")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for DateTimeUtcAfter {
    type Style = <DateTime<Utc> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTimeUtcAfter");
        // Default threshold to Unix epoch
        let threshold = DateTime::UNIX_EPOCH;
        loop {
            let dt = DateTime::<Utc>::elicit(communicator).await?;
            match Self::new(dt, threshold) {
                Ok(valid) => {
                    tracing::debug!(datetime = %valid.value, "Valid datetime after threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "DateTime not after threshold, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// DateTimeUtcBefore - DateTime<Utc> before a threshold
/// A `DateTime<Utc>` that is guaranteed to be before a threshold time.
///
/// Available with the `chrono` feature.
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct DateTimeUtcBefore {
    value: DateTime<Utc>,
    threshold: DateTime<Utc>,
}

#[cfg(all(feature = "chrono", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeUtcBefore(std::marker::PhantomData<()>);

#[cfg(feature = "chrono")]
#[cfg_attr(not(kani), instrumented_impl)]
impl DateTimeUtcBefore {
    /// Create a new DateTimeUtcBefore, validating value < threshold.
    #[cfg(not(kani))]
    pub fn new(value: DateTime<Utc>, threshold: DateTime<Utc>) -> Result<Self, ValidationError> {
        if value < threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust chrono, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: DateTime<Utc>, _threshold: DateTime<Utc>) -> Result<Self, ValidationError> {
        let is_before: bool = kani::any();
        if is_before {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the datetime value.
    #[cfg(not(kani))]
    pub fn get(&self) -> DateTime<Utc> {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> DateTime<Utc> {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> DateTime<Utc> {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> DateTime<Utc> {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner `DateTime<Utc>`.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> DateTime<Utc> {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for DateTimeUtcBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (UTC):")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for DateTimeUtcBefore {
    type Style = <DateTime<Utc> as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTimeUtcBefore");
        // Default threshold to now
        let threshold = Utc::now();
        loop {
            let dt = DateTime::<Utc>::elicit(communicator).await?;
            match Self::new(dt, threshold) {
                Ok(valid) => {
                    tracing::debug!(datetime = %valid.value, "Valid datetime before threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "DateTime not before threshold, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// NaiveDateTimeAfter - NaiveDateTime after a threshold
/// A NaiveDateTime that is guaranteed to be after a threshold time.
///
/// Available with the `chrono` feature.
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct NaiveDateTimeAfter {
    value: NaiveDateTime,
    threshold: NaiveDateTime,
}

#[cfg(all(feature = "chrono", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaiveDateTimeAfter(std::marker::PhantomData<()>);

#[cfg(feature = "chrono")]
#[cfg_attr(not(kani), instrumented_impl)]
impl NaiveDateTimeAfter {
    /// Create a new NaiveDateTimeAfter, validating value > threshold.
    #[cfg(not(kani))]
    #[spec(requires: [value > threshold])]
    pub fn new(value: NaiveDateTime, threshold: NaiveDateTime) -> Result<Self, ValidationError> {
        if value > threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust chrono, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: NaiveDateTime, _threshold: NaiveDateTime) -> Result<Self, ValidationError> {
        let is_after: bool = kani::any();
        if is_after {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the datetime value.
    #[cfg(not(kani))]
    pub fn get(&self) -> NaiveDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> NaiveDateTime {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> NaiveDateTime {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> NaiveDateTime {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner NaiveDateTime.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> NaiveDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> NaiveDateTime {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for NaiveDateTimeAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (no timezone):")
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for NaiveDateTimeAfter {
    type Style = <NaiveDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateTimeAfter");
        // Default threshold to Unix epoch
        let threshold = DateTime::<Utc>::UNIX_EPOCH.naive_utc();
        loop {
            let dt = NaiveDateTime::elicit(communicator).await?;
            match Self::new(dt, threshold) {
                Ok(valid) => {
                    tracing::debug!(datetime = %valid.value, "Valid naive datetime after threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "NaiveDateTime not after threshold, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

#[cfg(all(test, feature = "chrono"))]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_datetime_utc_after_valid() {
        let threshold = Utc::now();
        let value = threshold + Duration::hours(1);
        let result = DateTimeUtcAfter::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_datetime_utc_after_too_early() {
        let threshold = Utc::now();
        let value = threshold - Duration::hours(1);
        let result = DateTimeUtcAfter::new(value, threshold);
        assert!(result.is_err());
    }

    #[test]
    fn test_datetime_utc_after_get() {
        let threshold = Utc::now();
        let value = threshold + Duration::hours(1);
        let after = DateTimeUtcAfter::new(value, threshold).unwrap();
        assert_eq!(after.get(), value);
        assert_eq!(after.threshold(), threshold);
    }

    #[test]
    fn test_datetime_utc_before_valid() {
        let threshold = Utc::now();
        let value = threshold - Duration::hours(1);
        let result = DateTimeUtcBefore::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_datetime_utc_before_too_late() {
        let threshold = Utc::now();
        let value = threshold + Duration::hours(1);
        let result = DateTimeUtcBefore::new(value, threshold);
        assert!(result.is_err());
    }

    #[test]
    fn test_naive_datetime_after_valid() {
        let threshold = Utc::now().naive_utc();
        let value = threshold + Duration::hours(1);
        let result = NaiveDateTimeAfter::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_naive_datetime_after_too_early() {
        let threshold = Utc::now().naive_utc();
        let value = threshold - Duration::hours(1);
        let result = NaiveDateTimeAfter::new(value, threshold);
        assert!(result.is_err());
    }
}

// ============================================================================
// Jiff DateTime Contract Types
// ============================================================================

#[cfg(feature = "jiff")]
use jiff::Timestamp;

// TimestampAfter - Timestamp after a threshold
/// A Timestamp that is guaranteed to be after a threshold time.
///
/// Available with the `jiff` feature.
#[cfg(feature = "jiff")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct TimestampAfter {
    value: Timestamp,
    threshold: Timestamp,
}

#[cfg(all(feature = "jiff", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimestampAfter(std::marker::PhantomData<()>);

#[cfg(feature = "jiff")]
#[cfg_attr(not(kani), instrumented_impl)]
impl TimestampAfter {
    /// Create a new TimestampAfter, validating value > threshold.
    #[cfg(not(kani))]
    pub fn new(value: Timestamp, threshold: Timestamp) -> Result<Self, ValidationError> {
        if value > threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust jiff, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: Timestamp, _threshold: Timestamp) -> Result<Self, ValidationError> {
        let is_after: bool = kani::any();
        if is_after {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the timestamp value.
    #[cfg(not(kani))]
    pub fn get(&self) -> Timestamp {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> Timestamp {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> Timestamp {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> Timestamp {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner Timestamp.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> Timestamp {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> Timestamp {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "jiff")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for TimestampAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a timestamp:")
    }
}

#[cfg(feature = "jiff")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for TimestampAfter {
    type Style = <Timestamp as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimestampAfter");
        // Default threshold to Unix epoch
        let threshold = Timestamp::UNIX_EPOCH;
        loop {
            let ts = Timestamp::elicit(communicator).await?;
            match Self::new(ts, threshold) {
                Ok(valid) => {
                    tracing::debug!(timestamp = %valid.value, "Valid timestamp after threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Timestamp not after threshold, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

// TimestampBefore - Timestamp before a threshold
/// A Timestamp that is guaranteed to be before a threshold time.
///
/// Available with the `jiff` feature.
#[cfg(feature = "jiff")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct TimestampBefore {
    value: Timestamp,
    threshold: Timestamp,
}

#[cfg(all(feature = "jiff", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimestampBefore(std::marker::PhantomData<()>);

#[cfg(feature = "jiff")]
#[cfg_attr(not(kani), instrumented_impl)]
impl TimestampBefore {
    /// Create a new TimestampBefore, validating value < threshold.
    #[cfg(not(kani))]
    pub fn new(value: Timestamp, threshold: Timestamp) -> Result<Self, ValidationError> {
        if value < threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust jiff, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: Timestamp, _threshold: Timestamp) -> Result<Self, ValidationError> {
        let is_before: bool = kani::any();
        if is_before {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the timestamp value.
    #[cfg(not(kani))]
    pub fn get(&self) -> Timestamp {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> Timestamp {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> Timestamp {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> Timestamp {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner Timestamp.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> Timestamp {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> Timestamp {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "jiff")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for TimestampBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a timestamp:")
    }
}

#[cfg(feature = "jiff")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for TimestampBefore {
    type Style = <Timestamp as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimestampBefore");
        // Default threshold to now
        let threshold = Timestamp::now();
        loop {
            let ts = Timestamp::elicit(communicator).await?;
            match Self::new(ts, threshold) {
                Ok(valid) => {
                    tracing::debug!(timestamp = %valid.value, "Valid timestamp before threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Timestamp not before threshold, re-prompting");
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

#[cfg(all(test, feature = "jiff"))]
mod jiff_tests {
    use super::*;
    use jiff::ToSpan;

    #[test]
    fn test_timestamp_after_valid() {
        let threshold = Timestamp::now();
        let value = threshold + 1.hour();
        let result = TimestampAfter::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_timestamp_after_too_early() {
        let threshold = Timestamp::now();
        let value = threshold - 1.hour();
        let result = TimestampAfter::new(value, threshold);
        assert!(result.is_err());
    }

    #[test]
    fn test_timestamp_before_valid() {
        let threshold = Timestamp::now();
        let value = threshold - 1.hour();
        let result = TimestampBefore::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_timestamp_before_too_late() {
        let threshold = Timestamp::now();
        let value = threshold + 1.hour();
        let result = TimestampBefore::new(value, threshold);
        assert!(result.is_err());
    }
}

// ============================================================================
// Time DateTime Contract Types
// ============================================================================

#[cfg(feature = "time")]
use time::{OffsetDateTime, PrimitiveDateTime};

// ── JsonSchema helper for time::OffsetDateTime fields ─────────────────────────
//
// schemars has no built-in time support and the orphan rule prevents implementing
// JsonSchema directly on time::OffsetDateTime (both the trait and the type are
// foreign).  This thin wrapper is a local type, so we can implement JsonSchema
// on it once here.  Constraint types below reference it via
// #[schemars(with = "OffsetDateTimeWrap")], keeping the schema definition in one
// place.

/// Trenchcoat for `time::OffsetDateTime`.
///
/// Provides `JsonSchema`, `Serialize`/`Deserialize`, and the full elicitation
/// trait suite for `time::OffsetDateTime`.  Use `#[schemars(with =
/// "OffsetDateTimeWrap")]` on `OffsetDateTime` fields in other structs to
/// delegate their JSON schema to this single definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct OffsetDateTimeWrap(pub OffsetDateTime);

#[cfg(all(feature = "time", not(kani)))]
impl From<OffsetDateTime> for OffsetDateTimeWrap {
    fn from(inner: OffsetDateTime) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl OffsetDateTimeWrap {
    /// Extract the inner [`time::OffsetDateTime`].
    pub fn into_inner(self) -> OffsetDateTime {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for OffsetDateTimeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "OffsetDateTime".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::json!({
            "type": "string",
            "format": "date-time"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Prompt for OffsetDateTimeWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime with timezone offset:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Elicitation for OffsetDateTimeWrap {
    type Style = <OffsetDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTimeWrap");
        let dt = OffsetDateTime::elicit(communicator).await?;
        Ok(Self(dt))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("OffsetDateTimeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("OffsetDateTimeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("OffsetDateTimeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for OffsetDateTimeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "OffsetDateTimeWrap",
            description: <OffsetDateTimeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for OffsetDateTimeWrap {}

// ── PrimitiveDateTimeWrap — trenchcoat for time::PrimitiveDateTime ────────────

/// Trenchcoat for `time::PrimitiveDateTime`.
///
/// Provides `JsonSchema`, `Serialize`/`Deserialize`, and the full elicitation
/// trait suite for `time::PrimitiveDateTime`.  Use `#[schemars(with =
/// "PrimitiveDateTimeWrap")]` on `PrimitiveDateTime` fields in other structs to
/// delegate their JSON schema to this single definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PrimitiveDateTimeWrap(pub PrimitiveDateTime);

#[cfg(all(feature = "time", not(kani)))]
impl From<PrimitiveDateTime> for PrimitiveDateTimeWrap {
    fn from(inner: PrimitiveDateTime) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl PrimitiveDateTimeWrap {
    /// Extract the inner [`time::PrimitiveDateTime`].
    pub fn into_inner(self) -> PrimitiveDateTime {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for PrimitiveDateTimeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PrimitiveDateTime".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map: serde_json::Map<String, serde_json::Value> = serde_json::json!({
            "type": "string",
            "format": "partial-date-time"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Prompt for PrimitiveDateTimeWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter datetime without timezone:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
#[cfg_attr(not(kani), elicitation_derive::instrumented_impl)]
impl Elicitation for PrimitiveDateTimeWrap {
    type Style = <PrimitiveDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PrimitiveDateTimeWrap");
        let dt = PrimitiveDateTime::elicit(communicator).await?;
        Ok(Self(dt))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("PrimitiveDateTimeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("PrimitiveDateTimeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("PrimitiveDateTimeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for PrimitiveDateTimeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "PrimitiveDateTimeWrap",
            description: <PrimitiveDateTimeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for PrimitiveDateTimeWrap {}

// ── TimeWrap — trenchcoat for time::Time ──────────────────────────────────────
/// Trenchcoat wrapper for [`time::Time`] that satisfies `schemars::JsonSchema`.
///
/// Use `#[schemars(with = "TimeWrap")]` on `time::Time` fields in other structs
/// to delegate to this schema definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TimeWrap(pub time::Time);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::Time> for TimeWrap {
    fn from(inner: time::Time) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl TimeWrap {
    /// Extract the inner [`time::Time`].
    pub fn into_inner(self) -> time::Time {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for TimeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Time".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({"type": "string", "format": "partial-time"})
            .as_object()
            .cloned()
            .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for TimeWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter time of day (hour, minute, second):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for TimeWrap {
    type Style = <time::Time as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimeWrap");
        let t = time::Time::elicit(communicator).await?;
        Ok(Self(t))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("TimeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("TimeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("TimeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for TimeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "TimeWrap",
            description: <TimeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for TimeWrap {}

// ── DateWrap — trenchcoat for time::Date ──────────────────────────────────────
/// Trenchcoat wrapper for [`time::Date`] that satisfies `schemars::JsonSchema`.
///
/// Use `#[schemars(with = "DateWrap")]` on `time::Date` fields in other structs
/// to delegate to this schema definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DateWrap(pub time::Date);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::Date> for DateWrap {
    fn from(inner: time::Date) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl DateWrap {
    /// Extract the inner [`time::Date`].
    pub fn into_inner(self) -> time::Date {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for DateWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Date".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({"type": "string", "format": "date"})
            .as_object()
            .cloned()
            .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for DateWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter calendar date (year, month, day):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for DateWrap {
    type Style = <time::Date as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateWrap");
        let d = time::Date::elicit(communicator).await?;
        Ok(Self(d))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("DateWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("DateWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("DateWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for DateWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "DateWrap",
            description: <DateWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for DateWrap {}

// ── DurationWrap — trenchcoat for time::Duration ──────────────────────────────
/// Trenchcoat wrapper for [`time::Duration`] that satisfies `schemars::JsonSchema`.
///
/// Use `#[schemars(with = "DurationWrap")]` on `time::Duration` fields in other
/// structs to delegate to this schema definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DurationWrap(pub time::Duration);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::Duration> for DurationWrap {
    fn from(inner: time::Duration) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl DurationWrap {
    /// Extract the inner [`time::Duration`].
    pub fn into_inner(self) -> time::Duration {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for DurationWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TimeDuration".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "string",
            "description": "Signed duration as \"seconds.nanoseconds\" (e.g., \"1.500000000\", \"-2.000000000\")"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for DurationWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter duration (whole seconds and subsecond nanoseconds):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for DurationWrap {
    type Style = <time::Duration as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DurationWrap");
        let d = time::Duration::elicit(communicator).await?;
        Ok(Self(d))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("DurationWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("DurationWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("DurationWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for DurationWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "DurationWrap",
            description: <DurationWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for DurationWrap {}

// ── MonthWrap — trenchcoat for time::Month ────────────────────────────────────
/// Trenchcoat wrapper for [`time::Month`] that satisfies `schemars::JsonSchema`.
///
/// Use `#[schemars(with = "MonthWrap")]` on `time::Month` fields in other structs
/// to delegate to this schema definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct MonthWrap(pub time::Month);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::Month> for MonthWrap {
    fn from(inner: time::Month) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl MonthWrap {
    /// Extract the inner [`time::Month`].
    pub fn into_inner(self) -> time::Month {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for MonthWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Month".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let labels = <time::Month as crate::Select>::labels();
        let enum_values: Vec<serde_json::Value> =
            labels.into_iter().map(serde_json::Value::String).collect();
        let map = serde_json::json!({
            "type": "string",
            "enum": enum_values,
            "description": "A calendar month"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for MonthWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose a month:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for MonthWrap {
    type Style = <time::Month as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting MonthWrap");
        let m = time::Month::elicit(communicator).await?;
        Ok(Self(m))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("MonthWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("MonthWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("MonthWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for MonthWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "MonthWrap",
            description: <MonthWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants: <time::Month as crate::Select>::labels()
                    .into_iter()
                    .map(|label| crate::VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for MonthWrap {
    fn prompt_tree() -> crate::PromptTree {
        let labels = <time::Month as crate::Select>::labels();
        let count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose a month:").to_string(),
            type_name: "MonthWrap".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for MonthWrap {}

// ── WeekdayWrap — trenchcoat for time::Weekday ───────────────────────────────
/// Trenchcoat wrapper for [`time::Weekday`] that satisfies `schemars::JsonSchema`.
///
/// Use `#[schemars(with = "WeekdayWrap")]` on `time::Weekday` fields in other
/// structs to delegate to this schema definition.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct WeekdayWrap(pub time::Weekday);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::Weekday> for WeekdayWrap {
    fn from(inner: time::Weekday) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl WeekdayWrap {
    /// Extract the inner [`time::Weekday`].
    pub fn into_inner(self) -> time::Weekday {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for WeekdayWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Weekday".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let labels = <time::Weekday as crate::Select>::labels();
        let enum_values: Vec<serde_json::Value> =
            labels.into_iter().map(serde_json::Value::String).collect();
        let map = serde_json::json!({
            "type": "string",
            "enum": enum_values,
            "description": "A day of the week"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for WeekdayWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose a day of the week:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for WeekdayWrap {
    type Style = <time::Weekday as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WeekdayWrap");
        let w = time::Weekday::elicit(communicator).await?;
        Ok(Self(w))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("WeekdayWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("WeekdayWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("WeekdayWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for WeekdayWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "WeekdayWrap",
            description: <WeekdayWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants: <time::Weekday as crate::Select>::labels()
                    .into_iter()
                    .map(|label| crate::VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for WeekdayWrap {
    fn prompt_tree() -> crate::PromptTree {
        let labels = <time::Weekday as crate::Select>::labels();
        let count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a day of the week:")
                .to_string(),
            type_name: "WeekdayWrap".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for WeekdayWrap {}

// ── UtcDateTimeWrap — trenchcoat for time::UtcDateTime ───────────────────────
/// Trenchcoat wrapper for [`time::UtcDateTime`] that satisfies `schemars::JsonSchema`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UtcDateTimeWrap(pub time::UtcDateTime);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::UtcDateTime> for UtcDateTimeWrap {
    fn from(inner: time::UtcDateTime) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl UtcDateTimeWrap {
    /// Extract the inner [`time::UtcDateTime`].
    pub fn into_inner(self) -> time::UtcDateTime {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for UtcDateTimeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UtcDateTime".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({"type": "string", "format": "date-time"})
            .as_object()
            .cloned()
            .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for UtcDateTimeWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC datetime (year, month, day, hour, minute, second):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for UtcDateTimeWrap {
    type Style = <time::UtcDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UtcDateTimeWrap");
        let dt = time::UtcDateTime::elicit(communicator).await?;
        Ok(Self(dt))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("UtcDateTimeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("UtcDateTimeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("UtcDateTimeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for UtcDateTimeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "UtcDateTimeWrap",
            description: <UtcDateTimeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for UtcDateTimeWrap {}

// ── ConversionRangeWrap — trenchcoat for time::error::ConversionRange ────────
/// Trenchcoat wrapper for [`time::error::ConversionRange`] that satisfies
/// `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as JSON `null` (single-value unit type, no data to encode).
/// Deserializes by ignoring the JSON input and constructing the one possible value
/// via a known-failing `std::time::Duration` → `time::Duration` conversion.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionRangeWrap(pub time::error::ConversionRange);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::error::ConversionRange> for ConversionRangeWrap {
    fn from(inner: time::error::ConversionRange) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl ConversionRangeWrap {
    /// Extract the inner [`time::error::ConversionRange`].
    pub fn into_inner(self) -> time::error::ConversionRange {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for ConversionRangeWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for ConversionRangeWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        time::Duration::try_from(std::time::Duration::new(u64::MAX, 0))
            .err()
            .map(Self)
            .ok_or_else(|| {
                serde::de::Error::custom("expected ConversionRange error was not produced")
            })
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for ConversionRangeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ConversionRange".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "null",
            "description": "A time conversion range error (single-value unit type, pass null)"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for ConversionRangeWrap {
    fn prompt() -> Option<&'static str> {
        Some("time::error::ConversionRange (conversion out of range — single value)")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for ConversionRangeWrap {
    type Style = <time::error::ConversionRange as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ConversionRangeWrap");
        let e = time::error::ConversionRange::elicit(communicator).await?;
        Ok(Self(e))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("ConversionRangeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("ConversionRangeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("ConversionRangeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for ConversionRangeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "ConversionRangeWrap",
            description: <ConversionRangeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for ConversionRangeWrap {}

// ── DifferentVariantWrap — trenchcoat for time::error::DifferentVariant ──────
/// Trenchcoat wrapper for [`time::error::DifferentVariant`] that satisfies
/// `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as JSON `null` (single-value unit type, no data to encode).
/// Deserializes by ignoring the JSON input and constructing the one possible
/// value directly.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DifferentVariantWrap(pub time::error::DifferentVariant);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::error::DifferentVariant> for DifferentVariantWrap {
    fn from(inner: time::error::DifferentVariant) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl DifferentVariantWrap {
    /// Extract the inner [`time::error::DifferentVariant`].
    pub fn into_inner(self) -> time::error::DifferentVariant {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for DifferentVariantWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for DifferentVariantWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        Ok(Self(time::error::DifferentVariant))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for DifferentVariantWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "DifferentVariant".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "null",
            "description": "A format-description variant mismatch error (single-value unit type, pass null)"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for DifferentVariantWrap {
    fn prompt() -> Option<&'static str> {
        Some("time::error::DifferentVariant (wrong format-description variant — single value)")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for DifferentVariantWrap {
    type Style = <time::error::DifferentVariant as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DifferentVariantWrap");
        let e = time::error::DifferentVariant::elicit(communicator).await?;
        Ok(Self(e))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("DifferentVariantWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("DifferentVariantWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("DifferentVariantWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for DifferentVariantWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "DifferentVariantWrap",
            description: <DifferentVariantWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for DifferentVariantWrap {}

// ── FmtPaddingWrap — trenchcoat for time::format_description::modifier::Padding
/// Trenchcoat wrapper for [`time::format_description::modifier::Padding`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as a JSON string: `"Space"`, `"Zero"`, or `"None"`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtPaddingWrap(pub time::format_description::modifier::Padding);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Padding> for FmtPaddingWrap {
    fn from(inner: time::format_description::modifier::Padding) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtPaddingWrap {
    /// Extract the inner [`time::format_description::modifier::Padding`].
    pub fn into_inner(self) -> time::format_description::modifier::Padding {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtPaddingWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use time::format_description::modifier::Padding;
        let s = match self.0 {
            Padding::Space => "Space",
            Padding::Zero => "Zero",
            Padding::None => "None",
            _ => "None",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtPaddingWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::Padding;
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Space" => Ok(Self(Padding::Space)),
            "Zero" => Ok(Self(Padding::Zero)),
            "None" => Ok(Self(Padding::None)),
            other => Err(D::Error::unknown_variant(other, &["Space", "Zero", "None"])),
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtPaddingWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtPadding".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "string",
            "enum": ["Space", "Zero", "None"],
            "description": "Padding type for a format description component field"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtPaddingWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose a padding type:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtPaddingWrap {
    type Style = <time::format_description::modifier::Padding as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtPaddingWrap");
        let inner = time::format_description::modifier::Padding::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtPaddingWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtPaddingWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtPaddingWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtPaddingWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtPaddingWrap",
            description: <FmtPaddingWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants: <time::format_description::modifier::Padding as crate::Select>::labels()
                    .into_iter()
                    .map(|label| crate::VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtPaddingWrap {
    fn prompt_tree() -> crate::PromptTree {
        let labels = <time::format_description::modifier::Padding as crate::Select>::labels();
        let count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a padding type:")
                .to_string(),
            type_name: "FmtPaddingWrap".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtPaddingWrap {}

// ── FmtDayWrap — trenchcoat for time::format_description::modifier::Day ──────
/// Trenchcoat wrapper for [`time::format_description::modifier::Day`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as a JSON object: `{"padding": "Space"|"Zero"|"None"}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtDayWrap(pub time::format_description::modifier::Day);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Day> for FmtDayWrap {
    fn from(inner: time::format_description::modifier::Day) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtDayWrap {
    /// Extract the inner [`time::format_description::modifier::Day`].
    pub fn into_inner(self) -> time::format_description::modifier::Day {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtDayWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        use time::format_description::modifier::Padding;
        let padding_str = match self.0.padding {
            Padding::Space => "Space",
            Padding::Zero => "Zero",
            Padding::None => "None",
            _ => "None",
        };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("padding", padding_str)?;
        map.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtDayWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::Padding;
        #[derive(serde::Deserialize)]
        struct Helper {
            padding: String,
        }
        let helper = Helper::deserialize(deserializer)?;
        let padding = match helper.padding.as_str() {
            "Space" => Padding::Space,
            "Zero" => Padding::Zero,
            "None" => Padding::None,
            other => {
                return Err(D::Error::unknown_variant(other, &["Space", "Zero", "None"]));
            }
        };
        Ok(Self(
            time::format_description::modifier::Day::default().with_padding(padding),
        ))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtDayWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtDay".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "padding": { "type": "string", "enum": ["Space", "Zero", "None"] }
            },
            "required": ["padding"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtDayWrap {
    fn prompt() -> Option<&'static str> {
        Some("Configure day-of-month formatting:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtDayWrap {
    type Style = <time::format_description::modifier::Day as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtDayWrap");
        let inner = time::format_description::modifier::Day::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtDayWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtDayWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtDayWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtDayWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtDayWrap",
            description: <FmtDayWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "padding",
                    type_name: "time::format_description::modifier::Padding",
                    prompt: None,
                }],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtDayWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "FmtDayWrap".to_string(),
            fields: vec![(
                "padding".to_string(),
                Box::new(time::format_description::modifier::Padding::prompt_tree()),
            )],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtDayWrap {}

// ── FmtEndWrap — trenchcoat for time::format_description::modifier::End ──────
/// Trenchcoat wrapper for [`time::format_description::modifier::End`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as JSON `null` (single-value — `trailing_input` is `pub(crate)`).
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtEndWrap(pub time::format_description::modifier::End);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::End> for FmtEndWrap {
    fn from(inner: time::format_description::modifier::End) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtEndWrap {
    /// Extract the inner [`time::format_description::modifier::End`].
    pub fn into_inner(self) -> time::format_description::modifier::End {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtEndWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtEndWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let _: serde::de::IgnoredAny = serde::de::Deserialize::deserialize(deserializer)?;
        Ok(Self(time::format_description::modifier::End::default()))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtEndWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtEnd".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "null",
            "description": "End-of-input component (single value — pass null)"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtEndWrap {
    fn prompt() -> Option<&'static str> {
        Some("End-of-input component (single value)")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtEndWrap {
    type Style = <time::format_description::modifier::End as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtEndWrap");
        let inner = time::format_description::modifier::End::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtEndWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtEndWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtEndWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtEndWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtEndWrap",
            description: <FmtEndWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtEndWrap {}

// ── FmtTrailingInputWrap — trenchcoat for time::format_description::modifier::TrailingInput
/// Trenchcoat wrapper for [`time::format_description::modifier::TrailingInput`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as a JSON string: `"Prohibit"` or `"Discard"`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtTrailingInputWrap(pub time::format_description::modifier::TrailingInput);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::TrailingInput> for FmtTrailingInputWrap {
    fn from(inner: time::format_description::modifier::TrailingInput) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtTrailingInputWrap {
    /// Extract the inner [`time::format_description::modifier::TrailingInput`].
    pub fn into_inner(self) -> time::format_description::modifier::TrailingInput {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtTrailingInputWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use time::format_description::modifier::TrailingInput;
        let s = match self.0 {
            TrailingInput::Prohibit => "Prohibit",
            TrailingInput::Discard => "Discard",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtTrailingInputWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::TrailingInput;
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Prohibit" => Ok(Self(TrailingInput::Prohibit)),
            "Discard" => Ok(Self(TrailingInput::Discard)),
            other => Err(D::Error::unknown_variant(other, &["Prohibit", "Discard"])),
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtTrailingInputWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtTrailingInput".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "string",
            "enum": ["Prohibit", "Discard"],
            "description": "How to handle trailing input after the declared end component"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtTrailingInputWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose how to handle trailing input:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtTrailingInputWrap {
    type Style = <time::format_description::modifier::TrailingInput as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtTrailingInputWrap");
        let inner = time::format_description::modifier::TrailingInput::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtTrailingInputWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtTrailingInputWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtTrailingInputWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtTrailingInputWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtTrailingInputWrap",
            description: <FmtTrailingInputWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants:
                    <time::format_description::modifier::TrailingInput as crate::Select>::labels()
                        .into_iter()
                        .map(|label| crate::VariantMetadata {
                            label,
                            fields: vec![],
                        })
                        .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtTrailingInputWrap {
    fn prompt_tree() -> crate::PromptTree {
        let labels = <time::format_description::modifier::TrailingInput as crate::Select>::labels();
        let count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose how to handle trailing input:")
                .to_string(),
            type_name: "FmtTrailingInputWrap".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtTrailingInputWrap {}

// ── Padding-only trenchcoats ──────────────────────────────────────────────────
// Ordinal, Second, OffsetMinute, OffsetSecond, WeekNumberIso/Sunday/Monday
// all serialize as {"padding": "Space"|"Zero"|"None"} and share identical impls.

macro_rules! fmt_padding_wrap {
    (
        $wrap:ident,
        $inner:ty,
        $mod_path:literal,
        $schema_name:literal,
        $prompt:literal
    ) => {
        /// Trenchcoat wrapper satisfying `JsonSchema + Serialize + Deserialize`.
        ///
        /// Serializes as `{"padding": "Space"|"Zero"|"None"}`.
        ///
        /// Available with the `time` feature.
        #[cfg(all(feature = "time", not(kani)))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrap(pub $inner);

        #[cfg(all(feature = "time", not(kani)))]
        impl From<$inner> for $wrap {
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl $wrap {
            /// Extract the inner value.
            pub fn into_inner(self) -> $inner {
                self.0
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl serde::Serialize for $wrap {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap as _;
                use time::format_description::modifier::Padding;
                let padding_str = match self.0.padding {
                    Padding::Space => "Space",
                    Padding::Zero => "Zero",
                    Padding::None => "None",
                    _ => "None",
                };
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("padding", padding_str)?;
                map.end()
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl<'de> serde::Deserialize<'de> for $wrap {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                use serde::de::Error as _;
                use time::format_description::modifier::Padding;
                #[derive(serde::Deserialize)]
                struct Helper {
                    padding: String,
                }
                let helper = Helper::deserialize(deserializer)?;
                let padding = match helper.padding.as_str() {
                    "Space" => Padding::Space,
                    "Zero" => Padding::Zero,
                    "None" => Padding::None,
                    other => {
                        return Err(D::Error::unknown_variant(
                            other,
                            &["Space", "Zero", "None"],
                        ));
                    }
                };
                Ok(Self(<$inner>::default().with_padding(padding)))
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl schemars::JsonSchema for $wrap {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }

            fn json_schema(
                _generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                let map = serde_json::json!({
                    "type": "object",
                    "properties": {
                        "padding": { "type": "string", "enum": ["Space", "Zero", "None"] }
                    },
                    "required": ["padding"]
                })
                .as_object()
                .cloned()
                .unwrap_or_default();
                schemars::Schema::from(map)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Prompt for $wrap {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Elicitation for $wrap {
            type Style = <$inner as Elicitation>::Style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($wrap)));
                let inner = <$inner>::elicit(communicator).await?;
                Ok(Self(inner))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitIntrospect for $wrap {
            fn pattern() -> crate::ElicitationPattern {
                crate::ElicitationPattern::Survey
            }

            fn metadata() -> crate::TypeMetadata {
                crate::TypeMetadata {
                    type_name: stringify!($wrap),
                    description: <$wrap as Prompt>::prompt(),
                    details: crate::PatternDetails::Survey {
                        fields: vec![crate::FieldInfo {
                            name: "padding",
                            type_name: "time::format_description::modifier::Padding",
                            prompt: None,
                        }],
                    },
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitPromptTree for $wrap {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: <$wrap as Prompt>::prompt().map(str::to_string),
                    type_name: stringify!($wrap).to_string(),
                    fields: vec![(
                        "padding".to_string(),
                        Box::new(
                            time::format_description::modifier::Padding::prompt_tree(),
                        ),
                    )],
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitComplete for $wrap {}
    };
}

fmt_padding_wrap!(
    FmtOrdinalWrap,
    time::format_description::modifier::Ordinal,
    "time::format_description::modifier::Ordinal",
    "FmtOrdinal",
    "Configure ordinal day-of-year formatting:"
);
fmt_padding_wrap!(
    FmtSecondWrap,
    time::format_description::modifier::Second,
    "time::format_description::modifier::Second",
    "FmtSecond",
    "Configure second formatting:"
);
fmt_padding_wrap!(
    FmtOffsetMinuteWrap,
    time::format_description::modifier::OffsetMinute,
    "time::format_description::modifier::OffsetMinute",
    "FmtOffsetMinute",
    "Configure UTC offset minute formatting:"
);
fmt_padding_wrap!(
    FmtOffsetSecondWrap,
    time::format_description::modifier::OffsetSecond,
    "time::format_description::modifier::OffsetSecond",
    "FmtOffsetSecond",
    "Configure UTC offset second formatting:"
);
fmt_padding_wrap!(
    FmtWeekNumberIsoWrap,
    time::format_description::modifier::WeekNumberIso,
    "time::format_description::modifier::WeekNumberIso",
    "FmtWeekNumberIso",
    "Configure ISO week number formatting:"
);
fmt_padding_wrap!(
    FmtWeekNumberSundayWrap,
    time::format_description::modifier::WeekNumberSunday,
    "time::format_description::modifier::WeekNumberSunday",
    "FmtWeekNumberSunday",
    "Configure Sunday-based week number formatting:"
);
fmt_padding_wrap!(
    FmtWeekNumberMondayWrap,
    time::format_description::modifier::WeekNumberMonday,
    "time::format_description::modifier::WeekNumberMonday",
    "FmtWeekNumberMonday",
    "Configure Monday-based week number formatting:"
);

// ── Opaque-padding wrappers (store Padding directly) ─────────────────────────
// For types where the padding field is pub(crate): the wrapper owns the
// Padding so ToCodeLiteral can emit default().with_padding() faithfully.

macro_rules! fmt_opaque_padding_wrap {
    (
        $wrap:ident,
        $inner:ty,
        $mod_path:literal,
        $schema_name:literal,
        $prompt:literal
    ) => {
        /// Trenchcoat wrapper storing `Padding` directly for faithful code generation.
        ///
        /// Available with the `time` feature.
        #[cfg(all(feature = "time", not(kani)))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrap(pub time::format_description::modifier::Padding);

        #[cfg(all(feature = "time", not(kani)))]
        impl $wrap {
            /// Reconstruct the inner value from the stored padding.
            pub fn into_inner(self) -> $inner {
                <$inner>::default().with_padding(self.0)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl serde::Serialize for $wrap {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap as _;
                use time::format_description::modifier::Padding;
                let padding_str = match self.0 {
                    Padding::Space => "Space",
                    Padding::Zero => "Zero",
                    Padding::None => "None",
                    _ => "None",
                };
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("padding", padding_str)?;
                map.end()
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl<'de> serde::Deserialize<'de> for $wrap {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                use serde::de::Error as _;
                use time::format_description::modifier::Padding;
                #[derive(serde::Deserialize)]
                struct Helper {
                    padding: String,
                }
                let helper = Helper::deserialize(deserializer)?;
                let padding = match helper.padding.as_str() {
                    "Space" => Padding::Space,
                    "Zero" => Padding::Zero,
                    "None" => Padding::None,
                    other => {
                        return Err(D::Error::unknown_variant(
                            other,
                            &["Space", "Zero", "None"],
                        ));
                    }
                };
                Ok(Self(padding))
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl schemars::JsonSchema for $wrap {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }

            fn json_schema(
                _generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                let map = serde_json::json!({
                    "type": "object",
                    "properties": {
                        "padding": { "type": "string", "enum": ["Space", "Zero", "None"] }
                    },
                    "required": ["padding"]
                })
                .as_object()
                .cloned()
                .unwrap_or_default();
                schemars::Schema::from(map)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Prompt for $wrap {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Elicitation for $wrap {
            type Style = <$inner as Elicitation>::Style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($wrap)));
                let padding =
                    time::format_description::modifier::Padding::elicit(communicator).await?;
                Ok(Self(padding))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitIntrospect for $wrap {
            fn pattern() -> crate::ElicitationPattern {
                crate::ElicitationPattern::Survey
            }

            fn metadata() -> crate::TypeMetadata {
                crate::TypeMetadata {
                    type_name: stringify!($wrap),
                    description: <$wrap as Prompt>::prompt(),
                    details: crate::PatternDetails::Survey {
                        fields: vec![crate::FieldInfo {
                            name: "padding",
                            type_name: "time::format_description::modifier::Padding",
                            prompt: None,
                        }],
                    },
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitPromptTree for $wrap {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: <$wrap as Prompt>::prompt().map(str::to_string),
                    type_name: stringify!($wrap).to_string(),
                    fields: vec![(
                        "padding".to_string(),
                        Box::new(
                            time::format_description::modifier::Padding::prompt_tree(),
                        ),
                    )],
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitComplete for $wrap {}
    };
}

fmt_opaque_padding_wrap!(
    FmtHour12Wrap,
    time::format_description::modifier::Hour12,
    "time::format_description::modifier::Hour12",
    "FmtHour12",
    "Configure 12-hour clock hour formatting:"
);
fmt_opaque_padding_wrap!(
    FmtHour24Wrap,
    time::format_description::modifier::Hour24,
    "time::format_description::modifier::Hour24",
    "FmtHour24",
    "Configure 24-hour clock hour formatting:"
);
fmt_opaque_padding_wrap!(
    FmtMonthNumericalWrap,
    time::format_description::modifier::MonthNumerical,
    "time::format_description::modifier::MonthNumerical",
    "FmtMonthNumerical",
    "Configure numerical month formatting:"
);
fmt_opaque_padding_wrap!(
    FmtCalendarYearLastTwoWrap,
    time::format_description::modifier::CalendarYearLastTwo,
    "time::format_description::modifier::CalendarYearLastTwo",
    "FmtCalendarYearLastTwo",
    "Configure calendar year last-two-digits formatting:"
);
fmt_opaque_padding_wrap!(
    FmtIsoYearLastTwoWrap,
    time::format_description::modifier::IsoYearLastTwo,
    "time::format_description::modifier::IsoYearLastTwo",
    "FmtIsoYearLastTwo",
    "Configure ISO year last-two-digits formatting:"
);

// ── Opaque bool-field wrappers ────────────────────────────────────────────────
// $field_name_str: serde/schema key (e.g. "case_sensitive")
// $field_name: ident used in the Helper struct field
// $with_method: builder method on the inner type

macro_rules! fmt_opaque_bool_wrap {
    (
        $wrap:ident,
        $inner:ty,
        $schema_name:literal,
        $prompt:literal,
        $field_name_str:literal,
        $field_name:ident,
        $with_method:ident
    ) => {
        /// Trenchcoat wrapper storing the bool field directly for faithful code generation.
        ///
        /// Available with the `time` feature.
        #[cfg(all(feature = "time", not(kani)))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrap(pub bool);

        #[cfg(all(feature = "time", not(kani)))]
        impl $wrap {
            /// Reconstruct the inner value from the stored field.
            pub fn into_inner(self) -> $inner {
                <$inner>::default().$with_method(self.0)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl serde::Serialize for $wrap {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap as _;
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry($field_name_str, &self.0)?;
                map.end()
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl<'de> serde::Deserialize<'de> for $wrap {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                #[derive(serde::Deserialize)]
                struct Helper {
                    $field_name: bool,
                }
                let helper = Helper::deserialize(deserializer)?;
                Ok(Self(helper.$field_name))
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl schemars::JsonSchema for $wrap {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }

            fn json_schema(
                _generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                let map = serde_json::json!({
                    "type": "object",
                    "properties": {
                        $field_name_str: { "type": "boolean" }
                    },
                    "required": [$field_name_str]
                })
                .as_object()
                .cloned()
                .unwrap_or_default();
                schemars::Schema::from(map)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Prompt for $wrap {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Elicitation for $wrap {
            type Style = <$inner as Elicitation>::Style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($wrap)));
                let value = bool::elicit(communicator).await?;
                Ok(Self(value))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitIntrospect for $wrap {
            fn pattern() -> crate::ElicitationPattern {
                crate::ElicitationPattern::Survey
            }

            fn metadata() -> crate::TypeMetadata {
                crate::TypeMetadata {
                    type_name: stringify!($wrap),
                    description: <$wrap as Prompt>::prompt(),
                    details: crate::PatternDetails::Survey {
                        fields: vec![crate::FieldInfo {
                            name: $field_name_str,
                            type_name: "bool",
                            prompt: None,
                        }],
                    },
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitPromptTree for $wrap {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: <$wrap as Prompt>::prompt().map(str::to_string),
                    type_name: stringify!($wrap).to_string(),
                    fields: vec![(
                        $field_name_str.to_string(),
                        Box::new(bool::prompt_tree()),
                    )],
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitComplete for $wrap {}
    };
}

fmt_opaque_bool_wrap!(
    FmtMonthShortWrap,
    time::format_description::modifier::MonthShort,
    "FmtMonthShort",
    "Configure abbreviated month name formatting:",
    "case_sensitive",
    case_sensitive,
    with_case_sensitive
);
fmt_opaque_bool_wrap!(
    FmtMonthLongWrap,
    time::format_description::modifier::MonthLong,
    "FmtMonthLong",
    "Configure full month name formatting:",
    "case_sensitive",
    case_sensitive,
    with_case_sensitive
);
fmt_opaque_bool_wrap!(
    FmtWeekdayShortWrap,
    time::format_description::modifier::WeekdayShort,
    "FmtWeekdayShort",
    "Configure abbreviated weekday name formatting:",
    "case_sensitive",
    case_sensitive,
    with_case_sensitive
);
fmt_opaque_bool_wrap!(
    FmtWeekdayLongWrap,
    time::format_description::modifier::WeekdayLong,
    "FmtWeekdayLong",
    "Configure full weekday name formatting:",
    "case_sensitive",
    case_sensitive,
    with_case_sensitive
);
fmt_opaque_bool_wrap!(
    FmtWeekdaySundayWrap,
    time::format_description::modifier::WeekdaySunday,
    "FmtWeekdaySunday",
    "Configure Sunday-based weekday index formatting:",
    "one_indexed",
    one_indexed,
    with_one_indexed
);
fmt_opaque_bool_wrap!(
    FmtWeekdayMondayWrap,
    time::format_description::modifier::WeekdayMonday,
    "FmtWeekdayMonday",
    "Configure Monday-based weekday index formatting:",
    "one_indexed",
    one_indexed,
    with_one_indexed
);
fmt_opaque_bool_wrap!(
    FmtUnixTimestampSecondWrap,
    time::format_description::modifier::UnixTimestampSecond,
    "FmtUnixTimestampSecond",
    "Configure Unix timestamp (second precision) formatting:",
    "sign_is_mandatory",
    sign_is_mandatory,
    with_sign_is_mandatory
);
fmt_opaque_bool_wrap!(
    FmtUnixTimestampMillisecondWrap,
    time::format_description::modifier::UnixTimestampMillisecond,
    "FmtUnixTimestampMillisecond",
    "Configure Unix timestamp (millisecond precision) formatting:",
    "sign_is_mandatory",
    sign_is_mandatory,
    with_sign_is_mandatory
);
fmt_opaque_bool_wrap!(
    FmtUnixTimestampMicrosecondWrap,
    time::format_description::modifier::UnixTimestampMicrosecond,
    "FmtUnixTimestampMicrosecond",
    "Configure Unix timestamp (microsecond precision) formatting:",
    "sign_is_mandatory",
    sign_is_mandatory,
    with_sign_is_mandatory
);
fmt_opaque_bool_wrap!(
    FmtUnixTimestampNanosecondWrap,
    time::format_description::modifier::UnixTimestampNanosecond,
    "FmtUnixTimestampNanosecond",
    "Configure Unix timestamp (nanosecond precision) formatting:",
    "sign_is_mandatory",
    sign_is_mandatory,
    with_sign_is_mandatory
);

// ── Opaque padding+sign wrappers ──────────────────────────────────────────────

macro_rules! fmt_opaque_padding_sign_wrap {
    (
        $wrap:ident,
        $inner:ty,
        $schema_name:literal,
        $prompt:literal
    ) => {
        /// Trenchcoat wrapper storing `(Padding, sign_is_mandatory)` for faithful code
        /// generation.
        ///
        /// Available with the `time` feature.
        #[cfg(all(feature = "time", not(kani)))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $wrap {
            /// The padding type.
            pub padding: time::format_description::modifier::Padding,
            /// Whether the sign is mandatory for non-negative values.
            pub sign_is_mandatory: bool,
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl $wrap {
            /// Reconstruct the inner value from stored fields.
            pub fn into_inner(self) -> $inner {
                <$inner>::default()
                    .with_padding(self.padding)
                    .with_sign_is_mandatory(self.sign_is_mandatory)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl serde::Serialize for $wrap {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use serde::ser::SerializeMap as _;
                use time::format_description::modifier::Padding;
                let padding_str = match self.padding {
                    Padding::Space => "Space",
                    Padding::Zero => "Zero",
                    Padding::None => "None",
                    _ => "None",
                };
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("padding", padding_str)?;
                map.serialize_entry("sign_is_mandatory", &self.sign_is_mandatory)?;
                map.end()
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl<'de> serde::Deserialize<'de> for $wrap {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                use serde::de::Error as _;
                use time::format_description::modifier::Padding;
                #[derive(serde::Deserialize)]
                struct Helper {
                    padding: String,
                    sign_is_mandatory: bool,
                }
                let helper = Helper::deserialize(deserializer)?;
                let padding = match helper.padding.as_str() {
                    "Space" => Padding::Space,
                    "Zero" => Padding::Zero,
                    "None" => Padding::None,
                    other => {
                        return Err(D::Error::unknown_variant(
                            other,
                            &["Space", "Zero", "None"],
                        ));
                    }
                };
                Ok(Self {
                    padding,
                    sign_is_mandatory: helper.sign_is_mandatory,
                })
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl schemars::JsonSchema for $wrap {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                $schema_name.into()
            }

            fn json_schema(
                _generator: &mut schemars::SchemaGenerator,
            ) -> schemars::Schema {
                let map = serde_json::json!({
                    "type": "object",
                    "properties": {
                        "padding": { "type": "string", "enum": ["Space", "Zero", "None"] },
                        "sign_is_mandatory": { "type": "boolean" }
                    },
                    "required": ["padding", "sign_is_mandatory"]
                })
                .as_object()
                .cloned()
                .unwrap_or_default();
                schemars::Schema::from(map)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Prompt for $wrap {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl Elicitation for $wrap {
            type Style = <$inner as Elicitation>::Style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($wrap)));
                let padding =
                    time::format_description::modifier::Padding::elicit(communicator).await?;
                let sign_is_mandatory = bool::elicit(communicator).await?;
                Ok(Self {
                    padding,
                    sign_is_mandatory,
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_newtype_wrapper_harness(
                    stringify!($wrap),
                )
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitIntrospect for $wrap {
            fn pattern() -> crate::ElicitationPattern {
                crate::ElicitationPattern::Survey
            }

            fn metadata() -> crate::TypeMetadata {
                crate::TypeMetadata {
                    type_name: stringify!($wrap),
                    description: <$wrap as Prompt>::prompt(),
                    details: crate::PatternDetails::Survey {
                        fields: vec![
                            crate::FieldInfo {
                                name: "padding",
                                type_name: "time::format_description::modifier::Padding",
                                prompt: None,
                            },
                            crate::FieldInfo {
                                name: "sign_is_mandatory",
                                type_name: "bool",
                                prompt: None,
                            },
                        ],
                    },
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitPromptTree for $wrap {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: <$wrap as Prompt>::prompt().map(str::to_string),
                    type_name: stringify!($wrap).to_string(),
                    fields: vec![
                        (
                            "padding".to_string(),
                            Box::new(
                                time::format_description::modifier::Padding::prompt_tree(),
                            ),
                        ),
                        (
                            "sign_is_mandatory".to_string(),
                            Box::new(bool::prompt_tree()),
                        ),
                    ],
                }
            }
        }

        #[cfg(all(feature = "time", not(kani)))]
        impl crate::ElicitComplete for $wrap {}
    };
}

fmt_opaque_padding_sign_wrap!(
    FmtCalendarYearFullExtendedRangeWrap,
    time::format_description::modifier::CalendarYearFullExtendedRange,
    "FmtCalendarYearFullExtendedRange",
    "Configure calendar year (full, extended range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtCalendarYearFullStandardRangeWrap,
    time::format_description::modifier::CalendarYearFullStandardRange,
    "FmtCalendarYearFullStandardRange",
    "Configure calendar year (full, standard range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtCalendarYearCenturyExtendedRangeWrap,
    time::format_description::modifier::CalendarYearCenturyExtendedRange,
    "FmtCalendarYearCenturyExtendedRange",
    "Configure calendar year century (extended range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtCalendarYearCenturyStandardRangeWrap,
    time::format_description::modifier::CalendarYearCenturyStandardRange,
    "FmtCalendarYearCenturyStandardRange",
    "Configure calendar year century (standard range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtIsoYearFullExtendedRangeWrap,
    time::format_description::modifier::IsoYearFullExtendedRange,
    "FmtIsoYearFullExtendedRange",
    "Configure ISO year (full, extended range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtIsoYearFullStandardRangeWrap,
    time::format_description::modifier::IsoYearFullStandardRange,
    "FmtIsoYearFullStandardRange",
    "Configure ISO year (full, standard range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtIsoYearCenturyExtendedRangeWrap,
    time::format_description::modifier::IsoYearCenturyExtendedRange,
    "FmtIsoYearCenturyExtendedRange",
    "Configure ISO year century (extended range) formatting:"
);
fmt_opaque_padding_sign_wrap!(
    FmtIsoYearCenturyStandardRangeWrap,
    time::format_description::modifier::IsoYearCenturyStandardRange,
    "FmtIsoYearCenturyStandardRange",
    "Configure ISO year century (standard range) formatting:"
);

// ── FmtSubsecondDigitsWrap ────────────────────────────────────────────────────

/// Trenchcoat wrapper for [`time::format_description::modifier::SubsecondDigits`].
///
/// Serializes as a string: `"OneOrMore"`, `"One"` … `"Nine"`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtSubsecondDigitsWrap(pub time::format_description::modifier::SubsecondDigits);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::SubsecondDigits> for FmtSubsecondDigitsWrap {
    fn from(inner: time::format_description::modifier::SubsecondDigits) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtSubsecondDigitsWrap {
    /// Extract the inner value.
    pub fn into_inner(self) -> time::format_description::modifier::SubsecondDigits {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtSubsecondDigitsWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use time::format_description::modifier::SubsecondDigits;
        let s = match self.0 {
            SubsecondDigits::OneOrMore => "OneOrMore",
            SubsecondDigits::One => "One",
            SubsecondDigits::Two => "Two",
            SubsecondDigits::Three => "Three",
            SubsecondDigits::Four => "Four",
            SubsecondDigits::Five => "Five",
            SubsecondDigits::Six => "Six",
            SubsecondDigits::Seven => "Seven",
            SubsecondDigits::Eight => "Eight",
            SubsecondDigits::Nine => "Nine",
            _ => "OneOrMore",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtSubsecondDigitsWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::SubsecondDigits;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "OneOrMore" => SubsecondDigits::OneOrMore,
            "One" => SubsecondDigits::One,
            "Two" => SubsecondDigits::Two,
            "Three" => SubsecondDigits::Three,
            "Four" => SubsecondDigits::Four,
            "Five" => SubsecondDigits::Five,
            "Six" => SubsecondDigits::Six,
            "Seven" => SubsecondDigits::Seven,
            "Eight" => SubsecondDigits::Eight,
            "Nine" => SubsecondDigits::Nine,
            other => {
                return Err(D::Error::unknown_variant(
                    other,
                    &["OneOrMore", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"],
                ));
            }
        };
        Ok(Self(inner))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtSubsecondDigitsWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtSubsecondDigits".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "string",
            "enum": ["OneOrMore", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtSubsecondDigitsWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose subsecond digit count:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtSubsecondDigitsWrap {
    type Style = <time::format_description::modifier::SubsecondDigits as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtSubsecondDigitsWrap");
        let inner =
            time::format_description::modifier::SubsecondDigits::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtSubsecondDigitsWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtSubsecondDigitsWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness(
            "FmtSubsecondDigitsWrap",
        )
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtSubsecondDigitsWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtSubsecondDigitsWrap",
            description: <FmtSubsecondDigitsWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants: <time::format_description::modifier::SubsecondDigits as crate::Select>::labels()
                    .into_iter()
                    .map(|label| crate::VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtSubsecondDigitsWrap {
    fn prompt_tree() -> crate::PromptTree {
        time::format_description::modifier::SubsecondDigits::prompt_tree()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtSubsecondDigitsWrap {}

// ── FmtSubsecondWrap ──────────────────────────────────────────────────────────

/// Trenchcoat wrapper for [`time::format_description::modifier::Subsecond`].
///
/// Serializes as `{"digits": "OneOrMore"|"One"|…|"Nine"}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtSubsecondWrap(pub time::format_description::modifier::Subsecond);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Subsecond> for FmtSubsecondWrap {
    fn from(inner: time::format_description::modifier::Subsecond) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtSubsecondWrap {
    /// Extract the inner value.
    pub fn into_inner(self) -> time::format_description::modifier::Subsecond {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtSubsecondWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        FmtSubsecondDigitsWrap(self.0.digits).serialize(serializer)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtSubsecondWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::SubsecondDigits;
        #[derive(serde::Deserialize)]
        struct Helper {
            digits: String,
        }
        let h = Helper::deserialize(deserializer)?;
        let digits = match h.digits.as_str() {
            "OneOrMore" => SubsecondDigits::OneOrMore,
            "One" => SubsecondDigits::One,
            "Two" => SubsecondDigits::Two,
            "Three" => SubsecondDigits::Three,
            "Four" => SubsecondDigits::Four,
            "Five" => SubsecondDigits::Five,
            "Six" => SubsecondDigits::Six,
            "Seven" => SubsecondDigits::Seven,
            "Eight" => SubsecondDigits::Eight,
            "Nine" => SubsecondDigits::Nine,
            other => {
                return Err(D::Error::unknown_variant(
                    other,
                    &["OneOrMore", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"],
                ));
            }
        };
        Ok(Self(
            time::format_description::modifier::Subsecond::default().with_digits(digits),
        ))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtSubsecondWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtSubsecond".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "digits": {
                    "type": "string",
                    "enum": ["OneOrMore", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"]
                }
            },
            "required": ["digits"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtSubsecondWrap {
    fn prompt() -> Option<&'static str> {
        Some("Configure subsecond formatting:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtSubsecondWrap {
    type Style = <time::format_description::modifier::Subsecond as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtSubsecondWrap");
        let inner = time::format_description::modifier::Subsecond::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtSubsecondWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtSubsecondWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtSubsecondWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtSubsecondWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtSubsecondWrap",
            description: <FmtSubsecondWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "digits",
                    type_name: "time::format_description::modifier::SubsecondDigits",
                    prompt: None,
                }],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtSubsecondWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: <FmtSubsecondWrap as Prompt>::prompt().map(str::to_string),
            type_name: "FmtSubsecondWrap".to_string(),
            fields: vec![(
                "digits".to_string(),
                Box::new(
                    time::format_description::modifier::SubsecondDigits::prompt_tree(),
                ),
            )],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtSubsecondWrap {}

// ── FmtPeriodWrap ─────────────────────────────────────────────────────────────

/// Trenchcoat wrapper for [`time::format_description::modifier::Period`].
///
/// Serializes as `{"is_uppercase": bool, "case_sensitive": bool}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtPeriodWrap(pub time::format_description::modifier::Period);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Period> for FmtPeriodWrap {
    fn from(inner: time::format_description::modifier::Period) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtPeriodWrap {
    /// Extract the inner value.
    pub fn into_inner(self) -> time::format_description::modifier::Period {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtPeriodWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("is_uppercase", &self.0.is_uppercase)?;
        map.serialize_entry("case_sensitive", &self.0.case_sensitive)?;
        map.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtPeriodWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Helper {
            is_uppercase: bool,
            case_sensitive: bool,
        }
        let h = Helper::deserialize(deserializer)?;
        Ok(Self(
            time::format_description::modifier::Period::default()
                .with_is_uppercase(h.is_uppercase)
                .with_case_sensitive(h.case_sensitive),
        ))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtPeriodWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtPeriod".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "is_uppercase": { "type": "boolean" },
                "case_sensitive": { "type": "boolean" }
            },
            "required": ["is_uppercase", "case_sensitive"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtPeriodWrap {
    fn prompt() -> Option<&'static str> {
        Some("Configure AM/PM period formatting:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtPeriodWrap {
    type Style = <time::format_description::modifier::Period as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtPeriodWrap");
        let inner = time::format_description::modifier::Period::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtPeriodWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtPeriodWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtPeriodWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtPeriodWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtPeriodWrap",
            description: <FmtPeriodWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "is_uppercase",
                        type_name: "bool",
                        prompt: None,
                    },
                    crate::FieldInfo {
                        name: "case_sensitive",
                        type_name: "bool",
                        prompt: None,
                    },
                ],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtPeriodWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: <FmtPeriodWrap as Prompt>::prompt().map(str::to_string),
            type_name: "FmtPeriodWrap".to_string(),
            fields: vec![
                ("is_uppercase".to_string(), Box::new(bool::prompt_tree())),
                ("case_sensitive".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtPeriodWrap {}

// ── FmtOffsetHourWrap ─────────────────────────────────────────────────────────

/// Trenchcoat wrapper for [`time::format_description::modifier::OffsetHour`].
///
/// Serializes as `{"sign_is_mandatory": bool, "padding": "Space"|"Zero"|"None"}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtOffsetHourWrap(pub time::format_description::modifier::OffsetHour);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::OffsetHour> for FmtOffsetHourWrap {
    fn from(inner: time::format_description::modifier::OffsetHour) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtOffsetHourWrap {
    /// Extract the inner value.
    pub fn into_inner(self) -> time::format_description::modifier::OffsetHour {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtOffsetHourWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        use time::format_description::modifier::Padding;
        let padding_str = match self.0.padding {
            Padding::Space => "Space",
            Padding::Zero => "Zero",
            Padding::None => "None",
            _ => "None",
        };
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("sign_is_mandatory", &self.0.sign_is_mandatory)?;
        map.serialize_entry("padding", padding_str)?;
        map.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtOffsetHourWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::Padding;
        #[derive(serde::Deserialize)]
        struct Helper {
            sign_is_mandatory: bool,
            padding: String,
        }
        let h = Helper::deserialize(deserializer)?;
        let padding = match h.padding.as_str() {
            "Space" => Padding::Space,
            "Zero" => Padding::Zero,
            "None" => Padding::None,
            other => {
                return Err(D::Error::unknown_variant(
                    other,
                    &["Space", "Zero", "None"],
                ));
            }
        };
        Ok(Self(
            time::format_description::modifier::OffsetHour::default()
                .with_sign_is_mandatory(h.sign_is_mandatory)
                .with_padding(padding),
        ))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtOffsetHourWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtOffsetHour".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "sign_is_mandatory": { "type": "boolean" },
                "padding": { "type": "string", "enum": ["Space", "Zero", "None"] }
            },
            "required": ["sign_is_mandatory", "padding"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtOffsetHourWrap {
    fn prompt() -> Option<&'static str> {
        Some("Configure UTC offset hour formatting:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtOffsetHourWrap {
    type Style = <time::format_description::modifier::OffsetHour as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtOffsetHourWrap");
        let inner = time::format_description::modifier::OffsetHour::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtOffsetHourWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtOffsetHourWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtOffsetHourWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtOffsetHourWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtOffsetHourWrap",
            description: <FmtOffsetHourWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "sign_is_mandatory",
                        type_name: "bool",
                        prompt: None,
                    },
                    crate::FieldInfo {
                        name: "padding",
                        type_name: "time::format_description::modifier::Padding",
                        prompt: None,
                    },
                ],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtOffsetHourWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: <FmtOffsetHourWrap as Prompt>::prompt().map(str::to_string),
            type_name: "FmtOffsetHourWrap".to_string(),
            fields: vec![
                (
                    "sign_is_mandatory".to_string(),
                    Box::new(bool::prompt_tree()),
                ),
                (
                    "padding".to_string(),
                    Box::new(
                        time::format_description::modifier::Padding::prompt_tree(),
                    ),
                ),
            ],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtOffsetHourWrap {}

// ── WellKnownRfc2822Wrap ──────────────────────────────────────────────────────

/// Trenchcoat wrapper for [`time::format_description::well_known::Rfc2822`].
///
/// Unit struct — serializes as `{}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WellKnownRfc2822Wrap;

#[cfg(all(feature = "time", not(kani)))]
impl WellKnownRfc2822Wrap {
    /// Extract the inner value.
    pub fn into_inner(self) -> time::format_description::well_known::Rfc2822 {
        time::format_description::well_known::Rfc2822
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for WellKnownRfc2822Wrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        serializer.serialize_map(Some(0))?.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for WellKnownRfc2822Wrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Helper {}
        Helper::deserialize(deserializer)?;
        Ok(Self)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for WellKnownRfc2822Wrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "WellKnownRfc2822".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({ "type": "object", "properties": {} })
            .as_object()
            .cloned()
            .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for WellKnownRfc2822Wrap {
    fn prompt() -> Option<&'static str> {
        Some("RFC 2822 format (e.g. Fri, 21 Nov 1997 09:55:06 -0600) — no options to configure.")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for WellKnownRfc2822Wrap {
    type Style = <time::format_description::well_known::Rfc2822 as Elicitation>::Style;

    async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
        Ok(Self)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("WellKnownRfc2822Wrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("WellKnownRfc2822Wrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("WellKnownRfc2822Wrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for WellKnownRfc2822Wrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "WellKnownRfc2822Wrap",
            description: <WellKnownRfc2822Wrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for WellKnownRfc2822Wrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt: <WellKnownRfc2822Wrap as Prompt>::prompt()
                .unwrap_or("WellKnownRfc2822Wrap")
                .to_string(),
            type_name: "WellKnownRfc2822Wrap".to_string(),
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for WellKnownRfc2822Wrap {}

// ── FmtIgnoreWrap — trenchcoat for time::format_description::modifier::Ignore ─
/// Trenchcoat wrapper for [`time::format_description::modifier::Ignore`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as `{"count": u16}` where count is non-zero.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtIgnoreWrap(pub time::format_description::modifier::Ignore);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Ignore> for FmtIgnoreWrap {
    fn from(inner: time::format_description::modifier::Ignore) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtIgnoreWrap {
    /// Extract the inner [`time::format_description::modifier::Ignore`].
    pub fn into_inner(self) -> time::format_description::modifier::Ignore {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtIgnoreWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("count", &self.0.count.get())?;
        map.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtIgnoreWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        #[derive(serde::Deserialize)]
        struct Helper {
            count: u16,
        }
        let helper = Helper::deserialize(deserializer)?;
        let count = core::num::NonZero::<u16>::new(helper.count)
            .ok_or_else(|| D::Error::custom("Ignore count must be non-zero"))?;
        Ok(Self(time::format_description::modifier::Ignore::count(
            count,
        )))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtIgnoreWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtIgnore".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "count": { "type": "integer", "minimum": 1, "maximum": 65535 }
            },
            "required": ["count"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtIgnoreWrap {
    fn prompt() -> Option<&'static str> {
        Some("Number of bytes to ignore (non-zero):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtIgnoreWrap {
    type Style = <time::format_description::modifier::Ignore as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtIgnoreWrap");
        let inner = time::format_description::modifier::Ignore::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtIgnoreWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtIgnoreWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtIgnoreWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtIgnoreWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtIgnoreWrap",
            description: <FmtIgnoreWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "count",
                    type_name: "u16",
                    prompt: Some("Number of bytes to ignore (non-zero):"),
                }],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtIgnoreWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "FmtIgnoreWrap".to_string(),
            fields: vec![("count".to_string(), Box::new(u16::prompt_tree()))],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtIgnoreWrap {}

// ── FmtMinuteWrap — trenchcoat for time::format_description::modifier::Minute ─
/// Trenchcoat wrapper for [`time::format_description::modifier::Minute`] that
/// satisfies `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as `{"padding": "Space"|"Zero"|"None"}`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FmtMinuteWrap(pub time::format_description::modifier::Minute);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::format_description::modifier::Minute> for FmtMinuteWrap {
    fn from(inner: time::format_description::modifier::Minute) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl FmtMinuteWrap {
    /// Extract the inner [`time::format_description::modifier::Minute`].
    pub fn into_inner(self) -> time::format_description::modifier::Minute {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for FmtMinuteWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        use time::format_description::modifier::Padding;
        let padding_str = match self.0.padding {
            Padding::Space => "Space",
            Padding::Zero => "Zero",
            Padding::None => "None",
            _ => "None",
        };
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("padding", padding_str)?;
        map.end()
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for FmtMinuteWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        use time::format_description::modifier::Padding;
        #[derive(serde::Deserialize)]
        struct Helper {
            padding: String,
        }
        let helper = Helper::deserialize(deserializer)?;
        let padding = match helper.padding.as_str() {
            "Space" => Padding::Space,
            "Zero" => Padding::Zero,
            "None" => Padding::None,
            other => {
                return Err(D::Error::unknown_variant(other, &["Space", "Zero", "None"]));
            }
        };
        Ok(Self(
            time::format_description::modifier::Minute::default().with_padding(padding),
        ))
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for FmtMinuteWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FmtMinute".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "object",
            "properties": {
                "padding": { "type": "string", "enum": ["Space", "Zero", "None"] }
            },
            "required": ["padding"]
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for FmtMinuteWrap {
    fn prompt() -> Option<&'static str> {
        Some("Configure minute formatting:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for FmtMinuteWrap {
    type Style = <time::format_description::modifier::Minute as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting FmtMinuteWrap");
        let inner = time::format_description::modifier::Minute::elicit(communicator).await?;
        Ok(Self(inner))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("FmtMinuteWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("FmtMinuteWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("FmtMinuteWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for FmtMinuteWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Survey
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "FmtMinuteWrap",
            description: <FmtMinuteWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "padding",
                    type_name: "time::format_description::modifier::Padding",
                    prompt: None,
                }],
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for FmtMinuteWrap {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "FmtMinuteWrap".to_string(),
            fields: vec![(
                "padding".to_string(),
                Box::new(time::format_description::modifier::Padding::prompt_tree()),
            )],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for FmtMinuteWrap {}

// ── ComponentRangeWrap — trenchcoat for time::error::ComponentRange ──────────
/// Trenchcoat wrapper for [`time::error::ComponentRange`] that satisfies
/// `schemars::JsonSchema`, `Serialize`, and `Deserialize`.
///
/// Serializes as a plain string (the component name, e.g. `"hour"`, `"day"`).
/// Deserialization triggers the matching `time` API call to reproduce the error
/// value — the same technique used for other opaque error trenchcoats.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRangeWrap(pub time::error::ComponentRange);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::error::ComponentRange> for ComponentRangeWrap {
    fn from(inner: time::error::ComponentRange) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl ComponentRangeWrap {
    /// Extract the inner [`time::error::ComponentRange`].
    pub fn into_inner(self) -> time::error::ComponentRange {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl serde::Serialize for ComponentRangeWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.0.name())
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl<'de> serde::Deserialize<'de> for ComponentRangeWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = <String as serde::Deserialize<'de>>::deserialize(deserializer)?;
        <time::error::ComponentRange as crate::Select>::from_label(&name)
            .map(Self)
            .ok_or_else(|| {
                serde::de::Error::unknown_variant(
                    &name,
                    &[
                        "hour",
                        "minute",
                        "second",
                        "nanosecond",
                        "month",
                        "offset",
                        "day",
                    ],
                )
            })
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for ComponentRangeWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ComponentRange".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let labels = <time::error::ComponentRange as crate::Select>::labels();
        let enum_values: Vec<serde_json::Value> =
            labels.into_iter().map(serde_json::Value::String).collect();
        let map = serde_json::json!({
            "type": "string",
            "enum": enum_values,
            "description": "A time component range error — identifies which component was out of range"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for ComponentRangeWrap {
    fn prompt() -> Option<&'static str> {
        Some("Choose a time component range error:")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for ComponentRangeWrap {
    type Style = <time::error::ComponentRange as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ComponentRangeWrap");
        let e = time::error::ComponentRange::elicit(communicator).await?;
        Ok(Self(e))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("ComponentRangeWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("ComponentRangeWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("ComponentRangeWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for ComponentRangeWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Select
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "ComponentRangeWrap",
            description: <ComponentRangeWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Select {
                variants: <time::error::ComponentRange as crate::Select>::labels()
                    .into_iter()
                    .map(|label| crate::VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitPromptTree for ComponentRangeWrap {
    fn prompt_tree() -> crate::PromptTree {
        let labels = <time::error::ComponentRange as crate::Select>::labels();
        let count = labels.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a time component range error:")
                .to_string(),
            type_name: "ComponentRangeWrap".to_string(),
            options: labels,
            branches: vec![None; count],
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for ComponentRangeWrap {}

// ── UtcOffsetWrap — trenchcoat for time::UtcOffset ───────────────────────────
/// Trenchcoat wrapper for [`time::UtcOffset`] that satisfies `schemars::JsonSchema`.
///
/// Available with the `time` feature.
#[cfg(all(feature = "time", not(kani)))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UtcOffsetWrap(pub time::UtcOffset);

#[cfg(all(feature = "time", not(kani)))]
impl From<time::UtcOffset> for UtcOffsetWrap {
    fn from(inner: time::UtcOffset) -> Self {
        Self(inner)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl UtcOffsetWrap {
    /// Extract the inner [`time::UtcOffset`].
    pub fn into_inner(self) -> time::UtcOffset {
        self.0
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl schemars::JsonSchema for UtcOffsetWrap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "UtcOffset".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let map = serde_json::json!({
            "type": "string",
            "description": "UTC offset as ±HH:MM:SS (e.g., \"+01:00:00\", \"-05:30:00\", \"+00:00:00\")"
        })
        .as_object()
        .cloned()
        .unwrap_or_default();
        schemars::Schema::from(map)
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Prompt for UtcOffsetWrap {
    fn prompt() -> Option<&'static str> {
        Some("Enter UTC offset in whole seconds (e.g., 3600 for +01:00, -7200 for -02:00):")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl Elicitation for UtcOffsetWrap {
    type Style = <time::UtcOffset as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UtcOffsetWrap");
        let o = time::UtcOffset::elicit(communicator).await?;
        Ok(Self(o))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("UtcOffsetWrap")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("UtcOffsetWrap")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("UtcOffsetWrap")
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for UtcOffsetWrap {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "UtcOffsetWrap",
            description: <UtcOffsetWrap as Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for UtcOffsetWrap {}

// OffsetDateTimeAfter - OffsetDateTime after a threshold
/// An OffsetDateTime that is guaranteed to be after a threshold time.
///
/// Available with the `time` feature.
#[cfg(feature = "time")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[cfg(not(kani))]
pub struct OffsetDateTimeAfter {
    #[schemars(with = "OffsetDateTimeWrap")]
    value: OffsetDateTime,
    #[schemars(with = "OffsetDateTimeWrap")]
    threshold: OffsetDateTime,
}

#[cfg(all(feature = "time", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetDateTimeAfter(std::marker::PhantomData<()>);

#[cfg(feature = "time")]
#[cfg_attr(not(kani), instrumented_impl)]
impl OffsetDateTimeAfter {
    /// Create a new OffsetDateTimeAfter, validating value > threshold.
    #[cfg(not(kani))]
    #[spec(requires: [value > threshold])]
    pub fn new(value: OffsetDateTime, threshold: OffsetDateTime) -> Result<Self, ValidationError> {
        if value > threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust time crate, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(
        _value: OffsetDateTime,
        _threshold: OffsetDateTime,
    ) -> Result<Self, ValidationError> {
        let is_after: bool = kani::any();
        if is_after {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooEarly {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the datetime value.
    #[cfg(not(kani))]
    pub fn get(&self) -> OffsetDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> OffsetDateTime {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> OffsetDateTime {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> OffsetDateTime {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner OffsetDateTime.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> OffsetDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> OffsetDateTime {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "time")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for OffsetDateTimeAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime with timezone offset:")
    }
}

#[cfg(feature = "time")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for OffsetDateTimeAfter {
    type Style = <OffsetDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTimeAfter");
        // Default threshold to Unix epoch
        let threshold = OffsetDateTime::UNIX_EPOCH;
        loop {
            let dt = OffsetDateTime::elicit(communicator).await?;
            match Self::new(dt, threshold) {
                Ok(valid) => {
                    tracing::debug!(datetime = %valid.value, "Valid offset datetime after threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "OffsetDateTime not after threshold, re-prompting"
                    );
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("OffsetDateTimeAfter")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("OffsetDateTimeAfter")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("OffsetDateTimeAfter")
    }
}

// OffsetDateTimeBefore - OffsetDateTime before a threshold
/// An OffsetDateTime that is guaranteed to be before a threshold time.
///
/// Available with the `time` feature.
#[cfg(feature = "time")]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[cfg(not(kani))]
pub struct OffsetDateTimeBefore {
    #[schemars(with = "OffsetDateTimeWrap")]
    value: OffsetDateTime,
    #[schemars(with = "OffsetDateTimeWrap")]
    threshold: OffsetDateTime,
}

#[cfg(all(feature = "time", kani))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OffsetDateTimeBefore(std::marker::PhantomData<()>);

#[cfg(feature = "time")]
#[cfg_attr(not(kani), instrumented_impl)]
impl OffsetDateTimeBefore {
    /// Create a new OffsetDateTimeBefore, validating value < threshold.
    #[cfg(not(kani))]
    #[spec(requires: [value < threshold])]
    pub fn new(value: OffsetDateTime, threshold: OffsetDateTime) -> Result<Self, ValidationError> {
        if value < threshold {
            Ok(Self { value, threshold })
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: value.to_string(),
                threshold: threshold.to_string(),
            })
        }
    }

    /// Kani version: trust time crate, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(
        _value: OffsetDateTime,
        _threshold: OffsetDateTime,
    ) -> Result<Self, ValidationError> {
        let is_before: bool = kani::any();
        if is_before {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::DateTimeTooLate {
                value: "value".to_string(),
                threshold: "threshold".to_string(),
            })
        }
    }

    /// Get the datetime value.
    #[cfg(not(kani))]
    pub fn get(&self) -> OffsetDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn get(&self) -> OffsetDateTime {
        panic!("get() not supported in Kani verification")
    }

    /// Get the threshold.
    #[cfg(not(kani))]
    pub fn threshold(&self) -> OffsetDateTime {
        self.threshold
    }

    #[cfg(kani)]
    pub fn threshold(&self) -> OffsetDateTime {
        panic!("threshold() not supported in Kani verification")
    }

    /// Unwrap into the inner OffsetDateTime.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> OffsetDateTime {
        self.value
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> OffsetDateTime {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "time")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for OffsetDateTimeBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime with timezone offset:")
    }
}

#[cfg(feature = "time")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for OffsetDateTimeBefore {
    type Style = <OffsetDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTimeBefore");
        // Default threshold to now
        let threshold = OffsetDateTime::now_utc();
        loop {
            let dt = OffsetDateTime::elicit(communicator).await?;
            match Self::new(dt, threshold) {
                Ok(valid) => {
                    tracing::debug!(datetime = %valid.value, "Valid offset datetime before threshold");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "OffsetDateTime not before threshold, re-prompting"
                    );
                    continue;
                }
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_newtype_wrapper_harness("OffsetDateTimeBefore")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_newtype_wrapper_harness("OffsetDateTimeBefore")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_newtype_wrapper_harness("OffsetDateTimeBefore")
    }
}

#[cfg(all(test, feature = "time"))]
mod time_tests {
    use super::*;
    use time::Duration;

    #[test]
    fn test_offset_datetime_after_valid() {
        let threshold = OffsetDateTime::now_utc();
        let value = threshold + Duration::hours(1);
        let result = OffsetDateTimeAfter::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_offset_datetime_after_too_early() {
        let threshold = OffsetDateTime::now_utc();
        let value = threshold - Duration::hours(1);
        let result = OffsetDateTimeAfter::new(value, threshold);
        assert!(result.is_err());
    }

    #[test]
    fn test_offset_datetime_before_valid() {
        let threshold = OffsetDateTime::now_utc();
        let value = threshold - Duration::hours(1);
        let result = OffsetDateTimeBefore::new(value, threshold);
        assert!(result.is_ok());
    }

    #[test]
    fn test_offset_datetime_before_too_late() {
        let threshold = OffsetDateTime::now_utc();
        let value = threshold + Duration::hours(1);
        let result = OffsetDateTimeBefore::new(value, threshold);
        assert!(result.is_err());
    }
}

// ── ElicitIntrospect impls ────────────────────────────────────────────────────

#[cfg(feature = "chrono")]
macro_rules! impl_primitive_introspect_datetime {
    ($($ty:ty => $name:literal),+ $(,)?) => {
        $(
            impl crate::ElicitIntrospect for $ty {
                fn pattern() -> crate::ElicitationPattern {
                    crate::ElicitationPattern::Primitive
                }
                fn metadata() -> crate::TypeMetadata {
                    crate::TypeMetadata {
                        type_name: $name,
                        description: <$ty as crate::Prompt>::prompt(),
                        details: crate::PatternDetails::Primitive,
                    }
                }
            }
        )+
    };
}

#[cfg(all(feature = "chrono", not(kani)))]
impl_primitive_introspect_datetime!(
    DateTimeUtcAfter => "DateTimeUtcAfter",
);

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for OffsetDateTimeAfter {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "OffsetDateTimeAfter",
            description: <OffsetDateTimeAfter as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitIntrospect for OffsetDateTimeBefore {
    fn pattern() -> crate::ElicitationPattern {
        crate::ElicitationPattern::Primitive
    }

    fn metadata() -> crate::TypeMetadata {
        crate::TypeMetadata {
            type_name: "OffsetDateTimeBefore",
            description: <OffsetDateTimeBefore as crate::Prompt>::prompt(),
            details: crate::PatternDetails::Primitive,
        }
    }
}

// ── ToCodeLiteral impls ───────────────────────────────────────────────────────

#[cfg(feature = "chrono")]
mod emit_impls {
    use super::*;
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for DateTimeUtcAfter {
        fn to_code_literal(&self) -> TokenStream {
            let value_rfc3339 = self.get().to_rfc3339();
            let threshold_rfc3339 = self.threshold().to_rfc3339();
            quote::quote! {
                elicitation::DateTimeUtcAfter::new(
                    ::chrono::DateTime::parse_from_rfc3339(#value_rfc3339)?
                        .with_timezone(&::chrono::Utc),
                    ::chrono::DateTime::parse_from_rfc3339(#threshold_rfc3339)?
                        .with_timezone(&::chrono::Utc),
                )?
            }
        }
    }
}

#[cfg(all(feature = "time", not(kani)))]
mod time_emit_impls {
    use super::*;
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    // Extract an OffsetDateTime as its two infallible numeric projections and
    // emit generated code that reconstructs it using ? for error propagation.
    // unix_timestamp_nanos() -> i128 and offset().whole_seconds() -> i32 are
    // both infallible; from_unix_timestamp_nanos and from_whole_seconds return
    // Result and the generated code propagates those with ?.
    fn offset_dt_tokens(dt: time::OffsetDateTime) -> TokenStream {
        let nanos: i128 = dt.unix_timestamp_nanos();
        let offset_secs: i32 = dt.offset().whole_seconds();
        quote::quote! {
            ::time::OffsetDateTime::from_unix_timestamp_nanos(#nanos)?
                .to_offset(::time::UtcOffset::from_whole_seconds(#offset_secs)?)
        }
    }

    impl ToCodeLiteral for OffsetDateTimeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let dt = offset_dt_tokens(self.0);
            quote::quote! { elicitation::OffsetDateTimeWrap(#dt) }
        }
    }

    impl ToCodeLiteral for PrimitiveDateTimeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let year: i32 = self.0.year();
            let month: u8 = u8::from(self.0.month());
            let day: u8 = self.0.day();
            let hour: u8 = self.0.hour();
            let minute: u8 = self.0.minute();
            let second: u8 = self.0.second();
            let nanosecond: u32 = self.0.nanosecond();
            quote::quote! {
                elicitation::PrimitiveDateTimeWrap(::time::PrimitiveDateTime::new(
                    ::time::Date::from_calendar_date(
                        #year,
                        ::time::Month::try_from(#month)?,
                        #day,
                    )?,
                    ::time::Time::from_hms_nano(#hour, #minute, #second, #nanosecond)?,
                ))
            }
        }
    }

    impl ToCodeLiteral for TimeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let h: u8 = self.0.hour();
            let m: u8 = self.0.minute();
            let s: u8 = self.0.second();
            let ns: u32 = self.0.nanosecond();
            quote::quote! {
                elicitation::TimeWrap(::time::Time::from_hms_nano(#h, #m, #s, #ns)?)
            }
        }
    }

    impl ToCodeLiteral for DateWrap {
        fn to_code_literal(&self) -> TokenStream {
            let year: i32 = self.0.year();
            let month: u8 = u8::from(self.0.month());
            let day: u8 = self.0.day();
            quote::quote! {
                elicitation::DateWrap(
                    ::time::Date::from_calendar_date(#year, ::time::Month::try_from(#month)?, #day)?
                )
            }
        }
    }

    impl ToCodeLiteral for DurationWrap {
        fn to_code_literal(&self) -> TokenStream {
            let seconds: i64 = self.0.whole_seconds();
            let nanoseconds: i32 = self.0.subsec_nanoseconds();
            quote::quote! {
                elicitation::DurationWrap(::time::Duration::new(#seconds, #nanoseconds))
            }
        }
    }

    impl ToCodeLiteral for UtcDateTimeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let nanos: i128 = self.0.unix_timestamp_nanos();
            quote::quote! {
                elicitation::UtcDateTimeWrap(
                    ::time::UtcDateTime::from_unix_timestamp_nanos(#nanos)?
                )
            }
        }
    }

    impl ToCodeLiteral for UtcOffsetWrap {
        fn to_code_literal(&self) -> TokenStream {
            let seconds: i32 = self.0.whole_seconds();
            quote::quote! {
                elicitation::UtcOffsetWrap(::time::UtcOffset::from_whole_seconds(#seconds)?)
            }
        }
    }

    impl ToCodeLiteral for WeekdayWrap {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match self.0 {
                time::Weekday::Monday => "Monday",
                time::Weekday::Tuesday => "Tuesday",
                time::Weekday::Wednesday => "Wednesday",
                time::Weekday::Thursday => "Thursday",
                time::Weekday::Friday => "Friday",
                time::Weekday::Saturday => "Saturday",
                time::Weekday::Sunday => "Sunday",
            };
            let ident = proc_macro2::Ident::new(variant, proc_macro2::Span::call_site());
            quote::quote! { elicitation::WeekdayWrap(::time::Weekday::#ident) }
        }
    }

    impl ToCodeLiteral for MonthWrap {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match self.0 {
                time::Month::January => "January",
                time::Month::February => "February",
                time::Month::March => "March",
                time::Month::April => "April",
                time::Month::May => "May",
                time::Month::June => "June",
                time::Month::July => "July",
                time::Month::August => "August",
                time::Month::September => "September",
                time::Month::October => "October",
                time::Month::November => "November",
                time::Month::December => "December",
            };
            let ident = proc_macro2::Ident::new(variant, proc_macro2::Span::call_site());
            quote::quote! { elicitation::MonthWrap(::time::Month::#ident) }
        }
    }

    impl ToCodeLiteral for ComponentRangeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::ComponentRangeWrap(#inner) }
        }
    }

    impl ToCodeLiteral for ConversionRangeWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::ConversionRangeWrap(#inner) }
        }
    }

    impl ToCodeLiteral for DifferentVariantWrap {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { elicitation::DifferentVariantWrap(::time::error::DifferentVariant) }
        }
    }

    impl ToCodeLiteral for FmtDayWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::FmtDayWrap(#inner) }
        }
    }

    impl ToCodeLiteral for FmtEndWrap {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! {
                elicitation::FmtEndWrap(::time::format_description::modifier::End::default())
            }
        }
    }

    impl ToCodeLiteral for FmtTrailingInputWrap {
        fn to_code_literal(&self) -> TokenStream {
            use time::format_description::modifier::TrailingInput;
            match self.0 {
                TrailingInput::Prohibit => quote::quote! {
                    elicitation::FmtTrailingInputWrap(
                        ::time::format_description::modifier::TrailingInput::Prohibit,
                    )
                },
                TrailingInput::Discard => quote::quote! {
                    elicitation::FmtTrailingInputWrap(
                        ::time::format_description::modifier::TrailingInput::Discard,
                    )
                },
            }
        }
    }

    impl ToCodeLiteral for FmtPaddingWrap {
        fn to_code_literal(&self) -> TokenStream {
            use time::format_description::modifier::Padding;
            match self.0 {
                Padding::Space => quote::quote! {
                    elicitation::FmtPaddingWrap(
                        ::time::format_description::modifier::Padding::Space,
                    )
                },
                Padding::Zero => quote::quote! {
                    elicitation::FmtPaddingWrap(
                        ::time::format_description::modifier::Padding::Zero,
                    )
                },
                Padding::None => quote::quote! {
                    elicitation::FmtPaddingWrap(
                        ::time::format_description::modifier::Padding::None,
                    )
                },
                _ => quote::quote! {
                    elicitation::FmtPaddingWrap(
                        ::time::format_description::modifier::Padding::None,
                    )
                },
            }
        }
    }

    impl ToCodeLiteral for FmtIgnoreWrap {
        fn to_code_literal(&self) -> TokenStream {
            let count: u16 = self.0.count.get();
            quote::quote! {
                elicitation::FmtIgnoreWrap(
                    ::time::format_description::modifier::Ignore::count(
                        match ::core::num::NonZero::<u16>::new(#count) {
                            Some(n) => n,
                            None => ::core::num::NonZero::<u16>::MIN,
                        }
                    )
                )
            }
        }
    }

    impl ToCodeLiteral for FmtMinuteWrap {
        fn to_code_literal(&self) -> TokenStream {
            use time::format_description::modifier::Padding;
            let padding_tokens = match self.0.padding {
                Padding::Space => {
                    quote::quote! { ::time::format_description::modifier::Padding::Space }
                }
                Padding::Zero => {
                    quote::quote! { ::time::format_description::modifier::Padding::Zero }
                }
                _ => {
                    quote::quote! { ::time::format_description::modifier::Padding::None }
                }
            };
            quote::quote! {
                elicitation::FmtMinuteWrap(
                    ::time::format_description::modifier::Minute::default()
                        .with_padding(#padding_tokens)
                )
            }
        }
    }

    macro_rules! padding_wrap_to_code_literal {
        ($wrap:ident, $inner:path) => {
            impl ToCodeLiteral for $wrap {
                fn to_code_literal(&self) -> TokenStream {
                    use time::format_description::modifier::Padding;
                    let padding_tokens = match self.0.padding {
                        Padding::Space => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Space
                            }
                        }
                        Padding::Zero => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Zero
                            }
                        }
                        _ => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::None
                            }
                        }
                    };
                    quote::quote! {
                        elicitation::$wrap(
                            <$inner>::default().with_padding(#padding_tokens)
                        )
                    }
                }
            }
        };
    }

    padding_wrap_to_code_literal!(
        FmtOrdinalWrap,
        ::time::format_description::modifier::Ordinal
    );
    padding_wrap_to_code_literal!(
        FmtSecondWrap,
        ::time::format_description::modifier::Second
    );
    padding_wrap_to_code_literal!(
        FmtOffsetMinuteWrap,
        ::time::format_description::modifier::OffsetMinute
    );
    padding_wrap_to_code_literal!(
        FmtOffsetSecondWrap,
        ::time::format_description::modifier::OffsetSecond
    );
    padding_wrap_to_code_literal!(
        FmtWeekNumberIsoWrap,
        ::time::format_description::modifier::WeekNumberIso
    );
    padding_wrap_to_code_literal!(
        FmtWeekNumberSundayWrap,
        ::time::format_description::modifier::WeekNumberSunday
    );
    padding_wrap_to_code_literal!(
        FmtWeekNumberMondayWrap,
        ::time::format_description::modifier::WeekNumberMonday
    );

    macro_rules! opaque_padding_wrap_literal {
        ($wrap:ident) => {
            impl ToCodeLiteral for $wrap {
                fn to_code_literal(&self) -> TokenStream {
                    use time::format_description::modifier::Padding;
                    let padding_tokens = match self.0 {
                        Padding::Space => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Space
                            }
                        }
                        Padding::Zero => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Zero
                            }
                        }
                        _ => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::None
                            }
                        }
                    };
                    quote::quote! {
                        elicitation::$wrap(#padding_tokens)
                    }
                }
            }
        };
    }

    opaque_padding_wrap_literal!(FmtHour12Wrap);
    opaque_padding_wrap_literal!(FmtHour24Wrap);
    opaque_padding_wrap_literal!(FmtMonthNumericalWrap);
    opaque_padding_wrap_literal!(FmtCalendarYearLastTwoWrap);
    opaque_padding_wrap_literal!(FmtIsoYearLastTwoWrap);

    // Opaque bool-field wrappers: emit Wrap(value).
    macro_rules! opaque_bool_wrap_literal {
        ($wrap:ident) => {
            impl ToCodeLiteral for $wrap {
                fn to_code_literal(&self) -> TokenStream {
                    let value = self.0;
                    quote::quote! { elicitation::$wrap(#value) }
                }
            }
        };
    }

    opaque_bool_wrap_literal!(FmtMonthShortWrap);
    opaque_bool_wrap_literal!(FmtMonthLongWrap);
    opaque_bool_wrap_literal!(FmtWeekdayShortWrap);
    opaque_bool_wrap_literal!(FmtWeekdayLongWrap);
    opaque_bool_wrap_literal!(FmtWeekdaySundayWrap);
    opaque_bool_wrap_literal!(FmtWeekdayMondayWrap);
    opaque_bool_wrap_literal!(FmtUnixTimestampSecondWrap);
    opaque_bool_wrap_literal!(FmtUnixTimestampMillisecondWrap);
    opaque_bool_wrap_literal!(FmtUnixTimestampMicrosecondWrap);
    opaque_bool_wrap_literal!(FmtUnixTimestampNanosecondWrap);

    // Opaque padding+sign wrappers: emit Wrap { padding, sign_is_mandatory }.
    macro_rules! opaque_padding_sign_wrap_literal {
        ($wrap:ident) => {
            impl ToCodeLiteral for $wrap {
                fn to_code_literal(&self) -> TokenStream {
                    use time::format_description::modifier::Padding;
                    let padding_tokens = match self.padding {
                        Padding::Space => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Space
                            }
                        }
                        Padding::Zero => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::Zero
                            }
                        }
                        _ => {
                            quote::quote! {
                                ::time::format_description::modifier::Padding::None
                            }
                        }
                    };
                    let sign = self.sign_is_mandatory;
                    quote::quote! {
                        elicitation::$wrap {
                            padding: #padding_tokens,
                            sign_is_mandatory: #sign,
                        }
                    }
                }
            }
        };
    }

    opaque_padding_sign_wrap_literal!(FmtCalendarYearFullExtendedRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtCalendarYearFullStandardRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtCalendarYearCenturyExtendedRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtCalendarYearCenturyStandardRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtIsoYearFullExtendedRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtIsoYearFullStandardRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtIsoYearCenturyExtendedRangeWrap);
    opaque_padding_sign_wrap_literal!(FmtIsoYearCenturyStandardRangeWrap);

    impl ToCodeLiteral for WellKnownRfc2822Wrap {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { elicitation::WellKnownRfc2822Wrap }
        }
    }

    impl ToCodeLiteral for FmtSubsecondDigitsWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::FmtSubsecondDigitsWrap(#inner) }
        }
    }

    impl ToCodeLiteral for FmtSubsecondWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::FmtSubsecondWrap(#inner) }
        }
    }

    impl ToCodeLiteral for FmtPeriodWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::FmtPeriodWrap(#inner) }
        }
    }

    impl ToCodeLiteral for FmtOffsetHourWrap {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.to_code_literal();
            quote::quote! { elicitation::FmtOffsetHourWrap(#inner) }
        }
    }

    impl ToCodeLiteral for OffsetDateTimeAfter {
        fn to_code_literal(&self) -> TokenStream {
            let value = offset_dt_tokens(self.get());
            let threshold = offset_dt_tokens(self.threshold());
            quote::quote! {
                elicitation::OffsetDateTimeAfter::new(#value, #threshold)?
            }
        }
    }

    impl ToCodeLiteral for OffsetDateTimeBefore {
        fn to_code_literal(&self) -> TokenStream {
            let value = offset_dt_tokens(self.get());
            let threshold = offset_dt_tokens(self.threshold());
            quote::quote! {
                elicitation::OffsetDateTimeBefore::new(#value, #threshold)?
            }
        }
    }
}

// ── ElicitComplete declarations ───────────────────────────────────────────────

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for OffsetDateTimeAfter {}

#[cfg(all(feature = "time", not(kani)))]
impl crate::ElicitComplete for OffsetDateTimeBefore {}
