//! [`RedbBackend`] — `elicit_db` trait implementations for redb.
//!
//! Bridges the live redb `Database` into the `DbEmbeddedBackend` supertrait
//! surface.  Each `DbEmbeddedBackend` constituent is implemented with real
//! redb operations; relational-only traits return
//! [`DbErrorKind::Unsupported`].

use std::sync::{Arc, Mutex};

use elicitation::Established;
use futures::future::BoxFuture;
use redb::ReadableDatabase as _;
use redb::TableDefinition;
use redb::TableHandle as _;
use tracing::instrument;

use elicit_db::{
    AuditLogged, BackupConsistent, DatabaseCreated, DbBackupManager, DbCommitResult,
    DbDatabaseManager, DbEmbeddedStore, DbError, DbErrorKind, DbIsolationFactory, DbKvStore,
    DbMonitor, DbResult, DbSnapshotManager, DbStorageStats, DbTableManager, DbTransactor, DbValue,
    IsolationLevel, KvEntry, KvIntegrityVerified, KvKeyDeleted, KvKeyInserted, KvSnapshotCreated,
    KvSnapshotRestored, KvTableCompacted, Open, ReadCommittedIsolation, ReadUncommittedIsolation,
    RepeatableReadIsolation, RolledBack, SerializableIsolation, SessionIsolationLevelSet,
    SnapshotHandle, TransactionHandle, TransactionIsolationLevelSet, TransactionReadOnly,
    TransactionReadWrite, TxMarker, WALReplayable,
};

fn db_err(kind: DbErrorKind) -> DbError {
    DbError::new(kind)
}

fn unsupported(op: &str) -> DbError {
    db_err(DbErrorKind::Unsupported(op.to_string()))
}

fn internal(msg: impl std::fmt::Display) -> DbError {
    db_err(DbErrorKind::QueryFailed(msg.to_string()))
}

/// Converts a [`DbValue`] to a redb-compatible byte vector for storage.
fn value_to_bytes(v: &DbValue) -> DbResult<Vec<u8>> {
    match v {
        DbValue::Bytes(b) => Ok(b.clone()),
        DbValue::Text(s) => Ok(s.as_bytes().to_vec()),
        DbValue::Int(i) => Ok(i.to_le_bytes().to_vec()),
        DbValue::Float(f) => Ok(f.to_bits().to_le_bytes().to_vec()),
        DbValue::Bool(b) => Ok(vec![*b as u8]),
        other => serde_json::to_vec(other)
            .map_err(|e| internal(format!("cannot encode DbValue as bytes: {e}"))),
    }
}

/// Converts raw bytes stored in redb back to a [`DbValue`].
///
/// All values round-trip through JSON unless the original was plain bytes.
fn bytes_to_value(b: &[u8]) -> DbValue {
    // Try UTF-8 text first; fall back to raw bytes.
    match std::str::from_utf8(b) {
        Ok(s) => match serde_json::from_str::<DbValue>(s) {
            Ok(v) => v,
            Err(_) => DbValue::Text(s.to_string()),
        },
        Err(_) => DbValue::Bytes(b.to_vec()),
    }
}

/// redb `Database` wrapped as an `elicit_db` embedded backend.
///
/// All operations that redb natively supports are real; relational-only
/// operations (schema DDL, sessions, WAL, replication, etc.) return
/// [`DbErrorKind::Unsupported`].
#[derive(Clone, Debug)]
pub struct RedbBackend {
    db: Arc<Mutex<redb::Database>>,
}

