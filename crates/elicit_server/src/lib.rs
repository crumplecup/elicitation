//! Cross-crate workflow plugins for elicitation.
//!
//! `elicit_server` houses workflows that require visibility across multiple
//! elicitation crates simultaneously — things that can't live in `elicit_reqwest`
//! or `elicit_serde_json` without creating circular dependencies.
//!
//! # Plugins
//!
//! - [`EmitBinaryPlugin`] — recover agent tool compositions as compiled Rust
//!   binaries (requires `feature = "emit"`)
//! - [`SecureFetchPlugin`] — HTTPS-enforced URL validation + HTTP fetch
//!   (`elicit_url` + `elicit_reqwest`)
//! - [`FetchAndParsePlugin`] — HTTP fetch + JSON extraction
//!   (`elicit_reqwest` + `elicit_serde_json`)
//!
//! # Feature flags
//!
//! | Feature | Enables |
//! |---|---|
//! | `emit` | `EmitBinaryPlugin` + full code recovery pipeline |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod fetch_and_parse;
mod secure_fetch;
mod util;

#[cfg(feature = "emit")]
mod emit_plugin;

pub use fetch_and_parse::FetchAndParsePlugin;
pub use secure_fetch::SecureFetchPlugin;

#[cfg(feature = "emit")]
pub use emit_plugin::{EmitBinaryParams, EmitBinaryPlugin, WorkflowStep};

/// Look up a tool by name and deserialize its params, drawing from all
/// `elicit_server` handlers and its dep crates registered via `#[elicit_tool]`.
///
/// This is a thin wrapper around [`elicitation::emit_code::dispatch_emit`]
/// that anchors the linker to include every handler module in this crate and
/// its workflow dependencies, ensuring their `register_emit!` constructors are
/// present in the final binary.  Integration tests must call this function
/// (not the bare `dispatch_emit`) so the linker does not discard unreferenced
/// CGUs.
#[cfg(feature = "emit")]
pub fn emit_dispatch(
    tool: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    emit_dispatch_crate(tool, "", params)
}

/// Dispatch to a specific crate's emit registration by crate name.
///
/// Use this when multiple crates register the same tool name (e.g.
/// `"assert_future"` in `elicit_chrono`, `elicit_jiff`, `elicit_time`).
#[cfg(feature = "emit")]
pub fn emit_dispatch_crate(
    tool: &str,
    crate_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn elicitation::emit_code::EmitCode>, String> {
    // Each size_of call references a params type from a handler module,
    // pulling that CGU (and its register_emit! CTORs) into the link.
    let _ = [
        // elicit_server
        std::mem::size_of::<secure_fetch::SecureFetchParams>(),
        std::mem::size_of::<secure_fetch::ValidatedApiCallParams>(),
        std::mem::size_of::<fetch_and_parse::FetchAndExtractParams>(),
        std::mem::size_of::<fetch_and_parse::FetchAndValidateParams>(),
        // elicit_url
        std::mem::size_of::<elicit_url::ParseUrlParams>(),
        // elicit_reqwest
        std::mem::size_of::<elicit_reqwest::BuildRequestParams>(),
        // elicit_chrono
        std::mem::size_of::<elicit_chrono::ParseDateTimeParams>(),
        // elicit_jiff
        std::mem::size_of::<elicit_jiff::ParseTimestampParams>(),
        // elicit_time
        std::mem::size_of::<elicit_time::ParseOffsetParams>(),
        // elicit_serde_json
        std::mem::size_of::<elicit_serde_json::RawJson>(),
    ];
    if crate_name.is_empty() {
        elicitation::emit_code::dispatch_emit(tool, params)
    } else {
        elicitation::emit_code::dispatch_emit_from(tool, crate_name, params)
    }
}
