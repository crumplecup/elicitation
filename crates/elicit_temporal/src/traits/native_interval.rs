//! Native higher-order interval traits.
//!
//! These traits let higher-order interval logic operate on backend-native
//! fixed-instant carriers instead of descriptor round-trips.

use crate::{
    NativeOrderedProvenOffsetEndpointsResult, ProvenOffsetDateTimeCarrier, TemporalInstantProps,
};

/// Native interval operations over backend fixed-instant carriers.
pub trait TemporalNativeIntervalFactory: TemporalInstantProps + Send + Sync {
    /// Confirm chronological ordering between two native fixed-instant endpoints.
    fn order_offset_endpoints_native(
        &self,
        start: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        end: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
    ) -> NativeOrderedProvenOffsetEndpointsResult;
}