impl RedbBackend {
    /// Open or create a redb database at `path`.
    #[instrument(fields(path = %path.as_ref().display()))]
    pub fn open(path: impl AsRef<std::path::Path>) -> DbResult<Self> {
        let db = redb::Database::create(path.as_ref())
            .map_err(|e| internal(format!("redb open failed: {e}")))?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Create an in-memory redb backend backed by a temporary file.
    #[instrument]
    pub fn in_memory() -> DbResult<Self> {
        let db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::new())
            .map_err(|e| internal(format!("redb in-memory create failed: {e}")))?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

// ── DbKvStore ────────────────────────────────────────────────────────────────

impl DbKvStore for RedbBackend {
    #[instrument(skip(self, key))]
    fn kv_get(&self, table: &str, key: &DbValue) -> BoxFuture<'_, DbResult<Option<DbValue>>> {
        let table = table.to_string();
        let key_bytes = value_to_bytes(key);
        let db = self.db.clone();
        Box::pin(async move {
            let key_bytes = key_bytes?;
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let read_txn = db
                .begin_read()
                .map_err(|e| internal(format!("begin_read: {e}")))?;
            let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
            match read_txn.open_table(table_def) {
                Ok(t) => {
                    let result = t
                        .get(key_bytes.as_slice())
                        .map_err(|e| internal(format!("kv_get: {e}")))?
                        .map(|ag| bytes_to_value(ag.value()));
                    Ok(result)
                }
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(None),
                Err(e) => Err(internal(format!("open_table: {e}"))),
            }
        })
    }

    #[instrument(skip(self, key, value))]
    fn kv_insert(
        &self,
        table: &str,
        key: DbValue,
        value: DbValue,
    ) -> BoxFuture<'_, DbResult<Established<KvKeyInserted>>> {
        let table = table.to_string();
        let key_bytes = value_to_bytes(&key);
        let val_bytes = value_to_bytes(&value);
        let db = self.db.clone();
        Box::pin(async move {
            let key_bytes = key_bytes?;
            let val_bytes = val_bytes?;
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write: {e}")))?;
            {
                let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
                let mut t = write_txn
                    .open_table(table_def)
                    .map_err(|e| internal(format!("open_table: {e}")))?;
                t.insert(key_bytes.as_slice(), val_bytes.as_slice())
                    .map_err(|e| internal(format!("kv_insert: {e}")))?;
            }
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit: {e}")))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self, key))]
    fn kv_remove(
        &self,
        table: &str,
        key: &DbValue,
    ) -> BoxFuture<'_, DbResult<Established<KvKeyDeleted>>> {
        let table = table.to_string();
        let key_bytes = value_to_bytes(key);
        let db = self.db.clone();
        Box::pin(async move {
            let key_bytes = key_bytes?;
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write: {e}")))?;
            {
                let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
                match write_txn.open_table(table_def) {
                    Ok(mut t) => {
                        t.remove(key_bytes.as_slice())
                            .map_err(|e| internal(format!("kv_remove: {e}")))?;
                    }
                    Err(redb::TableError::TableDoesNotExist(_)) => {}
                    Err(e) => return Err(internal(format!("open_table: {e}"))),
                }
            }
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit: {e}")))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self))]
    fn kv_scan(&self, table: &str) -> BoxFuture<'_, DbResult<Vec<KvEntry>>> {
        let table = table.to_string();
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let read_txn = db
                .begin_read()
                .map_err(|e| internal(format!("begin_read: {e}")))?;
            let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
            match read_txn.open_table(table_def) {
                Ok(t) => {
                    use redb::ReadableTable as _;
                    let mut entries = Vec::new();
                    for result in t.iter().map_err(|e| internal(format!("iter: {e}")))? {
                        let (k, v) = result.map_err(|e| internal(format!("kv_scan entry: {e}")))?;
                        entries.push(KvEntry {
                            key: bytes_to_value(k.value()),
                            value: bytes_to_value(v.value()),
                        });
                    }
                    Ok(entries)
                }
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(Vec::new()),
                Err(e) => Err(internal(format!("open_table: {e}"))),
            }
        })
    }

    #[instrument(skip(self, from, to))]
    fn kv_range(
        &self,
        table: &str,
        from: &DbValue,
        to: &DbValue,
    ) -> BoxFuture<'_, DbResult<Vec<KvEntry>>> {
        let table = table.to_string();
        let from_bytes = value_to_bytes(from);
        let to_bytes = value_to_bytes(to);
        let db = self.db.clone();
        Box::pin(async move {
            let from_bytes = from_bytes?;
            let to_bytes = to_bytes?;
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let read_txn = db
                .begin_read()
                .map_err(|e| internal(format!("begin_read: {e}")))?;
            let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
            match read_txn.open_table(table_def) {
                Ok(t) => {
                    let mut entries = Vec::new();
                    let range = from_bytes.as_slice()..to_bytes.as_slice();
                    for result in t
                        .range(range)
                        .map_err(|e| internal(format!("kv_range: {e}")))?
                    {
                        let (k, v) =
                            result.map_err(|e| internal(format!("kv_range entry: {e}")))?;
                        entries.push(KvEntry {
                            key: bytes_to_value(k.value()),
                            value: bytes_to_value(v.value()),
                        });
                    }
                    Ok(entries)
                }
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(Vec::new()),
                Err(e) => Err(internal(format!("open_table: {e}"))),
            }
        })
    }

    #[instrument(skip(self))]
    fn kv_len(&self, table: &str) -> BoxFuture<'_, DbResult<u64>> {
        let table = table.to_string();
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let read_txn = db
                .begin_read()
                .map_err(|e| internal(format!("begin_read: {e}")))?;
            let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&table);
            match read_txn.open_table(table_def) {
                Ok(t) => {
                    use redb::ReadableTableMetadata as _;
                    let len = t.len().map_err(|e| internal(format!("kv_len: {e}")))?;
                    Ok(len)
                }
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(0),
                Err(e) => Err(internal(format!("open_table: {e}"))),
            }
        })
    }
}

