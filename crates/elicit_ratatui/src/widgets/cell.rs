//! [`ratatui::widgets::Cell`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Cell<'static>, as Cell);
elicit_newtype_traits!(Cell, ratatui::widgets::Cell<'static>, []);

#[reflect_methods]
impl Cell {
    /// Apply a style to this cell.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Cell {
        Cell(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }
}
impl serde::Serialize for Cell {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cell {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct CellJson { #[serde(default)] content: String }
        let CellJson { content } = CellJson::deserialize(d)?;
        Ok(Cell(std::sync::Arc::new(ratatui::widgets::Cell::new(content))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Cell {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Cell::new("") }
        }
    }
}
impl elicitation::ElicitComplete for Cell {}
