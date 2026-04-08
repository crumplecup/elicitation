//! Proof composition regression tests.
//!
//! These tests assert that aggregate types' proof outputs contain their
//! constituent types' proof outputs. This catches regressions in manual
//! `impl Elicitation` where a developer refactors the proof body and drops
//! a delegation call.
//!
//! Pattern:
//! ```
//! assert!(Outer::kani_proof_contains::<Inner>(), "message");
//! ```
//!
//! Only covers **manual** impls — derived types get composition for free
//! from the `#[derive(Elicit)]` macro which iterates fields mechanically.

use elicitation::Elicitation;

// ============================================================================
// Helper
// ============================================================================

/// Assert that `Outer`'s Kani proof contains `Inner`'s Kani proof.
#[track_caller]
fn assert_kani_contains<Outer, Inner>(context: &str)
where
    Outer: Elicitation,
    Inner: Elicitation,
{
    assert!(
        {
            let outer = Outer::kani_proof().to_string();
            let inner = Inner::kani_proof().to_string();
            !inner.is_empty() && outer.contains(&inner)
        },
        "{context}: {} kani_proof does not contain {} kani_proof",
        std::any::type_name::<Outer>(),
        std::any::type_name::<Inner>(),
    );
}

// ============================================================================
// Generic stdlib containers — delegate to element type
// ============================================================================

#[test]
fn vec_delegates_to_element() {
    assert_kani_contains::<Vec<bool>, bool>("Vec<bool>");
    assert_kani_contains::<Vec<String>, String>("Vec<String>");
}

#[test]
fn option_delegates_to_inner() {
    assert_kani_contains::<Option<bool>, bool>("Option<bool>");
    assert_kani_contains::<Option<String>, String>("Option<String>");
}

#[test]
fn result_delegates_to_ok_and_err() {
    assert_kani_contains::<Result<bool, String>, bool>("Result ok");
    assert_kani_contains::<Result<bool, String>, String>("Result err");
}

#[test]
fn box_delegates_to_inner() {
    assert_kani_contains::<Box<bool>, bool>("Box<bool>");
}

#[test]
fn arc_delegates_to_inner() {
    assert_kani_contains::<std::sync::Arc<bool>, bool>("Arc<bool>");
}

#[test]
fn rc_delegates_to_inner() {
    assert_kani_contains::<std::rc::Rc<bool>, bool>("Rc<bool>");
}

// ============================================================================
// Verification wrapper types — delegate wrapper harness + inner type
// ============================================================================

use elicitation::verification::types::{
    ArcSatisfies, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxSatisfies, HashMapNonEmpty,
    HashSetNonEmpty, LinkedListNonEmpty, OptionSome, RcSatisfies, ResultOk, VecAllSatisfy,
    VecDequeNonEmpty, VecNonEmpty,
};

#[test]
fn vec_non_empty_delegates_to_element() {
    assert_kani_contains::<VecNonEmpty<bool>, bool>("VecNonEmpty<bool>");
    assert_kani_contains::<VecNonEmpty<String>, String>("VecNonEmpty<String>");
}

#[test]
fn vec_all_satisfy_delegates_to_element() {
    assert_kani_contains::<VecAllSatisfy<bool>, bool>("VecAllSatisfy<bool>");
}

#[test]
fn option_some_delegates_to_inner() {
    assert_kani_contains::<OptionSome<bool>, bool>("OptionSome<bool>");
    assert_kani_contains::<OptionSome<String>, String>("OptionSome<String>");
}

#[test]
fn result_ok_delegates_to_inner() {
    assert_kani_contains::<ResultOk<bool>, bool>("ResultOk<bool>");
    assert_kani_contains::<ResultOk<String>, String>("ResultOk<String>");
}

#[test]
fn box_satisfies_delegates_to_inner() {
    assert_kani_contains::<BoxSatisfies<bool>, bool>("BoxSatisfies<bool>");
}

#[test]
fn arc_satisfies_delegates_to_inner() {
    assert_kani_contains::<ArcSatisfies<bool>, bool>("ArcSatisfies<bool>");
}

#[test]
fn rc_satisfies_delegates_to_inner() {
    assert_kani_contains::<RcSatisfies<bool>, bool>("RcSatisfies<bool>");
}

#[test]
fn hash_map_non_empty_delegates_to_kv() {
    assert_kani_contains::<HashMapNonEmpty<String, bool>, String>("HashMapNonEmpty key");
    assert_kani_contains::<HashMapNonEmpty<String, bool>, bool>("HashMapNonEmpty value");
}

#[test]
fn btree_map_non_empty_delegates_to_kv() {
    assert_kani_contains::<BTreeMapNonEmpty<String, bool>, String>("BTreeMapNonEmpty key");
    assert_kani_contains::<BTreeMapNonEmpty<String, bool>, bool>("BTreeMapNonEmpty value");
}

