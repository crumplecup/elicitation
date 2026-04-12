//! Navigation tree for the archive TUI.
//!
//! [`NavTree`] is the live data model fed to the ratatui frontend.  It holds
//! the pre-loaded schema/table list and is rebuilt by [`build_nav_tree`]
//! before the event loop starts.

use tracing::instrument;

use crate::archive::{
    ArchiveDbBackend, ArchiveResult, BackendKind, TableDescriptor, TableType,
    errors::{ArchiveError, ArchiveErrorKind},
};
use elicit_db::{DbSchemaManager, DbServerAdmin, DbTableManager};

// ── Data model ────────────────────────────────────────────────────────────────

/// A schema together with its pre-loaded table list.
#[derive(Debug, Clone)]
pub struct SchemaEntry {
    /// Schema name.
    pub name: String,
    /// Owning role.
    pub owner: String,
    /// Tables/views in this schema.
    pub tables: Vec<TableDescriptor>,
}

/// Pre-loaded navigation tree passed to the ratatui frontend.
#[derive(Debug, Clone)]
pub struct NavTree {
    /// Database / catalog name (first schema name or the supplied db name).
    pub db_name: String,
    /// Server version string.
    pub version: Option<String>,
    /// Detected backend technology.
    pub backend: BackendKind,
    /// Schemas in this database (in query order).
    pub schemas: Vec<SchemaEntry>,
}

impl NavTree {
    /// Construct a static demo tree (no live connection required).
    pub fn demo() -> Self {
        use crate::archive::ColumnDescriptor;
        let col = |n: &str, t: &str| ColumnDescriptor {
            name: n.to_string(),
            sql_type: t.to_string(),
            nullable: true,
            is_primary_key: false,
            is_foreign_key: false,
            default_value: None,
            is_spatial: false,
        };
        Self {
            db_name: "archive_demo".to_string(),
            version: Some("PostgreSQL 15.0 (demo)".to_string()),
            backend: BackendKind::Postgres,
            schemas: vec![
                SchemaEntry {
                    name: "public".to_string(),
                    owner: "postgres".to_string(),
                    tables: vec![
                        TableDescriptor {
                            schema: "public".to_string(),
                            table_name: "users".to_string(),
                            columns: vec![col("id", "int4"), col("email", "text")],
                            estimated_rows: Some(42),
                            table_type: TableType::Table,
                        },
                        TableDescriptor {
                            schema: "public".to_string(),
                            table_name: "sessions".to_string(),
                            columns: vec![
                                col("id", "uuid"),
                                col("user_id", "int4"),
                                col("created_at", "timestamptz"),
                            ],
                            estimated_rows: Some(128),
                            table_type: TableType::Table,
                        },
                        TableDescriptor {
                            schema: "public".to_string(),
                            table_name: "user_sessions".to_string(),
                            columns: vec![col("id", "int4"), col("email", "text")],
                            estimated_rows: None,
                            table_type: TableType::View,
                        },
                    ],
                },
                SchemaEntry {
                    name: "auth".to_string(),
                    owner: "postgres".to_string(),
                    tables: vec![TableDescriptor {
                        schema: "auth".to_string(),
                        table_name: "roles".to_string(),
                        columns: vec![col("id", "int4"), col("name", "text")],
                        estimated_rows: Some(5),
                        table_type: TableType::Table,
                    }],
                },
                SchemaEntry {
                    name: "pg_catalog".to_string(),
                    owner: "postgres".to_string(),
                    tables: vec![],
                },
            ],
        }
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Build a [`NavTree`] by querying the live database.
///
/// Loads all schemas and their tables in one pass before the event loop
/// starts so the TUI never blocks on I/O during interaction.
#[instrument(skip(backend))]
pub async fn build_nav_tree(backend: &ArchiveDbBackend, url: &str) -> ArchiveResult<NavTree> {
    let version = backend
        .server_version()
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Query(e.to_string())))?;

    let schema_names = backend
        .list_schemas()
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Query(e.to_string())))?;

    let mut schemas: Vec<SchemaEntry> = Vec::with_capacity(schema_names.len());

    for sname in &schema_names {
        let tables = backend
            .list_tables(sname)
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|t| TableDescriptor::from_db_table_info(&t))
            .collect();

        let owner = backend
            .schema_info(sname)
            .await
            .map(|s| s.owner.clone())
            .unwrap_or_else(|_| "unknown".to_string());

        schemas.push(SchemaEntry {
            name: sname.clone(),
            owner,
            tables,
        });
    }

    let db_name = schema_names
        .first()
        .cloned()
        .unwrap_or_else(|| "archive".to_string());

    Ok(NavTree {
        db_name,
        version: Some(version),
        backend: BackendKind::from_url(url),
        schemas,
    })
}
