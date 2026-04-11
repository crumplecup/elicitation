//! `INFORMATION_SCHEMA` introspection propositions.
//!
//! Source: ISO/IEC 9075-11 — SQL/Schemata (Information and Definition Schemas).

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// The queried table exists in the information schema.
    ///
    /// Source: ISO/IEC 9075-11 §TABLES view
    pub struct TableExists;

    /// The queried column exists on its table.
    ///
    /// Source: ISO/IEC 9075-11 §COLUMNS view
    pub struct ColumnExists;

    /// The queried schema exists in the catalog.
    ///
    /// Source: ISO/IEC 9075-11 §SCHEMATA view
    pub struct SchemaExists;

    /// The named foreign key constraint exists.
    ///
    /// Source: ISO/IEC 9075-11 §REFERENTIAL_CONSTRAINTS view
    pub struct ForeignKeyExists;

    macro_rules! is_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by INFORMATION_SCHEMA query */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by INFORMATION_SCHEMA query */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by INFORMATION_SCHEMA query */ }
                }
            }
        };
    }

    is_prop!(TableExists, "TableExists");
    is_prop!(ColumnExists, "ColumnExists");
    is_prop!(SchemaExists, "SchemaExists");
    is_prop!(ForeignKeyExists, "ForeignKeyExists");
}

pub use emit_impls::{ColumnExists, ForeignKeyExists, SchemaExists, TableExists};
