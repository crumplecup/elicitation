//! [`ratatui::widgets::List`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::List<'static>, as List);
elicit_newtype_traits!(List, ratatui::widgets::List<'static>, []);

#[reflect_methods]
impl List {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> List {
        List(std::sync::Arc::new(
            (*self.0).clone().block((*block.0).clone()),
        ))
    }

    /// Set the list style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> List {
        List(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Style applied to the selected item.
    #[tracing::instrument(skip(self))]
    pub fn highlight_style(&self, style: crate::Style) -> List {
        List(std::sync::Arc::new(
            (*self.0).clone().highlight_style(*style.0),
        ))
    }

    /// Symbol prefix for the selected item.
    #[tracing::instrument(skip(self))]
    pub fn highlight_symbol(&self, symbol: String) -> List {
        List(std::sync::Arc::new(
            (*self.0).clone().highlight_symbol(symbol),
        ))
    }

    /// Repeat the highlight symbol for each line of a multi-line item.
    #[tracing::instrument(skip(self))]
    pub fn repeat_highlight_symbol(&self, repeat: bool) -> List {
        List(std::sync::Arc::new(
            (*self.0).clone().repeat_highlight_symbol(repeat),
        ))
    }
}
impl serde::Serialize for List {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for List {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ListJson {
            #[serde(default)]
            items: Vec<crate::ListItem>,
        }
        let ListJson { items } = ListJson::deserialize(d)?;
        let ratatui_items: Vec<ratatui::widgets::ListItem<'static>> =
            items.into_iter().map(|i| (*i.0).clone()).collect();
        Ok(List(std::sync::Arc::new(ratatui::widgets::List::new(
            ratatui_items,
        ))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::List {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::List::new(::std::vec![]) }
        }
    }
}
impl elicitation::ElicitComplete for List {}
