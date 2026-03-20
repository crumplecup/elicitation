//! `sqlx::query_scalar!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `sqlx::query_scalar!()` macro invocation.
///
/// Emits `sqlx::query_scalar!(sql, param1, ...)`.
///
/// **Build-time constraint**: `DATABASE_URL` must be set at compile time of
/// the emitted binary.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryScalarParams {
    /// The SQL query returning a single scalar column (e.g.
    /// `"SELECT COUNT(*) FROM users"`).
    pub sql: String,

    /// Bind parameter expressions in order (`$1`, `$2`, …).
    #[serde(default)]
    pub params: Vec<String>,
}

impl EmitCode for QueryScalarParams {
    fn emit_code(&self) -> TokenStream {
        let sql = &self.sql;
        let params: Vec<TokenStream> = self
            .params
            .iter()
            .map(|p| p.parse().unwrap_or_else(|_| quote!(/* parse error */)))
            .collect();
        if params.is_empty() {
            quote! { sqlx::query_scalar!(#sql) }
        } else {
            quote! { sqlx::query_scalar!(#sql, #(#params),*) }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![CrateDep {
            name: "sqlx",
            version: "0.8",
            features: &["runtime-tokio", "any"],
        }]
    }
}

inventory::submit! {
    EmitEntry {
        tool: "query_scalar",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<QueryScalarParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
