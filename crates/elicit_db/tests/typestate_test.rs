use elicit_db::{Committed, IsolationLevel, Open, RolledBack, TxMarker};

#[test]
fn tx_marker_commit_transition() {
    let open = TxMarker::<Open>::open(IsolationLevel::ReadCommitted);
    assert_eq!(open.isolation, IsolationLevel::ReadCommitted);
    let committed = open.commit();
    let _: TxMarker<Committed> = committed;
    assert_eq!(committed.isolation, IsolationLevel::ReadCommitted);
}

#[test]
fn tx_marker_rollback_transition() {
    let open = TxMarker::<Open>::open(IsolationLevel::Serializable);
    assert_eq!(open.isolation, IsolationLevel::Serializable);
    let rolled_back = open.rollback();
    let _: TxMarker<RolledBack> = rolled_back;
    assert_eq!(rolled_back.isolation, IsolationLevel::Serializable);
}

#[test]
fn tx_marker_preserves_isolation_level_through_commit() {
    for level in [
        IsolationLevel::ReadUncommitted,
        IsolationLevel::ReadCommitted,
        IsolationLevel::RepeatableRead,
        IsolationLevel::Serializable,
    ] {
        let open = TxMarker::<Open>::open(level);
        let committed = open.commit();
        assert_eq!(committed.isolation, level);
    }
}

#[test]
fn tx_marker_preserves_isolation_level_through_rollback() {
    for level in [
        IsolationLevel::ReadUncommitted,
        IsolationLevel::ReadCommitted,
        IsolationLevel::RepeatableRead,
        IsolationLevel::Serializable,
    ] {
        let open = TxMarker::<Open>::open(level);
        let rolled = open.rollback();
        assert_eq!(rolled.isolation, level);
    }
}

#[test]
fn tx_marker_is_copy() {
    let open = TxMarker::<Open>::open(IsolationLevel::Serializable);
    let copy = open;
    assert_eq!(open.isolation, copy.isolation);
}
