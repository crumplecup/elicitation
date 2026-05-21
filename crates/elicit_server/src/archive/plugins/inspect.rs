//! `ArchiveInspectPlugin` — rich table inspection (FK, constraints, indexes, DDL).
//!
//! These queries run directly against PostgreSQL `information_schema` and
//! `pg_catalog` views to collect data not available at nav-tree build time.
//! Results are stored in `ArchiveNavModel::table_inspections`.

use std::collections::BTreeMap;

use sqlx::AnyPool;
use sqlx::Row as _;
use tracing::instrument;

use crate::archive::{
    ConstraintDescriptor, ConstraintKind, DdlDescriptor, FkAction, ForeignKeyDescriptor,
    IndexDescriptor, TableInspection,
};

// ── connection helper ─────────────────────────────────────────────────────────

async fn connect(url: &str) -> Result<AnyPool, String> {
    sqlx::any::install_default_drivers();
    sqlx::any::AnyPoolOptions::new()
        .max_connections(3)
        .connect(url)
        .await
        .map_err(|e| format!("connection failed: {e}"))
}

// ── foreign key query ─────────────────────────────────────────────────────────

async fn query_foreign_keys(
    pool: &AnyPool,
    schema: &str,
    table: &str,
) -> Result<Vec<ForeignKeyDescriptor>, String> {
    let sql = r"
        SELECT
            tc.constraint_name,
            kcu.column_name          AS from_column,
            ccu.table_schema         AS to_schema,
            ccu.table_name           AS to_table,
            ccu.column_name          AS to_column,
            rc.delete_rule,
            rc.update_rule
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
          AND tc.table_name   = $2
        ORDER BY tc.constraint_name, kcu.ordinal_position
    ";

    let rows = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("FK query failed: {e}"))?;

    rows.iter()
        .map(|r| {
            Ok(ForeignKeyDescriptor {
                constraint_name: r
                    .try_get::<String, _>("constraint_name")
                    .map_err(|e| e.to_string())?,
                from_column: r
                    .try_get::<String, _>("from_column")
                    .map_err(|e| e.to_string())?,
                to_schema: r
                    .try_get::<String, _>("to_schema")
                    .map_err(|e| e.to_string())?,
                to_table: r
                    .try_get::<String, _>("to_table")
                    .map_err(|e| e.to_string())?,
                to_column: r
                    .try_get::<String, _>("to_column")
                    .map_err(|e| e.to_string())?,
                on_delete: FkAction::from_rule(
                    &r.try_get::<String, _>("delete_rule").unwrap_or_default(),
                ),
                on_update: FkAction::from_rule(
                    &r.try_get::<String, _>("update_rule").unwrap_or_default(),
                ),
            })
        })
        .collect()
}

// ── constraints query ─────────────────────────────────────────────────────────

async fn query_constraints(
    pool: &AnyPool,
    schema: &str,
    table: &str,
) -> Result<Vec<ConstraintDescriptor>, String> {
    let sql = r"
        SELECT
            tc.constraint_name,
            tc.constraint_type,
            kcu.column_name,
            cc.check_clause
        FROM information_schema.table_constraints tc
        LEFT JOIN information_schema.key_column_usage kcu
               ON tc.constraint_name  = kcu.constraint_name
              AND tc.constraint_schema = kcu.constraint_schema
              AND tc.table_name        = kcu.table_name
        LEFT JOIN information_schema.check_constraints cc
               ON tc.constraint_name   = cc.constraint_name
              AND tc.constraint_schema = cc.constraint_schema
        WHERE tc.table_schema = $1
          AND tc.table_name   = $2
        ORDER BY tc.constraint_type, tc.constraint_name, kcu.ordinal_position
    ";

    let rows = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("constraint query failed: {e}"))?;

    // Group columns by constraint_name
    let mut map: BTreeMap<String, ConstraintDescriptor> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();

    for r in &rows {
        let name: String = r
            .try_get::<String, _>("constraint_name")
            .map_err(|e| e.to_string())?;
        let kind_str: String = r
            .try_get::<String, _>("constraint_type")
            .map_err(|e| e.to_string())?;
        let col: Option<String> = r.try_get::<String, _>("column_name").ok();
        let def: Option<String> = r.try_get::<String, _>("check_clause").ok();

        let entry = map.entry(name.clone()).or_insert_with(|| {
            order.push(name.clone());
            ConstraintDescriptor {
                name: name.clone(),
                kind: ConstraintKind::from_pg_type(&kind_str),
                columns: Vec::new(),
                definition: def.clone(),
            }
        });

        if let Some(c) = col
            && !entry.columns.contains(&c)
        {
            entry.columns.push(c);
        }
    }

    Ok(order.into_iter().filter_map(|k| map.remove(&k)).collect())
}

// ── indexes query ─────────────────────────────────────────────────────────────