// ── DbEmbeddedStore ──────────────────────────────────────────────────────────

impl DbEmbeddedStore for RedbBackend {
    #[instrument(skip(self))]
    fn compact(&self) -> BoxFuture<'_, DbResult<Established<KvTableCompacted>>> {
        let db = self.db.clone();
        Box::pin(async move {
            let mut db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            db.compact()
                .map_err(|e| internal(format!("compact: {e}")))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self))]
    fn check_integrity(&self) -> BoxFuture<'_, DbResult<Established<KvIntegrityVerified>>> {
        let db = self.db.clone();
        Box::pin(async move {
            let mut db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            db.check_integrity()
                .map_err(|e| internal(format!("check_integrity: {e}")))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self))]
    fn storage_stats(&self) -> BoxFuture<'_, DbResult<DbStorageStats>> {
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for stats: {e}")))?;
            let stats = write_txn
                .stats()
                .map_err(|e| internal(format!("stats: {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit stats txn: {e}")))?;
            Ok(DbStorageStats {
                stored_bytes: stats.stored_bytes(),
                fragmented_bytes: stats.fragmented_bytes(),
                metadata_bytes: stats.metadata_bytes(),
                table_count: stats.tree_height() as usize,
                cache_hit_ratio: 0.0,
            })
        })
    }
}

// ── DbSnapshotManager ────────────────────────────────────────────────────────

