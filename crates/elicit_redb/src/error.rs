//! Trenchcoats for all public `redb` error types.
//!
//! Each shadow type serializes over JSON and converts `From<redb::*Error>`.
//! The `StorageError::Io` and similar non-Clone variants are lossy-converted to
//! a `String` message.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── StorageError ──────────────────────────────────────────────────────────────

/// Shadow of `redb::StorageError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "detail")]
pub enum StorageError {
    /// Database file is corrupted; detail contains the message.
    Corrupted(String),
    /// Attempted to store a value that exceeds the storage backend's size limit.
    ValueTooLarge(usize),
    /// Underlying I/O error; detail is the `Display` message.
    Io(String),
    /// A previous I/O error has made the database unusable.
    PreviousIo,
    /// The database has been closed.
    DatabaseClosed,
    /// An internal mutex was poisoned; detail is the location string.
    LockPoisoned(String),
}

impl From<redb::StorageError> for StorageError {
    fn from(e: redb::StorageError) -> Self {
        match e {
            redb::StorageError::Corrupted(s) => Self::Corrupted(s),
            redb::StorageError::ValueTooLarge(n) => Self::ValueTooLarge(n),
            redb::StorageError::Io(io) => Self::Io(io.to_string()),
            redb::StorageError::PreviousIo => Self::PreviousIo,
            redb::StorageError::DatabaseClosed => Self::DatabaseClosed,
            redb::StorageError::LockPoisoned(loc) => Self::LockPoisoned(format!("{loc}")),
            _ => Self::Corrupted(format!("unknown storage error: {e}")),
        }
    }
}

// ── TableError ────────────────────────────────────────────────────────────────

/// Shadow of `redb::TableError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "detail")]
pub enum TableError {
    /// Table was opened with a different key/value type than it was created with.
    TableTypeMismatch {
        /// Name of the conflicting table.
        table: String,
    },
    /// Table is a multimap table; use multimap API instead.
    TableIsMultimap(String),
    /// Table is not a multimap table; use the regular table API instead.
    TableIsNotMultimap(String),
    /// Type definition for a key or value type has changed since the table was created.
    TypeDefinitionChanged {
        /// Name of the type whose definition changed.
        name: String,
    },
    /// Requested table does not exist.
    TableDoesNotExist(String),
    /// A table with this name already exists.
    TableExists(String),
    /// The table is already open in this write transaction.
    TableAlreadyOpen(String),
    /// Underlying storage error.
    Storage(StorageError),
}

impl From<redb::TableError> for TableError {
    fn from(e: redb::TableError) -> Self {
        match e {
            redb::TableError::TableTypeMismatch { table: t, .. } => {
                Self::TableTypeMismatch { table: t }
            }
            redb::TableError::TableIsMultimap(s) => Self::TableIsMultimap(s),
            redb::TableError::TableIsNotMultimap(s) => Self::TableIsNotMultimap(s),
            redb::TableError::TypeDefinitionChanged { name: n, .. } => {
                Self::TypeDefinitionChanged {
                    name: n.name().to_owned(),
                }
            }
            redb::TableError::TableDoesNotExist(s) => Self::TableDoesNotExist(s),
            redb::TableError::TableExists(s) => Self::TableExists(s),
            redb::TableError::TableAlreadyOpen(s, _loc) => Self::TableAlreadyOpen(s),
            redb::TableError::Storage(s) => Self::Storage(s.into()),
            _ => Self::TableDoesNotExist(format!("unknown table error: {e}")),
        }
    }
}

// ── DatabaseError ─────────────────────────────────────────────────────────────

/// Shadow of `redb::DatabaseError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "detail")]
pub enum DatabaseError {
    /// The database file is already open by another process or handle.
    DatabaseAlreadyOpen,
    /// Repair was aborted by the repair callback.
    RepairAborted,
    /// Database was created by an incompatible version; detail is the version number.
    UpgradeRequired(u8),
    /// A transaction is already in progress.
    TransactionInProgress,
    /// Underlying storage error.
    Storage(StorageError),
}

