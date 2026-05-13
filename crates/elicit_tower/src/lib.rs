//! Elicitation-enabled tower shadow crate.
//!
//! Provides MCP tools for tower middleware — concurrency/rate limiting,
//! retry policies, tower-http request/response layers, utility layers,
//! service builders, load balancing, and service routing. All config objects
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
//! | [`TowerUtilPlugin`] | `tower_util__*` | MapErr/Request/Response/Result, AndThen, Then layers, BoxService configs |
//! | [`TowerBuilderPlugin`] | `tower_builder__*` | Incremental tower ServiceBuilder composition |
//! | [`TowerBalancePlugin`] | `tower_balance__*` | p2c Balance, PeakEwma, PendingRequests load estimators |
//! | [`TowerSteerPlugin`] | `tower_steer__*` | Steer service routing |
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod balance;
mod builder;
mod http;
mod limit;
mod retry;
mod steer;
mod util;

pub use balance::{
    BalanceDescribeParams, BalanceIdResult, LoadIdResult, P2cNewParams, PeakEwmaNewParams,
    PendingRequestsNewParams, TowerBalanceCreated, TowerBalancePlugin, TowerLoadCreated,
};
pub use builder::{
    BuilderAddLayerParams, BuilderBuildParams, BuilderDescribeParams, BuilderIdResult,
    BuilderNewParams, TowerBuilderPlugin, TowerServiceBuilderCreated, TowerServiceBuilderDone,
    TowerServiceBuilderLayerAdded,
};
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
pub use steer::{
    SteerDescribeParams, SteerIdResult, SteerNewParams, TowerSteerCreated, TowerSteerPlugin,
};
pub use util::{
    AndThenLayerNewParams, BoxCloneServiceNewParams, BoxServiceDescribeParams, BoxServiceNewParams,
    MapErrLayerNewParams, MapRequestLayerNewParams, MapResponseLayerNewParams,
    MapResultLayerNewParams, ServiceIdResult, ThenLayerNewParams, TowerBoxServiceCreated,
    TowerUtilLayerCreated, TowerUtilPlugin, UtilLayerDescribeParams, UtilLayerIdResult,
};
