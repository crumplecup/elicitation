//! [`ratatui::widgets::Table`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Table<'static>, as Table);
elicit_newtype_traits!(Table, ratatui::widgets::Table<'static>, []);

#[reflect_methods]
impl Table {
    /// Set the header row.
    #[tracing::instrument(skip(self, header))]
    pub fn header(&self, header: crate::Row) -> Table {
        Table(std::sync::Arc::new(
            (*self.0).clone().header((*header.0).clone()),
        ))
    }

    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Table {
        Table(std::sync::Arc::new(
            (*self.0).clone().block((*block.0).clone()),
        ))
    }

    /// Set the table style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Table {
        Table(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Style for the selected row.
    #[tracing::instrument(skip(self))]
    pub fn highlight_style(&self, style: crate::Style) -> Table {
        Table(std::sync::Arc::new(
            (*self.0).clone().row_highlight_style(*style.0),
        ))
    }

    /// Symbol prefix for the selected row.
    #[tracing::instrument(skip(self))]
    pub fn highlight_symbol(&self, symbol: String) -> Table {
        Table(std::sync::Arc::new(
            (*self.0).clone().highlight_symbol(symbol),
        ))
    }

    /// Space between columns in characters.
    #[tracing::instrument(skip(self))]
    pub fn column_spacing(&self, spacing: u16) -> Table {
        Table(std::sync::Arc::new(
            (*self.0).clone().column_spacing(spacing),
        ))
    }
}
impl serde::Serialize for Table {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Table {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct TableJson {
            #[serde(default)]
            rows: Vec<crate::Row>,
            #[serde(default)]
            widths: Vec<crate::Constraint>,
        }
        let TableJson { rows, widths } = TableJson::deserialize(d)?;
        let ratatui_rows: Vec<ratatui::widgets::Row<'static>> =
            rows.into_iter().map(|r| (*r.0).clone()).collect();
        let ratatui_widths: Vec<ratatui::layout::Constraint> =
            widths.into_iter().map(|c| *c.0).collect();
        Ok(Table(std::sync::Arc::new(ratatui::widgets::Table::new(
            ratatui_rows,
            ratatui_widths,
        ))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Table {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Table::new(::std::vec![], ::std::vec![]) }
        }
    }
}
impl elicitation::ElicitComplete for Table {}
