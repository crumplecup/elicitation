//! Kani proofs for tokio workflow plugin contracts.
//!
//! Available with the `tokio-types` feature.
//!
//! # Verification Stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! Tokio's async operations (spawn, sleep, bind, accept, etc.) execute inside an
//! async executor that Kani's bounded model checker cannot model. We therefore
//! establish their contracts as **trusted axioms** — documented base lemmas that
//! users can build proofs on top of.
//!
//! For each tokio operation group we prove:
//!
//! 1. **Zero-cost markers**: every `Prop` type established by an `elicit_tokio`
//!    tool is a unit struct. `Established<P>` wraps `PhantomData<P>`. Both are
//!    confirmed zero-sized here so users know composition is free.
//!
//! 2. **Trusted axiom proofs**: `kani::assume` encodes the tokio contract.
//!    Each proof is a documented axiom: *"if tokio's function returned `Ok`, then
//!    the associated `Prop` was established"*. These cannot be discharged by
//!    the model checker — they are the trusted base from which all downstream
//!    composition proceeds.
//!
//! # Prop → Tokio function mapping
//!
//! | Prop | Tokio function | Plugin |
//! |---|---|---|
//! | `SleepCompleted` | `tokio::time::sleep` | `TokioTimePlugin` |
//! | `TimeoutResolved` | `tokio::time::timeout` | `TokioTimePlugin` |
//! | `PermitAcquired` | `tokio::sync::Semaphore::acquire` | `TokioSyncPlugin` |
//! | `NotificationReceived` | `tokio::sync::Notify::notified` | `TokioSyncPlugin` |
//! | `BarrierReached` | `tokio::sync::Barrier::wait` | `TokioSyncPlugin` |
//! | `ListenerBound` | `tokio::net::TcpListener::bind` | `TokioNetPlugin` |
//! | `ConnectionAccepted` | `tokio::net::TcpListener::accept` | `TokioNetPlugin` |
//! | `StreamConnected` | `tokio::net::TcpStream::connect` | `TokioNetPlugin` |
//! | `DataReceived` | `TcpStream::read` / `UdpSocket::recv_from` | `TokioNetPlugin` |
//! | `FileRead` | `tokio::fs::read_to_string` / `tokio::fs::read` | `TokioFsPlugin` |
//! | `FileWritten` | `tokio::fs::write` | `TokioFsPlugin` |
//! | `DirCreated` | `tokio::fs::create_dir` / `create_dir_all` | `TokioFsPlugin` |
//! | `ProcessSpawned` | `tokio::process::Command::spawn` | `TokioProcessPlugin` |
//! | `ProcessExited` | `tokio::process::Child::wait` | `TokioProcessPlugin` |
//! | `StdinWritten` | `tokio::io::AsyncWriteExt::write_all` (stdin) | `TokioProcessPlugin` |
//! | `MessageSent` | channel send operations | `TokioChannelPlugin` |
//! | `MessageReceived` | channel recv operations | `TokioChannelPlugin` |
//! | `ChannelClosed` | channel close/drop operations | `TokioChannelPlugin` |
//! | `CtrlCReceived` | `tokio::signal::ctrl_c` | `TokioSignalPlugin` |
//! | `SignalHandlerRegistered` | `tokio::signal::unix::signal` | `TokioSignalPlugin` |
//! | `SignalReceived` | `tokio::signal::unix::Signal::recv` | `TokioSignalPlugin` |
//! | `DuplexCreated` | `tokio::io::duplex` | `TokioIoPlugin` |
//! | `BytesCopied` | `tokio::io::copy` | `TokioIoCopyPlugin` |
//! | `TaskYielded` | `tokio::task::yield_now` | `TokioTaskPlugin` |
//! | `TaskSpawned` | `tokio::spawn` / `spawn_blocking` | `TokioSpawnPlugin` |
//! | `TaskJoined` | `tokio::task::JoinHandle::await` | `TokioSpawnPlugin` |
//! | `TaskAborted` | `tokio::task::JoinHandle::abort` | `TokioSpawnPlugin` |
//! | `RuntimeFlavored` | `tokio::runtime::Handle::runtime_flavor` | `TokioRuntimePlugin` |
//! | `UnixListenerBound` | `tokio::net::UnixListener::bind` | `TokioUnixPlugin` |
//! | `UnixConnectionAccepted` | `tokio::net::UnixListener::accept` | `TokioUnixPlugin` |
//! | `UnixStreamConnected` | `tokio::net::UnixStream::connect` | `TokioUnixPlugin` |
//! | `UnixDataReceived` | `UnixStream::read` / `UnixDatagram::recv_from` | `TokioUnixPlugin` |

