//! [`RedbBackend`] — direct redb integration for the `elicit_db` embedded-store traits.
//!
//! Implements the four traits that map naturally onto redb's API:
//!
//! | Trait | redb mapping |
//! |---|---|
//! | [`DbKvStore`] | Auto-transacted per-call KV ops on named tables |
//! | [`DbEmbeddedStore`] | `Database::compact` / `check_integrity` / `stats` |
//! | [`DbTransactor`] | Explicit `WriteTransaction` lifecycle via UUID handles |
//! | [`DbSnapshotManager`] | Persistent savepoints via `WriteTransaction` |

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use elicit_db::{
    DbCommitResult, DbEmbeddedStore, DbError, DbErrorKind, DbKvStore, DbSnapshotManager,
    DbSpatialValue, DbStorageStats, DbTransactor, DbValue, Durable, IsolationLevel, KvEntry,
    KvIntegrityVerified, KvKeyDeleted, KvKeyInserted, KvSnapshotCreated, KvSnapshotRestored,
    KvTableCompacted, Open, RolledBack, SnapshotHandle, TransactionCommitted, TransactionHandle,
    TxMarker,
};
use elicitation::Established;
use futures::future::BoxFuture;
use redb::{ReadableDatabase as _, ReadableTable as _, ReadableTableMetadata as _};
use tracing::instrument;
use uuid::Uuid;

// ── error helpers ─────────────────────────────────────────────────────────────

#[track_caller]
fn err_conn(msg: impl std::fmt::Display) -> DbError {
    DbError::new(DbErrorKind::ConnectionFailed(msg.to_string()))
}

#[track_caller]
fn err_query(msg: impl std::fmt::Display) -> DbError {
    DbError::new(DbErrorKind::QueryFailed(msg.to_string()))
}

#[track_caller]
fn err_txn(msg: impl std::fmt::Display) -> DbError {
    DbError::new(DbErrorKind::TransactionError(msg.to_string()))
}

#[track_caller]
fn err_serial(msg: impl std::fmt::Display) -> DbError {
    DbError::new(DbErrorKind::Serialization(msg.to_string()))
}

// ── DbValue codec ─────────────────────────────────────────────────────────────

fn spatial_to_key(prefix: &str, g: &DbSpatialValue) -> String {
    match g {
        DbSpatialValue::Wkt(s) => format!("{prefix}:{s}"),
        DbSpatialValue::Wkb(b) => format!("{prefix}:wkb:{}", hex::encode(b)),
    }
}

/// Convert a [`DbValue`] to a string key for redb `&str` table keys.
fn dbvalue_to_key(v: &DbValue) -> String {
    match v {
        DbValue::Text(s) => s.clone(),
        DbValue::Int(n) => n.to_string(),
        DbValue::Float(f) => f.to_string(),
        DbValue::Bool(b) => b.to_string(),
        DbValue::Bytes(b) => hex::encode(b),
        DbValue::Null => "null".to_string(),
        DbValue::Json(v) => v.to_string(),
        DbValue::Geometry(g) => spatial_to_key("geo", g),
        DbValue::Geography(g) => spatial_to_key("geog", g),
    }
}

fn encode_value(v: &DbValue) -> Result<String, DbError> {
    serde_json::to_string(v).map_err(|e| err_serial(e))
}

fn decode_value(s: &str) -> Result<DbValue, DbError> {
    serde_json::from_str(s).map_err(|e| err_serial(e))
}

/// Key for the ephemeral savepoint map: `(transaction_handle_string, savepoint_name)`.
type SavepointKey = (String, String);

// ── RedbBackend ───────────────────────────────────────────────────────────────

