//! [`ratatui::style::Style`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::style::Style, as Style, forward_serde);
elicit_newtype_traits!(Style, ratatui::style::Style, []);

#[reflect_methods]
impl Style {
    /// Set the foreground colour.
    #[tracing::instrument(skip(self))]
    pub fn fg(&self, color: crate::Color) -> Style {
        Style(std::sync::Arc::new((*self.0).fg(color.into_inner())))
    }

    /// Set the background colour.
    #[tracing::instrument(skip(self))]
    pub fn bg(&self, color: crate::Color) -> Style {
        Style(std::sync::Arc::new((*self.0).bg(color.into_inner())))
    }

    /// Add a text modifier (bold, italic, etc.).
    #[tracing::instrument(skip(self))]
    pub fn add_modifier(&self, modifier: crate::Modifier) -> Style {
        Style(std::sync::Arc::new((*self.0).add_modifier(*modifier.0)))
    }

    /// Remove a text modifier.
    #[tracing::instrument(skip(self))]
    pub fn remove_modifier(&self, modifier: crate::Modifier) -> Style {
        Style(std::sync::Arc::new((*self.0).remove_modifier(*modifier.0)))
    }

    /// Apply bold.
    #[tracing::instrument(skip(self))]
    pub fn bold(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).bold()))
    }

    /// Apply italic.
    #[tracing::instrument(skip(self))]
    pub fn italic(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).italic()))
    }

    /// Apply underline.
    #[tracing::instrument(skip(self))]
    pub fn underlined(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).underlined()))
    }

    /// Apply dim.
    #[tracing::instrument(skip(self))]
    pub fn dim(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).dim()))
    }

    /// Apply slow blink.
    #[tracing::instrument(skip(self))]
    pub fn slow_blink(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).slow_blink()))
    }

    /// Apply rapid blink.
    #[tracing::instrument(skip(self))]
    pub fn rapid_blink(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).rapid_blink()))
    }

    /// Apply reversed (swap fg/bg).
    #[tracing::instrument(skip(self))]
    pub fn reversed(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).reversed()))
    }

    /// Apply hidden.
    #[tracing::instrument(skip(self))]
    pub fn hidden(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).hidden()))
    }

    /// Apply crossed-out.
    #[tracing::instrument(skip(self))]
    pub fn crossed_out(&self) -> Style {
        Style(std::sync::Arc::new((*self.0).crossed_out()))
    }

    /// Patch this style with another, overriding only set fields.
    #[tracing::instrument(skip(self, other))]
    pub fn patch(&self, other: Style) -> Style {
        Style(std::sync::Arc::new((*self.0).patch(*other.0)))
    }
}

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for super::Style {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Style::from(#inner) }
        }
    }
}

impl elicitation::ElicitComplete for Style {}
