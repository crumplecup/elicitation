//! [`ratatui::widgets::Block`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Block<'static>, as Block);
elicit_newtype_traits!(Block, ratatui::widgets::Block<'static>, []);

#[reflect_methods]
impl Block {
    /// Set the block title.
    #[tracing::instrument(skip(self))]
    pub fn title(&self, title: String) -> Block {
        Block(std::sync::Arc::new((*self.0).clone().title(title)))
    }

    /// Set which borders to show.
    #[tracing::instrument(skip(self))]
    pub fn borders(&self, borders: crate::Borders) -> Block {
        Block(std::sync::Arc::new((*self.0).clone().borders(*borders)))
    }

    /// Set the border line style.
    #[tracing::instrument(skip(self))]
    pub fn border_type(&self, border_type: crate::BorderType) -> Block {
        Block(std::sync::Arc::new(
            (*self.0).clone().border_type(*border_type),
        ))
    }

    /// Set the block's overall style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Block {
        Block(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set the border style.
    #[tracing::instrument(skip(self))]
    pub fn border_style(&self, style: crate::Style) -> Block {
        Block(std::sync::Arc::new(
            (*self.0).clone().border_style(*style.0),
        ))
    }

    /// Set inner padding.
    #[tracing::instrument(skip(self))]
    pub fn padding(&self, padding: crate::Padding) -> Block {
        Block(std::sync::Arc::new(
            (*self.0).clone().padding(padding.into()),
        ))
    }
}

impl serde::Serialize for Block {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Block {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(Block(std::sync::Arc::new(ratatui::widgets::Block::new())))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Block {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Block::from(::ratatui::widgets::Block::new()) }
        }
    }
}
impl elicitation::ElicitComplete for Block {}
