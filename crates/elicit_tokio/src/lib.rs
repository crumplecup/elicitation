//! Elicitation-enabled tokio shadow crate.
//!
//! Provides MCP tools for async time management, sync primitives, and runtime
//! inspection. All tokio runtime objects are stored server-side in UUID-keyed
//! registries — only serializable handles (UUIDs, durations, booleans) cross
//! the MCP boundary.
//!
//! # Shadow-crate workflow plugins
//!
//! | Plugin | Namespace | Tools |
//! |---|---|---|
//! | [`TokioTimePlugin`] | `tokio_time__*` | sleep, sleep_until, timeout_create, timeout_check, timeout_await, interval_create, interval_tick |
//! | [`TokioSyncPlugin`] | `tokio_sync__*` | semaphore_new, semaphore_acquire, semaphore_try_acquire, semaphore_release, semaphore_available, semaphore_close, notify_new, notify_one, notify_waiters, notified, barrier_new, barrier_wait |
//! | [`TokioRuntimePlugin`] | `tokio_runtime__*` | inspect_flavor, build_current_thread *(emit-only)*, build_multi_thread *(emit-only)*, block_on *(emit-only)* |
//! | [`TokioFsPlugin`] | `tokio_fs__*` | read_to_string, read_bytes, write_text, write_bytes, create_dir, create_dir_all, remove_dir, remove_dir_all, remove_file, rename, copy, metadata, read_dir, canonicalize |
//! | [`TokioNetPlugin`] | `tokio_net__*` | tcp_listener_bind, tcp_listener_accept, tcp_listener_local_addr, tcp_listener_close, tcp_stream_connect, tcp_stream_read, tcp_stream_write, tcp_stream_local_addr, tcp_stream_peer_addr, tcp_stream_close, udp_socket_bind, udp_socket_send_to, udp_socket_recv_from, udp_socket_local_addr, udp_socket_close |
//! | [`TokioProcessPlugin`] | `tokio_process__*` | process_run, process_spawn, process_stdin_write, process_stdout_read, process_stderr_read, process_wait, process_try_wait, process_kill, process_id |
//! | [`TokioTaskPlugin`] | `tokio_task__*` | yield_now, spawn *(emit-only)*, spawn_blocking *(emit-only)*, block_in_place *(emit-only)* |
//! | [`TokioSpawnPlugin`] | `tokio_spawn__*` | join, try_join, abort + one dynamic tool per registered workload type |
//! | [`TokioChannelPlugin`] | `tokio_channel__*` | mpsc_create, mpsc_send, mpsc_try_send, mpsc_recv, mpsc_try_recv, mpsc_sender_close, mpsc_receiver_close, oneshot_create, oneshot_send, oneshot_recv, oneshot_try_recv, watch_create, watch_send, watch_borrow, watch_changed, watch_subscribe, watch_sender_close, watch_receiver_close, broadcast_create, broadcast_send, broadcast_recv, broadcast_try_recv, broadcast_subscribe, broadcast_sender_close, broadcast_receiver_close, mutex_create, mutex_lock, mutex_update, mutex_try_lock, mutex_close, rwlock_create, rwlock_read, rwlock_write, rwlock_try_read, rwlock_try_write, rwlock_close |
//! | [`TokioSignalPlugin`] | `tokio_signal__*` | ctrl_c, unix_signal_create (unix), unix_signal_recv (unix), unix_signal_close (unix) |
//! | [`TokioIoPlugin`] | `tokio_io__*` | duplex_create, duplex_read, duplex_write, duplex_close |
//! | [`TokioUnixPlugin`] | `tokio_unix__*` | unix_listener_bind, unix_listener_accept, unix_listener_local_addr, unix_listener_close, unix_stream_connect, unix_stream_read, unix_stream_write, unix_stream_local_addr, unix_stream_peer_addr, unix_stream_close, unix_datagram_bind, unix_datagram_send_to, unix_datagram_recv_from, unix_datagram_local_addr, unix_datagram_close (**unix only**) |
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod channels;
mod fs;
mod io;
mod net;
mod process;
mod runtime;
mod signal;
mod spawn;
mod sync;
mod task;
mod time;
#[cfg(unix)]
mod unix;

