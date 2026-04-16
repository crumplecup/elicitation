//! Navigation tree for the archive TUI.
//!
//! [`NavTree`] is the live data model fed to the ratatui frontend.  It holds
//! the pre-loaded schema/table list and is rebuilt by [`build_nav_tree`]
//! before the event loop starts.

use std::collections::BTreeMap;

use sqlx::Row;
use sqlx::any::AnyRow;
use tracing::instrument;

use crate::archive::{
    ArchiveDbBackend, ArchiveResult, BackendKind, CompositeTypeAttribute, CompositeTypeDescriptor,
    DomainDescriptor, EnumDescriptor, FunctionDescriptor, FunctionVolatility, SequenceDescriptor,
    TableDescriptor, TableType, TriggerDescriptor, TriggerEvents,
    errors::{ArchiveError, ArchiveErrorKind},
};
use elicit_db::{DbSchemaManager, DbServerAdmin, DbTableManager};

// ── Data model ────────────────────────────────────────────────────────────────

/// A schema together with its pre-loaded table list and Phase 4 object types.
#[derive(Debug, Clone)]
pub struct SchemaEntry {
    /// Schema name.
    pub name: String,
    /// Owning role.
    pub owner: String,
    /// Tables/views in this schema.
    pub tables: Vec<TableDescriptor>,
    /// Functions and procedures (PostgreSQL only; empty on other backends).
    pub functions: Vec<FunctionDescriptor>,
    /// Sequences (PostgreSQL only; empty on other backends).
    pub sequences: Vec<SequenceDescriptor>,
    /// Enum types (PostgreSQL only; empty on other backends).
    pub enums: Vec<EnumDescriptor>,
    /// Domain types (PostgreSQL only; empty on other backends).
    pub domains: Vec<DomainDescriptor>,
    /// Composite types (PostgreSQL only; empty on other backends).
    pub composites: Vec<CompositeTypeDescriptor>,
    /// Triggers defined on tables in this schema (PostgreSQL only; empty on other backends).
    pub triggers: Vec<TriggerDescriptor>,
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
                    functions: vec![],
                    sequences: vec![],
                    enums: vec![],
                    domains: vec![],
                    composites: vec![],
                    triggers: vec![],
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
                    functions: vec![],
                    sequences: vec![],
                    enums: vec![],
                    domains: vec![],
                    composites: vec![],
                    triggers: vec![],
                },
                SchemaEntry {
                    name: "pg_catalog".to_string(),
                    owner: "postgres".to_string(),
                    tables: vec![],
                    functions: vec![],
                    sequences: vec![],
                    enums: vec![],
                    domains: vec![],
                    composites: vec![],
                    triggers: vec![],
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
/// For PostgreSQL backends, also loads Phase 4 objects (functions, sequences,
/// and user-defined types).  Non-Postgres backends gracefully receive empty
/// Vecs for those fields.
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

    // Phase 4: create a one-off pool for Postgres-specific queries.
    let pg_pool: Option<sqlx::AnyPool> = if BackendKind::from_url(url) == BackendKind::Postgres {
        sqlx::any::install_default_drivers();
        sqlx::any::AnyPoolOptions::new()
            .max_connections(3)
            .connect(url)
            .await
            .ok()
    } else {
        None
    };

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

        let (functions, sequences, enums, domains, composites, triggers) =
            if let Some(pool) = &pg_pool {
                let fns = fetch_pg_functions(pool, sname).await;
                let seqs = fetch_pg_sequences(pool, sname).await;
                let (en, dom, comp) = fetch_pg_types(pool, sname).await;
                let trigs = fetch_pg_triggers(pool, sname).await;
                (fns, seqs, en, dom, comp, trigs)
            } else {
                (vec![], vec![], vec![], vec![], vec![], vec![])
            };

        schemas.push(SchemaEntry {
            name: sname.clone(),
            owner,
            tables,
            functions,
            sequences,
            enums,
            domains,
            composites,
            triggers,
        });
    }

    if let Some(pool) = pg_pool {
        pool.close().await;
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

async fn fetch_pg_functions(pool: &sqlx::AnyPool, schema: &str) -> Vec<FunctionDescriptor> {
    let rows: Vec<AnyRow> = sqlx::query(
        "SELECT p.oid::bigint, p.proname, \
                pg_get_function_arguments(p.oid), \
                pg_get_function_result(p.oid), \
                l.lanname, \
                CASE p.provolatile \
                  WHEN 'i' THEN 'immutable' \
                  WHEN 's' THEN 'stable' \
                  ELSE 'volatile' END, \
                p.prokind = 'p', \
                left(pg_get_functiondef(p.oid), 512) \
         FROM pg_proc p \
         JOIN pg_namespace n ON n.oid = p.pronamespace \
         JOIN pg_language l  ON l.oid = p.prolang \
         WHERE n.nspname = $1 \
           AND p.prokind IN ('f','p') \
         ORDER BY p.proname",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.iter()
        .map(|row| {
            let vol_str: String = row.try_get::<String, _>(5).unwrap_or_default();
            let volatility = match vol_str.as_str() {
                "immutable" => FunctionVolatility::Immutable,
                "stable" => FunctionVolatility::Stable,
                _ => FunctionVolatility::Volatile,
            };
            FunctionDescriptor {
                oid: row.try_get::<i64, _>(0).unwrap_or_default(),
                schema: schema.to_string(),
                name: row.try_get::<String, _>(1).unwrap_or_default(),
                arguments: row.try_get::<String, _>(2).unwrap_or_default(),
                return_type: row.try_get::<String, _>(3).unwrap_or_default(),
                language: row.try_get::<String, _>(4).unwrap_or_default(),
                volatility,
                is_procedure: row.try_get::<bool, _>(6).unwrap_or(false),
                body_preview: row.try_get::<String, _>(7).unwrap_or_default(),
            }
        })
        .collect()
}

async fn fetch_pg_sequences(pool: &sqlx::AnyPool, schema: &str) -> Vec<SequenceDescriptor> {
    let rows: Vec<AnyRow> = sqlx::query(
        "SELECT s.sequence_name, \
                s.start_value::bigint, \
                s.increment::bigint, \
                s.minimum_value::bigint, \
                s.maximum_value::bigint, \
                s.cycle_option = 'YES', \
                d.refobjid::text \
         FROM information_schema.sequences s \
         LEFT JOIN pg_class c ON c.relname = s.sequence_name \
              AND c.relnamespace = (SELECT oid FROM pg_namespace WHERE nspname = $1) \
         LEFT JOIN pg_depend d ON d.objid = c.oid \
              AND d.classid = 'pg_class'::regclass \
              AND d.deptype = 'a' \
         WHERE s.sequence_schema = $1 \
         ORDER BY s.sequence_name",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.iter()
        .map(|row| SequenceDescriptor {
            schema: schema.to_string(),
            name: row.try_get::<String, _>(0).unwrap_or_default(),
            current_value: None,
            start_value: row.try_get::<i64, _>(1).unwrap_or(1),
            increment_by: row.try_get::<i64, _>(2).unwrap_or(1),
            min_value: row.try_get::<i64, _>(3).unwrap_or(1),
            max_value: row.try_get::<i64, _>(4).unwrap_or(i64::MAX),
            cycle: row.try_get::<bool, _>(5).unwrap_or(false),
            owned_by: row.try_get::<String, _>(6).ok().filter(|s| !s.is_empty()),
        })
        .collect()
}

async fn fetch_pg_types(
    pool: &sqlx::AnyPool,
    schema: &str,
) -> (
    Vec<EnumDescriptor>,
    Vec<DomainDescriptor>,
    Vec<CompositeTypeDescriptor>,
) {
    // ── enums ──────────────────────────────────────────────────────────────────
    let enum_rows: Vec<AnyRow> = sqlx::query(
        "SELECT t.typname, e.enumlabel \
         FROM pg_type t \
         JOIN pg_namespace n ON n.oid = t.typnamespace \
         JOIN pg_enum e      ON e.enumtypid = t.oid \
         WHERE n.nspname = $1 AND t.typtype = 'e' \
         ORDER BY t.typname, e.enumsortorder",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut enum_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in &enum_rows {
        let name: String = row.try_get::<String, _>(0).unwrap_or_default();
        let label: String = row.try_get::<String, _>(1).unwrap_or_default();
        enum_map.entry(name).or_default().push(label);
    }
    let enums: Vec<EnumDescriptor> = enum_map
        .into_iter()
        .map(|(name, labels)| EnumDescriptor {
            schema: schema.to_string(),
            name,
            labels,
        })
        .collect();

    // ── domains ────────────────────────────────────────────────────────────────
    let domain_rows: Vec<AnyRow> = sqlx::query(
        "SELECT t.typname, bt.typname, t.typnotnull, \
                d.adsrc, \
                array_to_string(array_agg(c.consrc ORDER BY c.conname), '; ') \
         FROM pg_type t \
         JOIN pg_namespace n ON n.oid = t.typnamespace \
         JOIN pg_type bt     ON bt.oid = t.typbasetype \
         LEFT JOIN pg_attrdef d  ON d.adtypid = t.oid \
         LEFT JOIN pg_constraint c ON c.contypid = t.oid \
         WHERE n.nspname = $1 AND t.typtype = 'd' \
         GROUP BY t.typname, bt.typname, t.typnotnull, d.adsrc \
         ORDER BY t.typname",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let domains: Vec<DomainDescriptor> = domain_rows
        .iter()
        .map(|row| {
            let checks_str: Option<String> = row.try_get::<String, _>(4).ok();
            DomainDescriptor {
                schema: schema.to_string(),
                name: row.try_get::<String, _>(0).unwrap_or_default(),
                base_type: row.try_get::<String, _>(1).unwrap_or_default(),
                not_null: row.try_get::<bool, _>(2).unwrap_or(false),
                default_expr: row.try_get::<String, _>(3).ok().filter(|s| !s.is_empty()),
                check_constraints: checks_str
                    .unwrap_or_default()
                    .split("; ")
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned)
                    .collect(),
            }
        })
        .collect();

    // ── composite types ────────────────────────────────────────────────────────
    let comp_rows: Vec<AnyRow> = sqlx::query(
        "SELECT t.typname, a.attname, bt.typname \
         FROM pg_type t \
         JOIN pg_namespace n  ON n.oid = t.typnamespace \
         JOIN pg_class   c    ON c.oid = t.typrelid \
         JOIN pg_attribute a  ON a.attrelid = c.oid AND a.attnum > 0 \
         JOIN pg_type bt      ON bt.oid = a.atttypid \
         WHERE n.nspname = $1 AND t.typtype = 'c' AND c.relkind = 'c' \
         ORDER BY t.typname, a.attnum",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut comp_map: BTreeMap<String, Vec<CompositeTypeAttribute>> = BTreeMap::new();
    for row in &comp_rows {
        let type_name: String = row.try_get::<String, _>(0).unwrap_or_default();
        let attr_name: String = row.try_get::<String, _>(1).unwrap_or_default();
        let attr_type: String = row.try_get::<String, _>(2).unwrap_or_default();
        comp_map
            .entry(type_name)
            .or_default()
            .push(CompositeTypeAttribute {
                name: attr_name,
                type_name: attr_type,
            });
    }
    let composites: Vec<CompositeTypeDescriptor> = comp_map
        .into_iter()
        .map(|(name, attributes)| CompositeTypeDescriptor {
            schema: schema.to_string(),
            name,
            attributes,
        })
        .collect();

    (enums, domains, composites)
}

// ── Trigger fetch ─────────────────────────────────────────────────────────────

async fn fetch_pg_triggers(pool: &sqlx::AnyPool, schema: &str) -> Vec<TriggerDescriptor> {
    let rows: Vec<AnyRow> = sqlx::query(
        "SELECT t.trigger_name, t.event_object_table, t.action_timing, \
                string_agg(t.event_manipulation, ',' ORDER BY t.event_manipulation), \
                t.action_orientation = 'ROW', \
                COALESCE(t.action_statement, ''), \
                COALESCE(t.trigger_name, '') <> '' \
         FROM information_schema.triggers t \
         WHERE t.trigger_schema = $1 \
         GROUP BY t.trigger_name, t.event_object_table, t.action_timing, \
                  t.action_orientation, t.action_statement \
         ORDER BY t.event_object_table, t.trigger_name",
    )
    .bind(schema)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    rows.iter()
        .map(|row| {
            let events_str: String = row.try_get::<String, _>(3).unwrap_or_default();
            let events = TriggerEvents {
                on_insert: events_str.contains("INSERT"),
                on_update: events_str.contains("UPDATE"),
                on_delete: events_str.contains("DELETE"),
                on_truncate: events_str.contains("TRUNCATE"),
            };
            TriggerDescriptor {
                schema: schema.to_string(),
                table: row.try_get::<String, _>(1).unwrap_or_default(),
                name: row.try_get::<String, _>(0).unwrap_or_default(),
                timing: row.try_get::<String, _>(2).unwrap_or_default(),
                events,
                row_level: row.try_get::<bool, _>(4).unwrap_or(false),
                function: row.try_get::<String, _>(5).unwrap_or_default(),
                enabled: true,
            }
        })
        .collect()
}

/// Build an [`ErdDiagram`] for `schema` by querying all tables and their FK
/// relationships.
///
/// Uses [`DbTableManager::list_tables`] for node data and a raw
/// `information_schema` FK query for edge data.
#[instrument(skip(backend))]
pub async fn fetch_erd(
    backend: &ArchiveDbBackend,
    url: &str,
    schema: &str,
) -> ArchiveResult<crate::archive::ErdDiagram> {
    use crate::archive::{ErdColumn, ErdDiagram, ErdEdge, ErdNode};

    // ── Nodes ──────────────────────────────────────────────────────────────
    let tables = backend
        .list_tables(schema)
        .await
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Query(e.to_string())))?;

    let nodes: Vec<ErdNode> = tables
        .iter()
        .map(|t| {
            let columns = t
                .columns
                .iter()
                .map(|c| ErdColumn {
                    name: c.name.clone(),
                    sql_type: c.ty.clone(),
                    is_pk: c.primary_key,
                    is_fk: false, // enriched below
                })
                .collect();
            ErdNode {
                schema: schema.to_string(),
                table: t.name.clone(),
                columns,
            }
        })
        .collect();

    // ── Edges (FK) — PostgreSQL only ───────────────────────────────────────
    let edges = if BackendKind::from_url(url) == BackendKind::Postgres {
        sqlx::any::install_default_drivers();
        let pool = sqlx::any::AnyPoolOptions::new()
            .max_connections(2)
            .connect(url)
            .await
            .ok();

        if let Some(pool) = pool {
            let sql = r"
                SELECT
                    tc.constraint_name,
                    tc.table_schema         AS from_schema,
                    tc.table_name           AS from_table,
                    kcu.column_name         AS from_column,
                    ccu.table_schema        AS to_schema,
                    ccu.table_name          AS to_table,
                    ccu.column_name         AS to_column
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage kcu
                     ON tc.constraint_name = kcu.constraint_name
                    AND tc.table_schema    = kcu.table_schema
                    AND tc.table_name      = kcu.table_name
                JOIN information_schema.referential_constraints rc
                     ON tc.constraint_name    = rc.constraint_name
                    AND tc.constraint_schema  = rc.constraint_schema
                JOIN information_schema.constraint_column_usage ccu
                     ON rc.unique_constraint_name   = ccu.constraint_name
                    AND rc.unique_constraint_schema = ccu.constraint_schema
                WHERE tc.constraint_type = 'FOREIGN KEY'
                  AND tc.table_schema = $1
                ORDER BY tc.constraint_name, kcu.ordinal_position
            ";
            let rows = sqlx::query(sql)
                .bind(schema)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();
            pool.close().await;
            rows.iter()
                .filter_map(|r| {
                    Some(ErdEdge {
                        constraint_name: r.try_get::<String, _>("constraint_name").ok()?,
                        from_schema: r.try_get::<String, _>("from_schema").ok()?,
                        from_table: r.try_get::<String, _>("from_table").ok()?,
                        from_column: r.try_get::<String, _>("from_column").ok()?,
                        to_schema: r.try_get::<String, _>("to_schema").ok()?,
                        to_table: r.try_get::<String, _>("to_table").ok()?,
                        to_column: r.try_get::<String, _>("to_column").ok()?,
                    })
                })
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // ── FK enrichment — mark ErdColumn.is_fk = true for FK columns ────────
    let mut nodes = nodes;
    for edge in &edges {
        if edge.from_schema == schema {
            if let Some(node) = nodes.iter_mut().find(|n| n.table == edge.from_table) {
                if let Some(col) = node.columns.iter_mut().find(|c| c.name == edge.from_column) {
                    col.is_fk = true;
                }
            }
        }
    }

    Ok(ErdDiagram {
        schema: schema.to_string(),
        nodes,
        edges,
    })
}