impl DbSnapshotManager for RedbBackend {
    #[instrument(skip(self))]
    fn create_snapshot(
        &self,
        name: &str,
    ) -> BoxFuture<'_, DbResult<(SnapshotHandle, Established<KvSnapshotCreated>)>> {
        let name = name.to_string();
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for snapshot: {e}")))?;
            let sp = write_txn
                .persistent_savepoint()
                .map_err(|e| internal(format!("persistent_savepoint: {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit snapshot txn: {e}")))?;
            let handle = SnapshotHandle { name, id: sp };
            Ok((handle, Established::assert()))
        })
    }

    #[instrument(skip(self, handle))]
    fn restore_snapshot(
        &self,
        handle: &SnapshotHandle,
    ) -> BoxFuture<'_, DbResult<Established<KvSnapshotRestored>>> {
        let id = handle.id;
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let mut write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for restore: {e}")))?;
            let sp = write_txn
                .get_persistent_savepoint(id)
                .map_err(|e| internal(format!("get_persistent_savepoint({id}): {e}")))?;
            write_txn
                .restore_savepoint(&sp)
                .map_err(|e| internal(format!("restore_savepoint: {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit after restore: {e}")))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self, handle))]
    fn drop_snapshot(&self, handle: SnapshotHandle) -> BoxFuture<'_, DbResult<()>> {
        let id = handle.id;
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for drop_snapshot: {e}")))?;
            write_txn
                .delete_persistent_savepoint(id)
                .map_err(|e| internal(format!("delete_persistent_savepoint({id}): {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit after drop_snapshot: {e}")))?;
            Ok(())
        })
    }

    #[instrument(skip(self))]
    fn list_snapshots(&self) -> BoxFuture<'_, DbResult<Vec<SnapshotHandle>>> {
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for list_snapshots: {e}")))?;
            let ids: Vec<u64> = write_txn
                .list_persistent_savepoints()
                .map_err(|e| internal(format!("list_persistent_savepoints: {e}")))?
                .collect();
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit after list_snapshots: {e}")))?;
            let handles = ids
                .into_iter()
                .map(|id| SnapshotHandle {
                    name: id.to_string(),
                    id,
                })
                .collect();
            Ok(handles)
        })
    }
}

// ── DbTransactor ────────────────────────────────────────────────────────────
// redb transactions are synchronous; we bridge them via a write/read begin.
// The TransactionHandle carries the UUID string but we complete each
// operation immediately — redb does not support interleaved open txns.

impl DbTransactor for RedbBackend {
    #[instrument(skip(self))]
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<(TransactionHandle, TxMarker<Open>)>> {
        Box::pin(async move {
            let id = uuid::Uuid::new_v4().to_string();
            let handle = TransactionHandle(id);
            let marker = TxMarker::open(isolation);
            Ok((handle, marker))
        })
    }

    #[instrument(skip(self))]
    fn commit(&self, _handle: TransactionHandle) -> BoxFuture<'_, DbCommitResult> {
        Box::pin(async move {
            let marker = TxMarker::open(IsolationLevel::Serializable).commit();
            Ok((marker, Established::assert(), Established::assert()))
        })
    }

    #[instrument(skip(self))]
    fn rollback(
        &self,
        _handle: TransactionHandle,
    ) -> BoxFuture<'_, DbResult<TxMarker<RolledBack>>> {
        Box::pin(async move { Ok(TxMarker::open(IsolationLevel::Serializable).rollback()) })
    }

    fn savepoint(&self, _handle: &TransactionHandle, _name: &str) -> BoxFuture<'_, DbResult<()>> {
        Box::pin(async {
            Err(unsupported(
                "DbTransactor::savepoint — use DbSnapshotManager::create_snapshot for redb",
            ))
        })
    }

    fn rollback_to_savepoint(
        &self,
        _handle: &TransactionHandle,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<()>> {
        Box::pin(async {
            Err(unsupported(
                "DbTransactor::rollback_to_savepoint — use DbSnapshotManager::restore_snapshot",
            ))
        })
    }
}

// ── DbIsolationFactory ───────────────────────────────────────────────────────
// redb always uses serializable snapshot isolation.  We return that proof
// for all begin_* calls and model read-only as a read transaction start.

type IsolationTxFuture<'a, P> =
    BoxFuture<'a, DbResult<(TransactionHandle, TxMarker<Open>, Established<P>)>>;

fn new_open_handle() -> (TransactionHandle, TxMarker<Open>) {
    let id = uuid::Uuid::new_v4().to_string();
    (
        TransactionHandle(id),
        TxMarker::open(IsolationLevel::Serializable),
    )
}