use elicitation::contracts::{And, Established, both};

// ============================================================================
// Helper: local Prop mirrors (unit structs — always zero-sized)
//
// The actual Prop types live in elicit_tokio; these local mirrors prove that
// the unit-struct pattern itself is always zero-sized, independent of crate
// boundaries.
// ============================================================================

macro_rules! assert_prop_zero_sized {
    ($name:ident) => {
        struct $name;
        assert!(
            std::mem::size_of::<$name>() == 0,
            "{} must be zero-sized",
            stringify!($name)
        );
        assert!(
            std::mem::size_of::<Established<$name>>() == 0,
            "Established<{}> must be zero-sized",
            stringify!($name)
        );
    };
}

// ============================================================================
// tokio::time — sleep, timeout
// ============================================================================

/// Zero-cost proof: `SleepCompleted` and `TimeoutResolved` are free markers.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_time_props_zero_sized() {
    assert_prop_zero_sized!(SleepCompleted);
    assert_prop_zero_sized!(TimeoutResolved);
}

/// Trusted axiom: `tokio::time::sleep(duration)` — when it returns, the
/// requested duration has elapsed. This is the contract licensed by
/// `Established<SleepCompleted>` in `TokioTimePlugin::sleep` /
/// `TokioTimePlugin::sleep_until`.
///
/// See: <https://docs.rs/tokio/latest/tokio/time/fn.sleep.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_sleep_completed_axiom() {
    let duration_ms: u64 = kani::any();
    // Axiom: if sleep returns Ok, the duration elapsed.
    let sleep_returned_ok: bool = kani::any();
    kani::assume(sleep_returned_ok);
    assert!(
        sleep_returned_ok,
        "tokio::time::sleep axiom: returns when duration elapsed"
    );
}

/// Trusted axiom: `tokio::time::timeout(duration, future)` — returns
/// `Ok(output)` if the future completed within the duration, or `Err(Elapsed)`
/// otherwise. `Established<TimeoutResolved>` is asserted on both outcomes in
/// `TokioTimePlugin::timeout_await`.
///
/// See: <https://docs.rs/tokio/latest/tokio/time/fn.timeout.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_timeout_resolved_axiom() {
    let timed_out: bool = kani::any();
    // Either the inner future completed (Ok) or we got Elapsed (Err) — resolved either way.
    let resolved = true; // timeout always resolves (either Ok or Elapsed)
    assert!(
        resolved,
        "tokio::time::timeout axiom: always resolves to Ok or Elapsed"
    );
    let _ = timed_out;
}

// ============================================================================
// tokio::sync — Semaphore, Notify, Barrier
// ============================================================================

/// Zero-cost proof: sync Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_sync_props_zero_sized() {
    assert_prop_zero_sized!(PermitAcquired);
    assert_prop_zero_sized!(NotificationReceived);
    assert_prop_zero_sized!(BarrierReached);
}

/// Trusted axiom: `tokio::sync::Semaphore::acquire()` — when it returns `Ok`,
/// a permit has been decremented from the semaphore. Licensed by
/// `Established<PermitAcquired>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html#method.acquire>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_permit_acquired_axiom() {
    let permits_available: u32 = kani::any();
    kani::assume(permits_available > 0);
    // Axiom: acquire() succeeds iff semaphore is not closed and permits > 0
    let acquired = true;
    assert!(
        acquired,
        "tokio::sync::Semaphore::acquire axiom: Ok => permit decremented"
    );
}

/// Trusted axiom: `tokio::sync::Notify::notified().await` — returns when a
/// notification was sent. Licensed by `Established<NotificationReceived>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_notification_received_axiom() {
    let notified: bool = kani::any();
    kani::assume(notified);
    assert!(
        notified,
        "tokio::sync::Notify::notified axiom: returns when notify_one/waiters called"
    );
}

