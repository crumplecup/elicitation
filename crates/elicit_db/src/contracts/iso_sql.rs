//! SQL standard propositions — ISO/IEC 9075.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Table was successfully created by a DDL statement.
    ///
    /// Source: ISO/IEC 9075-2 §11.3 — `<table definition>`
    pub struct TableCreated;

    /// All table and column constraints are satisfied.
    ///
    /// Source: ISO/IEC 9075-2 §11.6 — `<table constraint definition>`
    pub struct ConstraintSatisfied;

    /// Foreign key relationships are intact; no dangling references.
    ///
    /// Source: ISO/IEC 9075-2 §11.8 — `<referential constraint definition>`
    pub struct ReferentialIntegrityMaintained;

    /// View was successfully created.
    ///
    /// Source: ISO/IEC 9075-2 §11.32 — `<view definition>`
    pub struct ViewCreated;

    /// At least one row was inserted.
    ///
    /// Source: ISO/IEC 9075-2 §14.8 — `<insert statement>`
    pub struct RowInserted;

    /// At least one row was updated.
    ///
    /// Source: ISO/IEC 9075-2 §14.11 — `<update statement: searched>`
    pub struct RowUpdated;

    /// At least one row was deleted.
    ///
    /// Source: ISO/IEC 9075-2 §14.7 — `<delete statement: searched>`
    pub struct RowDeleted;

    /// Query returned at least one row.
    ///
    /// Source: ISO/IEC 9075-2 §14.1 — `<query expression>`
    pub struct NonEmptyResult;

    /// Transaction was durably committed.
    ///
    /// Source: ISO/IEC 9075-2 §17.3 — `<commit statement>`
    pub struct TransactionCommitted;

    /// Operation is atomic — either fully applied or fully aborted.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Atomicity (ACID properties)
    pub struct Atomic;

    /// Committed data survives system failure.
    ///
    /// Source: ISO/IEC 9075-2 §4.33 — Durability (ACID properties)
    pub struct Durable;

    /// Database was successfully created.
    ///
    /// Source: ISO/IEC 9075-2 §17 — `<SQL-schema statement>`
    pub struct DatabaseCreated;

    /// Schema was successfully created.
    ///
    /// Source: ISO/IEC 9075-2 §11.1 — `<schema definition>`
    pub struct SchemaCreated;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by DDL/DML success */ }
                }
            }
        };
    }

    structural_prop!(TableCreated, "TableCreated");
    structural_prop!(ConstraintSatisfied, "ConstraintSatisfied");
    structural_prop!(
        ReferentialIntegrityMaintained,
        "ReferentialIntegrityMaintained"
    );
    structural_prop!(ViewCreated, "ViewCreated");
    structural_prop!(RowInserted, "RowInserted");
    structural_prop!(RowUpdated, "RowUpdated");
    structural_prop!(RowDeleted, "RowDeleted");
    structural_prop!(NonEmptyResult, "NonEmptyResult");
    structural_prop!(TransactionCommitted, "TransactionCommitted");
    structural_prop!(Atomic, "Atomic");
    structural_prop!(Durable, "Durable");
    structural_prop!(DatabaseCreated, "DatabaseCreated");
    structural_prop!(SchemaCreated, "SchemaCreated");
}

pub use emit_impls::{
    Atomic, ConstraintSatisfied, DatabaseCreated, Durable, NonEmptyResult,
    ReferentialIntegrityMaintained, RowDeleted, RowInserted, RowUpdated, SchemaCreated,
    TableCreated, TransactionCommitted, ViewCreated,
};