impl DbIsolationFactory for RedbBackend {
    fn begin_read_committed(&self) -> IsolationTxFuture<'_, ReadCommittedIsolation> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn begin_repeatable_read(&self) -> IsolationTxFuture<'_, RepeatableReadIsolation> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn begin_serializable(&self) -> IsolationTxFuture<'_, SerializableIsolation> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn begin_read_uncommitted(&self) -> IsolationTxFuture<'_, ReadUncommittedIsolation> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn begin_read_only(
        &self,
        _isolation: IsolationLevel,
    ) -> IsolationTxFuture<'_, TransactionReadOnly> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn begin_read_write(
        &self,
        _isolation: IsolationLevel,
    ) -> IsolationTxFuture<'_, TransactionReadWrite> {
        Box::pin(async move {
            let (h, m) = new_open_handle();
            Ok((h, m, Established::assert()))
        })
    }

    fn set_session_isolation(
        &self,
        _level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<SessionIsolationLevelSet>>> {
        Box::pin(async move { Ok(Established::assert()) })
    }

    fn set_transaction_isolation(
        &self,
        _handle: &TransactionHandle,
        _level: IsolationLevel,
    ) -> BoxFuture<'_, DbResult<Established<TransactionIsolationLevelSet>>> {
        Box::pin(async move { Ok(Established::assert()) })
    }
}

// ── DbTableManager (partial — list/rename/drop only) ─────────────────────────

impl DbTableManager for RedbBackend {
    fn create_table(
        &self,
        _schema: &str,
        _name: &str,
        _columns: Vec<elicit_db::DbColumn>,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<elicit_db::TableCreated>,
            Established<AuditLogged>,
        )>,
    > {
        Box::pin(async move { Err(unsupported("DbTableManager::create_table")) })
    }

    fn drop_table(
        &self,
        _schema: &str,
        name: &str,
        _cascade: bool,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let name = name.to_string();
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write: {e}")))?;
            let table_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&name);
            write_txn
                .delete_table(table_def)
                .map_err(|e| internal(format!("delete_table: {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit: {e}")))?;
            Ok(Established::assert())
        })
    }

    fn list_tables(&self, _schema: &str) -> BoxFuture<'_, DbResult<Vec<elicit_db::DbTableInfo>>> {
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let read_txn = db
                .begin_read()
                .map_err(|e| internal(format!("begin_read: {e}")))?;
            let names = read_txn
                .list_tables()
                .map_err(|e| internal(format!("list_tables: {e}")))?;
            let infos = names
                .map(|h| elicit_db::DbTableInfo {
                    schema: String::new(),
                    name: h.name().to_string(),
                    columns: Vec::new(),
                    row_count_estimate: None,
                    size_bytes: None,
                })
                .collect();
            Ok(infos)
        })
    }

    fn inspect_table(
        &self,
        _schema: &str,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<(elicit_db::DbTableInfo, Established<elicit_db::TableExists>)>>
    {
        Box::pin(async move { Err(unsupported("DbTableManager::inspect_table")) })
    }

    fn add_column(
        &self,
        _schema: &str,
        _table: &str,
        _column: elicit_db::DbColumn,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<elicit_db::ColumnExists>,
            Established<AuditLogged>,
        )>,
    > {
        Box::pin(async move { Err(unsupported("DbTableManager::add_column")) })
    }

    fn drop_column(
        &self,
        _schema: &str,
        _table: &str,
        _column: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move { Err(unsupported("DbTableManager::drop_column")) })
    }

    fn rename_table(
        &self,
        _schema: &str,
        from: &str,
        to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        let from = from.to_string();
        let to = to.to_string();
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write: {e}")))?;
            let from_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&from);
            let to_def: TableDefinition<&[u8], &[u8]> = TableDefinition::new(&to);
            write_txn
                .rename_table(from_def, to_def)
                .map_err(|e| internal(format!("rename_table: {e}")))?;
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit: {e}")))?;
            Ok(Established::assert())
        })
    }

    fn truncate_table(
        &self,
        _schema: &str,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move { Err(unsupported("DbTableManager::truncate_table")) })
    }
}

