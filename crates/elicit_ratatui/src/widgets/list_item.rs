//! [`ratatui::widgets::ListItem`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::ListItem<'static>, as ListItem);
elicit_newtype_traits!(ListItem, ratatui::widgets::ListItem<'static>, []);

#[reflect_methods]
impl ListItem {
    /// Apply a style to this item.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> ListItem {
        ListItem(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }
}
impl serde::Serialize for ListItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ListItemJson {
            #[serde(default)]
            content: String,
        }
        let ListItemJson { content } = ListItemJson::deserialize(d)?;
        Ok(ListItem(std::sync::Arc::new(
            ratatui::widgets::ListItem::new(content),
        )))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::ListItem {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::ListItem::new("") }
        }
    }
}
impl elicitation::ElicitComplete for ListItem {}