async fn query_indexes(
    pool: &AnyPool,
    schema: &str,
    table: &str,
) -> Result<Vec<IndexDescriptor>, String> {
    // Use pg_catalog for richer access-method info
    let sql = r"
        SELECT
            i.relname               AS index_name,
            a.attname               AS column_name,
            ix.indisunique          AS is_unique,
            am.amname               AS index_method
        FROM pg_class t
        JOIN pg_index     ix ON t.oid        = ix.indrelid
        JOIN pg_class      i ON i.oid        = ix.indexrelid
        JOIN pg_attribute  a ON a.attrelid   = t.oid
                             AND a.attnum     = ANY(ix.indkey)
        JOIN pg_am        am ON am.oid        = i.relam
        JOIN pg_namespace  n ON n.oid         = t.relnamespace
        WHERE n.nspname = $1
          AND t.relname = $2
        ORDER BY i.relname, a.attnum
    ";

    let rows = sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("index query failed: {e}"))?;

    let mut map: BTreeMap<String, IndexDescriptor> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();

    for r in &rows {
        let idx_name: String = r
            .try_get::<String, _>("index_name")
            .map_err(|e| e.to_string())?;
        let col: String = r
            .try_get::<String, _>("column_name")
            .map_err(|e| e.to_string())?;
        let unique: bool = r.try_get::<bool, _>("is_unique").unwrap_or(false);
        let method: String = r
            .try_get::<String, _>("index_method")
            .unwrap_or_else(|_| "btree".to_string());

        let entry = map.entry(idx_name.clone()).or_insert_with(|| {
            order.push(idx_name.clone());
            IndexDescriptor {
                index_name: idx_name.clone(),
                schema: schema.to_string(),
                table_name: table.to_string(),
                column_names: Vec::new(),
                is_unique: unique,
                index_method: method,
            }
        });

        if !entry.column_names.contains(&col) {
            entry.column_names.push(col);
        }
    }

    Ok(order.into_iter().filter_map(|k| map.remove(&k)).collect())
}

// ── DDL generation ────────────────────────────────────────────────────────────

/// Reconstruct a `CREATE TABLE` DDL string from the inspection data.
///
/// Uses `information_schema.columns` for accurate type info; falls back to
/// the columns already in the nav model (passed as `col_sql_types`).
async fn build_ddl(
    pool: &AnyPool,
    schema: &str,
    table: &str,
    inspection: &TableInspection,
) -> Result<String, String> {
    let col_sql = r"
        SELECT column_name, data_type, character_maximum_length,
               numeric_precision, numeric_scale,
               is_nullable, column_default, ordinal_position
        FROM information_schema.columns
        WHERE table_schema = $1
          AND table_name   = $2
        ORDER BY ordinal_position
    ";

    let rows = sqlx::query(col_sql)
        .bind(schema)
        .bind(table)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("column query for DDL failed: {e}"))?;

    let mut parts: Vec<String> = Vec::new();

    for r in &rows {
        let col_name: String = r
            .try_get::<String, _>("column_name")
            .map_err(|e| e.to_string())?;
        let data_type: String = r
            .try_get::<String, _>("data_type")
            .map_err(|e| e.to_string())?;
        let nullable: String = r
            .try_get::<String, _>("is_nullable")
            .unwrap_or_else(|_| "YES".to_string());
        let default: Option<String> = r.try_get::<String, _>("column_default").ok();
        let char_max: Option<i64> = r.try_get::<i64, _>("character_maximum_length").ok();

        let type_str = if let Some(n) = char_max {
            format!("{data_type}({n})")
        } else {
            data_type.clone()
        };

        let mut col = format!("    \"{col_name}\" {type_str}");
        if nullable == "NO" {
            col.push_str(" NOT NULL");
        }
        if let Some(d) = default {
            col.push_str(&format!(" DEFAULT {d}"));
        }
        parts.push(col);
    }

    // Append constraint clauses
    for c in &inspection.constraints {
        match c.kind {
            ConstraintKind::PrimaryKey => {
                let cols = c
                    .columns
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                parts.push(format!(
                    "    CONSTRAINT \"{}\" PRIMARY KEY ({})",
                    c.name, cols
                ));
            }
            ConstraintKind::Unique => {
                let cols = c
                    .columns
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ");
                parts.push(format!("    CONSTRAINT \"{}\" UNIQUE ({})", c.name, cols));
            }
            ConstraintKind::Check => {
                if let Some(def) = &c.definition {
                    parts.push(format!("    CONSTRAINT \"{}\" CHECK ({})", c.name, def));
                }
            }
            _ => {}
        }
    }

    // Append FK clauses
    for fk in &inspection.foreign_keys {
        parts.push(format!(
            "    CONSTRAINT \"{}\" FOREIGN KEY (\"{}\") REFERENCES \"{}\".\"{}\" (\"{}\") ON DELETE {} ON UPDATE {}",
            fk.constraint_name,
            fk.from_column,
            fk.to_schema,
            fk.to_table,
            fk.to_column,
            fk.on_delete,
            fk.on_update,
        ));
    }

    let body = parts.join(",\n");
    Ok(format!(
        "CREATE TABLE \"{schema}\".\"{table}\" (\n{body}\n);"
    ))
}

