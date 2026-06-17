//! [`ratatui::widgets::Gauge`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Gauge<'static>, as Gauge);
elicit_newtype_traits!(Gauge, ratatui::widgets::Gauge<'static>, []);

#[reflect_methods]
impl Gauge {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().block((*block.0).clone())))
    }

    /// Set ratio (0.0–1.0).
    #[tracing::instrument(skip(self))]
    pub fn ratio(&self, ratio: f64) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().ratio(ratio)))
    }

    /// Set percentage (0–100).
    #[tracing::instrument(skip(self))]
    pub fn percent(&self, percent: u16) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().percent(percent)))
    }

    /// Set the label displayed inside the gauge.
    #[tracing::instrument(skip(self))]
    pub fn label(&self, label: String) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().label(label)))
    }

    /// Set the gauge's overall style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set the style of the filled portion.
    #[tracing::instrument(skip(self))]
    pub fn gauge_style(&self, style: crate::Style) -> Gauge {
        Gauge(std::sync::Arc::new((*self.0).clone().gauge_style(*style.0)))
    }
}
impl serde::Serialize for Gauge {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Gauge {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(Gauge(std::sync::Arc::new(ratatui::widgets::Gauge::default())))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Gauge {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Gauge::from(::ratatui::widgets::Gauge::default()) }
        }
    }
}
impl elicitation::ElicitComplete for Gauge {}
