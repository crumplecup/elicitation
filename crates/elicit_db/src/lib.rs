//! `elicit_db` — Database contract interface crate.
//!
//! Provides a standards-anchored vocabulary of database propositions ([`contracts`]),
//! typestate markers ([`typestate`]), and a complete family of object-safe async
//! traits ([`traits`]) for pgAdmin-style database management.
//!
//! # Design
//!
//! This is an **interface crate**, not an implementation. DB drivers (sqlx, diesel,
//! sea-orm) implement the traits; consumers depend on this crate only.
//!
//! Traits use [`Established<P>`] contract return types instead of associated types,
//! giving object safety (`dyn DbTableManager`) and a common proof language at call sites.
//!
//! # Standards
//!
//! - ISO/IEC 9075 (SQL semantics)
//! - ANSI isolation model (phenomena P0–P3)
//! - PostgreSQL documentation (execution truth)
//! - ISO/IEC 27001 (security contracts)
//! - OpenTelemetry Specification (observability)
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_db::{DbTableManager, DbColumn, TableCreated, AuditLogged};
//! use elicitation::Established;
//!
//! async fn ensure_users_table(mgr: &dyn DbTableManager) {
//!     let cols = vec![
//!         DbColumn {
//!             name: "id".into(), ty: "bigint".into(),
//!             nullable: false, default_value: None, primary_key: true,
//!         },
//!         DbColumn {
//!             name: "email".into(), ty: "text".into(),
//!             nullable: false, default_value: None, primary_key: false,
//!         },
//!     ];
//!     let (Established::<TableCreated> { .. }, Established::<AuditLogged> { .. }) =
//!         mgr.create_table("public", "users", cols).await.unwrap();
//! }
//! ```
//!
//! [`Established<P>`]: elicitation::Established

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contracts;
mod error;
mod traits;
mod types;
mod typestate;

pub use contracts::information_schema::{
    ColumnExists, ForeignKeyExists, SchemaExists, TableExists,
};
pub use contracts::iso_sql::{
    Atomic, ConstraintSatisfied, DatabaseCreated, Durable, NonEmptyResult,
    ReferentialIntegrityMaintained, RowDeleted, RowInserted, RowUpdated, SchemaCreated,
    TableCreated, TransactionCommitted, ViewCreated,
};
pub use contracts::isolation::{
    NoDirtyReads, NoPhantomReads, PreventsDirtyRead, PreventsDirtyWrite, PreventsNonRepeatableRead,
    PreventsPhantomRead, ReadCommittedIsolation, ReadUncommittedIsolation, RepeatableReadIsolation,
    SerializableIsolation,
};
pub use contracts::observability::{MetricsRecorded, SpanLinkedToOperation, TraceEmitted};
pub use contracts::postgres::{
    AdvisoryLockHeld, IndexExists, MVCCSnapshotValid, RowVisible, SnapshotIsolation,
    VacuumedRecently,
};
pub use contracts::recovery::{BackupConsistent, PointInTimeRecoverable, WALReplayable};
pub use contracts::security::{
    AccessAuthorized, AuditLogged, EncryptedAtRest, EncryptedInTransit, LeastPrivilegeEnforced,
};
pub use contracts::transport::{ConnectionEstablished, RequestWellFormed, ResponseSerializable};

pub use elicitation::ElicitComplete;
pub use error::{DbError, DbErrorKind, DbResult};
pub use traits::{
    DbBackend, DbBackupManager, DbDatabaseManager, DbIndexManager, DbMonitor, DbQueryExecutor,
    DbRoleManager, DbSchemaManager, DbServerAdmin, DbSessionManager, DbTableManager, DbTransactor,
};
pub use types::{
    ConnectionId, DbColumn, DbCommitResult, DbExecuteResult, DbExplain, DbIndexInfo,
    DbQueryRowsResult, DbRoleInfo, DbRow, DbRows, DbSchema, DbSessionInfo, DbSpatialValue,
    DbStatActivity, DbTableInfo, DbTransactionalExecuteResult, DbValue, IsolationLevel,
    TransactionHandle,
};
pub use typestate::{Committed, Executed, Open, Prepared, RolledBack, TxMarker};