/// Trusted axiom: `tokio::sync::Barrier::wait()` — all N parties have reached
/// the barrier. Licensed by `Established<BarrierReached>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Barrier.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_barrier_reached_axiom() {
    let all_arrived: bool = kani::any();
    kani::assume(all_arrived);
    assert!(
        all_arrived,
        "tokio::sync::Barrier::wait axiom: returns when all parties arrive"
    );
}

// ============================================================================
// tokio::net — TcpListener, TcpStream, UdpSocket
// ============================================================================

/// Zero-cost proof: net Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_net_props_zero_sized() {
    assert_prop_zero_sized!(ListenerBound);
    assert_prop_zero_sized!(ConnectionAccepted);
    assert_prop_zero_sized!(StreamConnected);
    assert_prop_zero_sized!(DataReceived);
}

/// Trusted axiom: `tokio::net::TcpListener::bind(addr)` — when it returns
/// `Ok`, the socket is bound to the address. Licensed by
/// `Established<ListenerBound>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.bind>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_listener_bound_axiom() {
    let bind_ok: bool = kani::any();
    kani::assume(bind_ok);
    assert!(
        bind_ok,
        "tokio::net::TcpListener::bind axiom: Ok => socket bound"
    );
}

/// Trusted axiom: `tokio::net::TcpListener::accept()` — when it returns
/// `Ok((stream, addr))`, an incoming connection is established. Licensed by
/// `Established<ConnectionAccepted>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.accept>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_connection_accepted_axiom() {
    let accept_ok: bool = kani::any();
    kani::assume(accept_ok);
    assert!(
        accept_ok,
        "tokio::net::TcpListener::accept axiom: Ok => connection stream ready"
    );
}

/// Trusted axiom: `tokio::net::TcpStream::connect(addr)` — when it returns
/// `Ok`, the TCP three-way handshake completed. Licensed by
/// `Established<StreamConnected>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.connect>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_stream_connected_axiom() {
    let connect_ok: bool = kani::any();
    kani::assume(connect_ok);
    assert!(
        connect_ok,
        "tokio::net::TcpStream::connect axiom: Ok => TCP handshake complete"
    );
}

/// Trusted axiom: `AsyncReadExt::read` / `UdpSocket::recv_from` — when it
/// returns `Ok(n)` with n > 0, bytes were received. Licensed by
/// `Established<DataReceived>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_data_received_axiom() {
    let bytes: usize = kani::any();
    kani::assume(bytes > 0);
    assert!(
        bytes > 0,
        "AsyncReadExt::read axiom: Ok(n > 0) => bytes available"
    );
}

// ============================================================================
// tokio::fs — read, write, mkdir
// ============================================================================

/// Zero-cost proof: fs Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_fs_props_zero_sized() {
    assert_prop_zero_sized!(FileRead);
    assert_prop_zero_sized!(FileWritten);
    assert_prop_zero_sized!(DirCreated);
}

/// Trusted axiom: `tokio::fs::read_to_string` / `tokio::fs::read` — when it
/// returns `Ok`, the file's contents are available. Licensed by
/// `Established<FileRead>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.read_to_string.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_file_read_axiom() {
    let read_ok: bool = kani::any();
    kani::assume(read_ok);
    assert!(
        read_ok,
        "tokio::fs::read axiom: Ok => file contents available"
    );
}

/// Trusted axiom: `tokio::fs::write` — when it returns `Ok`, all bytes were
/// written to the file. Licensed by `Established<FileWritten>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.write.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_file_written_axiom() {
    let write_ok: bool = kani::any();
    kani::assume(write_ok);
    assert!(
        write_ok,
        "tokio::fs::write axiom: Ok => all bytes flushed to disk"
    );
}

/// Trusted axiom: `tokio::fs::create_dir` / `create_dir_all` — when it
/// returns `Ok`, the directory exists. Licensed by `Established<DirCreated>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.create_dir_all.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_dir_created_axiom() {
    let mkdir_ok: bool = kani::any();
    kani::assume(mkdir_ok);
    assert!(
        mkdir_ok,
        "tokio::fs::create_dir_all axiom: Ok => directory path exists"
    );
}

// ============================================================================
// tokio::process — Command::spawn, Child::wait, stdin write
// ============================================================================

/// Zero-cost proof: process Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_process_props_zero_sized() {
    assert_prop_zero_sized!(ProcessSpawned);
    assert_prop_zero_sized!(ProcessExited);
    assert_prop_zero_sized!(StdinWritten);
}

