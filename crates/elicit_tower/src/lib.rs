//! Elicitation-enabled tower shadow crate.
//!
//! Provides MCP tools for tower middleware — concurrency/rate limiting,
//! retry policies, and tower-http request/response layers. All config objects
//! are stored server-side in UUID-keyed registries; only serializable handles
//! (UUIDs, primitive config values) cross the MCP boundary.
//!
//! # Shadow-crate workflow plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`TowerLimitPlugin`] | `tower_limit__*` | Concurrency, rate-limit, timeout, buffer, load-shed, spawn-ready layers |
//! | [`TowerRetryPlugin`] | `tower_retry__*` | Exponential backoff, TPS budget, retry and filter layers |
//! | [`TowerHttpPlugin`] | `tower_http__*` | tower-http middleware layers (compression, CORS, tracing, headers, etc.) |
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod http;
mod limit;
mod retry;

pub use http::{
    CompressionLayerNewParams, CorsLayerNewParams, DecompressionLayerNewParams,
    HttpLayerDescribeParams, HttpTimeoutLayerNewParams, NormalizePathLayerNewParams,
    PropagateHeaderLayerNewParams, SensitiveRequestHeadersLayerNewParams,
    SensitiveResponseHeadersLayerNewParams, SetRequestHeaderLayerNewParams,
    SetResponseHeaderLayerNewParams, SetStatusLayerNewParams, TowerHttpLayerCreated,
    TowerHttpPlugin, ValidateRequestHeaderLayerNewParams,
};
pub use limit::{
    BufferLayerNewParams, ConcurrencyLimitLayerNewParams, LayerDescribeParams, LayerIdResult,
    LoadShedLayerNewParams, RateLimitLayerNewParams, RateNewParams, SpawnReadyLayerNewParams,
    TimeoutLayerNewParams, TowerLayerCreated, TowerLimitPlugin, TowerRateCreated,
};
pub use retry::{
    BackoffDescribeParams, BackoffNewParams, BudgetDescribeParams, BudgetNewParams,
    FilterLayerNewParams, RetryLayerNewParams, TowerBackoffCreated, TowerBudgetCreated,
    TowerRetryLayerCreated, TowerRetryPlugin,
};
