//! Elicitation implementations for [`tower`] and [`tower_http`] middleware types.
//!
//! Provides serializable mirror structs and [`ElicitComplete`](crate::ElicitComplete)
//! impls for all public tower 0.5 and tower-http 0.6 types, organized into:
//!
//! - [`rate`] — `TowerRate` (rate limit config)
//! - [`layers`] — config layer types (timeout, concurrency, buffer, load-shed, etc.)
//! - [`backoff`] — retry backoff types (`TowerExponentialBackoffMaker`, `TowerTpsBudget`)
//! - [`handles`] — UUID handle newtypes for runtime non-serializable services
//! - [`errors`] — tower error types (`TowerElapsed`, `TowerOverloaded`, etc.)
//! - [`http_layers`] — tower-http config layer types
//! - [`http_handles`] — UUID handle newtypes for tower-http service wrappers
//!
//! # Enabled by the `tower-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["tower-types"] }
//! ```

mod backoff;
mod builder_types;
mod errors;
mod handles;
mod http_handles;
mod http_layers;
mod layers;
mod load_types;
mod rate;
mod util_layers;

pub use backoff::{TowerExponentialBackoffMaker, TowerTpsBudget};
pub use builder_types::{
    TowerLayerKind, TowerLayerKindStyle, TowerServiceBuilder, TowerServiceBuilderStyle,
};
pub use errors::{TowerClosed, TowerElapsed, TowerOverloaded, TowerServiceError};
pub use handles::{
    TowerAndThenHandle, TowerBalanceHandle, TowerBoxCloneServiceHandle, TowerBoxServiceHandle,
    TowerBufferHandle, TowerConcurrencyLimitHandle, TowerFilterHandle, TowerLoadShedHandle,
    TowerMapErrHandle, TowerMapRequestHandle, TowerMapResponseHandle, TowerMapResultHandle,
    TowerPeakEwmaHandle, TowerPendingRequestsHandle, TowerRateLimitHandle, TowerRetryHandle,
    TowerServiceBuilderHandle, TowerSteerHandle, TowerThenHandle, TowerTimeoutHandle,
};
pub use http_handles::TowerHttpServiceHandle;
pub use http_layers::{
    TowerCatchPanicLayer, TowerCompressionLayer, TowerCorsLayer, TowerDecompressionLayer,
    TowerHttpTimeoutLayer, TowerNormalizePathLayer, TowerPropagateHeaderLayer,
    TowerSetRequestHeaderLayer, TowerSetResponseHeaderLayer, TowerSetSensitiveRequestHeadersLayer,
    TowerSetSensitiveResponseHeadersLayer, TowerSetStatusLayer, TowerTraceLayer,
    TowerValidateRequestHeaderLayer,
};
pub use layers::{
    TowerBufferLayer, TowerConcurrencyLimitLayer, TowerFilterLayer, TowerLoadShedLayer,
    TowerRateLimitLayer, TowerRetryLayer, TowerSpawnReadyLayer, TowerTimeoutLayer,
};
pub use load_types::{TowerBalance, TowerPeakEwma, TowerPendingRequests, TowerSteer};
pub use rate::TowerRate;
pub use util_layers::{
    TowerAndThenLayer, TowerBoxCloneServiceConfig, TowerBoxServiceConfig, TowerMapErrLayer,
    TowerMapRequestLayer, TowerMapResponseLayer, TowerMapResultLayer, TowerThenLayer,
};