/// A non-MCP wrapper around `redb::Database` implementing the `elicit_db`
/// embedded-store trait family.
///
/// Obtain via [`RedbBackend::open`] (file-backed) or [`RedbBackend::in_memory`].
pub struct RedbBackend {
    db: Arc<Mutex<redb::Database>>,
    /// Live explicit write transactions keyed by [`TransactionHandle`] UUID string.
    txns: Arc<Mutex<HashMap<String, redb::WriteTransaction>>>,
    /// Ephemeral savepoints per transaction: `(txn_handle, savepoint_name)` → `Savepoint`.
    savepoints: Arc<Mutex<HashMap<SavepointKey, redb::Savepoint>>>,
    /// Persistent snapshot name → savepoint ID (`u64`).
    snapshot_ids: Arc<Mutex<HashMap<String, u64>>>,
}

impl RedbBackend {
    /// Open or create a redb file at `path`.
    #[instrument(fields(%path))]
    pub fn open(path: &str) -> Result<Self, DbError> {
        let db = redb::Database::create(path).map_err(|e| err_conn(e))?;
        Ok(Self::wrap(db))
    }

    /// Create a transient in-memory redb instance (for tests / ephemeral sessions).
    pub fn in_memory() -> Result<Self, DbError> {
        let db = redb::Database::builder()
            .create_with_backend(redb::backends::InMemoryBackend::new())
            .map_err(|e| err_conn(e))?;
        Ok(Self::wrap(db))
    }

