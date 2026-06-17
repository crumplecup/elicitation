//! [`ratatui::widgets::Dataset`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Dataset<'static>, as Dataset);
elicit_newtype_traits!(Dataset, ratatui::widgets::Dataset<'static>, []);

#[reflect_methods]
impl Dataset {
    /// Set the dataset name shown in the legend.
    #[tracing::instrument(skip(self))]
    pub fn name(&self, name: String) -> Dataset {
        Dataset(std::sync::Arc::new((*self.0).clone().name(name)))
    }

    /// Set the data points as `(x, y)` pairs.
    #[tracing::instrument(skip(self, data))]
    pub fn data(&self, data: Vec<(f64, f64)>) -> Dataset {
        let data: &'static [(f64, f64)] = Box::leak(data.into_boxed_slice());
        Dataset(std::sync::Arc::new((*self.0).clone().data(data)))
    }

    /// Set the marker style: `"Dot"`, `"Block"`, `"Bar"`, `"Braille"`, `"HalfBlock"`.
    #[tracing::instrument(skip(self))]
    pub fn marker(&self, marker: String) -> Dataset {
        let m = match marker.as_str() {
            "Block" => ratatui::symbols::Marker::Block,
            "Bar" => ratatui::symbols::Marker::Bar,
            "Braille" => ratatui::symbols::Marker::Braille,
            "HalfBlock" => ratatui::symbols::Marker::HalfBlock,
            _ => ratatui::symbols::Marker::Dot,
        };
        Dataset(std::sync::Arc::new((*self.0).clone().marker(m)))
    }

    /// Set the line style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Dataset {
        Dataset(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }
}
impl serde::Serialize for Dataset {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dataset {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(Dataset(std::sync::Arc::new(ratatui::widgets::Dataset::default())))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Dataset {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Dataset::from(::ratatui::widgets::Dataset::default()) }
        }
    }
}
impl elicitation::ElicitComplete for Dataset {}