pub use channels::{
    BroadcastCreateParams, BroadcastReceiverCloseParams, BroadcastRecvParams, BroadcastSendParams,
    BroadcastSenderCloseParams, BroadcastSubscribeParams, BroadcastTryRecvParams, MpscCreateParams,
    MpscReceiverCloseParams, MpscRecvParams, MpscSendParams, MpscSenderCloseParams,
    MpscTryRecvParams, MpscTrySendParams, MutexCloseParams, MutexCreateParams, MutexLockParams,
    MutexTryLockParams, MutexUpdateParams, OneshotCreateParams, OneshotRecvParams,
    OneshotSendParams, OneshotTryRecvParams, RwLockCloseParams, RwLockCreateParams,
    RwLockReadParams, RwLockTryReadParams, RwLockTryWriteParams, RwLockWriteParams,
    TokioChannelPlugin, WatchBorrowParams, WatchChangedParams, WatchCreateParams,
    WatchReceiverCloseParams, WatchSendParams, WatchSenderCloseParams, WatchSubscribeParams,
};
pub use fs::{
    FromToParams, PathParams, ReadBytesParams, ReadToStringParams, TokioFsPlugin, WriteBytesParams,
    WriteTextParams,
};
pub use io::{
    DuplexCloseParams, DuplexCreateParams, DuplexReadParams, DuplexWriteParams, TokioIoPlugin,
};
pub use net::{
    ConnectionAccepted, DataReceived, ListenerBound, StreamConnected, TcpListenerAcceptParams,
    TcpListenerBindParams, TcpListenerCloseParams, TcpListenerLocalAddrParams,
    TcpStreamCloseParams, TcpStreamConnectParams, TcpStreamLocalAddrParams,
    TcpStreamPeerAddrParams, TcpStreamReadParams, TcpStreamWriteParams, TokioNetPlugin,
    UdpSocketBindParams, UdpSocketCloseParams, UdpSocketLocalAddrParams, UdpSocketRecvFromParams,
    UdpSocketSendToParams,
};
pub use process::{
    ProcessIdParams, ProcessKillParams, ProcessRunParams, ProcessSpawnParams,
    ProcessStderrReadParams, ProcessStdinWriteParams, ProcessStdoutReadParams,
    ProcessTryWaitParams, ProcessWaitParams, TokioProcessPlugin,
};
pub use runtime::{
    BlockOnParams as RuntimeBlockOnParams, BuildCurrentThreadParams, BuildMultiThreadParams,
    InspectFlavorParams, RuntimeFlavorKind, TokioRuntimePlugin,
};
pub use signal::{CtrlCParams, TokioSignalPlugin, UnixSignalKind};
#[cfg(unix)]
pub use signal::{UnixSignalCloseParams, UnixSignalCreateParams, UnixSignalRecvParams};
pub use spawn::{
    AbortParams, AsyncWorkload, BlockingWorkload, JoinParams, SpawnPluginBuilder, TokioSpawnPlugin,
    TryJoinParams,
};
pub use sync::{
    BarrierNewParams, BarrierReached, BarrierWaitParams, NotificationReceived, NotifiedParams,
    NotifyNewParams, NotifyOneParams, NotifyWaitersParams, PermitAcquired, SemaphoreAcquireParams,
    SemaphoreAvailableParams, SemaphoreCloseParams, SemaphoreNewParams, SemaphoreReleaseParams,
    SemaphoreTryAcquireParams, TokioSyncPlugin,
};
pub use task::{
    BlockInPlaceParams, SpawnBlockingParams, SpawnParams, TokioTaskPlugin, YieldNowParams,
};
pub use time::{
    IntervalCreateParams, IntervalTickParams, SleepCompleted, SleepParams, SleepUntilParams,
    TimeoutAwaitParams, TimeoutCheckParams, TimeoutCreateParams, TimeoutResolved, TokioTimePlugin,
};
#[cfg(unix)]
pub use unix::{
    TokioUnixPlugin, UnixDatagramBindParams, UnixDatagramCloseParams, UnixDatagramLocalAddrParams,
    UnixDatagramRecvFromParams, UnixDatagramSendToParams, UnixListenerAcceptParams,
    UnixListenerBindParams, UnixListenerCloseParams, UnixListenerLocalAddrParams,
    UnixStreamCloseParams, UnixStreamConnectParams, UnixStreamLocalAddrParams,
    UnixStreamPeerAddrParams, UnixStreamReadParams, UnixStreamWriteParams,
};
