//! [`ratatui::widgets::LineGauge`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::LineGauge<'static>, as LineGauge);
elicit_newtype_traits!(LineGauge, ratatui::widgets::LineGauge<'static>, []);

#[reflect_methods]
impl LineGauge {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> LineGauge {
        LineGauge(std::sync::Arc::new(
            (*self.0).clone().block((*block.0).clone()),
        ))
    }

    /// Set ratio (0.0–1.0).
    #[tracing::instrument(skip(self))]
    pub fn ratio(&self, ratio: f64) -> LineGauge {
        LineGauge(std::sync::Arc::new((*self.0).clone().ratio(ratio)))
    }

    /// Set the label span.
    #[tracing::instrument(skip(self))]
    pub fn label(&self, label: crate::Span) -> LineGauge {
        LineGauge(std::sync::Arc::new(
            (*self.0).clone().label((*label.0).clone()),
        ))
    }

    /// Set the overall style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> LineGauge {
        LineGauge(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set the filled-portion line style.
    #[tracing::instrument(skip(self))]
    pub fn filled_style(&self, style: crate::Style) -> LineGauge {
        LineGauge(std::sync::Arc::new(
            (*self.0).clone().filled_style(*style.0),
        ))
    }

    /// Set the unfilled-portion line style.
    #[tracing::instrument(skip(self))]
    pub fn unfilled_style(&self, style: crate::Style) -> LineGauge {
        LineGauge(std::sync::Arc::new(
            (*self.0).clone().unfilled_style(*style.0),
        ))
    }
}
impl serde::Serialize for LineGauge {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for LineGauge {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(LineGauge(std::sync::Arc::new(
            ratatui::widgets::LineGauge::default(),
        )))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::LineGauge {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::LineGauge::from(::ratatui::widgets::LineGauge::default()) }
        }
    }
}
impl elicitation::ElicitComplete for LineGauge {}
