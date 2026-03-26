use verus_builtin_macros::verus;

verus! {

// ============================================================================
// tokio workflow plugin contracts
//
// Trust the source. Verify the wrapper contracts.
//
// Tokio's async operations execute inside an async executor that Verus cannot
// model. We establish each contract as an abstract boolean-parameter proof:
// the parameter encodes the tokio contract, and the function proves that if
// the contract holds, the result is established. These form the trusted base
// for downstream proof composition.
//
// Pattern: `pub fn verify_<op>(contract_holds: bool) -> (result: bool)
//               ensures result == contract_holds, { contract_holds }`
//
// Prop → tokio function mapping documented in:
//   crates/elicitation_kani/src/tokio_types.rs (full table)
// ============================================================================

// ============================================================================
// tokio::time — SleepCompleted, TimeoutResolved
// ============================================================================

/// Contract: `tokio::time::sleep` — returns when duration elapsed.
/// `Established<SleepCompleted>` is valid iff `sleep_returned_ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/time/fn.sleep.html>
pub fn verify_sleep_completed(sleep_returned_ok: bool) -> (result: bool)
    ensures result == sleep_returned_ok,
{
    sleep_returned_ok
}

/// Contract: `tokio::time::timeout` — resolves to Ok or Elapsed.
/// `Established<TimeoutResolved>` is always valid when the future resolved.
///
/// See: <https://docs.rs/tokio/latest/tokio/time/fn.timeout.html>
pub fn verify_timeout_resolved(timeout_resolved: bool) -> (result: bool)
    ensures result == timeout_resolved,
{
    timeout_resolved
}

// ============================================================================
// tokio::sync — PermitAcquired, NotificationReceived, BarrierReached
// ============================================================================

/// Contract: `Semaphore::acquire` — `Ok(permit)` iff permit was decremented.
/// `Established<PermitAcquired>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html#method.acquire>
pub fn verify_permit_acquired(acquire_returned_ok: bool) -> (result: bool)
    ensures result == acquire_returned_ok,
{
    acquire_returned_ok
}

/// Contract: `Notify::notified` — returns when notify_one/waiters was called.
/// `Established<NotificationReceived>` is valid on return.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Notify.html>
pub fn verify_notification_received(was_notified: bool) -> (result: bool)
    ensures result == was_notified,
{
    was_notified
}

/// Contract: `Barrier::wait` — all N parties arrived.
/// `Established<BarrierReached>` is valid on return.
///
/// See: <https://docs.rs/tokio/latest/tokio/sync/struct.Barrier.html>
pub fn verify_barrier_reached(all_arrived: bool) -> (result: bool)
    ensures result == all_arrived,
{
    all_arrived
}

// ============================================================================
// tokio::net — ListenerBound, ConnectionAccepted, StreamConnected, DataReceived
// ============================================================================

/// Contract: `TcpListener::bind` — `Ok` iff socket bound to addr.
/// `Established<ListenerBound>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.bind>
pub fn verify_listener_bound(bind_returned_ok: bool) -> (result: bool)
    ensures result == bind_returned_ok,
{
    bind_returned_ok
}

/// Contract: `TcpListener::accept` — `Ok((stream, addr))` iff TCP handshake complete.
/// `Established<ConnectionAccepted>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html#method.accept>
pub fn verify_connection_accepted(accept_returned_ok: bool) -> (result: bool)
    ensures result == accept_returned_ok,
{
    accept_returned_ok
}

/// Contract: `TcpStream::connect` — `Ok` iff three-way handshake complete.
/// `Established<StreamConnected>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html#method.connect>
pub fn verify_stream_connected(connect_returned_ok: bool) -> (result: bool)
    ensures result == connect_returned_ok,
{
    connect_returned_ok
}

/// Contract: `AsyncReadExt::read` — `Ok(n > 0)` iff bytes available from peer.
/// `Established<DataReceived>` is valid on `Ok(n > 0)`.
pub fn verify_data_received(bytes_available: bool) -> (result: bool)
    ensures result == bytes_available,
{
    bytes_available
}

// ============================================================================
// tokio::fs — FileRead, FileWritten, DirCreated
// ============================================================================

/// Contract: `tokio::fs::read_to_string` / `read` — `Ok` iff file contents loaded.
/// `Established<FileRead>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.read_to_string.html>
pub fn verify_file_read(read_returned_ok: bool) -> (result: bool)
    ensures result == read_returned_ok,
{
    read_returned_ok
}

/// Contract: `tokio::fs::write` — `Ok` iff all bytes flushed.
/// `Established<FileWritten>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.write.html>
pub fn verify_file_written(write_returned_ok: bool) -> (result: bool)
    ensures result == write_returned_ok,
{
    write_returned_ok
}

/// Contract: `tokio::fs::create_dir_all` — `Ok` iff directory path exists.
/// `Established<DirCreated>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/fs/fn.create_dir_all.html>
pub fn verify_dir_created(mkdir_returned_ok: bool) -> (result: bool)
    ensures result == mkdir_returned_ok,
{
    mkdir_returned_ok
}

// ============================================================================
// tokio::process — ProcessSpawned, ProcessExited, StdinWritten
// ============================================================================

/// Contract: `Command::spawn` — `Ok(child)` iff OS process is running.
/// `Established<ProcessSpawned>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/process/struct.Command.html#method.spawn>
pub fn verify_process_spawned(spawn_returned_ok: bool) -> (result: bool)
    ensures result == spawn_returned_ok,
{
    spawn_returned_ok
}

/// Contract: `Child::wait` — `Ok(status)` iff child has exited.
/// `Established<ProcessExited>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/process/struct.Child.html#method.wait>
pub fn verify_process_exited(wait_returned_ok: bool) -> (result: bool)
    ensures result == wait_returned_ok,
{
    wait_returned_ok
}

/// Contract: `write_all(stdin, data)` — `Ok` iff all bytes delivered to pipe.
/// `Established<StdinWritten>` is valid on `Ok`.
pub fn verify_stdin_written(write_returned_ok: bool) -> (result: bool)
    ensures result == write_returned_ok,
{
    write_returned_ok
}

// ============================================================================
// tokio channel operations — MessageSent, MessageReceived, ChannelClosed
// ============================================================================

/// Contract: channel `send(value)` — `Ok` iff value enqueued for receivers.
/// `Established<MessageSent>` is valid on `Ok`.
///
/// Covers mpsc, oneshot, broadcast, watch send operations.
pub fn verify_message_sent(send_returned_ok: bool) -> (result: bool)
    ensures result == send_returned_ok,
{
    send_returned_ok
}

/// Contract: channel `recv()` — `Some(v)` / `Ok(v)` iff value was sent.
/// `Established<MessageReceived>` is valid on `Some`/`Ok`.
///
/// Covers mpsc, oneshot, broadcast, watch recv operations.
pub fn verify_message_received(recv_returned_some: bool) -> (result: bool)
    ensures result == recv_returned_some,
{
    recv_returned_some
}

/// Contract: channel endpoint drop/close — senders/receivers observe closure.
/// `Established<ChannelClosed>` is valid after endpoint removal.
pub fn verify_channel_closed(endpoint_closed: bool) -> (result: bool)
    ensures result == endpoint_closed,
{
    endpoint_closed
}

// ============================================================================
// tokio::signal — CtrlCReceived, SignalHandlerRegistered, SignalReceived
// ============================================================================

/// Contract: `signal::ctrl_c()` — `Ok` iff SIGINT was delivered.
/// `Established<CtrlCReceived>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/signal/fn.ctrl_c.html>
pub fn verify_ctrl_c_received(ctrl_c_returned_ok: bool) -> (result: bool)
    ensures result == ctrl_c_returned_ok,
{
    ctrl_c_returned_ok
}

/// Contract: `signal::unix::signal(kind)` — `Ok(signal)` iff OS handler installed.
/// `Established<SignalHandlerRegistered>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/signal/unix/fn.signal.html>
pub fn verify_signal_handler_registered(handler_installed: bool) -> (result: bool)
    ensures result == handler_installed,
{
    handler_installed
}

/// Contract: `Signal::recv()` — `Some(())` iff signal was delivered.
/// `Established<SignalReceived>` is valid on `Some`.
pub fn verify_signal_received(signal_delivered: bool) -> (result: bool)
    ensures result == signal_delivered,
{
    signal_delivered
}

// ============================================================================
// tokio::io — DuplexCreated, BytesCopied
// ============================================================================

/// Contract: `tokio::io::duplex(max_buf)` — infallible, always returns pair.
/// `Established<DuplexCreated>` is always valid.
///
/// See: <https://docs.rs/tokio/latest/tokio/io/fn.duplex.html>
pub fn verify_duplex_created(call_completed: bool) -> (result: bool)
    ensures result == call_completed,
{
    call_completed
}

/// Contract: `tokio::io::copy(&mut r, &mut w)` — `Ok(n)` iff n bytes transferred.
/// `Established<BytesCopied>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/io/fn.copy.html>
pub fn verify_bytes_copied(copy_returned_ok: bool) -> (result: bool)
    ensures result == copy_returned_ok,
{
    copy_returned_ok
}

// ============================================================================
// tokio::task — TaskYielded, TaskSpawned, TaskJoined, TaskAborted
// ============================================================================

/// Contract: `task::yield_now()` — infallible, yields then returns.
/// `Established<TaskYielded>` is always valid.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html>
pub fn verify_task_yielded(call_completed: bool) -> (result: bool)
    ensures result == call_completed,
{
    call_completed
}

/// Contract: `tokio::spawn` / `spawn_blocking` — JoinHandle returned iff task accepted.
/// `Established<TaskSpawned>` is valid when JoinHandle is returned.
///
/// See: <https://docs.rs/tokio/latest/tokio/fn.spawn.html>
pub fn verify_task_spawned(spawn_returned_handle: bool) -> (result: bool)
    ensures result == spawn_returned_handle,
{
    spawn_returned_handle
}

/// Contract: `JoinHandle::await` — `Ok(output)` iff task completed without panic.
/// `Established<TaskJoined>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html>
pub fn verify_task_joined(join_returned_ok: bool) -> (result: bool)
    ensures result == join_returned_ok,
{
    join_returned_ok
}

/// Contract: `JoinHandle::abort()` — infallible, schedules cancellation.
/// `Established<TaskAborted>` is valid after abort() returns.
///
/// See: <https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort>
pub fn verify_task_aborted(abort_scheduled: bool) -> (result: bool)
    ensures result == abort_scheduled,
{
    abort_scheduled
}

// ============================================================================
// tokio::runtime — RuntimeFlavored
// ============================================================================

/// Contract: `Handle::runtime_flavor()` — always returns flavor inside runtime.
/// `Established<RuntimeFlavored>` is valid when called inside a runtime.
///
/// See: <https://docs.rs/tokio/latest/tokio/runtime/struct.Handle.html#method.runtime_flavor>
pub fn verify_runtime_flavored(inside_runtime: bool) -> (result: bool)
    ensures result == inside_runtime,
{
    inside_runtime
}

// ============================================================================
// tokio::net::unix — UnixListenerBound, UnixConnectionAccepted,
//                    UnixStreamConnected, UnixDataReceived  (unix only)
// ============================================================================

/// Contract: `UnixListener::bind(path)` — `Ok` iff socket file created.
/// `Established<UnixListenerBound>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.UnixListener.html#method.bind>
pub fn verify_unix_listener_bound(bind_returned_ok: bool) -> (result: bool)
    ensures result == bind_returned_ok,
{
    bind_returned_ok
}

/// Contract: `UnixListener::accept()` — `Ok` iff client connected.
/// `Established<UnixConnectionAccepted>` is valid on `Ok`.
pub fn verify_unix_connection_accepted(accept_returned_ok: bool) -> (result: bool)
    ensures result == accept_returned_ok,
{
    accept_returned_ok
}

/// Contract: `UnixStream::connect(path)` — `Ok` iff connected to socket.
/// `Established<UnixStreamConnected>` is valid on `Ok`.
///
/// See: <https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html#method.connect>
pub fn verify_unix_stream_connected(connect_returned_ok: bool) -> (result: bool)
    ensures result == connect_returned_ok,
{
    connect_returned_ok
}

/// Contract: `AsyncReadExt::read` / `UnixDatagram::recv_from` — `Ok(n > 0)` iff bytes available.
/// `Established<UnixDataReceived>` is valid on `Ok(n > 0)`.
pub fn verify_unix_data_received(bytes_available: bool) -> (result: bool)
    ensures result == bytes_available,
{
    bytes_available
}

// ============================================================================
// Proposition combinator structural contracts
// ============================================================================

/// Contract: `Established<P>` is zero-sized for any unit-struct `P`.
/// The type carries `PhantomData<P>` — zero runtime overhead.
pub fn verify_established_is_zero_sized(size: usize) -> bool {
    size == 0
}

/// Contract: `And<P, Q>` is zero-sized — unit struct combinator.
pub fn verify_and_combinator_is_zero_sized(size: usize) -> bool {
    size == 0
}

/// Contract: `both(p, q)` produces `Established<And<P,Q>>` — also zero-sized.
pub fn verify_both_result_is_zero_sized(size: usize) -> bool {
    size == 0
}

/// Contract: three-way `And<And<P,Q>,R>` composition is still zero-sized.
pub fn verify_three_way_composition_is_zero_sized(size: usize) -> bool {
    size == 0
}

} // verus!
