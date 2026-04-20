//! Shadow types for `surrealdb::opt` configuration types.
//!
//! `surrealdb::opt::Config` has all-private fields, so we shadow it with our own
//! field-bearing struct and use the builder API in connection tools.

use elicitation::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Query planner strategy.
///
/// Maps to `surrealdb::opt::PlannerStrategy`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
pub enum PlannerStrategy {
    /// Attempt to use an index if available; fall back to full-table scan.
    Default,
    /// Force full-table scan regardless of indexes.
    FullTableScan,
    /// Force index-based scan; error if no index is available.
    IndexedScan,
}

/// Experimental features that can be enabled on a SurrealDB connection.
///
/// Maps to `surrealdb::opt::ExperimentalFeature`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
pub enum ExperimentalFeature {
    /// File-storage backend experiment.
    Files,
    /// Surrealism framework experiment.
    Surrealism,
}

/// Connection capabilities configuration.
///
/// Shadow of `surrealdb::opt::Capabilities`. Because the upstream type uses a builder-method
/// API to configure allow-lists, this shadow captures the concepts as optional boolean fields.
/// Connection tools use these fields to emit the appropriate Rust builder calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct Capabilities {
    /// Allow all scripting functions.  Default: `false`.
    #[serde(default)]
    pub allow_scripting: bool,
    /// Allow guest access (unauthenticated queries).  Default: `false`.
    #[serde(default)]
    pub allow_guests: bool,
    /// Allow all network targets. Default: `false`.
    #[serde(default)]
    pub allow_all_net: bool,
    /// Allow all SurrealQL functions.  Default: `false`.
    #[serde(default)]
    pub allow_all_functions: bool,
}

/// Client connection configuration.
///
/// Shadow of `surrealdb::opt::Config` (whose fields are all `pub(crate)`).
/// Connection tools read this struct and emit the corresponding Rust `Config` builder calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct Config {
    /// Optional request/query timeout in seconds.
    #[serde(default)]
    pub query_timeout_secs: Option<u64>,
    /// Optional connection/transaction timeout in seconds.
    #[serde(default)]
    pub transaction_timeout_secs: Option<u64>,
    /// Capability settings.
    #[serde(default)]
    pub capabilities: Option<Capabilities>,
    /// Experimental features to enable.
    #[serde(default)]
    pub experimental: Vec<ExperimentalFeature>,
    /// Query planner strategy.
    #[serde(default)]
    pub planner_strategy: Option<PlannerStrategy>,
}