    fn wrap(db: redb::Database) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            txns: Arc::new(Mutex::new(HashMap::new())),
            savepoints: Arc::new(Mutex::new(HashMap::new())),
            snapshot_ids: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn lock_db(&self) -> Result<std::sync::MutexGuard<'_, redb::Database>, DbError> {
        self.db
            .lock()
            .map_err(|e| err_conn(format!("db lock poisoned: {e}")))
    }

    fn lock_txns(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, HashMap<String, redb::WriteTransaction>>, DbError> {
        self.txns
            .lock()
            .map_err(|e| err_txn(format!("txn lock poisoned: {e}")))
    }

    fn lock_savepoints(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, HashMap<SavepointKey, redb::Savepoint>>, DbError> {
        self.savepoints
            .lock()
            .map_err(|e| err_txn(format!("savepoint lock poisoned: {e}")))
    }

    fn lock_snapshot_ids(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, HashMap<String, u64>>, DbError> {
        self.snapshot_ids
            .lock()
            .map_err(|e| err_txn(format!("snapshot lock poisoned: {e}")))
    }
}

// ── DbKvStore ─────────────────────────────────────────────────────────────────

impl DbKvStore for RedbBackend {
    #[instrument(skip(self, key), fields(%table))]
    fn kv_get(
        &self,
        table: &str,
        key: &DbValue,
    ) -> BoxFuture<'_, elicit_db::DbResult<Option<DbValue>>> {
        let table = table.to_owned();
        let key_str = dbvalue_to_key(key);
        Box::pin(async move {
            let db = self.lock_db()?;
            let rtxn = db.begin_read().map_err(|e| err_query(e))?;
            let def = redb::TableDefinition::<&str, &str>::new(&table);
            match rtxn.open_table(def) {
                Ok(t) => match t.get(key_str.as_str()).map_err(|e| err_query(e))? {
                    Some(ag) => Ok(Some(decode_value(ag.value())?)),
                    None => Ok(None),
                },
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(None),
                Err(e) => Err(err_query(e)),
            }
        })
    }

    #[instrument(skip(self, value), fields(%table))]
    fn kv_insert(
        &self,
        table: &str,
        key: DbValue,
        value: DbValue,
    ) -> BoxFuture<'_, elicit_db::DbResult<Established<KvKeyInserted>>> {
        let table = table.to_owned();
        Box::pin(async move {
            let key_str = dbvalue_to_key(&key);
            let val_str = encode_value(&value)?;
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_query(e))?;
            {
                let def = redb::TableDefinition::<&str, &str>::new(&table);
                let mut t = txn.open_table(def).map_err(|e| err_query(e))?;
                t.insert(key_str.as_str(), val_str.as_str())
                    .map_err(|e| err_query(e))?;
            }
            txn.commit().map_err(|e| err_query(e))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self, key), fields(%table))]
    fn kv_remove(
        &self,
        table: &str,
        key: &DbValue,
    ) -> BoxFuture<'_, elicit_db::DbResult<Established<KvKeyDeleted>>> {
        let table = table.to_owned();
        let key_str = dbvalue_to_key(key);
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_query(e))?;
            {
                let def = redb::TableDefinition::<&str, &str>::new(&table);
                let mut t = txn.open_table(def).map_err(|e| err_query(e))?;
                t.remove(key_str.as_str()).map_err(|e| err_query(e))?;
            }
            txn.commit().map_err(|e| err_query(e))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self), fields(%table))]
    fn kv_scan(&self, table: &str) -> BoxFuture<'_, elicit_db::DbResult<Vec<KvEntry>>> {
        let table = table.to_owned();
        Box::pin(async move {
            let db = self.lock_db()?;
            let rtxn = db.begin_read().map_err(|e| err_query(e))?;
            let def = redb::TableDefinition::<&str, &str>::new(&table);
            let t = match rtxn.open_table(def) {
                Ok(t) => t,
                Err(redb::TableError::TableDoesNotExist(_)) => return Ok(vec![]),
                Err(e) => return Err(err_query(e)),
            };
            let mut entries = Vec::new();
            for item in t.iter().map_err(|e| err_query(e))? {
                let (k, v) = item.map_err(|e| err_query(e))?;
                let key = DbValue::Text(k.value().to_owned());
                let value = decode_value(v.value())?;
                entries.push(KvEntry { key, value });
            }
            Ok(entries)
        })
    }

    #[instrument(skip(self, from, to), fields(%table))]
    fn kv_range(
        &self,
        table: &str,
        from: &DbValue,
        to: &DbValue,
    ) -> BoxFuture<'_, elicit_db::DbResult<Vec<KvEntry>>> {
        let table = table.to_owned();
        let from_str = dbvalue_to_key(from);
        let to_str = dbvalue_to_key(to);
        Box::pin(async move {
            let db = self.lock_db()?;
            let rtxn = db.begin_read().map_err(|e| err_query(e))?;
            let def = redb::TableDefinition::<&str, &str>::new(&table);
            let t = match rtxn.open_table(def) {
                Ok(t) => t,
                Err(redb::TableError::TableDoesNotExist(_)) => return Ok(vec![]),
                Err(e) => return Err(err_query(e)),
            };
            let from_ref: &str = &from_str;
            let to_ref: &str = &to_str;
            let mut entries = Vec::new();
            for item in t.range(from_ref..to_ref).map_err(|e| err_query(e))? {
                let (k, v) = item.map_err(|e| err_query(e))?;
                let key = DbValue::Text(k.value().to_owned());
                let value = decode_value(v.value())?;
                entries.push(KvEntry { key, value });
            }
            Ok(entries)
        })
    }

    #[instrument(skip(self), fields(%table))]
    fn kv_len(&self, table: &str) -> BoxFuture<'_, elicit_db::DbResult<u64>> {
        let table = table.to_owned();
        Box::pin(async move {
            let db = self.lock_db()?;
            let rtxn = db.begin_read().map_err(|e| err_query(e))?;
            let def = redb::TableDefinition::<&str, &str>::new(&table);
            match rtxn.open_table(def) {
                Ok(t) => Ok(t.len().map_err(|e| err_query(e))?),
                Err(redb::TableError::TableDoesNotExist(_)) => Ok(0),
                Err(e) => Err(err_query(e)),
            }
        })
    }
}

// ── DbEmbeddedStore ───────────────────────────────────────────────────────────