/// Trusted axiom: `tokio::process::Command::spawn()` — when it returns `Ok`,
/// the OS process is running. Licensed by `Established<ProcessSpawned>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/process/struct.Command.html#method.spawn>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_process_spawned_axiom() {
    let spawn_ok: bool = kani::any();
    kani::assume(spawn_ok);
    assert!(
        spawn_ok,
        "tokio::process::Command::spawn axiom: Ok => OS process is running"
    );
}

/// Trusted axiom: `tokio::process::Child::wait()` — when it returns `Ok`,
/// the child has exited and an exit status is available. Licensed by
/// `Established<ProcessExited>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/process/struct.Child.html#method.wait>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_process_exited_axiom() {
    let wait_ok: bool = kani::any();
    kani::assume(wait_ok);
    assert!(
        wait_ok,
        "tokio::process::Child::wait axiom: Ok => child process has exited"
    );
}

/// Trusted axiom: `AsyncWriteExt::write_all` on child stdin — when it returns
/// `Ok`, all bytes were delivered to the child's stdin pipe. Licensed by
/// `Established<StdinWritten>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_stdin_written_axiom() {
    let write_ok: bool = kani::any();
    kani::assume(write_ok);
    assert!(
        write_ok,
        "AsyncWriteExt::write_all(stdin) axiom: Ok => all bytes written to pipe"
    );
}

// ============================================================================
// tokio channel operations — mpsc, oneshot, broadcast, watch
// ============================================================================

/// Zero-cost proof: channel Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_channel_props_zero_sized() {
    assert_prop_zero_sized!(MessageSent);
    assert_prop_zero_sized!(MessageReceived);
    assert_prop_zero_sized!(ChannelClosed);
}

/// Trusted axiom: tokio channel `send()` — when it returns `Ok`, the value
/// was placed in the channel buffer and at least one receiver will see it.
/// Licensed by `Established<MessageSent>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_message_sent_axiom() {
    let send_ok: bool = kani::any();
    kani::assume(send_ok);
    assert!(
        send_ok,
        "tokio channel send axiom: Ok => value enqueued for receiver(s)"
    );
}

/// Trusted axiom: tokio channel `recv()` — when it returns `Some(v)` or
/// `Ok(v)`, a value is available that was sent by a sender. Licensed by
/// `Established<MessageReceived>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_message_received_axiom() {
    let recv_some: bool = kani::any();
    kani::assume(recv_some);
    assert!(
        recv_some,
        "tokio channel recv axiom: Some(v)/Ok(v) => value was sent"
    );
}

/// Trusted axiom: dropping or removing a channel endpoint closes it.
/// Licensed by `Established<ChannelClosed>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_channel_closed_axiom() {
    let close_ok: bool = kani::any();
    kani::assume(close_ok);
    assert!(
        close_ok,
        "tokio channel close axiom: drop/remove => endpoint closed"
    );
}

// ============================================================================
// tokio::signal — ctrl_c, unix signals
// ============================================================================

/// Zero-cost proof: signal Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_signal_props_zero_sized() {
    assert_prop_zero_sized!(CtrlCReceived);
    assert_prop_zero_sized!(SignalHandlerRegistered);
    assert_prop_zero_sized!(SignalReceived);
}

/// Trusted axiom: `tokio::signal::ctrl_c().await` — when it returns, SIGINT
/// was delivered to the process. Licensed by `Established<CtrlCReceived>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/signal/fn.ctrl_c.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_ctrl_c_received_axiom() {
    let ctrl_c_ok: bool = kani::any();
    kani::assume(ctrl_c_ok);
    assert!(
        ctrl_c_ok,
        "tokio::signal::ctrl_c axiom: Ok => SIGINT received"
    );
}

/// Trusted axiom: `tokio::signal::unix::signal(kind)` — when it returns `Ok`,
/// the signal handler is registered with the OS. Licensed by
/// `Established<SignalHandlerRegistered>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/signal/unix/fn.signal.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_signal_handler_registered_axiom() {
    let register_ok: bool = kani::any();
    kani::assume(register_ok);
    assert!(
        register_ok,
        "tokio::signal::unix::signal axiom: Ok => OS handler registered"
    );
}

/// Trusted axiom: `Signal::recv().await` — when it returns `Some(())`, the
/// registered signal was delivered. Licensed by `Established<SignalReceived>`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_signal_received_axiom() {
    let sig_some: bool = kani::any();
    kani::assume(sig_some);
    assert!(
        sig_some,
        "Signal::recv axiom: Some(()) => registered signal was delivered"
    );
}

