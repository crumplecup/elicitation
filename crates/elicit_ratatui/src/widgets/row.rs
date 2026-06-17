//! [`ratatui::widgets::Row`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Row<'static>, as Row);
elicit_newtype_traits!(Row, ratatui::widgets::Row<'static>, []);

#[reflect_methods]
impl Row {
    /// Set row height in lines.
    #[tracing::instrument(skip(self))]
    pub fn height(&self, height: u16) -> Row {
        Row(std::sync::Arc::new((*self.0).clone().height(height)))
    }

    /// Apply a style to this row.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Row {
        Row(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set the top margin.
    #[tracing::instrument(skip(self))]
    pub fn top_margin(&self, margin: u16) -> Row {
        Row(std::sync::Arc::new((*self.0).clone().top_margin(margin)))
    }

    /// Set the bottom margin.
    #[tracing::instrument(skip(self))]
    pub fn bottom_margin(&self, margin: u16) -> Row {
        Row(std::sync::Arc::new((*self.0).clone().bottom_margin(margin)))
    }
}
impl serde::Serialize for Row {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Row {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct RowJson { #[serde(default)] cells: Vec<crate::Cell> }
        let RowJson { cells } = RowJson::deserialize(d)?;
        let ratatui_cells: Vec<ratatui::widgets::Cell<'static>> =
            cells.into_iter().map(|c| (*c.0).clone()).collect();
        Ok(Row(std::sync::Arc::new(ratatui::widgets::Row::new(ratatui_cells))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Row {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Row::new(::std::vec![]) }
        }
    }
}
impl elicitation::ElicitComplete for Row {}
