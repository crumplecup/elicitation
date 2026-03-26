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
//! # Shadow-crate workflow plugins (registered via emit inventory)
//!
//! The following plugins live in their own crates but are included in the emit
//! dispatch pipeline when the `emit` feature is enabled:
//!
//! | Plugin | Crate | Tools |
//! |---|---|---|
//! | `ChronoWorkflowPlugin` | `elicit_chrono` | parse_datetime, assert_future, assert_in_range, compute_duration, add_seconds |
//! | `JiffWorkflowPlugin` | `elicit_jiff` | parse_timestamp, assert_future, assert_in_range, add_seconds |
//! | `TimeWorkflowPlugin` | `elicit_time` | parse_offset, assert_utc, format_offset |
//! | `UrlWorkflowPlugin` | `elicit_url` | parse_url, assert_https, assert_host |
//! | `ReqwestWorkflowPlugin` | `elicit_reqwest` | fetch_json, fetch_text, url_build, build_request, status_summary, assert_success, assert_json, head_request, fetch_bytes |
//! | `JsonWorkflowPlugin` | `elicit_serde_json` | parse_and_focus, validate_object, safe_merge, pointer_update, field_chain |
//! | `RegexWorkflowPlugin` | `elicit_regex` | compile, is_match, find_all, replace_all, capture_groups |
//! | `SqlxWorkflowPlugin` | `elicit_sqlx` | connect, query, execute, query_typed, transaction |
//! | `TokioTimePlugin` | `elicit_tokio` | sleep, sleep_until, timeout_create, timeout_check, timeout_await, interval_create, interval_tick |
//! | `TokioSyncPlugin` | `elicit_tokio` | semaphore_new, semaphore_acquire, semaphore_try_acquire, semaphore_release, semaphore_available, semaphore_close, notify_new, notify_one, notify_waiters, notified, barrier_new, barrier_wait |
//! | `TokioRuntimePlugin` | `elicit_tokio` | inspect_flavor, build_current_thread *(emit-only)*, build_multi_thread *(emit-only)*, block_on *(emit-only)* |
//! | `TokioFsPlugin` | `elicit_tokio` | read_to_string, read_bytes, write_text, write_bytes, create_dir, create_dir_all, remove_dir, remove_dir_all, remove_file, rename, copy, metadata, read_dir, canonicalize |
//! | `TokioNetPlugin` | `elicit_tokio` | tcp_listener_bind, tcp_listener_accept, tcp_listener_local_addr, tcp_listener_close, tcp_stream_connect, tcp_stream_read, tcp_stream_write, tcp_stream_local_addr, tcp_stream_peer_addr, tcp_stream_close, udp_socket_bind, udp_socket_send_to, udp_socket_recv_from, udp_socket_local_addr, udp_socket_close |
//! | `TokioProcessPlugin` | `elicit_tokio` | process_run, process_spawn, process_stdin_write, process_stdout_read, process_stderr_read, process_wait, process_try_wait, process_kill, process_id |
//! | `TokioTaskPlugin` | `elicit_tokio` | yield_now, spawn *(emit-only)*, spawn_blocking *(emit-only)*, block_in_place *(emit-only)* |
//! | `TokioChannelPlugin` | `elicit_tokio` | mpsc_create, mpsc_send, mpsc_try_send, mpsc_recv, mpsc_try_recv, mpsc_sender_close, mpsc_receiver_close, oneshot_create, oneshot_send, oneshot_recv, oneshot_try_recv, watch_create, watch_send, watch_borrow, watch_changed, watch_subscribe, watch_sender_close, watch_receiver_close, broadcast_create, broadcast_send, broadcast_recv, broadcast_try_recv, broadcast_subscribe, broadcast_sender_close, broadcast_receiver_close, mutex_create, mutex_lock, mutex_update, mutex_try_lock, mutex_close, rwlock_create, rwlock_read, rwlock_write, rwlock_try_read, rwlock_try_write, rwlock_close |
//! | `TokioSignalPlugin` | `elicit_tokio` | ctrl_c, unix_signal_create (unix), unix_signal_recv (unix), unix_signal_close (unix) |
//! | `TokioIoPlugin` | `elicit_tokio` | duplex_create, duplex_read, duplex_write, duplex_close |
//! | `TokioUnixPlugin` | `elicit_tokio` | unix_listener_bind, unix_listener_accept, unix_listener_local_addr, unix_listener_close, unix_stream_connect, unix_stream_read, unix_stream_write, unix_stream_local_addr, unix_stream_peer_addr, unix_stream_close, unix_datagram_bind, unix_datagram_send_to, unix_datagram_recv_from, unix_datagram_local_addr, unix_datagram_close (**unix only**) |
//! | `TokioSpawnPlugin` | `elicit_tokio` | join, try_join, abort + one dynamic tool per registered workload type |
//!
//! # Feature flags
//!
//! | Feature | Enables |
//! |---|---|
//! | `emit` | `EmitBinaryPlugin` + full code recovery pipeline |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod fetch_and_parse;
pub mod ledger;
mod secure_fetch;

#[cfg(feature = "emit")]
mod emit_plugin;

pub use fetch_and_parse::{FetchAndParsePlugin, http_get};
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
        std::mem::size_of::<elicit_serde_json::ParseAndFocusEmit>(),
        // elicit_sqlx (workflow emit newtypes)
        std::mem::size_of::<elicit_sqlx::workflow::WfConnectParams>(),
        // elicit_regex
        std::mem::size_of::<elicit_regex::CompileParams>(),
        // elicit_tokio
        std::mem::size_of::<elicit_tokio::SleepParams>(),
        std::mem::size_of::<elicit_tokio::SemaphoreNewParams>(),
        std::mem::size_of::<elicit_tokio::InspectFlavorParams>(),
        std::mem::size_of::<elicit_tokio::BuildCurrentThreadParams>(),
        std::mem::size_of::<elicit_tokio::BuildMultiThreadParams>(),
        std::mem::size_of::<elicit_tokio::ReadToStringParams>(),
        std::mem::size_of::<elicit_tokio::TcpListenerBindParams>(),
        std::mem::size_of::<elicit_tokio::ProcessRunParams>(),
        std::mem::size_of::<elicit_tokio::YieldNowParams>(),
        std::mem::size_of::<elicit_tokio::SpawnParams>(),
        std::mem::size_of::<elicit_tokio::SpawnBlockingParams>(),
        std::mem::size_of::<elicit_tokio::BlockInPlaceParams>(),
        std::mem::size_of::<elicit_tokio::MpscCreateParams>(),
        std::mem::size_of::<elicit_tokio::CtrlCParams>(),
        std::mem::size_of::<elicit_tokio::DuplexCreateParams>(),
        // unix-only anchors compiled on all platforms via cfg
        #[cfg(unix)]
        std::mem::size_of::<elicit_tokio::UnixListenerBindParams>(),
        std::mem::size_of::<elicit_tokio::JoinParams>(),
    ];
    if crate_name.is_empty() {
        elicitation::emit_code::dispatch_emit(tool, params)
    } else {
        elicitation::emit_code::dispatch_emit_from(tool, crate_name, params)
    }
}
