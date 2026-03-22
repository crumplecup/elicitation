//! Proof coverage tests — assert every Prop has non-empty kani/verus/creusot proofs.
//!
//! A failing test here means a Prop was added without implementing proof methods.

#![cfg(feature = "proofs")]

use elicit_tokio::{
    BarrierReached,
    BytesCopied,
    ChannelClosed,
    ConnectionAccepted,
    // signal
    CtrlCReceived,
    DataReceived,
    DirCreated,
    // io
    DuplexCreated,
    // fs
    FileRead,
    FileWritten,
    // net
    ListenerBound,
    MessageReceived,
    // channels
    MessageSent,
    NotificationReceived,
    // sync
    PermitAcquired,
    ProcessExited,
    // process
    ProcessSpawned,
    // runtime
    RuntimeFlavored,
    // time
    SleepCompleted,
    StdinWritten,
    StreamConnected,
    TaskAborted,
    TaskJoined,
    TaskSpawned,
    // task
    TaskYielded,
    TimeoutResolved,
};
#[cfg(unix)]
use elicit_tokio::{
    SignalHandlerRegistered, SignalReceived, UnixConnectionAccepted, UnixDataReceived,
    UnixListenerBound, UnixStreamConnected,
};
use elicitation::contracts::Prop;

macro_rules! assert_prop_proofs {
    ($($T:ty),+ $(,)?) => {
        $(
            assert!(!<$T as Prop>::kani_proof().is_empty(),
                "{} missing kani_proof", stringify!($T));
            assert!(!<$T as Prop>::verus_proof().is_empty(),
                "{} missing verus_proof", stringify!($T));
            assert!(!<$T as Prop>::creusot_proof().is_empty(),
                "{} missing creusot_proof", stringify!($T));
        )+
    };
}

#[test]
fn all_tokio_props_have_proof_coverage() {
    assert_prop_proofs!(
        SleepCompleted,
        TimeoutResolved,
        PermitAcquired,
        NotificationReceived,
        BarrierReached,
        ListenerBound,
        ConnectionAccepted,
        StreamConnected,
        DataReceived,
        FileRead,
        FileWritten,
        DirCreated,
        ProcessSpawned,
        ProcessExited,
        StdinWritten,
        MessageSent,
        MessageReceived,
        ChannelClosed,
        CtrlCReceived,
        DuplexCreated,
        BytesCopied,
        TaskYielded,
        TaskSpawned,
        TaskJoined,
        TaskAborted,
        RuntimeFlavored,
    );
}

#[cfg(unix)]
#[test]
fn all_tokio_unix_props_have_proof_coverage() {
    assert_prop_proofs!(
        SignalHandlerRegistered,
        SignalReceived,
        UnixListenerBound,
        UnixConnectionAccepted,
        UnixStreamConnected,
        UnixDataReceived,
    );
}
