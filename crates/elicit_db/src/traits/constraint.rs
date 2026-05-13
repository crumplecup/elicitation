//! `DbConstraintFactory` — SQL constraint DDL factory (Role 1a).
//! `DbConstraintMeta`    — constraint catalog reporter (Role 2).
//!
//! # Three-role taxonomy
//!
//! | Role | Trait | Purpose |
//! |------|-------|---------|
//! | 1a (leaf factory) | [`DbConstraintFactory`] | ADD/DROP constraints; returns proof tokens |
//! | 2 (reporter) | [`DbConstraintMeta`] | Reads information_schema.table_constraints; no proof tokens |
//!
//! This trait is complementary to [`crate::DbTableManager`]: `DbTableManager`
//! handles table-level DDL (CREATE TABLE, ADD COLUMN), while `DbConstraintFactory`
//! handles the explicit constraint surface that produces typed proof tokens.
//!
//! Source: ISO/IEC 9075-2 §11 — Schema definition statements.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuditLogged, CheckConstraintDefined, ConstraintSatisfied, DbResult, ForeignKeyDefined,
    NotNullConstraintDefined, PrimaryKeyDefined, UniqueConstraintDefined,
};

// ── Role 1a: SQL constraint DDL factory ───────────────────────────────────────

/// Adds and removes explicit SQL constraints on tables, returning proof tokens.
///
/// Source: ISO/IEC 9075-2 §11.6 — `<table constraint definition>`.
pub trait DbConstraintFactory: Send + Sync {
    /// Add a `CHECK` constraint to a table.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — `<check constraint definition>`
    fn add_check_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        expression: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<CheckConstraintDefined>,
            Established<AuditLogged>,
        )>,
    >;

    /// Add a `PRIMARY KEY` constraint to a table.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — `<unique constraint definition>` (PRIMARY KEY)
    fn add_primary_key(
        &self,
        schema: &str,
        table: &str,
        columns: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<PrimaryKeyDefined>, Established<AuditLogged>)>>;

    /// Add a `UNIQUE` constraint to a table.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — `<unique constraint definition>`
    fn add_unique_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        columns: &[String],
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<UniqueConstraintDefined>,
            Established<AuditLogged>,
        )>,
    >;

    /// Add a `FOREIGN KEY` constraint between two tables.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — `<referential constraint definition>`
    fn add_foreign_key(
        &self,
        schema: &str,
        table: &str,
        name: &str,
        columns: &[String],
        referenced_table: &str,
        referenced_columns: &[String],
    ) -> BoxFuture<'_, DbResult<(Established<ForeignKeyDefined>, Established<AuditLogged>)>>;

    /// Add a `NOT NULL` constraint to a column.
    ///
    /// Source: ISO/IEC 9075-2 §11.5 — `<column constraint definition>`
    fn add_not_null(
        &self,
        schema: &str,
        table: &str,
        column: &str,
    ) -> BoxFuture<
        '_,
        DbResult<(
            Established<NotNullConstraintDefined>,
            Established<AuditLogged>,
        )>,
    >;

    /// Drop a named constraint from a table.
    ///
    /// Source: ISO/IEC 9075-2 §11.11 — `ALTER TABLE … DROP CONSTRAINT`
    fn drop_constraint(
        &self,
        schema: &str,
        table: &str,
        name: &str,
    ) -> BoxFuture<'_, DbResult<Established<AuditLogged>>>;
}

// ── Role 2: constraint catalog reporter ──────────────────────────────────────

/// Orthogonal reporter for constraint metadata.
///
/// Queries `information_schema.table_constraints` and
/// `information_schema.referential_constraints`.
/// No proof tokens are consumed or produced.
///
/// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view.
pub trait DbConstraintMeta: Send + Sync {
    /// List all constraint names and types on a table.
    ///
    /// Returns `(constraint_name, constraint_type)` pairs where `constraint_type`
    /// is one of `CHECK`, `UNIQUE`, `PRIMARY KEY`, `FOREIGN KEY`.
    ///
    /// Source: ISO/IEC 9075-11 §TABLE_CONSTRAINTS view
    fn list_constraints(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<'_, DbResult<Vec<(String, String)>>>;

    /// Verify that all constraints on a table are currently satisfied.
    ///
    /// Returns `ConstraintSatisfied` — all constraints pass validation.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — Constraint checking;
    ///         PostgreSQL docs §5.4 — Constraints
    fn verify_constraints(
        &self,
        schema: &str,
        table: &str,
    ) -> BoxFuture<'_, DbResult<Established<ConstraintSatisfied>>>;
}
