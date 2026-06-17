//! [`ratatui::widgets::Chart`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Chart<'static>, as Chart);
elicit_newtype_traits!(Chart, ratatui::widgets::Chart<'static>, []);

#[reflect_methods]
impl Chart {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Chart {
        Chart(std::sync::Arc::new(
            (*self.0).clone().block((*block.0).clone()),
        ))
    }

    /// Set the x-axis.
    #[tracing::instrument(skip(self, axis))]
    pub fn x_axis(&self, axis: crate::Axis) -> Chart {
        Chart(std::sync::Arc::new(
            (*self.0).clone().x_axis((*axis.0).clone()),
        ))
    }

    /// Set the y-axis.
    #[tracing::instrument(skip(self, axis))]
    pub fn y_axis(&self, axis: crate::Axis) -> Chart {
        Chart(std::sync::Arc::new(
            (*self.0).clone().y_axis((*axis.0).clone()),
        ))
    }

    /// Set the chart style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Chart {
        Chart(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }
}
impl serde::Serialize for Chart {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Chart {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ChartJson {
            #[serde(default)]
            datasets: Vec<crate::Dataset>,
        }
        let ChartJson { datasets } = ChartJson::deserialize(d)?;
        let ratatui_datasets: Vec<ratatui::widgets::Dataset<'static>> =
            datasets.into_iter().map(|d| (*d.0).clone()).collect();
        Ok(Chart(std::sync::Arc::new(ratatui::widgets::Chart::new(
            ratatui_datasets,
        ))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Chart {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Chart::new(::std::vec![]) }
        }
    }
}
impl elicitation::ElicitComplete for Chart {}