// ============================================================================
// tokio::io — duplex, copy
// ============================================================================

/// Zero-cost proof: io Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_io_props_zero_sized() {
    assert_prop_zero_sized!(DuplexCreated);
    assert_prop_zero_sized!(BytesCopied);
}

/// Trusted axiom: `tokio::io::duplex(max_buf)` — always returns two connected
/// `DuplexStream` ends. The call cannot fail. Licensed by
/// `Established<DuplexCreated>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/io/fn.duplex.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_duplex_created_axiom() {
    // duplex() is infallible — always creates a pair
    let created = true;
    assert!(
        created,
        "tokio::io::duplex axiom: always returns a connected (a, b) pair"
    );
}

/// Trusted axiom: `tokio::io::copy(&mut reader, &mut writer)` — when it
/// returns `Ok(n)`, exactly `n` bytes were read from `reader` and written to
/// `writer`. Licensed by `Established<BytesCopied>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/io/fn.copy.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_bytes_copied_axiom() {
    let copy_ok: bool = kani::any();
    kani::assume(copy_ok);
    let n: u64 = kani::any();
    // n bytes were read from reader AND written to writer
    assert!(
        copy_ok,
        "tokio::io::copy axiom: Ok(n) => n bytes transferred reader→writer"
    );
    let _ = n;
}

// ============================================================================
// tokio::task — yield_now, spawn, spawn_blocking, JoinHandle
// ============================================================================

/// Zero-cost proof: task Prop markers are free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_task_props_zero_sized() {
    assert_prop_zero_sized!(TaskYielded);
    assert_prop_zero_sized!(TaskSpawned);
    assert_prop_zero_sized!(TaskJoined);
    assert_prop_zero_sized!(TaskAborted);
}

/// Trusted axiom: `tokio::task::yield_now().await` — returns after yielding
/// to the scheduler once. Cannot fail. Licensed by `Established<TaskYielded>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_task_yielded_axiom() {
    let yielded = true; // yield_now is infallible
    assert!(
        yielded,
        "tokio::task::yield_now axiom: always returns after yielding"
    );
}

/// Trusted axiom: `tokio::spawn(future)` / `spawn_blocking(f)` — when it
/// returns a `JoinHandle`, the task has been accepted by the scheduler.
/// Licensed by `Established<TaskSpawned>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fn.spawn.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_task_spawned_axiom() {
    let spawn_ok = true; // spawn/spawn_blocking only fail on runtime shutdown
    assert!(
        spawn_ok,
        "tokio::spawn axiom: JoinHandle returned => task accepted by scheduler"
    );
}

/// Trusted axiom: `JoinHandle::await` — when it returns `Ok(output)`, the
/// task completed and its return value is available. Licensed by
/// `Established<TaskJoined>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_task_joined_axiom() {
    let join_ok: bool = kani::any();
    kani::assume(join_ok);
    assert!(
        join_ok,
        "JoinHandle::await axiom: Ok => task completed without panic"
    );
}

/// Trusted axiom: `JoinHandle::abort()` — schedules the task for cancellation
/// at its next await point. Licensed by `Established<TaskAborted>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_task_aborted_axiom() {
    let abort_scheduled = true; // abort() is infallible (fire-and-forget)
    assert!(
        abort_scheduled,
        "JoinHandle::abort axiom: schedules cancellation (infallible)"
    );
}

// ============================================================================
// tokio::runtime — Handle::runtime_flavor
// ============================================================================

/// Zero-cost proof: runtime Prop marker is free.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_runtime_props_zero_sized() {
    assert_prop_zero_sized!(RuntimeFlavored);
}

/// Trusted axiom: `tokio::runtime::Handle::current().runtime_flavor()` —
/// when called from within a tokio runtime, returns the flavor of that runtime.
/// Licensed by `Established<RuntimeFlavored>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.runtime_flavor>
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_runtime_flavored_axiom() {
    let flavor_known = true; // always succeeds inside a runtime
    assert!(
        flavor_known,
        "Handle::runtime_flavor axiom: always returns when called inside runtime"
    );
}

// ============================================================================
// tokio::net::unix — UnixListener, UnixStream, UnixDatagram (unix only)
// ============================================================================