impl DbEmbeddedStore for RedbBackend {
    #[instrument(skip(self))]
    fn compact(&self) -> BoxFuture<'_, elicit_db::DbResult<Established<KvTableCompacted>>> {
        Box::pin(async move {
            let mut db = self.lock_db()?;
            db.compact().map_err(|e| err_query(e))?;
            Ok(Established::assert())
        })
    }

    #[instrument(skip(self))]
    fn check_integrity(
        &self,
    ) -> BoxFuture<'_, elicit_db::DbResult<Established<KvIntegrityVerified>>> {
        Box::pin(async move {
            let mut db = self.lock_db()?;
            let ok = db.check_integrity().map_err(|e| err_query(e))?;
            if ok {
                Ok(Established::assert())
            } else {
                Err(DbError::new(DbErrorKind::QueryFailed(
                    "integrity check failed".to_string(),
                )))
            }
        })
    }

    #[instrument(skip(self))]
    fn storage_stats(&self) -> BoxFuture<'_, elicit_db::DbResult<DbStorageStats>> {
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_query(e))?;
            let stats = txn.stats().map_err(|e| err_query(e))?;
            let table_count = txn.list_tables().map_err(|e| err_query(e))?.count();
            drop(txn);
            let cache = db.cache_stats();
            let total_reads = cache.read_hits() + cache.read_misses();
            let hit_ratio = if total_reads > 0 {
                cache.read_hits() as f64 / total_reads as f64
            } else {
                0.0
            };
            Ok(DbStorageStats {
                stored_bytes: stats.stored_bytes(),
                fragmented_bytes: stats.fragmented_bytes(),
                metadata_bytes: stats.metadata_bytes(),
                table_count,
                cache_hit_ratio: hit_ratio,
            })
        })
    }
}

// ── DbTransactor ──────────────────────────────────────────────────────────────

impl DbTransactor for RedbBackend {
    #[instrument(skip(self), fields(?isolation))]
    fn begin(
        &self,
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, elicit_db::DbResult<(TransactionHandle, TxMarker<Open>)>> {
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_txn(e))?;
            let handle = Uuid::new_v4().to_string();
            self.lock_txns()?.insert(handle.clone(), txn);
            Ok((TransactionHandle(handle), TxMarker::open(isolation)))
        })
    }

    #[instrument(skip(self), fields(handle = %handle.0))]
    fn commit(&self, handle: TransactionHandle) -> BoxFuture<'_, DbCommitResult> {
        Box::pin(async move {
            let txn = self
                .lock_txns()?
                .remove(&handle.0)
                .ok_or_else(|| err_txn(format!("unknown txn: {}", handle.0)))?;
            let marker = TxMarker::<Open>::open(IsolationLevel::Serializable).commit();
            txn.commit().map_err(|e| err_txn(e))?;
            Ok((
                marker,
                Established::<TransactionCommitted>::assert(),
                Established::<Durable>::assert(),
            ))
        })
    }

    #[instrument(skip(self), fields(handle = %handle.0))]
    fn rollback(
        &self,
        handle: TransactionHandle,
    ) -> BoxFuture<'_, elicit_db::DbResult<TxMarker<RolledBack>>> {
        Box::pin(async move {
            let txn = self
                .lock_txns()?
                .remove(&handle.0)
                .ok_or_else(|| err_txn(format!("unknown txn: {}", handle.0)))?;
            drop(txn);
            Ok(TxMarker::<Open>::open(IsolationLevel::Serializable).rollback())
        })
    }

    #[instrument(skip(self, handle), fields(%name))]
    fn savepoint(
        &self,
        handle: &TransactionHandle,
        name: &str,
    ) -> BoxFuture<'_, elicit_db::DbResult<()>> {
        let handle = handle.0.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let mut txns = self.lock_txns()?;
            let txn = txns
                .get_mut(&handle)
                .ok_or_else(|| err_txn(format!("unknown txn: {handle}")))?;
            let sp = txn
                .ephemeral_savepoint()
                .map_err(|e| err_txn(format!("ephemeral_savepoint: {e}")))?;
            self.lock_savepoints()?.insert((handle, name), sp);
            Ok(())
        })
    }

    #[instrument(skip(self, handle), fields(%name))]
    fn rollback_to_savepoint(
        &self,
        handle: &TransactionHandle,
        name: &str,
    ) -> BoxFuture<'_, elicit_db::DbResult<()>> {
        let handle = handle.0.clone();
        let name = name.to_owned();
        Box::pin(async move {
            let sp = self
                .lock_savepoints()?
                .remove(&(handle.clone(), name))
                .ok_or_else(|| err_txn("savepoint not found"))?;
            let mut txns = self.lock_txns()?;
            let txn = txns
                .get_mut(&handle)
                .ok_or_else(|| err_txn(format!("unknown txn: {handle}")))?;
            txn.restore_savepoint(&sp)
                .map_err(|e| err_txn(format!("restore_savepoint: {e}")))?;
            Ok(())
        })
    }
}

