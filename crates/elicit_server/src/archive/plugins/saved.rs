//! Persistent saved-query snippets for the archive browser.
//!
//! [`SavedQueryStore`] opens (or creates) the same local SQLite database used
//! by [`crate::archive::plugins::history::HistoryStore`] and manages a
//! `saved_queries` table.  Snippets are never pruned — the user manages them
//! manually via delete.

use sqlx::SqlitePool;
use tracing::instrument;

use crate::archive::{
    ArchiveResult, SavedQuery,
    errors::{ArchiveError, ArchiveErrorKind},
};

/// Handle to the saved-queries table in the local SQLite database.
#[derive(Clone)]
pub struct SavedQueryStore {
    pool: SqlitePool,
}

impl SavedQueryStore {
    /// Open (or create) the database and ensure the `saved_queries` table
    /// exists.  Reuses the same path as [`crate::archive::HistoryStore`].
    #[instrument]
    pub async fn open() -> ArchiveResult<Self> {
        let path = saved_db_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                    "cannot create saved-query dir: {e}"
                )))
            })?;
        }
        let url = format!("sqlite://{}?mode=rwc", path.display());
        let pool = SqlitePool::connect(&url).await.map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot open saved-query db: {e}"
            )))
        })?;
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS saved_queries (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                name       TEXT    NOT NULL,
                sql        TEXT    NOT NULL,
                created_at TEXT    NOT NULL,
                updated_at TEXT    NOT NULL
            )"#,
        )
        .execute(&pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot init saved_queries table: {e}"
            )))
        })?;
        Ok(Self { pool })
    }

    /// Insert or update a saved query by name.
    ///
    /// If a row with the same `name` exists it is updated in place; otherwise
    /// a new row is inserted.
    #[instrument(skip(self, sql))]
    pub async fn save(&self, name: &str, sql: &str) -> ArchiveResult<()> {
        let now = chrono::Utc::now().to_rfc3339();
        // Upsert: update existing row or insert new one.
        let existing: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM saved_queries WHERE name = ?")
                .bind(name)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                        "cannot query saved queries: {e}"
                    )))
                })?;
        if let Some((id,)) = existing {
            sqlx::query("UPDATE saved_queries SET sql = ?, updated_at = ? WHERE id = ?")
                .bind(sql)
                .bind(&now)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                        "cannot update saved query: {e}"
                    )))
                })?;
        } else {
            sqlx::query(
                "INSERT INTO saved_queries (name, sql, created_at, updated_at) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(name)
            .bind(sql)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                    "cannot insert saved query: {e}"
                )))
            })?;
        }
        Ok(())
    }

    /// Delete a saved query by its row ID.
    #[instrument(skip(self))]
    pub async fn delete(&self, id: i64) -> ArchiveResult<()> {
        sqlx::query("DELETE FROM saved_queries WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                    "cannot delete saved query: {e}"
                )))
            })?;
        Ok(())
    }

    /// Return all saved queries ordered by name.
    #[instrument(skip(self))]
    pub async fn all(&self) -> ArchiveResult<Vec<SavedQuery>> {
        let rows = sqlx::query_as::<_, SavedRow>(
            "SELECT id, name, sql, created_at, updated_at \
             FROM saved_queries ORDER BY name ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            ArchiveError::new(ArchiveErrorKind::Frontend(format!(
                "cannot read saved queries: {e}"
            )))
        })?;
        Ok(rows.into_iter().map(SavedRow::into_query).collect())
    }

    /// Fire-and-forget save: spawns a background task.  Errors are silently
    /// discarded (persistence is best-effort).
    pub fn save_spawn(&self, name: String, sql: String) {
        let store = self.clone();
        tokio::spawn(async move {
            let _ = store.save(&name, &sql).await;
        });
    }

    /// Fire-and-forget delete: spawns a background task.
    pub fn delete_spawn(&self, id: i64) {
        let store = self.clone();
        tokio::spawn(async move {
            let _ = store.delete(id).await;
        });
    }
}

// ── Internal SQLite row type ──────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct SavedRow {
    id: i64,
    name: String,
    sql: String,
    created_at: String,
    updated_at: String,
}

impl SavedRow {
    fn into_query(self) -> SavedQuery {
        let parse = |s: &str| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now())
        };
        SavedQuery {
            id: self.id,
            name: self.name,
            sql: self.sql,
            created_at: parse(&self.created_at),
            updated_at: parse(&self.updated_at),
        }
    }
}

// ── Path helper ───────────────────────────────────────────────────────────────

fn saved_db_path() -> ArchiveResult<std::path::PathBuf> {
    let base = dirs::config_dir().ok_or_else(|| {
        ArchiveError::new(ArchiveErrorKind::Frontend(
            "cannot determine config directory".to_string(),
        ))
    })?;
    // Reuse the same file as HistoryStore.
    Ok(base.join("archive").join("history.db"))
}
