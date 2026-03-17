//! `format!` macro emit tool.

use elicitation::emit_code::{CrateDep, EmitCode, EmitEntry};
use proc_macro2::TokenStream;
use quote::quote;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parameters for a `format!` macro invocation.
///
/// The emitted expression is `format!("{template}", {args…})`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FormatParams {
    /// The format string template (e.g. `"Hello, {}! You have {} messages."`).
    pub template: String,
    /// Rust expressions to interpolate, in order (e.g. `["name", "count"]`).
    ///
    /// Each element is treated as a raw Rust expression in the emitted source.
    #[serde(default)]
    pub args: Vec<String>,
}

impl EmitCode for FormatParams {
    fn emit_code(&self) -> TokenStream {
        let template = &self.template;
        let args: Vec<TokenStream> = self
            .args
            .iter()
            .map(|a| a.parse().unwrap_or_else(|_| quote!(/* parse error */)))
            .collect();
        if args.is_empty() {
            quote! { format!(#template) }
        } else {
            quote! { format!(#template, #(#args),*) }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![] // format! is in std
    }
}

inventory::submit! {
    EmitEntry {
        tool: "format",
        crate_name: "elicit_std",
        constructor: |v| {
            serde_json::from_value::<FormatParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
