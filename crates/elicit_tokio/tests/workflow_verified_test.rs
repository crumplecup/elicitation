//! `VerifiedWorkflow` validation tests for elicit_tokio propositions.

use elicit_tokio::{
    BarrierReached, BytesCopied, ChannelClosed, ConnectionAccepted, CtrlCReceived, DataReceived,
    DirCreated, DuplexCreated, FileRead, FileWritten, ListenerBound, MessageReceived, MessageSent,
    NotificationReceived, PermitAcquired, ProcessExited, ProcessSpawned, RuntimeFlavored,
    SleepCompleted, StdinWritten, StreamConnected, TaskAborted, TaskJoined, TaskSpawned,
    TaskYielded, TimeoutResolved,
};
#[cfg(unix)]
use elicit_tokio::{
    SignalHandlerRegistered, SignalReceived, UnixConnectionAccepted, UnixDataReceived,
    UnixListenerBound, UnixStreamConnected,
};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn tokio_time_props_non_empty() {
    assert_verified::<SleepCompleted>("SleepCompleted");
    assert_verified::<TimeoutResolved>("TimeoutResolved");
}

#[test]
fn tokio_sync_props_non_empty() {
    assert_verified::<PermitAcquired>("PermitAcquired");
    assert_verified::<NotificationReceived>("NotificationReceived");
    assert_verified::<BarrierReached>("BarrierReached");
}

#[test]
fn tokio_net_props_non_empty() {
    assert_verified::<ListenerBound>("ListenerBound");
    assert_verified::<ConnectionAccepted>("ConnectionAccepted");
    assert_verified::<StreamConnected>("StreamConnected");
    assert_verified::<DataReceived>("DataReceived");
}

#[test]
fn tokio_fs_props_non_empty() {
    assert_verified::<FileRead>("FileRead");
    assert_verified::<FileWritten>("FileWritten");
    assert_verified::<DirCreated>("DirCreated");
}

#[test]
fn tokio_process_props_non_empty() {
    assert_verified::<ProcessSpawned>("ProcessSpawned");
    assert_verified::<ProcessExited>("ProcessExited");
    assert_verified::<StdinWritten>("StdinWritten");
}

#[test]
fn tokio_channels_props_non_empty() {
    assert_verified::<MessageSent>("MessageSent");
    assert_verified::<MessageReceived>("MessageReceived");
    assert_verified::<ChannelClosed>("ChannelClosed");
}

#[test]
fn tokio_misc_props_non_empty() {
    assert_verified::<CtrlCReceived>("CtrlCReceived");
    assert_verified::<DuplexCreated>("DuplexCreated");
    assert_verified::<BytesCopied>("BytesCopied");
    assert_verified::<TaskYielded>("TaskYielded");
    assert_verified::<TaskSpawned>("TaskSpawned");
    assert_verified::<TaskJoined>("TaskJoined");
    assert_verified::<TaskAborted>("TaskAborted");
    assert_verified::<RuntimeFlavored>("RuntimeFlavored");
}

#[cfg(unix)]
#[test]
fn tokio_unix_props_non_empty() {
    assert_verified::<SignalHandlerRegistered>("SignalHandlerRegistered");
    assert_verified::<SignalReceived>("SignalReceived");
    assert_verified::<UnixListenerBound>("UnixListenerBound");
    assert_verified::<UnixConnectionAccepted>("UnixConnectionAccepted");
    assert_verified::<UnixStreamConnected>("UnixStreamConnected");
    assert_verified::<UnixDataReceived>("UnixDataReceived");
}

#[test]
fn tokio_and_contains_constituents() {
    type SC = And<SleepCompleted, TimeoutResolved>;
    type LB = And<ListenerBound, ConnectionAccepted>;
    type MS = And<MessageSent, MessageReceived>;

    assert!(SC::kani_proof_contains::<SleepCompleted>());
    assert!(SC::kani_proof_contains::<TimeoutResolved>());
    assert!(LB::kani_proof_contains::<ListenerBound>());
    assert!(LB::kani_proof_contains::<ConnectionAccepted>());
    assert!(MS::kani_proof_contains::<MessageSent>());
    assert!(MS::kani_proof_contains::<MessageReceived>());
}
