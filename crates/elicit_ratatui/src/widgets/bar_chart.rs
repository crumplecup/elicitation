//! [`ratatui::widgets::BarChart`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::BarChart<'static>, as BarChart);
elicit_newtype_traits!(BarChart, ratatui::widgets::BarChart<'static>, []);

#[reflect_methods]
impl BarChart {

    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().block((*block.0).clone())))
    }

    /// Width of each bar in characters.
    #[tracing::instrument(skip(self))]
    pub fn bar_width(&self, width: u16) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().bar_width(width)))
    }

    /// Gap between bars.
    #[tracing::instrument(skip(self))]
    pub fn bar_gap(&self, gap: u16) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().bar_gap(gap)))
    }

    /// Cap the max value shown.
    #[tracing::instrument(skip(self))]
    pub fn max(&self, max: u64) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().max(max)))
    }

    /// Overall chart style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Bar fill style.
    #[tracing::instrument(skip(self))]
    pub fn bar_style(&self, style: crate::Style) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().bar_style(*style.0)))
    }

    /// Label text style.
    #[tracing::instrument(skip(self))]
    pub fn label_style(&self, style: crate::Style) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().label_style(*style.0)))
    }

    /// Value text style.
    #[tracing::instrument(skip(self))]
    pub fn value_style(&self, style: crate::Style) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().value_style(*style.0)))
    }

    /// Bar direction (Vertical or Horizontal).
    #[tracing::instrument(skip(self))]
    pub fn direction(&self, direction: crate::Direction) -> BarChart {
        BarChart(std::sync::Arc::new((*self.0).clone().direction(direction.into_inner())))
    }
}
impl serde::Serialize for BarChart {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for BarChart {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(BarChart(std::sync::Arc::new(ratatui::widgets::BarChart::default())))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::BarChart {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::BarChart::from(::ratatui::widgets::BarChart::default()) }
        }
    }
}
impl elicitation::ElicitComplete for BarChart {}
