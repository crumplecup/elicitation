//! `sqlx::migrate!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `sqlx::migrate!()` macro invocation.
///
/// Emits `sqlx::migrate!("migrations/").run(&pool).await?`.
///
/// **Build-time constraint**: `DATABASE_URL` must be set at compile time of
/// the emitted binary.  The migrations path is relative to the emitted
/// source file.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MigrateParams {
    /// Path to the migrations directory, relative to the emitted file
    /// (e.g. `"migrations"`).
    ///
    /// Defaults to `"migrations"` if omitted.
    #[serde(default = "default_migrations_path")]
    pub migrations_path: String,

    /// Name of the pool variable at the call site (e.g. `"pool"`).
    ///
    /// The emitted code calls `.run(&pool_var).await?`.
    #[serde(default = "default_pool_var")]
    pub pool_var: String,
}

fn default_migrations_path() -> String {
    "migrations".to_string()
}

fn default_pool_var() -> String {
    "pool".to_string()
}

impl EmitCode for MigrateParams {
    fn emit_code(&self) -> TokenStream {
        let path = &self.migrations_path;
        let pool: TokenStream = self
            .pool_var
            .parse()
            .unwrap_or_else(|_| quote!(/* parse error */));
        quote! {
            sqlx::migrate!(#path).run(&#pool).await?
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![CrateDep {
            name: "sqlx",
            version: "0.8",
            features: &["runtime-tokio", "any", "migrate"],
        }]
    }
}

inventory::submit! {
    EmitEntry {
        tool: "migrate",
        crate_name: "elicit_sqlx",
        constructor: |v| {
            serde_json::from_value::<MigrateParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