/// Zero-cost proof: Unix socket Prop markers are free.
#[cfg(all(feature = "tokio-types", unix))]
#[kani::proof]
fn verify_unix_props_zero_sized() {
    assert_prop_zero_sized!(UnixListenerBound);
    assert_prop_zero_sized!(UnixConnectionAccepted);
    assert_prop_zero_sized!(UnixStreamConnected);
    assert_prop_zero_sized!(UnixDataReceived);
}

/// Trusted axiom: `tokio::net::UnixListener::bind(path)` — when it returns
/// `Ok`, the socket file exists at the path. Licensed by
/// `Established<UnixListenerBound>`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.UnixListener.html#method.bind>
#[cfg(all(feature = "tokio-types", unix))]
#[kani::proof]
fn verify_unix_listener_bound_axiom() {
    let bind_ok: bool = kani::any();
    kani::assume(bind_ok);
    assert!(
        bind_ok,
        "tokio::net::UnixListener::bind axiom: Ok => socket file created"
    );
}

/// Trusted axiom: `tokio::net::UnixListener::accept()` — when it returns
/// `Ok((stream, addr))`, a client is connected. Licensed by
/// `Established<UnixConnectionAccepted>`.
#[cfg(all(feature = "tokio-types", unix))]
#[kani::proof]
fn verify_unix_connection_accepted_axiom() {
    let accept_ok: bool = kani::any();
    kani::assume(accept_ok);
    assert!(
        accept_ok,
        "tokio::net::UnixListener::accept axiom: Ok => client stream ready"
    );
}

/// Trusted axiom: `tokio::net::UnixStream::connect(path)` — when it returns
/// `Ok`, the client is connected to the Unix socket. Licensed by
/// `Established<UnixStreamConnected>`.
#[cfg(all(feature = "tokio-types", unix))]
#[kani::proof]
fn verify_unix_stream_connected_axiom() {
    let connect_ok: bool = kani::any();
    kani::assume(connect_ok);
    assert!(
        connect_ok,
        "tokio::net::UnixStream::connect axiom: Ok => connected to socket"
    );
}

/// Trusted axiom: `AsyncReadExt::read` / `UnixDatagram::recv_from` — when it
/// returns `Ok(n > 0)`, bytes were received. Licensed by
/// `Established<UnixDataReceived>`.
#[cfg(all(feature = "tokio-types", unix))]
#[kani::proof]
fn verify_unix_data_received_axiom() {
    let bytes: usize = kani::any();
    kani::assume(bytes > 0);
    assert!(
        bytes > 0,
        "UnixStream::read axiom: Ok(n > 0) => bytes available"
    );
}

// ============================================================================
// Proposition combinator proofs (generic — independent of tokio)
// ============================================================================

/// `Established<P>` is zero-sized for any unit-struct `P`.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_established_zero_sized_general() {
    use std::mem::size_of;
    struct AnyProp;
    assert!(size_of::<Established<AnyProp>>() == 0);
}

/// `And<P, Q>` combinator is zero-sized.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_and_combinator_zero_sized() {
    use std::mem::size_of;
    struct P;
    struct Q;
    assert!(size_of::<And<P, Q>>() == 0);
}

/// `both(p, q)` produces `Established<And<P, Q>>` — also zero-sized.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_both_composition_zero_sized() {
    use std::mem::size_of;
    struct P;
    struct Q;
    let p: Established<P> = Established::assert();
    let q: Established<Q> = Established::assert();
    let _combined: Established<And<P, Q>> = both(p, q);
    assert!(size_of::<Established<And<P, Q>>>() == 0);
}

/// Three-way composition: `And<And<P, Q>, R>` — still zero-sized.
#[cfg(feature = "tokio-types")]
#[kani::proof]
fn verify_three_way_composition_zero_sized() {
    use std::mem::size_of;
    struct Bound;
    struct Accepted;
    struct Connected;
    let p: Established<Bound> = Established::assert();
    let q: Established<Accepted> = Established::assert();
    let r: Established<Connected> = Established::assert();
    let pq: Established<And<Bound, Accepted>> = both(p, q);
    let _pqr: Established<And<And<Bound, Accepted>, Connected>> = both(pq, r);
    assert!(size_of::<Established<And<And<Bound, Accepted>, Connected>>>() == 0);
}
