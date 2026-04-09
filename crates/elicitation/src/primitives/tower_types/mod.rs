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
mod errors;
mod handles;
mod http_handles;
mod http_layers;
mod layers;
mod rate;

pub use backoff::{TowerExponentialBackoffMaker, TowerTpsBudget};
pub use errors::{TowerClosed, TowerElapsed, TowerOverloaded, TowerServiceError};
pub use handles::{
    TowerAndThenHandle, TowerBoxServiceHandle, TowerBufferHandle, TowerConcurrencyLimitHandle,
    TowerFilterHandle, TowerLoadShedHandle, TowerMapErrHandle, TowerMapRequestHandle,
    TowerMapResponseHandle, TowerRateLimitHandle, TowerRetryHandle, TowerServiceBuilderHandle,
    TowerThenHandle, TowerTimeoutHandle,
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
pub use rate::TowerRate;
