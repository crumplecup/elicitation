//! DateTime contract types for chrono, time, and jiff libraries.
//!
//! Available with the `chrono`, `time`, or `jiff` features.

#[cfg(feature = "chrono")]
use super::ValidationError;
#[cfg(feature = "chrono")]
use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
#[cfg(feature = "chrono")]
use chrono::{DateTime, NaiveDateTime, Utc};
#[cfg(feature = "chrono")]
use elicitation_macros::instrumented_impl;

// DateTimeUtcAfter - DateTime<Utc> after a threshold
/// A DateTime<Utc> that is guaranteed to be after a threshold time.
///
/// Available with the `chrono` feature.
#[cfg(feature = "chrono")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeUtcAfter {
    value: DateTime<Utc>,
    threshold: DateTime<Utc>,
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl DateTimeUtcAfter {
    /// Create a new DateTimeUtcAfter, validating value > threshold.
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

    /// Get the datetime value.
    pub fn get(&self) -> DateTime<Utc> {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> DateTime<Utc> {
        self.threshold
    }

    /// Unwrap into the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.value
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Prompt for DateTimeUtcAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (UTC):")
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Elicitation for DateTimeUtcAfter {
    type Style = <DateTime<Utc> as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTimeUtcAfter");
        // Default threshold to Unix epoch
        let threshold = DateTime::UNIX_EPOCH;
        loop {
            let dt = DateTime::<Utc>::elicit(client).await?;
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
pub struct DateTimeUtcBefore {
    value: DateTime<Utc>,
    threshold: DateTime<Utc>,
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl DateTimeUtcBefore {
    /// Create a new DateTimeUtcBefore, validating value < threshold.
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

    /// Get the datetime value.
    pub fn get(&self) -> DateTime<Utc> {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> DateTime<Utc> {
        self.threshold
    }

    /// Unwrap into the inner DateTime<Utc>.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.value
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Prompt for DateTimeUtcBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (UTC):")
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Elicitation for DateTimeUtcBefore {
    type Style = <DateTime<Utc> as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DateTimeUtcBefore");
        // Default threshold to now
        let threshold = Utc::now();
        loop {
            let dt = DateTime::<Utc>::elicit(client).await?;
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
pub struct NaiveDateTimeAfter {
    value: NaiveDateTime,
    threshold: NaiveDateTime,
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl NaiveDateTimeAfter {
    /// Create a new NaiveDateTimeAfter, validating value > threshold.
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

    /// Get the datetime value.
    pub fn get(&self) -> NaiveDateTime {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> NaiveDateTime {
        self.threshold
    }

    /// Unwrap into the inner NaiveDateTime.
    pub fn into_inner(self) -> NaiveDateTime {
        self.value
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Prompt for NaiveDateTimeAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime (no timezone):")
    }
}

#[cfg(feature = "chrono")]
#[instrumented_impl]
impl Elicitation for NaiveDateTimeAfter {
    type Style = <NaiveDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting NaiveDateTimeAfter");
        // Default threshold to Unix epoch
        let threshold = DateTime::<Utc>::UNIX_EPOCH.naive_utc();
        loop {
            let dt = NaiveDateTime::elicit(client).await?;
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
pub struct TimestampAfter {
    value: Timestamp,
    threshold: Timestamp,
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl TimestampAfter {
    /// Create a new TimestampAfter, validating value > threshold.
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

    /// Get the timestamp value.
    pub fn get(&self) -> Timestamp {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> Timestamp {
        self.threshold
    }

    /// Unwrap into the inner Timestamp.
    pub fn into_inner(self) -> Timestamp {
        self.value
    }
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl Prompt for TimestampAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a timestamp:")
    }
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl Elicitation for TimestampAfter {
    type Style = <Timestamp as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimestampAfter");
        // Default threshold to Unix epoch
        let threshold = Timestamp::UNIX_EPOCH;
        loop {
            let ts = Timestamp::elicit(client).await?;
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
pub struct TimestampBefore {
    value: Timestamp,
    threshold: Timestamp,
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl TimestampBefore {
    /// Create a new TimestampBefore, validating value < threshold.
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

    /// Get the timestamp value.
    pub fn get(&self) -> Timestamp {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> Timestamp {
        self.threshold
    }

    /// Unwrap into the inner Timestamp.
    pub fn into_inner(self) -> Timestamp {
        self.value
    }
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl Prompt for TimestampBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a timestamp:")
    }
}

#[cfg(feature = "jiff")]
#[instrumented_impl]
impl Elicitation for TimestampBefore {
    type Style = <Timestamp as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TimestampBefore");
        // Default threshold to now
        let threshold = Timestamp::now();
        loop {
            let ts = Timestamp::elicit(client).await?;
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
pub struct OffsetDateTimeAfter {
    value: OffsetDateTime,
    threshold: OffsetDateTime,
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl OffsetDateTimeAfter {
    /// Create a new OffsetDateTimeAfter, validating value > threshold.
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

    /// Get the datetime value.
    pub fn get(&self) -> OffsetDateTime {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> OffsetDateTime {
        self.threshold
    }

    /// Unwrap into the inner OffsetDateTime.
    pub fn into_inner(self) -> OffsetDateTime {
        self.value
    }
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl Prompt for OffsetDateTimeAfter {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime with timezone offset:")
    }
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl Elicitation for OffsetDateTimeAfter {
    type Style = <OffsetDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTimeAfter");
        // Default threshold to Unix epoch
        let threshold = OffsetDateTime::UNIX_EPOCH;
        loop {
            let dt = OffsetDateTime::elicit(client).await?;
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
pub struct OffsetDateTimeBefore {
    value: OffsetDateTime,
    threshold: OffsetDateTime,
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl OffsetDateTimeBefore {
    /// Create a new OffsetDateTimeBefore, validating value < threshold.
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

    /// Get the datetime value.
    pub fn get(&self) -> OffsetDateTime {
        self.value
    }

    /// Get the threshold.
    pub fn threshold(&self) -> OffsetDateTime {
        self.threshold
    }

    /// Unwrap into the inner OffsetDateTime.
    pub fn into_inner(self) -> OffsetDateTime {
        self.value
    }
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl Prompt for OffsetDateTimeBefore {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a datetime with timezone offset:")
    }
}

#[cfg(feature = "time")]
#[instrumented_impl]
impl Elicitation for OffsetDateTimeBefore {
    type Style = <OffsetDateTime as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting OffsetDateTimeBefore");
        // Default threshold to now
        let threshold = OffsetDateTime::now_utc();
        loop {
            let dt = OffsetDateTime::elicit(client).await?;
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
