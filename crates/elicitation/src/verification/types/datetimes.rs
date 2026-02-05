//! DateTime contract types for chrono, time, and jiff libraries.
//!
//! Available with the `chrono`, `time`, or `jiff` features.

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use super::ValidationError;

#[cfg(all(not(kani), any(feature = "chrono", feature = "jiff", feature = "time")))]
use crate::{ElicitClient, ElicitCommunicator, ElicitResult, Elicitation, Prompt};

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use elicitation_macros::instrumented_impl;

#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};

// DateTimeUtcAfter - DateTime<Utc> after a threshold
/// A DateTime<Utc> that is guaranteed to be after a threshold time.
///
/// Available with the `chrono` feature.
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Unwrap into the inner DateTime<Utc>.
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
}

// DateTimeUtcBefore - DateTime<Utc> before a threshold
/// A DateTime<Utc> that is guaranteed to be before a threshold time.
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

    /// Unwrap into the inner DateTime<Utc>.
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
use time::OffsetDateTime;

// OffsetDateTimeAfter - OffsetDateTime after a threshold
/// An OffsetDateTime that is guaranteed to be after a threshold time.
///
/// Available with the `time` feature.
#[cfg(feature = "time")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct OffsetDateTimeAfter {
    value: OffsetDateTime,
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
}

// OffsetDateTimeBefore - OffsetDateTime before a threshold
/// An OffsetDateTime that is guaranteed to be before a threshold time.
///
/// Available with the `time` feature.
#[cfg(feature = "time")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(not(kani))]
pub struct OffsetDateTimeBefore {
    value: OffsetDateTime,
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
