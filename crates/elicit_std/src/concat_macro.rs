//! `concat!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `concat!` macro invocation.
///
/// The emitted expression is `concat!("a", "b", …)`.  All parts are
/// concatenated at **compile time** into a single `&'static str`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConcatParams {
    /// String literal parts to concatenate.
    ///
    /// Example: `["Hello", ", ", "world", "!"]` → `concat!("Hello", ", ", "world", "!")`.
    pub parts: Vec<String>,
}

impl EmitCode for ConcatParams {
    fn emit_code(&self) -> TokenStream {
        let parts = &self.parts;
        quote! { concat!(#(#parts),*) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![] // concat! is in std
    }
}

inventory::submit! {
    EmitEntry {
        tool: "concat",
        crate_name: "elicit_std",
        constructor: |v| {
            serde_json::from_value::<ConcatParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
