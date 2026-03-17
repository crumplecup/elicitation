//! `env!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for an `env!` macro invocation.
///
/// The emitted expression is `env!("{var}")` or `env!("{var}", "{message}")`.
/// The environment variable is read at **compile time** of the emitted binary,
/// not at the time this tool is called.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EnvParams {
    /// Name of the environment variable to read at compile time.
    ///
    /// Example: `"DATABASE_URL"`, `"CARGO_PKG_VERSION"`.
    pub var: String,

    /// Optional compile-time error message if the variable is not set.
    ///
    /// When provided, the emitted code is `env!("VAR", "message")`.
    #[serde(default)]
    pub error_message: Option<String>,
}

impl EmitCode for EnvParams {
    fn emit_code(&self) -> TokenStream {
        let var = &self.var;
        match &self.error_message {
            Some(msg) => quote! { env!(#var, #msg) },
            None => quote! { env!(#var) },
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![] // env! is in std
    }
}

inventory::submit! {
    EmitEntry {
        tool: "env",
        crate_name: "elicit_std",
        constructor: |v| {
            serde_json::from_value::<EnvParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
