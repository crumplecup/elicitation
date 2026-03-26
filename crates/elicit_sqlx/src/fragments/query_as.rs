//! `sqlx::query_as!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `sqlx::query_as!()` macro invocation.
///
/// Emits `sqlx::query_as!(TargetType, sql, param1, ...)`.
///
/// **Build-time constraint**: `DATABASE_URL` must be set at compile time of
/// the emitted binary.  The target type must implement `sqlx::FromRow`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryAsParams {
    /// The Rust type to deserialize each row into (e.g. `"User"`).
    ///
    /// Must be in scope at the call site and implement `sqlx::FromRow`.
    pub target_type: String,

    /// The SQL query string.
    pub sql: String,

    /// Bind parameter expressions in order (`$1`, `$2`, …).
    #[serde(default)]
    pub params: Vec<String>,
}

impl EmitCode for QueryAsParams {
    fn emit_code(&self) -> TokenStream {
        let target: TokenStream = self
            .target_type
            .parse()
            .unwrap_or_else(|_| quote!(/* parse error */));
        let sql = &self.sql;
        let params: Vec<TokenStream> = self
            .params
            .iter()
            .map(|p| p.parse().unwrap_or_else(|_| quote!(/* parse error */)))
            .collect();
        if params.is_empty() {
            quote! { sqlx::query_as!(#target, #sql) }
        } else {
            quote! { sqlx::query_as!(#target, #sql, #(#params),*) }
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
        tool: "query_as",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<QueryAsParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
