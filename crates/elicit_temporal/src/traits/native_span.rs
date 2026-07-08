//! Native descriptor/carrier bridges for temporal span families.
//!
//! These traits are narrower and more conditional than the instant and zone
//! bridges. A backend should implement them only when its upstream carrier
//! types retain enough ISO/CalConnect structure to lawfully re-issue the
//! relevant descriptor-side proof branches.

use crate::{
    DurationDescriptor, DurationSemanticBundle, ProvenDurationCarrier,
    ProvenRecurringIntervalCarrier, ProvenTimeIntervalCarrier, RealizedProvenDurationResult,
    RealizedProvenRecurringIntervalResult, RealizedProvenTimeIntervalResult,
    RecurringIntervalDescriptor, RecurringIntervalSemanticBundle, ReflectedProvenDurationResult,
    ReflectedProvenRecurringIntervalResult, ReflectedProvenTimeIntervalResult,
    TemporalDurationProps, TemporalRecurringIntervalProps, TemporalTimeIntervalProps,
    TimeIntervalDescriptor, TimeIntervalSemanticBundle,
};

/// Realize and reflect native duration carriers.
///
/// This trait is lawful only when the backend-native duration carrier preserves
/// enough nominal ISO duration structure for the caller to carry the duration
/// representation-family branch explicitly across the seam.
pub trait TemporalDurationNativeBridge: TemporalDurationProps + Send + Sync {
    /// Realize a validated duration descriptor as a proven backend-native carrier.
    fn realize_duration(
        &self,
        duration: &DurationDescriptor,
        semantics: &DurationSemanticBundle,
    ) -> RealizedProvenDurationResult<Self::Duration>;

    /// Reflect a proven backend-native duration carrier into the neutral descriptor accord.
    fn reflect_duration(
        &self,
        duration: &ProvenDurationCarrier<Self::Duration>,
    ) -> ReflectedProvenDurationResult;
}

/// Realize and reflect native interval carriers.
pub trait TemporalTimeIntervalNativeBridge: TemporalTimeIntervalProps + Send + Sync {
    /// Realize a validated interval descriptor as a proven backend-native carrier.
    fn realize_time_interval(
        &self,
        interval: &TimeIntervalDescriptor,
        semantics: &TimeIntervalSemanticBundle,
    ) -> RealizedProvenTimeIntervalResult<Self::TimeInterval>;

    /// Reflect a proven backend-native interval carrier into the neutral descriptor accord.
    fn reflect_time_interval(
        &self,
        interval: &ProvenTimeIntervalCarrier<Self::TimeInterval>,
    ) -> ReflectedProvenTimeIntervalResult;
}

/// Realize and reflect native recurring-interval carriers.
pub trait TemporalRecurringIntervalNativeBridge:
    TemporalRecurringIntervalProps + Send + Sync
{
    /// Realize a validated recurring-interval descriptor as a proven backend-native carrier.
    fn realize_recurring_interval(
        &self,
        recurring: &RecurringIntervalDescriptor,
        semantics: &RecurringIntervalSemanticBundle,
    ) -> RealizedProvenRecurringIntervalResult<Self::RecurringInterval>;

    /// Reflect a proven backend-native recurring-interval carrier into the neutral descriptor accord.
    fn reflect_recurring_interval(
        &self,
        recurring: &ProvenRecurringIntervalCarrier<Self::RecurringInterval>,
    ) -> ReflectedProvenRecurringIntervalResult;
}

/// Aggregate native span bridge for backends with lawful duration, interval, and recurrence carriers.
pub trait TemporalNativeSpanBridge:
    TemporalDurationNativeBridge
    + TemporalTimeIntervalNativeBridge
    + TemporalRecurringIntervalNativeBridge
{
}

impl<T> TemporalNativeSpanBridge for T where
    T: TemporalDurationNativeBridge
        + TemporalTimeIntervalNativeBridge
        + TemporalRecurringIntervalNativeBridge
{
}
