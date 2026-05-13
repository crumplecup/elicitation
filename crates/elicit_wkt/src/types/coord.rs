//! `Coord` — elicitation-enabled wrapper around `elicitation::WktCoord`.

use elicitation::{WktCoord, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktCoord, as Coord, serde);

impl Coord {
    /// Creates a 2D WKT coordinate from x and y values.
    #[instrument]
    pub fn new(x: f64, y: f64) -> Self {
        WktCoord {
            x,
            y,
            z: None,
            m: None,
        }
        .into()
    }

    /// Creates a 3D WKT coordinate from x, y, and z values.
    #[instrument]
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        WktCoord {
            x,
            y,
            z: Some(z),
            m: None,
        }
        .into()
    }

    /// Creates a measured WKT coordinate from x, y, and m values.
    #[instrument]
    pub fn new_with_m(x: f64, y: f64, m: f64) -> Self {
        WktCoord {
            x,
            y,
            z: None,
            m: Some(m),
        }
        .into()
    }
}

#[reflect_methods]
impl Coord {
    /// Returns the x component.
    #[instrument(skip(self))]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the y component.
    #[instrument(skip(self))]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the z component, if present.
    #[instrument(skip(self))]
    pub fn z(&self) -> Option<f64> {
        self.z
    }

    /// Returns the measure component, if present.
    #[instrument(skip(self))]
    pub fn m(&self) -> Option<f64> {
        self.m
    }
}

mod emit_impls {
    use super::Coord;

    impl elicitation::emit_code::ToCodeLiteral for Coord {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Coord is serializable");
            quote::quote! {
                ::elicit_wkt::Coord::from(
                    ::serde_json::from_str::<::elicitation::WktCoord>(#json)
                        .expect("valid Coord JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Coord {}