impl From<redb::DatabaseError> for DatabaseError {
    fn from(e: redb::DatabaseError) -> Self {
        match e {
            redb::DatabaseError::DatabaseAlreadyOpen => Self::DatabaseAlreadyOpen,
            redb::DatabaseError::RepairAborted => Self::RepairAborted,
            redb::DatabaseError::UpgradeRequired(v) => Self::UpgradeRequired(v),
            redb::DatabaseError::TransactionInProgress => Self::TransactionInProgress,
            redb::DatabaseError::Storage(s) => Self::Storage(s.into()),
            _ => Self::Storage(StorageError::Corrupted(format!("unknown db error: {e}"))),
        }
    }
}

// ── SavepointError ────────────────────────────────────────────────────────────

/// Shadow of `redb::SavepointError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum SavepointError {
    /// The savepoint is no longer valid (e.g., was restored from and then committed).
    InvalidSavepoint,
    /// Restoring this savepoint requires `Durability::Immediate`.
    ImmediateDurabilityRequired,
    /// Underlying storage error.
    Storage(StorageError),
}

impl From<redb::SavepointError> for SavepointError {
    fn from(e: redb::SavepointError) -> Self {
        match e {
            redb::SavepointError::InvalidSavepoint => Self::InvalidSavepoint,
            redb::SavepointError::ImmediateDurabilityRequired => Self::ImmediateDurabilityRequired,
            redb::SavepointError::Storage(s) => Self::Storage(s.into()),
            _ => Self::InvalidSavepoint,
        }
    }
}

// ── CompactionError ───────────────────────────────────────────────────────────

/// Shadow of `redb::CompactionError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum CompactionError {
    /// Compaction is blocked by an existing persistent savepoint.
    PersistentSavepointExists,
    /// Compaction is blocked by an existing ephemeral savepoint.
    EphemeralSavepointExists,
    /// A transaction is in progress; commit or abort it first.
    TransactionInProgress,
    /// Underlying storage error.
    Storage(StorageError),
}

impl From<redb::CompactionError> for CompactionError {
    fn from(e: redb::CompactionError) -> Self {
        match e {
            redb::CompactionError::PersistentSavepointExists => Self::PersistentSavepointExists,
            redb::CompactionError::EphemeralSavepointExists => Self::EphemeralSavepointExists,
            redb::CompactionError::TransactionInProgress => Self::TransactionInProgress,
            redb::CompactionError::Storage(s) => Self::Storage(s.into()),
            _ => Self::TransactionInProgress,
        }
    }
}

// ── SetDurabilityError ────────────────────────────────────────────────────────

/// Shadow of `redb::SetDurabilityError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum SetDurabilityError {
    /// A persistent savepoint has been modified; durability can no longer be changed.
    PersistentSavepointModified,
}

impl From<redb::SetDurabilityError> for SetDurabilityError {
    fn from(e: redb::SetDurabilityError) -> Self {
        match e {
            redb::SetDurabilityError::PersistentSavepointModified => {
                Self::PersistentSavepointModified
            }
            _ => Self::PersistentSavepointModified,
        }
    }
}

// ── TransactionError ──────────────────────────────────────────────────────────

/// Shadow of `redb::TransactionError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "detail")]
pub enum TransactionError {
    /// Underlying storage error.
    Storage(StorageError),
    /// A read transaction is still open; close it before beginning a write transaction.
    ReadTransactionStillInUse,
}

impl From<redb::TransactionError> for TransactionError {
    fn from(e: redb::TransactionError) -> Self {
        match e {
            redb::TransactionError::Storage(s) => Self::Storage(s.into()),
            redb::TransactionError::ReadTransactionStillInUse(_) => Self::ReadTransactionStillInUse,
            _ => Self::ReadTransactionStillInUse,
        }
    }
}

// ── CommitError ───────────────────────────────────────────────────────────────

/// Shadow of `redb::CommitError`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", content = "detail")]
pub enum CommitError {
    /// Underlying storage error.
    Storage(StorageError),
}

impl From<redb::CommitError> for CommitError {
    fn from(e: redb::CommitError) -> Self {
        match e {
            redb::CommitError::Storage(s) => Self::Storage(s.into()),
            _ => Self::Storage(StorageError::Corrupted(format!(
                "unknown commit error: {e}"
            ))),
        }
    }
}