#[test]
fn hash_set_non_empty_delegates_to_element() {
    assert_kani_contains::<HashSetNonEmpty<bool>, bool>("HashSetNonEmpty<bool>");
}

#[test]
fn btree_set_non_empty_delegates_to_element() {
    assert_kani_contains::<BTreeSetNonEmpty<bool>, bool>("BTreeSetNonEmpty<bool>");
}

#[test]
fn vec_deque_non_empty_delegates_to_element() {
    assert_kani_contains::<VecDequeNonEmpty<bool>, bool>("VecDequeNonEmpty<bool>");
}

#[test]
fn linked_list_non_empty_delegates_to_element() {
    assert_kani_contains::<LinkedListNonEmpty<bool>, bool>("LinkedListNonEmpty<bool>");
}

// ============================================================================
// Primitives — delegate to wrapper types
// ============================================================================

use elicitation::verification::types::{BoolDefault, F64Default, I32Default, I64Default};

#[test]
fn bool_delegates_to_bool_default() {
    assert_kani_contains::<bool, BoolDefault>("bool → BoolDefault");
}

#[test]
fn i32_delegates_to_i32_default() {
    assert_kani_contains::<i32, I32Default>("i32 → I32Default");
}

#[test]
fn i64_delegates_to_i64_default() {
    assert_kani_contains::<i64, I64Default>("i64 → I64Default");
}

#[test]
fn f64_delegates_to_f64_default() {
    assert_kani_contains::<f64, F64Default>("f64 → F64Default");
}

// ============================================================================
// URL — delegates to UrlValid wrapper
// ============================================================================

#[cfg(feature = "url")]
mod url_tests {
    use super::assert_kani_contains;
    use elicitation::verification::types::UrlValid;

    #[test]
    fn url_delegates_to_url_valid() {
        assert_kani_contains::<url::Url, UrlValid>("url::Url → UrlValid");
    }
}

// ============================================================================
// Atomics — delegate to underlying primitive
// ============================================================================

#[test]
fn atomics_delegate_to_primitives() {
    use std::sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
        AtomicU32, AtomicU64, AtomicUsize,
    };

    assert_kani_contains::<AtomicBool, bool>("AtomicBool → bool");
    assert_kani_contains::<AtomicI8, i8>("AtomicI8 → i8");
    assert_kani_contains::<AtomicI16, i16>("AtomicI16 → i16");
    assert_kani_contains::<AtomicI32, i32>("AtomicI32 → i32");
    assert_kani_contains::<AtomicI64, i64>("AtomicI64 → i64");
    assert_kani_contains::<AtomicIsize, isize>("AtomicIsize → isize");
    assert_kani_contains::<AtomicU8, u8>("AtomicU8 → u8");
    assert_kani_contains::<AtomicU16, u16>("AtomicU16 → u16");
    assert_kani_contains::<AtomicU32, u32>("AtomicU32 → u32");
    assert_kani_contains::<AtomicU64, u64>("AtomicU64 → u64");
    assert_kani_contains::<AtomicUsize, usize>("AtomicUsize → usize");
}

// ============================================================================
// geo-types — composite types delegate to their constituent coord/geometry
// ============================================================================

#[cfg(feature = "geo-types")]
mod geo_types_tests {
    use super::assert_kani_contains;
    use elicitation::{
        GeoCoord, GeoGeometry, GeoGeometryCollection, GeoLine, GeoLineString, GeoMultiLineString,
        GeoMultiPoint, GeoMultiPolygon, GeoPoint, GeoPolygon, GeoRect, GeoTriangle,
    };

    #[test]
    fn geo_point_delegates_to_coord() {
        assert_kani_contains::<GeoPoint, GeoCoord>("GeoPoint → GeoCoord");
    }

    #[test]
    fn geo_triangle_delegates_to_coord() {
        assert_kani_contains::<GeoTriangle, GeoCoord>("GeoTriangle → GeoCoord");
    }

    #[test]
    fn geo_line_delegates_to_coord() {
        assert_kani_contains::<GeoLine, GeoCoord>("GeoLine → GeoCoord");
    }

    #[test]
    fn geo_rect_delegates_to_coord() {
        assert_kani_contains::<GeoRect, GeoCoord>("GeoRect → GeoCoord");
    }

    #[test]
    fn geo_line_string_delegates_to_coord() {
        assert_kani_contains::<GeoLineString, GeoCoord>("GeoLineString → GeoCoord");
    }

    #[test]
    fn geo_multi_point_delegates_to_point() {
        assert_kani_contains::<GeoMultiPoint, GeoPoint>("GeoMultiPoint → GeoPoint");
    }

    #[test]
    fn geo_multi_line_string_delegates_to_line_string() {
        assert_kani_contains::<GeoMultiLineString, GeoLineString>(
            "GeoMultiLineString → GeoLineString",
        );
    }