// ── DbSnapshotManager ─────────────────────────────────────────────────────────

impl DbSnapshotManager for RedbBackend {
    /// Create a durable persistent savepoint and register it under `name`.
    #[instrument(skip(self), fields(%name))]
    fn create_snapshot(
        &self,
        name: &str,
    ) -> BoxFuture<'_, elicit_db::DbResult<(SnapshotHandle, Established<KvSnapshotCreated>)>> {
        let name = name.to_owned();
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_txn(e))?;
            let id = txn
                .persistent_savepoint()
                .map_err(|e| err_txn(format!("persistent_savepoint: {e}")))?;
            txn.commit().map_err(|e| err_txn(e))?;
            self.lock_snapshot_ids()?.insert(name.clone(), id);
            Ok((SnapshotHandle { name, id }, Established::assert()))
        })
    }

    /// Restore the database to the state captured in `handle`.
    fn restore_snapshot(
        &self,
        handle: &SnapshotHandle,
    ) -> BoxFuture<'_, elicit_db::DbResult<Established<KvSnapshotRestored>>> {
        let id = handle.id;
        Box::pin(async move {
            let db = self.lock_db()?;
            let mut txn = db.begin_write().map_err(|e| err_txn(e))?;
            let sp = txn
                .get_persistent_savepoint(id)
                .map_err(|e| err_txn(format!("get_persistent_savepoint: {e}")))?;
            txn.restore_savepoint(&sp)
                .map_err(|e| err_txn(format!("restore_savepoint: {e}")))?;
            txn.commit().map_err(|e| err_txn(e))?;
            Ok(Established::assert())
        })
    }

    /// Release `handle`, allowing the snapshot's storage to be reclaimed.
    #[instrument(skip(self, handle), fields(handle_name = %handle.name))]
    fn drop_snapshot(&self, handle: SnapshotHandle) -> BoxFuture<'_, elicit_db::DbResult<()>> {
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_txn(e))?;
            txn.delete_persistent_savepoint(handle.id)
                .map_err(|e| err_txn(format!("delete_persistent_savepoint: {e}")))?;
            txn.commit().map_err(|e| err_txn(e))?;
            self.lock_snapshot_ids()?.remove(&handle.name);
            Ok(())
        })
    }

    /// List all durable snapshots tracked by this backend.
    #[instrument(skip(self))]
    fn list_snapshots(&self) -> BoxFuture<'_, elicit_db::DbResult<Vec<SnapshotHandle>>> {
        Box::pin(async move {
            let db = self.lock_db()?;
            let txn = db.begin_write().map_err(|e| err_txn(e))?;
            let ids: Vec<u64> = txn
                .list_persistent_savepoints()
                .map_err(|e| err_txn(format!("list_persistent_savepoints: {e}")))?
                .collect();
            drop(txn);
            let snapshot_ids = self.lock_snapshot_ids()?;
            let reverse: HashMap<u64, &str> = snapshot_ids
                .iter()
                .map(|(n, &id)| (id, n.as_str()))
                .collect();
            let handles = ids
                .into_iter()
                .map(|id| {
                    let name = reverse
                        .get(&id)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| id.to_string());
                    SnapshotHandle { name, id }
                })
                .collect();
            Ok(handles)
        })
    }
}
