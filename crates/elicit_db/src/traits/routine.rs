//! `DbRoutineFactory` — stored routine lifecycle factory (Role 1a).
//! `DbRoutineMeta`    — routine catalog reporter (Role 2).
//!
//! # Three-role taxonomy
//!
//! | Role | Trait | Purpose |
//! |------|-------|---------|
//! | 1a (leaf factory) | [`DbRoutineFactory`] | CREATE/DROP/ALTER FUNCTION/PROCEDURE; returns proof tokens |
//! | 2 (reporter) | [`DbRoutineMeta`] | Reads pg_proc/information_schema; no proof tokens |
//!
//! Source: ISO/IEC 9075-4 — PSM (Persistent Stored Modules);
//!         PostgreSQL docs §39 — PL/pgSQL.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AnonymousBlockExecuted, AuditLogged, DbResult, DbRoutineDescriptor, FunctionAltered,
    FunctionCreated, FunctionDropped, FunctionParallelRestricted, FunctionParallelSafe,
    FunctionParallelUnsafe, FunctionSecurityDefiner, FunctionSecurityInvoker, ProcedureCreated,
    ProcedureDropped, TriggerFunctionCreated, TriggerWhenConditionDefined,
};

type CreateFunctionFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        DbRoutineDescriptor,
        Established<FunctionCreated>,
        Established<AuditLogged>,
    )>,
>;

type DropFunctionFuture<'a> =
    BoxFuture<'a, DbResult<(Established<FunctionDropped>, Established<AuditLogged>)>>;

type AlterFunctionFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        DbRoutineDescriptor,
        Established<FunctionAltered>,
        Established<AuditLogged>,
    )>,
>;

type CreateProcedureFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        DbRoutineDescriptor,
        Established<ProcedureCreated>,
        Established<AuditLogged>,
    )>,
>;

type DropProcedureFuture<'a> =
    BoxFuture<'a, DbResult<(Established<ProcedureDropped>, Established<AuditLogged>)>>;

type DeclareParallelSafeFuture<'a> =
    BoxFuture<'a, DbResult<(Established<FunctionParallelSafe>, Established<AuditLogged>)>>;

type DeclareParallelRestrictedFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        Established<FunctionParallelRestricted>,
        Established<AuditLogged>,
    )>,
>;

type DeclareParallelUnsafeFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        Established<FunctionParallelUnsafe>,
        Established<AuditLogged>,
    )>,
>;

type SetSecurityDefinerFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        Established<FunctionSecurityDefiner>,
        Established<AuditLogged>,
    )>,
>;

type SetSecurityInvokerFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        Established<FunctionSecurityInvoker>,
        Established<AuditLogged>,
    )>,
>;

type CreateTriggerFunctionFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        DbRoutineDescriptor,
        Established<TriggerFunctionCreated>,
        Established<AuditLogged>,
    )>,
>;

type DefineTriggerWhenFuture<'a> = BoxFuture<
    'a,
    DbResult<(
        Established<TriggerWhenConditionDefined>,
        Established<AuditLogged>,
    )>,
>;

// ── Role 1a: stored routine lifecycle factory ─────────────────────────────────

/// Creates, modifies, and drops stored functions and procedures.
///
/// Each method returns a proof token asserting that the operation satisfied
/// the relevant PSM invariant.  Proof tokens can be composed via the
/// `ProvableFrom` chains in `contracts::proof_composition`.
///
/// Source: ISO/IEC 9075-4 §10 — `<SQL-invoked routine>`;
///         PostgreSQL docs §39 — PL/pgSQL.
pub trait DbRoutineFactory: Send + Sync {
    /// Create a new stored function.
    ///
    /// Source: ISO/IEC 9075-4 §10 — `<create function statement>`
    fn create_function(&self, descriptor: DbRoutineDescriptor) -> CreateFunctionFuture<'_>;

