//! Derive implementation for structs (Survey pattern).
//!
//! **Note**: Full implementation planned for Phase 5.

use proc_macro::TokenStream;
use syn::DeriveInput;

/// Expand #[derive(Elicit)] for structs.
///
/// **Status**: Stub implementation for Phase 4. Full implementation in Phase 5.
///
/// Will generate implementations of:
/// - Prompt (with optional custom prompt from #[prompt] attribute)
/// - Survey (field metadata)
/// - Elicit (sequential field elicitation)
pub fn expand_struct(_input: DeriveInput) -> TokenStream {
    let error = syn::Error::new(
        proc_macro2::Span::call_site(),
        "Struct derive implementation planned for Phase 5. \
         Currently only enum derivation is supported.",
    );
    error.to_compile_error().into()
}
