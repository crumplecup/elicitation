//! Primitive type re-exports and newtype wrappers.
//!
//! Select enums come from `elicitation`'s built-in ratatui primitives (feature
//! `ratatui`). Struct types without a pre-built elicitation impl are wrapped
//! with `elicit_newtype!`. Value types that ratatui serializes (via its `serde`
//! feature) use the `forward_serde` variant so parameter structs in
//! `#[reflect_methods]` blocks satisfy the `Serialize + Deserialize` bounds.

use elicitation::{elicit_newtype, elicit_newtype_traits};

// ── Re-exports from elicitation's ratatui feature ────────────────────────────

/// Alignment select wrapper (Left / Center / Right).
pub use elicitation::AlignmentSelect as Alignment;
/// Border-type select wrapper.
pub use elicitation::BorderTypeSelect as BorderType;
/// Borders select wrapper (preset combinations).
pub use elicitation::BordersSelect as Borders;
/// Color select wrapper (named + RGB + indexed).
pub use elicitation::ColorSelect as Color;
/// Direction select wrapper (Vertical / Horizontal).
pub use elicitation::RatatuiDirectionSelect as Direction;
/// Margin survey wrapper (horizontal, vertical fields).
pub use elicitation::RatatuiMargin as Margin;
/// Padding survey wrapper (left, right, top, bottom fields).
pub use elicitation::RatatuiPadding as Padding;
/// Scrollbar-orientation select wrapper.
pub use elicitation::ScrollbarOrientationSelect as ScrollbarOrientation;

// ── elicit_newtype! wrappers — forward serde from ratatui's serde feature ────

elicit_newtype!(ratatui::layout::Constraint, as Constraint, forward_serde);
elicit_newtype_traits!(Constraint, ratatui::layout::Constraint, []);

elicit_newtype!(ratatui::layout::Rect, as Rect, forward_serde);
elicit_newtype_traits!(Rect, ratatui::layout::Rect, []);

elicit_newtype!(ratatui::style::Modifier, as Modifier, forward_serde);
elicit_newtype_traits!(Modifier, ratatui::style::Modifier, []);

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for super::Constraint {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Constraint::from(#inner) }
        }
    }

    impl ToCodeLiteral for super::Rect {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Rect::from(#inner) }
        }
    }

    impl ToCodeLiteral for super::Modifier {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Modifier::from(#inner) }
        }
    }
}

impl elicitation::ElicitComplete for Constraint {}
impl elicitation::ElicitComplete for Rect {}
impl elicitation::ElicitComplete for Modifier {}
