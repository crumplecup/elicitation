//! Persistent Stored Modules (PSM) propositions.
//!
//! Source: ISO/IEC 9075-4:2023 — SQL/PSM (Persistent Stored Modules); PostgreSQL §§37–40.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // -- Functions (ISO/IEC 9075-4 §8 / PostgreSQL §37) --

    /// CREATE FUNCTION succeeded.
    ///
    /// Source: ISO/IEC 9075-4 §8 — Function definition; PostgreSQL §37.3
    pub struct FunctionCreated;
    structural_prop!(FunctionCreated, "FunctionCreated");

    /// Function return type matches declared signature.
    ///
    /// Source: ISO/IEC 9075-4 §8 — Returns clause type conformance
    pub struct FunctionReturnTypeCorrect;
    structural_prop!(FunctionReturnTypeCorrect, "FunctionReturnTypeCorrect");

    /// LANGUAGE clause is present (sql, plpgsql, c, etc.).
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: LANGUAGE clause
    pub struct FunctionLanguageDeclared;
    structural_prop!(FunctionLanguageDeclared, "FunctionLanguageDeclared");

    /// PL/pgSQL function was compiled and created.
    ///
    /// Source: PostgreSQL §42 — PL/pgSQL function language
    pub struct PlpgsqlFunctionCreated;
    structural_prop!(PlpgsqlFunctionCreated, "PlpgsqlFunctionCreated");

    /// Pure SQL function was created.
    ///
    /// Source: PostgreSQL §37.5 — SQL-language functions
    pub struct SqlFunctionCreated;
    structural_prop!(SqlFunctionCreated, "SqlFunctionCreated");

    /// C language function was created.
    ///
    /// Source: PostgreSQL §37.10 — C-language functions
    pub struct CFunctionCreated;
    structural_prop!(CFunctionCreated, "CFunctionCreated");

    /// STRICT declared: function returns NULL on NULL input.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: STRICT / CALLED ON NULL INPUT
    pub struct FunctionIsStrict;
    structural_prop!(FunctionIsStrict, "FunctionIsStrict");

    /// IMMUTABLE declared: result depends only on arguments, not DB state.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: IMMUTABLE volatility category
    pub struct FunctionIsImmutable;
    structural_prop!(FunctionIsImmutable, "FunctionIsImmutable");

    /// STABLE declared: result may vary within a single transaction.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: STABLE volatility category
    pub struct FunctionIsStable;
    structural_prop!(FunctionIsStable, "FunctionIsStable");

    /// VOLATILE declared: result may change at any time.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: VOLATILE volatility category
    pub struct FunctionIsVolatile;
    structural_prop!(FunctionIsVolatile, "FunctionIsVolatile");

    /// SECURITY DEFINER: function runs with creator's privileges.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: SECURITY DEFINER
    pub struct FunctionSecurityDefiner;
    structural_prop!(FunctionSecurityDefiner, "FunctionSecurityDefiner");

    /// SECURITY INVOKER: function runs with caller's privileges.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: SECURITY INVOKER
    pub struct FunctionSecurityInvoker;
    structural_prop!(FunctionSecurityInvoker, "FunctionSecurityInvoker");

    /// search_path is explicitly set to prevent privilege escalation.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: SET search_path security consideration
    pub struct FunctionSearchPathSet;
    structural_prop!(FunctionSearchPathSet, "FunctionSearchPathSet");

    /// COST annotation provides planner hint.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: COST execution_cost
    pub struct FunctionCostEstimated;
    structural_prop!(FunctionCostEstimated, "FunctionCostEstimated");

    /// ROWS annotation provides planner hint for set-returning functions.
    ///
    /// Source: PostgreSQL §37.3 — CREATE FUNCTION: ROWS result_rows
    pub struct FunctionRowsEstimated;
    structural_prop!(FunctionRowsEstimated, "FunctionRowsEstimated");

    /// Function returns SETOF type.
    ///
    /// Source: PostgreSQL §37.4 — Set-returning functions (SRFs)
    pub struct SetReturningFunctionDefined;
    structural_prop!(SetReturningFunctionDefined, "SetReturningFunctionDefined");

    // -- Procedures (ISO/IEC 9075-4 §11) --

    /// CREATE PROCEDURE succeeded.
    ///
    /// Source: ISO/IEC 9075-4 §11 — Procedure definition; PostgreSQL §37.3
    pub struct ProcedureCreated;
    structural_prop!(ProcedureCreated, "ProcedureCreated");

    /// CALL statement invoked the procedure.
    ///
    /// Source: ISO/IEC 9075-4 §11 — CALL statement
    pub struct ProcedureCalledViaCAll;
    structural_prop!(ProcedureCalledViaCAll, "ProcedureCalledViaCAll");

    /// Procedure used COMMIT/ROLLBACK inside body.
    ///
    /// Source: PostgreSQL §37.3 — Procedures: transaction control in procedure body
    pub struct ProcedureTransactionControlAllowed;
    structural_prop!(
        ProcedureTransactionControlAllowed,
        "ProcedureTransactionControlAllowed"
    );

    // -- Trigger functions (PostgreSQL §39) --

    /// Trigger function (returns trigger) created.
    ///
    /// Source: PostgreSQL §39.3 — CREATE FUNCTION: RETURNS trigger
    pub struct TriggerFunctionCreated;
    structural_prop!(TriggerFunctionCreated, "TriggerFunctionCreated");

    /// CREATE TRIGGER bound trigger function to a table.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER statement
    pub struct TriggerBoundToTable;
    structural_prop!(TriggerBoundToTable, "TriggerBoundToTable");

    /// Trigger fires on INSERT events.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: INSERT event
    pub struct TriggerFiredOnInsert;
    structural_prop!(TriggerFiredOnInsert, "TriggerFiredOnInsert");

    /// Trigger fires on UPDATE events.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: UPDATE event
    pub struct TriggerFiredOnUpdate;
    structural_prop!(TriggerFiredOnUpdate, "TriggerFiredOnUpdate");

    /// Trigger fires on DELETE events.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: DELETE event
    pub struct TriggerFiredOnDelete;
    structural_prop!(TriggerFiredOnDelete, "TriggerFiredOnDelete");

    /// Trigger fires on TRUNCATE events.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: TRUNCATE event
    pub struct TriggerFiredOnTruncate;
    structural_prop!(TriggerFiredOnTruncate, "TriggerFiredOnTruncate");

    /// BEFORE trigger fires before the row change.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: BEFORE timing
    pub struct TriggerFiredBefore;
    structural_prop!(TriggerFiredBefore, "TriggerFiredBefore");

    /// AFTER trigger fires after the row change.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: AFTER timing
    pub struct TriggerFiredAfter;
    structural_prop!(TriggerFiredAfter, "TriggerFiredAfter");

    /// INSTEAD OF trigger fires in place of the row change (view triggers).
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: INSTEAD OF timing
    pub struct TriggerFiredInsteadOf;
    structural_prop!(TriggerFiredInsteadOf, "TriggerFiredInsteadOf");

    /// FOR EACH ROW trigger fires per modified row.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: FOR EACH ROW
    pub struct TriggerFiredPerRow;
    structural_prop!(TriggerFiredPerRow, "TriggerFiredPerRow");

    /// FOR EACH STATEMENT trigger fires once per statement.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: FOR EACH STATEMENT
    pub struct TriggerFiredPerStatement;
    structural_prop!(TriggerFiredPerStatement, "TriggerFiredPerStatement");

    /// WHEN clause evaluated to TRUE and trigger fired.
    ///
    /// Source: PostgreSQL §39.1 — CREATE TRIGGER: WHEN condition
    pub struct TriggerConditionPassed;
    structural_prop!(TriggerConditionPassed, "TriggerConditionPassed");

    /// Trigger is currently enabled.
    ///
    /// Source: PostgreSQL §39 — ALTER TABLE ... ENABLE TRIGGER
    pub struct TriggerEnabled;
    structural_prop!(TriggerEnabled, "TriggerEnabled");

    /// Trigger has been disabled with ALTER TABLE ... DISABLE TRIGGER.
    ///
    /// Source: PostgreSQL §39 — ALTER TABLE ... DISABLE TRIGGER
    pub struct TriggerDisabled;
    structural_prop!(TriggerDisabled, "TriggerDisabled");

    /// Trigger was dropped.
    ///
    /// Source: PostgreSQL §39 — DROP TRIGGER statement
    pub struct TriggerDropped;
    structural_prop!(TriggerDropped, "TriggerDropped");

    // -- Aggregate functions (PostgreSQL §37.12) --

    /// CREATE AGGREGATE with SFUNC and STYPE defined.
    ///
    /// Source: PostgreSQL §37.12 — CREATE AGGREGATE: SFUNC and STYPE
    pub struct AggregateFunctionCreated;
    structural_prop!(AggregateFunctionCreated, "AggregateFunctionCreated");

    /// SFUNC accumulates state correctly.
    ///
    /// Source: PostgreSQL §37.12 — Aggregate state transition function correctness
    pub struct AggregateStateFunctionCorrect;
    structural_prop!(
        AggregateStateFunctionCorrect,
        "AggregateStateFunctionCorrect"
    );

    /// FINALFUNC produces the correct aggregate output.
    ///
    /// Source: PostgreSQL §37.12 — Aggregate final function correctness
    pub struct AggregateFinalFunctionCorrect;
    structural_prop!(
        AggregateFinalFunctionCorrect,
        "AggregateFinalFunctionCorrect"
    );

    /// ORDERED SET aggregate (e.g., percentile_cont) defined.
    ///
    /// Source: PostgreSQL §37.12 — Ordered-set aggregates
    pub struct AggregateSortedSetDefined;
    structural_prop!(AggregateSortedSetDefined, "AggregateSortedSetDefined");

    /// Hypothetical-set aggregate defined.
    ///
    /// Source: PostgreSQL §37.12 — Hypothetical-set aggregates
    pub struct AggregateHypotheticalSetDefined;
    structural_prop!(
        AggregateHypotheticalSetDefined,
        "AggregateHypotheticalSetDefined"
    );

    // -- Window functions (PostgreSQL §4.2.8) --

    /// `CREATE FUNCTION` with `WINDOW` keyword — user-defined window function object registered.
    ///
    /// Source: PostgreSQL §4.2.8 — Window functions: WINDOW keyword in CREATE FUNCTION
    pub struct UserDefinedWindowFunctionCreated;
    structural_prop!(
        UserDefinedWindowFunctionCreated,
        "UserDefinedWindowFunctionCreated"
    );

    /// OVER clause is syntactically and semantically valid.
    ///
    /// Source: PostgreSQL §4.2.8 — Window function calls: OVER clause validity
    pub struct WindowFunctionOverClauseValid;
    structural_prop!(
        WindowFunctionOverClauseValid,
        "WindowFunctionOverClauseValid"
    );

    /// Window function produces deterministic results given same frame.
    ///
    /// Source: PostgreSQL §4.2.8 — Window frame determinism
    pub struct WindowFunctionResultDeterministic;
    structural_prop!(
        WindowFunctionResultDeterministic,
        "WindowFunctionResultDeterministic"
    );

    // -- Function/Procedure lifecycle --

    /// Function was removed via `DROP FUNCTION`.
    ///
    /// Source: ISO/IEC 9075-4 §14.30 — `<drop routine statement>`; PostgreSQL §DROP FUNCTION
    pub struct FunctionDropped;
    structural_prop!(FunctionDropped, "FunctionDropped");

    /// Procedure was removed via `DROP PROCEDURE`.
    ///
    /// Source: ISO/IEC 9075-4 §14.30 — `<drop routine statement>`; PostgreSQL §DROP PROCEDURE
    pub struct ProcedureDropped;
    structural_prop!(ProcedureDropped, "ProcedureDropped");

    /// Function was altered via `ALTER FUNCTION` (e.g. volatility, cost, security).
    ///
    /// Source: ISO/IEC 9075-4 §14.29 — `<alter routine statement>`; PostgreSQL §ALTER FUNCTION
    pub struct FunctionAltered;
    structural_prop!(FunctionAltered, "FunctionAltered");

    // -- Parallel safety --

    /// Function is marked `PARALLEL SAFE` — safe for parallel query workers.
    ///
    /// Source: ISO/IEC 9075-4 §<routine characteristics>; PostgreSQL §37.7 — parallel safety
    pub struct FunctionParallelSafe;
    structural_prop!(FunctionParallelSafe, "FunctionParallelSafe");

    /// Function is marked `PARALLEL RESTRICTED` — only allowed in the parallel leader.
    ///
    /// Source: ISO/IEC 9075-4 §<routine characteristics>; PostgreSQL §37.7 — parallel safety
    pub struct FunctionParallelRestricted;
    structural_prop!(FunctionParallelRestricted, "FunctionParallelRestricted");

    /// Function is marked `PARALLEL UNSAFE` — prevents parallel plan generation.
    ///
    /// Source: ISO/IEC 9075-4 §<routine characteristics>; PostgreSQL §37.7 — parallel safety
    pub struct FunctionParallelUnsafe;
    structural_prop!(FunctionParallelUnsafe, "FunctionParallelUnsafe");

    // -- PL/pgSQL exception handling --

    /// A PL/pgSQL `EXCEPTION` block caught the raised error.
    ///
    /// Source: PostgreSQL §43.6.8 — Trapping Errors (`BEGIN … EXCEPTION … END`)
    pub struct PlpgsqlExceptionHandled;
    structural_prop!(PlpgsqlExceptionHandled, "PlpgsqlExceptionHandled");

    /// A PL/pgSQL `RAISE` used a specific `SQLSTATE` code.
    ///
    /// Source: PostgreSQL §43.9 — `RAISE` statement with `USING ERRCODE`
    pub struct PlpgsqlRaisedWithSqlstate;
    structural_prop!(PlpgsqlRaisedWithSqlstate, "PlpgsqlRaisedWithSqlstate");

    // -- Anonymous blocks --

    /// An anonymous PL/pgSQL block was executed via `DO $$ … $$`.
    ///
    /// Source: ISO/IEC 9075-4 §13.6 — `<SQL-invoked procedure>` anonymous; PostgreSQL §DO
    pub struct AnonymousBlockExecuted;
    structural_prop!(AnonymousBlockExecuted, "AnonymousBlockExecuted");

    // -- Trigger condition --

    /// A `WHEN (condition)` clause was defined on a row-level trigger.
    ///
    /// Source: ISO/IEC 9075-2 §11.38 — `<trigger definition>` WHEN clause; PostgreSQL §CREATE TRIGGER
    pub struct TriggerWhenConditionDefined;
    structural_prop!(TriggerWhenConditionDefined, "TriggerWhenConditionDefined");
}

