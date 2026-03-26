//! `sqlx::query!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `sqlx::query!()` macro invocation.
///
/// Emits `sqlx::query!(sql, param1, param2, ...)`.
///
/// **Build-time constraint**: the binary that uses this fragment must have
/// `DATABASE_URL` set in its environment at compile time.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryParams {
    /// The SQL query string (e.g. `"SELECT id, name FROM users WHERE id = $1"`).
    pub sql: String,

    /// Bind parameter expressions in order (`$1`, `$2`, …).
    ///
    /// Each element is a raw Rust expression.  Pass `"user_id"` to bind the
    /// local variable `user_id` as `$1`.
    #[serde(default)]
    pub params: Vec<String>,
}

impl EmitCode for QueryParams {
    fn emit_code(&self) -> TokenStream {
        let sql = &self.sql;
        let params: Vec<TokenStream> = self
            .params
            .iter()
            .map(|p| p.parse().unwrap_or_else(|_| quote!(/* parse error */)))
            .collect();
        if params.is_empty() {
            quote! { sqlx::query!(#sql) }
        } else {
            quote! { sqlx::query!(#sql, #(#params),*) }
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
        tool: "query",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<QueryParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
