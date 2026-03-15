//! Naming helpers for `#[reflect_trait]` code generation.
//!
//! Converts fully-qualified Rust paths and method names into the snake_case
//! identifiers and MCP tool name strings used throughout the generated code.

use proc_macro2::Span;
use syn::Ident;

/// Convert a fully-qualified trait path to a snake_case factory struct name.
///
/// `"diesel::Insertable"` → `"InsertableFactory"`
/// `"my_crate::sub::MyTrait"` → `"MyTraitFactory"`
pub fn factory_struct_name(trait_path: &str) -> Ident {
    let last = last_segment(trait_path);
    Ident::new(&format!("{last}Factory"), Span::call_site())
}

/// Convert a fully-qualified trait path to a snake_case vtable struct name.
///
/// `"diesel::Insertable"` → `"InsertableVTable"`
pub fn vtable_struct_name(trait_path: &str) -> Ident {
    let last = last_segment(trait_path);
    Ident::new(&format!("{last}VTable"), Span::call_site())
}

/// Convert a fully-qualified trait path to the MCP meta-tool name.
///
/// Convert a fully-qualified path to double-underscore-separated snake_case.
///
/// `"my_crate::MyTrait"` → `"my_crate__my_trait"`
pub fn to_snake_path(path: &str) -> String {
    path.split("::")
        .map(camel_to_snake)
        .collect::<Vec<_>>()
        .join("__")
}

/// Convert a method name to a param struct ident.
///
/// `"insert"` → `InsertParams`  
/// `"batch_insert"` → `BatchInsertParams`
pub fn param_struct_name(method_name: &str) -> Ident {
    let pascal = snake_to_pascal(method_name);
    Ident::new(&format!("{pascal}Params"), Span::call_site())
}

/// Extract the last `::` segment from a path string.
pub fn last_segment_str(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

/// Extract the last `::` segment from a path string.
fn last_segment(path: &str) -> &str {
    last_segment_str(path)
}

/// Convert a CamelCase identifier to snake_case.
///
/// `"MyTrait"` → `"my_trait"`
/// `"MyHTTPClient"` → `"my_h_t_t_p_client"` (conservative, matches runtime)
pub fn camel_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Convert a snake_case identifier to PascalCase.
///
/// `"batch_insert"` → `"BatchInsert"`
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
