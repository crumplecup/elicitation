//! Elicitation support for [`georaster::geotiff::Pixels`].
//!
//! `Pixels<'a, R>` is a runtime pixel iterator tied to a `GeoTiffReader` by
//! lifetime `'a`. The `'a` lifetime prevents implementing `Elicitation: 'static`
//! and `ElicitIntrospect: Elicitation`, so only the three standalone traits
//! (`ElicitSpec`, `ElicitPromptTree`, `ToCodeLiteral`) are implemented here.

use std::io::{Read, Seek};

use georaster::geotiff::Pixels;

impl<'a, R: Read + Seek> crate::ElicitPromptTree for Pixels<'a, R> {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt: "Pixels is a runtime pixel iterator and cannot be constructed interactively."
                .to_string(),
            type_name: "georaster::geotiff::Pixels".to_string(),
        }
    }
}

impl<'a, R: Read + Seek> crate::emit_code::ToCodeLiteral for Pixels<'a, R> {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        // Pixels is an I/O iterator with no public constructor — no code literal.
        unimplemented!("Pixels cannot be converted to a code literal")
    }
}
