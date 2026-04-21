//! Shadow types for SurrealDB capabilities and connection configuration.
//!
//! Mirrors `surrealdb::opt::capabilities` and `surrealdb::opt::config`.
//! The upstream types use all-private fields behind a builder API;
//! these shadow types model the publicly configurable parameters and
//! implement [`elicitation::ElicitComplete`] so user code can
//! `#[derive(Elicit)]` with these as fields.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Strategy for the streaming query planner.
///
/// Mirrors `surrealdb::opt::capabilities::PlannerStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum PlannerStrategy {
    /// Try the new planner for read-only statements; fall back to compute on
    /// unimplemented features.
    BestEffort,
    /// Skip the new planner entirely; always use the compute executor.
    ComputeOnly,
    /// Require the new planner for all read-only statements; unimplemented
    /// features become hard errors.
    AllReadOnly,
}

/// Experimental features that can be enabled in capabilities.
///
/// Mirrors `surrealdb::opt::capabilities::ExperimentalFeature`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ExperimentalFeature {
    /// Enable the Files feature.
    Files,
    /// Enable the Surrealism feature.
    Surrealism,
}

/// Capabilities configuration for a SurrealDB database instance.
///
/// Models the publicly configurable surface of
/// `surrealdb::opt::capabilities::Capabilities`.
///
/// # Allow/deny lists
///
/// `allow_functions` / `allow_net`: `None` means allow all; `Some(vec![])`
/// means allow none; `Some(patterns)` matches only those patterns.
/// `deny_functions` / `deny_net` are explicit deny lists.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SurrealCapabilities {
    /// Whether users can execute scripts.
    pub scripting: bool,
    /// Whether unauthenticated users can execute queries.
    pub guest_access: bool,
    /// Allow-list for functions (`None` = all allowed, patterns like `"http::*"`).
    pub allow_functions: Option<Vec<String>>,
    /// Deny-list for functions (overrides the allow-list).
    pub deny_functions: Vec<String>,
    /// Allow-list for network targets (`None` = all allowed).
    pub allow_net: Option<Vec<String>>,
    /// Deny-list for network targets.
    pub deny_net: Vec<String>,
    /// Experimental features to enable.
    pub experimental: Vec<ExperimentalFeature>,
}

/// Connection configuration for a SurrealDB client.
///
/// Models the publicly configurable surface of `surrealdb::opt::config::Config`.
/// All timeout durations are expressed as seconds for JSON-schema friendliness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct SurrealConfig {
    /// Query timeout in seconds (`None` = no timeout).
    pub query_timeout_secs: Option<u64>,
    /// Transaction timeout in seconds (`None` = no timeout).
    pub transaction_timeout_secs: Option<u64>,
    /// Capabilities for the database.
    pub capabilities: SurrealCapabilities,
    /// Optional streaming query planner strategy.
    pub planner_strategy: Option<PlannerStrategy>,
    /// Whether to send queries as AST payload.
    pub ast_payload: bool,
}
