use elicit_db::{
    AccessAuthorized, AdvisoryLockHeld, Atomic, AuditLogged, BackupConsistent, ColumnExists,
    ConnectionEstablished, ConstraintSatisfied, DatabaseCreated, Durable, EncryptedAtRest,
    EncryptedInTransit, ForeignKeyExists, IndexExists, LeastPrivilegeEnforced, MVCCSnapshotValid,
    MetricsRecorded, NoDirtyReads, NoPhantomReads, NonEmptyResult, PointInTimeRecoverable,
    PreventsDirtyRead, PreventsDirtyWrite, PreventsNonRepeatableRead, PreventsPhantomRead,
    ReadCommittedIsolation, ReadUncommittedIsolation, ReferentialIntegrityMaintained,
    RepeatableReadIsolation, RequestWellFormed, ResponseSerializable, RowDeleted, RowInserted,
    RowUpdated, RowVisible, SchemaCreated, SchemaExists, SerializableIsolation, SnapshotIsolation,
    SpanLinkedToOperation, TableCreated, TableExists, TraceEmitted, TransactionCommitted,
    VacuumedRecently, ViewCreated, WALReplayable,
};
use elicitation::Established;
use std::mem::size_of;

#[test]
fn props_are_zero_sized() {
    // iso_sql
    assert_eq!(size_of::<TableCreated>(), 0);
    assert_eq!(size_of::<ConstraintSatisfied>(), 0);
    assert_eq!(size_of::<ReferentialIntegrityMaintained>(), 0);
    assert_eq!(size_of::<ViewCreated>(), 0);
    assert_eq!(size_of::<RowInserted>(), 0);
    assert_eq!(size_of::<RowUpdated>(), 0);
    assert_eq!(size_of::<RowDeleted>(), 0);
    assert_eq!(size_of::<NonEmptyResult>(), 0);
    assert_eq!(size_of::<TransactionCommitted>(), 0);
    assert_eq!(size_of::<Atomic>(), 0);
    assert_eq!(size_of::<Durable>(), 0);
    assert_eq!(size_of::<DatabaseCreated>(), 0);
    assert_eq!(size_of::<SchemaCreated>(), 0);

    // isolation
    assert_eq!(size_of::<ReadUncommittedIsolation>(), 0);
    assert_eq!(size_of::<ReadCommittedIsolation>(), 0);
    assert_eq!(size_of::<RepeatableReadIsolation>(), 0);
    assert_eq!(size_of::<SerializableIsolation>(), 0);
    assert_eq!(size_of::<PreventsDirtyRead>(), 0);
    assert_eq!(size_of::<PreventsNonRepeatableRead>(), 0);
    assert_eq!(size_of::<PreventsPhantomRead>(), 0);
    assert_eq!(size_of::<PreventsDirtyWrite>(), 0);
    assert_eq!(size_of::<NoDirtyReads>(), 0);
    assert_eq!(size_of::<NoPhantomReads>(), 0);

    // postgres
    assert_eq!(size_of::<MVCCSnapshotValid>(), 0);
    assert_eq!(size_of::<SnapshotIsolation>(), 0);
    assert_eq!(size_of::<AdvisoryLockHeld>(), 0);
    assert_eq!(size_of::<RowVisible>(), 0);
    assert_eq!(size_of::<IndexExists>(), 0);
    assert_eq!(size_of::<VacuumedRecently>(), 0);

    // information_schema
    assert_eq!(size_of::<TableExists>(), 0);
    assert_eq!(size_of::<ColumnExists>(), 0);
    assert_eq!(size_of::<SchemaExists>(), 0);
    assert_eq!(size_of::<ForeignKeyExists>(), 0);

    // security
    assert_eq!(size_of::<AccessAuthorized>(), 0);
    assert_eq!(size_of::<AuditLogged>(), 0);
    assert_eq!(size_of::<LeastPrivilegeEnforced>(), 0);
    assert_eq!(size_of::<EncryptedAtRest>(), 0);
    assert_eq!(size_of::<EncryptedInTransit>(), 0);

    // recovery
    assert_eq!(size_of::<BackupConsistent>(), 0);
    assert_eq!(size_of::<WALReplayable>(), 0);
    assert_eq!(size_of::<PointInTimeRecoverable>(), 0);

    // transport
    assert_eq!(size_of::<RequestWellFormed>(), 0);
    assert_eq!(size_of::<ResponseSerializable>(), 0);
    assert_eq!(size_of::<ConnectionEstablished>(), 0);

    // observability
    assert_eq!(size_of::<TraceEmitted>(), 0);
    assert_eq!(size_of::<SpanLinkedToOperation>(), 0);
    assert_eq!(size_of::<MetricsRecorded>(), 0);
}

#[test]
fn established_is_zero_sized() {
    assert_eq!(size_of::<Established<TableCreated>>(), 0);
    assert_eq!(size_of::<Established<AuditLogged>>(), 0);
    assert_eq!(size_of::<Established<SerializableIsolation>>(), 0);
    assert_eq!(size_of::<Established<MVCCSnapshotValid>>(), 0);
    assert_eq!(size_of::<Established<WALReplayable>>(), 0);
    assert_eq!(size_of::<Established<TraceEmitted>>(), 0);
    assert_eq!(size_of::<Established<ConnectionEstablished>>(), 0);
    assert_eq!(size_of::<Established<BackupConsistent>>(), 0);
    assert_eq!(size_of::<Established<Durable>>(), 0);
    assert_eq!(size_of::<Established<TransactionCommitted>>(), 0);
}
