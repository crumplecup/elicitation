//! [`DbKvStore`] — typed key-value operations for embedded stores.
//!
//! Provides the KV primitive surface: point lookup, insert, remove, full
//! scan, range scan, and cardinality query.  All operations address a named
//! table within the backend's single database file.
//!
//! Source: redb documentation — Table access and iteration.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{DbResult, DbValue, KvEntry, KvKeyDeleted, KvKeyInserted};

/// Typed key-value operations on named tables within an embedded store.
///
/// Tables are identified by name strings.  Keys and values are represented
/// as [`DbValue`] variants, serialized by the implementation.
pub trait DbKvStore: Send + Sync {
    /// Retrieve the value associated with `key` in `table`, or `None` if absent.
    fn kv_get(&self, table: &str, key: &DbValue) -> BoxFuture<'_, DbResult<Option<DbValue>>>;

    /// Insert or replace the entry for `key` in `table`.
    ///
    /// Returns a proof that the key was inserted.
    fn kv_insert(
        &self,
        table: &str,
        key: DbValue,
        value: DbValue,
    ) -> BoxFuture<'_, DbResult<Established<KvKeyInserted>>>;

    /// Remove the entry for `key` from `table`.
    ///
    /// Returns a proof that the key was deleted.  If the key was not present,
    /// implementations may still return `Ok(Established<KvKeyDeleted>)`.
    fn kv_remove(
        &self,
        table: &str,
        key: &DbValue,
    ) -> BoxFuture<'_, DbResult<Established<KvKeyDeleted>>>;

    /// Return all entries in `table` in ascending key order.
    fn kv_scan(&self, table: &str) -> BoxFuture<'_, DbResult<Vec<KvEntry>>>;

    /// Return entries whose key falls in the range `[from, to)`.
    ///
    /// Both bounds are compared by the backend's natural byte ordering.
    fn kv_range(
        &self,
        table: &str,
        from: &DbValue,
        to: &DbValue,
    ) -> BoxFuture<'_, DbResult<Vec<KvEntry>>>;

    /// Return the number of entries currently stored in `table`.
    fn kv_len(&self, table: &str) -> BoxFuture<'_, DbResult<u64>>;
}