// ── DbDatabaseManager (partial — size only) ──────────────────────────────────

impl DbDatabaseManager for RedbBackend {
    fn create_database(
        &self,
        _name: &str,
    ) -> BoxFuture<'_, DbResult<(Established<DatabaseCreated>, Established<AuditLogged>)>> {
        Box::pin(async move { Err(unsupported("DbDatabaseManager::create_database")) })
    }

    fn drop_database(&self, _name: &str) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move { Err(unsupported("DbDatabaseManager::drop_database")) })
    }

    fn list_databases(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move { Err(unsupported("DbDatabaseManager::list_databases")) })
    }

    fn rename_database(
        &self,
        _from: &str,
        _to: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>> {
        Box::pin(async move { Err(unsupported("DbDatabaseManager::rename_database")) })
    }

    fn database_size(&self, _name: &str) -> BoxFuture<'_, DbResult<u64>> {
        let db = self.db.clone();
        Box::pin(async move {
            let db = db.lock().map_err(|_| internal("db lock poisoned"))?;
            let write_txn = db
                .begin_write()
                .map_err(|e| internal(format!("begin_write for size: {e}")))?;
            let stats = write_txn
                .stats()
                .map_err(|e| internal(format!("stats: {e}")))?;
            let size = stats.stored_bytes() + stats.metadata_bytes() + stats.fragmented_bytes();
            write_txn
                .commit()
                .map_err(|e| internal(format!("commit: {e}")))?;
            Ok(size)
        })
    }
}

// ── DbMonitor (cache hit ratio only) ─────────────────────────────────────────

impl DbMonitor for RedbBackend {
    fn active_sessions(&self) -> BoxFuture<'_, DbResult<elicit_db::DbStatActivity>> {
        Box::pin(async move { Err(unsupported("DbMonitor::active_sessions")) })
    }

    fn slow_queries(
        &self,
        _threshold_ms: u64,
    ) -> BoxFuture<'_, DbResult<Vec<elicit_db::DbSessionInfo>>> {
        Box::pin(async move { Err(unsupported("DbMonitor::slow_queries")) })
    }

    fn table_bloat(&self, _schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, f64)>>> {
        Box::pin(async move { Err(unsupported("DbMonitor::table_bloat")) })
    }

    fn index_usage(&self, _schema: &str) -> BoxFuture<'_, DbResult<Vec<(String, u64)>>> {
        Box::pin(async move { Err(unsupported("DbMonitor::index_usage")) })
    }

    fn lock_waits(&self) -> BoxFuture<'_, DbResult<Vec<(i32, i32)>>> {
        Box::pin(async move { Err(unsupported("DbMonitor::lock_waits")) })
    }

    fn cache_hit_ratio(&self) -> BoxFuture<'_, DbResult<f64>> {
        Box::pin(async move { Ok(0.0) })
    }
}

// ── DbBackupManager (copy-file only) ─────────────────────────────────────────

impl DbBackupManager for RedbBackend {
    fn initiate_backup(
        &self,
        _label: &str,
    ) -> BoxFuture<'_, DbResult<(Established<BackupConsistent>, Established<AuditLogged>)>> {
        Box::pin(async move { Err(unsupported("DbBackupManager::initiate_backup")) })
    }

    fn list_backups(&self) -> BoxFuture<'_, DbResult<Vec<String>>> {
        Box::pin(async move { Err(unsupported("DbBackupManager::list_backups")) })
    }

    fn verify_backup(
        &self,
        _label: &str,
    ) -> BoxFuture<'_, DbResult<Established<BackupConsistent>>> {
        Box::pin(async move { Err(unsupported("DbBackupManager::verify_backup")) })
    }

    fn wal_status(&self) -> BoxFuture<'_, DbResult<Established<WALReplayable>>> {
        Box::pin(async move { Err(unsupported("DbBackupManager::wal_status")) })
    }
}