pub use emit_impls::{
    AggregateFinalFunctionCorrect, AggregateFunctionCreated, AggregateHypotheticalSetDefined,
    AggregateSortedSetDefined, AggregateStateFunctionCorrect, AnonymousBlockExecuted,
    CFunctionCreated, FunctionAltered, FunctionCostEstimated, FunctionCreated, FunctionDropped,
    FunctionIsImmutable, FunctionIsStable, FunctionIsStrict, FunctionIsVolatile,
    FunctionLanguageDeclared, FunctionParallelRestricted, FunctionParallelSafe,
    FunctionParallelUnsafe, FunctionReturnTypeCorrect, FunctionRowsEstimated,
    FunctionSearchPathSet, FunctionSecurityDefiner, FunctionSecurityInvoker,
    PlpgsqlExceptionHandled, PlpgsqlFunctionCreated, PlpgsqlRaisedWithSqlstate,
    ProcedureCalledViaCAll, ProcedureCreated, ProcedureDropped, ProcedureTransactionControlAllowed,
    SetReturningFunctionDefined, SqlFunctionCreated, TriggerBoundToTable, TriggerConditionPassed,
    TriggerDisabled, TriggerDropped, TriggerEnabled, TriggerFiredAfter, TriggerFiredBefore,
    TriggerFiredInsteadOf, TriggerFiredOnDelete, TriggerFiredOnInsert, TriggerFiredOnTruncate,
    TriggerFiredOnUpdate, TriggerFiredPerRow, TriggerFiredPerStatement, TriggerFunctionCreated,
    TriggerWhenConditionDefined, UserDefinedWindowFunctionCreated, WindowFunctionOverClauseValid,
    WindowFunctionResultDeterministic,
};
