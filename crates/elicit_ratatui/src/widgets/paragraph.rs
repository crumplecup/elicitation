//! [`ratatui::widgets::Paragraph`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Paragraph<'static>, as Paragraph);
elicit_newtype_traits!(Paragraph, ratatui::widgets::Paragraph<'static>, []);

#[reflect_methods]
impl Paragraph {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Paragraph {
        Paragraph(std::sync::Arc::new((*self.0).clone().block((*block.0).clone())))
    }

    /// Set the text style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Paragraph {
        Paragraph(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set text alignment.
    #[tracing::instrument(skip(self))]
    pub fn alignment(&self, align: crate::Alignment) -> Paragraph {
        Paragraph(std::sync::Arc::new((*self.0).clone().alignment(align.into_inner())))
    }

    /// Enable line wrapping.
    #[tracing::instrument(skip(self))]
    pub fn wrap(&self, trim: bool) -> Paragraph {
        Paragraph(std::sync::Arc::new((*self.0).clone().wrap(ratatui::widgets::Wrap { trim })))
    }

    /// Set scroll offset `(vertical, horizontal)`.
    #[tracing::instrument(skip(self))]
    pub fn scroll(&self, vertical: u16, horizontal: u16) -> Paragraph {
        Paragraph(std::sync::Arc::new((*self.0).clone().scroll((vertical, horizontal))))
    }
}
impl serde::Serialize for Paragraph {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Paragraph {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ParagraphJson { text: Option<crate::Text> }
        let ParagraphJson { text } = ParagraphJson::deserialize(d)?;
        let rt = text
            .map(|t| (*t.0).clone())
            .unwrap_or_default();
        Ok(Paragraph(std::sync::Arc::new(ratatui::widgets::Paragraph::new(rt))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Paragraph {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Paragraph::new(::elicit_ratatui::Text::raw("")) }
        }
    }
}
impl elicitation::ElicitComplete for Paragraph {}