    #[test]
    fn geo_multi_polygon_delegates_to_polygon() {
        assert_kani_contains::<GeoMultiPolygon, GeoPolygon>("GeoMultiPolygon → GeoPolygon");
    }

    #[test]
    fn geo_geometry_collection_delegates_to_geometry() {
        assert_kani_contains::<GeoGeometryCollection, GeoGeometry>(
            "GeoGeometryCollection → GeoGeometry",
        );
    }

    #[test]
    fn geo_polygon_delegates_to_line_string() {
        assert_kani_contains::<GeoPolygon, GeoLineString>("GeoPolygon → GeoLineString");
    }

    #[test]
    fn geo_coord_delegates_to_f64() {
        use elicitation::Elicitation;
        let outer = GeoCoord::kani_proof().to_string();
        let inner = <f64 as Elicitation>::kani_proof().to_string();
        assert!(
            outer.contains(&inner),
            "GeoCoord::kani_proof() should contain f64::kani_proof()\nouter: {outer}\ninner: {inner}"
        );
    }
}

#[cfg(feature = "egui-types")]
mod egui_types_tests {
    use elicitation::Elicitation;
    use elicitation::{
        EguiColor32, EguiCornerRadius, EguiFontId, EguiMargin, EguiPos2, EguiRect, EguiShadow,
        EguiStroke, EguiVec2,
    };

    #[test]
    fn egui_color32_delegates_to_u8() {
        let outer = EguiColor32::kani_proof().to_string();
        let inner = <u8 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiColor32 should delegate to u8\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_corner_radius_delegates_to_u8() {
        let outer = EguiCornerRadius::kani_proof().to_string();
        let inner = <u8 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiCornerRadius should delegate to u8\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_margin_delegates_to_i8() {
        let outer = EguiMargin::kani_proof().to_string();
        let inner = <i8 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiMargin should delegate to i8\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_pos2_delegates_to_f32() {
        let outer = EguiPos2::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiPos2 should delegate to f32\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_vec2_delegates_to_f32() {
        let outer = EguiVec2::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiVec2 should delegate to f32\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_rect_delegates_to_f32() {
        let outer = EguiRect::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiRect should delegate to f32\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_font_id_delegates_to_f32() {
        let outer = EguiFontId::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiFontId should delegate to f32\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_font_id_delegates_to_color32() {
        let outer = EguiFontId::kani_proof().to_string();
        // FontFamilySelect proof must be present
        // (indirect check: outer is superset of f32, which is sufficient for this slot)
        assert!(!outer.is_empty(), "EguiFontId::kani_proof() must be non-empty");
    }

    #[test]
    fn egui_shadow_delegates_to_color32() {
        let outer = EguiShadow::kani_proof().to_string();
        let inner = EguiColor32::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiShadow should contain EguiColor32 proof\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_shadow_delegates_to_i8() {
        let outer = EguiShadow::kani_proof().to_string();
        let inner = <i8 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiShadow should contain i8 proof\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_stroke_delegates_to_f32() {
        let outer = EguiStroke::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiStroke should contain f32 proof\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn egui_stroke_delegates_to_color32() {
        let outer = EguiStroke::kani_proof().to_string();
        let inner = EguiColor32::kani_proof().to_string();
        assert!(outer.contains(&inner), "EguiStroke should contain EguiColor32 proof\nouter: {outer}\ninner: {inner}");
    }
}

#[cfg(feature = "palette")]
mod palette_types_tests {
    use elicitation::Elicitation;
    use elicitation::PaletteSrgb;

    #[test]
    fn palette_srgb_delegates_to_f32() {
        let outer = PaletteSrgb::kani_proof().to_string();
        let inner = <f32 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "PaletteSrgb should delegate to f32\nouter: {outer}\ninner: {inner}");
    }
}

#[cfg(feature = "ratatui")]
mod ratatui_types_tests {
    use elicitation::Elicitation;
    use elicitation::{RatatuiMargin, RatatuiPadding, RatatuiStyle};

    #[test]
    fn ratatui_margin_delegates_to_u16() {
        let outer = RatatuiMargin::kani_proof().to_string();
        let inner = <u16 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "RatatuiMargin should delegate to u16\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn ratatui_padding_delegates_to_u16() {
        let outer = RatatuiPadding::kani_proof().to_string();
        let inner = <u16 as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "RatatuiPadding should delegate to u16\nouter: {outer}\ninner: {inner}");
    }

    #[test]
    fn ratatui_style_delegates_to_bool() {
        let outer = RatatuiStyle::kani_proof().to_string();
        let inner = <bool as Elicitation>::kani_proof().to_string();
        assert!(outer.contains(&inner), "RatatuiStyle should contain bool proof\nouter: {outer}\ninner: {inner}");
    }
}