// ── public API ────────────────────────────────────────────────────────────────

/// Fetch FK, constraint, and index data for a table in one pass.
///
/// Returned as [`TableInspection`] suitable for storage in the nav model.
#[instrument(skip(url))]
pub async fn inspect_table_direct(
    url: &str,
    schema: &str,
    table: &str,
) -> Result<TableInspection, String> {
    let pool = connect(url).await?;

    // Run all three queries concurrently
    let (fks, constraints, indexes) = tokio::try_join!(
        query_foreign_keys(&pool, schema, table),
        query_constraints(&pool, schema, table),
        query_indexes(&pool, schema, table),
    )
    .map_err(|e| e)?;

    pool.close().await;

    Ok(TableInspection {
        foreign_keys: fks,
        constraints,
        indexes,
    })
}

/// Generate a reconstructed `CREATE TABLE` DDL for a table.
///
/// Combines inspection data with `information_schema.columns` for a
/// faithful DDL representation.
#[instrument(skip(url))]
pub async fn generate_ddl_direct(
    url: &str,
    schema: &str,
    table: &str,
) -> Result<DdlDescriptor, String> {
    let pool = connect(url).await?;

    // First gather inspection data (needed for DDL constraints block)
    let (fks, constraints, indexes) = tokio::try_join!(
        query_foreign_keys(&pool, schema, table),
        query_constraints(&pool, schema, table),
        query_indexes(&pool, schema, table),
    )
    .map_err(|e| e)?;

    let inspection = TableInspection {
        foreign_keys: fks,
        constraints,
        indexes,
    };

    let ddl = build_ddl(&pool, schema, table, &inspection).await?;
    pool.close().await;

    Ok(DdlDescriptor {
        schema: schema.to_string(),
        object_name: table.to_string(),
        ddl,
    })
}

// ── column stats query ────────────────────────────────────────────────────────

/// Fetch per-column planner statistics from `pg_stats`.
///
/// Returns `Ok(vec![])` for non-PostgreSQL backends (query will fail gracefully).
#[instrument(skip(url))]
pub async fn get_column_stats_direct(
    url: &str,
    schema: &str,
    table: &str,
) -> Result<Vec<crate::archive::ColumnStats>, String> {
    use crate::archive::ColumnStats;

    let pool = connect(url).await?;

    let sql = r"
        SELECT
            attname      AS column_name,
            null_frac    AS null_fraction,
            avg_width    AS avg_width_bytes,
            n_distinct,
            correlation
        FROM pg_stats
        WHERE schemaname = $1
          AND tablename  = $2
        ORDER BY attname
    ";

    let rows = match sqlx::query(sql)
        .bind(schema)
        .bind(table)
        .fetch_all(&pool)
        .await
    {
        Ok(r) => r,
        Err(_) => {
            pool.close().await;
            // Non-Postgres backend — return empty rather than error
            return Ok(Vec::new());
        }
    };

    pool.close().await;

    rows.iter()
        .map(|r| {
            Ok(ColumnStats {
                column_name: r
                    .try_get::<String, _>("column_name")
                    .map_err(|e| e.to_string())?,
                null_fraction: r.try_get::<f32, _>("null_fraction").unwrap_or(0.0) as f64,
                avg_width_bytes: r.try_get::<i32, _>("avg_width_bytes").unwrap_or(0),
                n_distinct: r.try_get::<f32, _>("n_distinct").unwrap_or(0.0) as f64,
                correlation: r.try_get::<f32, _>("correlation").ok().map(|v| v as f64),
            })
        })
        .collect()
}

// ── EXPLAIN query ─────────────────────────────────────────────────────────────

/// Run `EXPLAIN (ANALYZE, FORMAT JSON)` on a SQL string and parse the plan.
///
/// Returns an [`crate::archive::ExplainPlan`] arena rooted at the top-level plan node.
#[instrument(skip(url, sql))]
pub async fn explain_sql_direct(
    url: &str,
    sql: &str,
) -> Result<crate::archive::ExplainPlan, String> {
    use crate::archive::ExplainPlan;

    let pool = connect(url).await?;

    let explain_sql = format!("EXPLAIN (ANALYZE, FORMAT JSON) {sql}");

    let rows = sqlx::query(&explain_sql)
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("EXPLAIN failed: {e}"))?;

    pool.close().await;

    // The result is a single row with a single text column containing the JSON
    let json_text: String = rows
        .first()
        .ok_or_else(|| "EXPLAIN returned no rows".to_string())?
        .try_get::<String, _>(0)
        .map_err(|e| e.to_string())?;

    ExplainPlan::parse_explain_output(&json_text)
}
