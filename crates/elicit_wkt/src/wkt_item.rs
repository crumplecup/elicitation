//! `WktItem` — parsed WKT geometry wrapper.

use elicitation::{WktGeom, elicit_newtype};
use elicitation_derive::reflect_methods;
use std::str::FromStr;
use tracing::instrument;

elicit_newtype!(WktGeom, as WktItem, serde);

fn parse_wkt_item(wkt: &str) -> Result<WktItem, String> {
    wkt::Wkt::<f64>::from_str(wkt)
        .map(WktGeom::from)
        .map(WktItem::from)
        .map_err(|error| error.to_string())
}

impl FromStr for WktItem {
    type Err = String;

    #[instrument]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_wkt_item(s)
    }
}

#[reflect_methods]
impl WktItem {
    /// Serializes this geometry back to a WKT string.
    #[instrument(skip(self))]
    pub fn wkt_string(&self) -> String {
        let geom: &WktGeom = self.as_ref();
        let wkt: wkt::Wkt<f64> = geom.clone().into();
        wkt.to_string()
    }

    /// Returns the concrete geometry type name.
    #[instrument(skip(self))]
    pub fn geometry_type(&self) -> String {
        match self.as_ref() {
            WktGeom::Point(_) => "Point",
            WktGeom::LineString(_) => "LineString",
            WktGeom::Polygon(_) => "Polygon",
            WktGeom::MultiPoint(_) => "MultiPoint",
            WktGeom::MultiLineString(_) => "MultiLineString",
            WktGeom::MultiPolygon(_) => "MultiPolygon",
            WktGeom::GeometryCollection(_) => "GeometryCollection",
        }
        .to_string()
    }
}

mod emit_impls {
    use super::WktItem;

    impl elicitation::emit_code::ToCodeLiteral for WktItem {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("WktItem is serializable");
            quote::quote! {
                ::elicit_wkt::WktItem::from(
                    ::serde_json::from_str::<::elicitation::WktGeom>(#json)
                        .expect("valid WktItem JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for WktItem {}