    /// Drop an existing function by name and argument type list.
    ///
    /// Source: ISO/IEC 9075-4 §10 — `<drop routine statement>`
    fn drop_function(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> DropFunctionFuture<'_>;

    /// Alter a function's definition or properties.
    ///
    /// Source: PostgreSQL docs §43.12 — `ALTER FUNCTION`
    fn alter_function(&self, descriptor: DbRoutineDescriptor) -> AlterFunctionFuture<'_>;

    /// Create a stored procedure.
    ///
    /// Source: ISO/IEC 9075-4 §10 — `<create procedure statement>`
    fn create_procedure(&self, descriptor: DbRoutineDescriptor) -> CreateProcedureFuture<'_>;

    /// Drop a stored procedure.
    ///
    /// Source: ISO/IEC 9075-4 §10 — `<drop procedure statement>`
    fn drop_procedure(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> DropProcedureFuture<'_>;

    /// Declare a function as `PARALLEL SAFE`.
    ///
    /// Source: PostgreSQL docs §39.2 — Parallel Safety
    fn declare_parallel_safe(&self, schema: &str, name: &str) -> DeclareParallelSafeFuture<'_>;

    /// Declare a function as `PARALLEL RESTRICTED`.
    ///
    /// Source: PostgreSQL docs §39.2 — Parallel Safety
    fn declare_parallel_restricted(
        &self,
        schema: &str,
        name: &str,
    ) -> DeclareParallelRestrictedFuture<'_>;

    /// Declare a function as `PARALLEL UNSAFE` (the default).
    ///
    /// Source: PostgreSQL docs §39.2 — Parallel Safety
    fn declare_parallel_unsafe(&self, schema: &str, name: &str) -> DeclareParallelUnsafeFuture<'_>;

    /// Set `SECURITY DEFINER` on a function.
    ///
    /// Source: PostgreSQL docs §39.6 — Security
    fn set_security_definer(&self, schema: &str, name: &str) -> SetSecurityDefinerFuture<'_>;

    /// Set `SECURITY INVOKER` on a function.
    ///
    /// Source: PostgreSQL docs §39.6 — Security
    fn set_security_invoker(&self, schema: &str, name: &str) -> SetSecurityInvokerFuture<'_>;

    /// Execute a `DO $$ … $$` anonymous block.
    ///
    /// Source: PostgreSQL docs — `DO` statement
    fn execute_anonymous_block(
        &self,
        body: &str,
        language: &str,
    ) -> BoxFuture<'_, DbResult<Established<AnonymousBlockExecuted>>>;

    /// Create a trigger function (returns `trigger`).
    ///
    /// Source: ISO/IEC 9075-4 §11 — Triggers;
    ///         PostgreSQL docs §40 — Trigger Functions
    fn create_trigger_function(
        &self,
        descriptor: DbRoutineDescriptor,
    ) -> CreateTriggerFunctionFuture<'_>;

    /// Add a `WHEN` condition to a trigger definition.
    ///
    /// Source: ISO/IEC 9075-2 §11.39 — `<trigger definition>`;
    ///         PostgreSQL docs §40.2 — Visibility of Data Changes
    fn define_trigger_when(
        &self,
        schema: &str,
        table: &str,
        trigger_name: &str,
        when_expr: &str,
    ) -> DefineTriggerWhenFuture<'_>;
}

// ── Role 2: routine catalog reporter ─────────────────────────────────────────

/// Orthogonal reporter for the routine catalog.
///
/// Queries `information_schema.routines` and `pg_catalog.pg_proc`.
/// No proof tokens are consumed or produced.
///
/// Source: ISO/IEC 9075-11 §ROUTINES view; PostgreSQL docs §54.39 — pg_proc.
pub trait DbRoutineMeta: Send + Sync {
    /// List all user-defined functions in a schema.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view (`ROUTINE_TYPE = 'FUNCTION'`)
    fn list_functions(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbRoutineDescriptor>>>;

    /// List all stored procedures in a schema.
    ///
    /// Source: ISO/IEC 9075-11 §ROUTINES view (`ROUTINE_TYPE = 'PROCEDURE'`)
    fn list_procedures(&self, schema: &str) -> BoxFuture<'_, DbResult<Vec<DbRoutineDescriptor>>>;

    /// Return full metadata for a specific routine by name and argument types.
    ///
    /// Source: PostgreSQL docs §54.39 — pg_proc
    fn routine_info(
        &self,
        schema: &str,
        name: &str,
        arg_types: &[String],
    ) -> BoxFuture<'_, DbResult<DbRoutineDescriptor>>;
}
