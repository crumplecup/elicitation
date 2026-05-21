//! Persistent query history for the archive browser.
//!
//! [`HistoryStore`] opens (or creates) a local SQLite database at
//! `~/.config/archive/history.db` and provides append + recall operations.
//! It is used by all frontends to persist the last [`MAX_HISTORY`] queries.

use sqlx::SqlitePool;
use tracing::instrument;

use crate::archive::{
    ArchiveResult, QueryHistoryEntry,
    errors::{ArchiveError, ArchiveErrorKind},
};

/// Maximum number of history entries retained.
pub const MAX_HISTORY: i64 = 200;

/// Handle to the local SQLite history database.
#[derive(Clone)]
pub struct HistoryStore {
    pool: SqlitePool,
}

impl HistoryStore {
    /// Open (or create) the history database at the default path.
    ///
    /// Creates `~/.config/archive/` if it does not exist.
    #[instrument]
    pub async fn open() -> ArchiveResult<Self> {
        let path = history_db_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                    "cannot create history dir: {e}"
                )))
            })?;
        }
        let url = format!("sqlite://{}?mode=rwc", path.display());
        let pool = SqlitePool::connect(&url).await.map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot open history db: {e}"
            )))
        })?;
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS query_history (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                executed_at  TEXT    NOT NULL,
                sql          TEXT    NOT NULL,
                duration_ms  INTEGER NOT NULL DEFAULT 0,
                row_count    INTEGER,
                error        TEXT
            )"#,
        )
        .execute(&pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot init history table: {e}"
            )))
        })?;
        Ok(Self { pool })
    }

    /// Append a new entry and prune old rows beyond the history limit.
    #[instrument(skip(self, sql))]
    pub async fn append(
        &self,
        sql: &str,
        duration_ms: u64,
        row_count: Option<u64>,
        error: Option<&str>,
    ) -> ArchiveResult<()> {
        let now = chrono::Utc::now().to_rfc3339();
        let duration = duration_ms as i64;
        let rc = row_count.map(|v| v as i64);
        sqlx::query(
            "INSERT INTO query_history (executed_at, sql, duration_ms, row_count, error) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&now)
        .bind(sql)
        .bind(duration)
        .bind(rc)
        .bind(error)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot write history: {e}"
            )))
        })?;

        // Prune oldest rows beyond MAX_HISTORY.
        sqlx::query(
            "DELETE FROM query_history WHERE id NOT IN \
             (SELECT id FROM query_history ORDER BY id DESC LIMIT ?)",
        )
        .bind(MAX_HISTORY)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot prune history: {e}"
            )))
        })?;

        Ok(())
    }

    /// Return the most recent `limit` history entries, newest first.
    #[instrument(skip(self))]
    pub async fn recent(&self, limit: i64) -> ArchiveResult<Vec<QueryHistoryEntry>> {
        let rows = sqlx::query_as::<_, HistoryRow>(
            "SELECT id, executed_at, sql, duration_ms, row_count, error \
             FROM query_history ORDER BY id DESC LIMIT ?",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot read history: {e}"
            )))
        })?;

        Ok(rows.into_iter().map(HistoryRow::into_entry).collect())
    }

    /// Fire-and-forget append: spawns a background task so the caller need not
    /// be async.  Errors are silently discarded (history is best-effort).
    pub fn append_spawn(
        &self,
        sql: String,
        duration_ms: u64,
        row_count: Option<u64>,
        error: Option<String>,
    ) {
        let store = self.clone();
        tokio::spawn(async move {
            let _ = store
                .append(&sql, duration_ms, row_count, error.as_deref())
                .await;
        });
    }
}

// ── Internal SQLite row type ──────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct HistoryRow {
    id: i64,
    executed_at: String,
    sql: String,
    duration_ms: i64,
    row_count: Option<i64>,
    error: Option<String>,
}

impl HistoryRow {
    fn into_entry(self) -> QueryHistoryEntry {
        let executed_at = chrono::DateTime::parse_from_rfc3339(&self.executed_at)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());
        QueryHistoryEntry {
            id: self.id,
            executed_at,
            sql: self.sql,
            duration_ms: self.duration_ms as u64,
            row_count: self.row_count.map(|v| v as u64),
            error: self.error,
        }
    }
}

// ── Path helper ───────────────────────────────────────────────────────────────

fn history_db_path() -> ArchiveResult<std::path::PathBuf> {
    let base = dirs::config_dir().ok_or_else(|| {
        ArchiveError::new(ArchiveErrorKind::Frontend(
            "cannot determine config directory".to_string(),
        ))
    })?;
    Ok(base.join("archive").join("history.db"))
}
