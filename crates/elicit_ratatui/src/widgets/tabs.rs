//! [`ratatui::widgets::Tabs`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Tabs<'static>, as Tabs);
elicit_newtype_traits!(Tabs, ratatui::widgets::Tabs<'static>, []);

#[reflect_methods]
impl Tabs {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().block((*block.0).clone())))
    }

    /// Select a tab by index.
    #[tracing::instrument(skip(self))]
    pub fn select(&self, index: usize) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().select(index)))
    }

    /// Overall style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Style for the selected tab.
    #[tracing::instrument(skip(self))]
    pub fn highlight_style(&self, style: crate::Style) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().highlight_style(*style.0)))
    }

    /// Divider string between tabs.
    #[tracing::instrument(skip(self))]
    pub fn divider(&self, divider: String) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().divider(divider)))
    }

    /// Left padding inside each tab.
    #[tracing::instrument(skip(self))]
    pub fn padding_left(&self, padding: String) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().padding_left(padding)))
    }

    /// Right padding inside each tab.
    #[tracing::instrument(skip(self))]
    pub fn padding_right(&self, padding: String) -> Tabs {
        Tabs(std::sync::Arc::new((*self.0).clone().padding_right(padding)))
    }
}
impl serde::Serialize for Tabs {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tabs {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct TabsJson { #[serde(default)] titles: Vec<String> }
        let TabsJson { titles } = TabsJson::deserialize(d)?;
        Ok(Tabs(std::sync::Arc::new(ratatui::widgets::Tabs::new(titles))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Tabs {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Tabs::new(::std::vec![]) }
        }
    }
}
impl elicitation::ElicitComplete for Tabs {}
