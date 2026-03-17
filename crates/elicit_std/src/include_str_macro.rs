//! `include_str!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for an `include_str!` macro invocation.
///
/// The emitted expression is `include_str!("{path}")`.  The path is relative
/// to the source file that contains the emitted code, which is the standard
/// Rust compile-time contract for `include_str!`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IncludeStrParams {
    /// Path to the file to embed (relative to the emitted source file).
    ///
    /// Example: `"data/config.toml"`, `"../assets/schema.json"`.
    pub path: String,
}

impl EmitCode for IncludeStrParams {
    fn emit_code(&self) -> TokenStream {
        let path = &self.path;
        quote! { include_str!(#path) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![] // include_str! is in std
    }
}

inventory::submit! {
    EmitEntry {
        tool: "include_str",
        crate_name: "elicit_std",
        constructor: |v| {
            serde_json::from_value::<IncludeStrParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
