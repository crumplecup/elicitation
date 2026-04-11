//! Non-empty proof validation tests.
//!
//! Calls `validate_proofs_non_empty()` on every manually-implemented
//! `Elicitation` type to ensure no method returns `TokenStream::new()`.
//! This catches regressions where a refactor accidentally drops proof
//! content from a manual impl.

use elicitation::Elicitation;

/// Assert all three proof methods return non-empty TokenStreams.
#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
    assert!(
        !T::verus_proof().is_empty(),
        "{label}: verus_proof is empty"
    );
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: creusot_proof is empty"
    );
}

// ============================================================================
// Primitives — bool
// ============================================================================

use elicitation::verification::types::{BoolDefault, BoolFalse, BoolTrue};

#[test]
fn bool_proofs_non_empty() {
    assert_proofs_non_empty::<bool>("bool");
    assert_proofs_non_empty::<BoolDefault>("BoolDefault");
    assert_proofs_non_empty::<BoolTrue>("BoolTrue");
    assert_proofs_non_empty::<BoolFalse>("BoolFalse");
}

// ============================================================================
// Primitives — integers
// ============================================================================

use elicitation::verification::types::{
    I8Default, I8NonNegative, I8NonZero, I8Positive, I16Default, I16NonNegative, I16NonZero,
    I16Positive, I32Default, I32NonNegative, I32NonZero, I32Positive, I64Default, I64NonNegative,
    I64NonZero, I64Positive, U8Default, U8NonZero, U8Positive, U16Default, U16NonZero, U16Positive,
    U32Default, U32NonZero, U32Positive, U64Default, U64NonZero, U64Positive,
};

#[test]
fn integer_proofs_non_empty() {
    assert_proofs_non_empty::<i8>("i8");
    assert_proofs_non_empty::<i16>("i16");
    assert_proofs_non_empty::<i32>("i32");
    assert_proofs_non_empty::<i64>("i64");
    assert_proofs_non_empty::<u8>("u8");
    assert_proofs_non_empty::<u16>("u16");
    assert_proofs_non_empty::<u32>("u32");
    assert_proofs_non_empty::<u64>("u64");
}

#[test]
fn integer_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<I8Default>("I8Default");
    assert_proofs_non_empty::<I8Positive>("I8Positive");
    assert_proofs_non_empty::<I8NonNegative>("I8NonNegative");
    assert_proofs_non_empty::<I8NonZero>("I8NonZero");
    assert_proofs_non_empty::<I16Default>("I16Default");
    assert_proofs_non_empty::<I16Positive>("I16Positive");
    assert_proofs_non_empty::<I16NonNegative>("I16NonNegative");
    assert_proofs_non_empty::<I16NonZero>("I16NonZero");
    assert_proofs_non_empty::<I32Default>("I32Default");
    assert_proofs_non_empty::<I32Positive>("I32Positive");
    assert_proofs_non_empty::<I32NonNegative>("I32NonNegative");
    assert_proofs_non_empty::<I32NonZero>("I32NonZero");
    assert_proofs_non_empty::<I64Default>("I64Default");
    assert_proofs_non_empty::<I64Positive>("I64Positive");
    assert_proofs_non_empty::<I64NonNegative>("I64NonNegative");
    assert_proofs_non_empty::<I64NonZero>("I64NonZero");
    assert_proofs_non_empty::<U8Default>("U8Default");
    assert_proofs_non_empty::<U8Positive>("U8Positive");
    assert_proofs_non_empty::<U8NonZero>("U8NonZero");
    assert_proofs_non_empty::<U16Default>("U16Default");
    assert_proofs_non_empty::<U16Positive>("U16Positive");
    assert_proofs_non_empty::<U16NonZero>("U16NonZero");
    assert_proofs_non_empty::<U32Default>("U32Default");
    assert_proofs_non_empty::<U32Positive>("U32Positive");
    assert_proofs_non_empty::<U32NonZero>("U32NonZero");
    assert_proofs_non_empty::<U64Default>("U64Default");
    assert_proofs_non_empty::<U64Positive>("U64Positive");
    assert_proofs_non_empty::<U64NonZero>("U64NonZero");
}

// ============================================================================
// Primitives — floats
// ============================================================================

use elicitation::verification::types::{
    F32Default, F32Finite, F32NonNegative, F32Positive, F64Default, F64Finite, F64NonNegative,
    F64Positive,
};

#[test]
fn float_proofs_non_empty() {
    assert_proofs_non_empty::<f32>("f32");
    assert_proofs_non_empty::<f64>("f64");
}

#[test]
fn float_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<F32Default>("F32Default");
    assert_proofs_non_empty::<F32Finite>("F32Finite");
    assert_proofs_non_empty::<F32NonNegative>("F32NonNegative");
    assert_proofs_non_empty::<F32Positive>("F32Positive");
    assert_proofs_non_empty::<F64Default>("F64Default");
    assert_proofs_non_empty::<F64Finite>("F64Finite");
    assert_proofs_non_empty::<F64NonNegative>("F64NonNegative");
    assert_proofs_non_empty::<F64Positive>("F64Positive");
}

// ============================================================================
// Primitives — char, String, PathBuf, Duration, SystemTime
// ============================================================================

use elicitation::verification::types::{
    CharAlphabetic, CharAlphanumeric, CharNumeric, DurationPositive, PathBufExists, PathBufIsDir,
    PathBufIsFile, PathBufReadable,
};

#[test]
fn char_proofs_non_empty() {
    assert_proofs_non_empty::<char>("char");
    assert_proofs_non_empty::<CharAlphabetic>("CharAlphabetic");
    assert_proofs_non_empty::<CharAlphanumeric>("CharAlphanumeric");
    assert_proofs_non_empty::<CharNumeric>("CharNumeric");
}

#[test]
fn string_proofs_non_empty() {
    assert_proofs_non_empty::<String>("String");
}

#[test]
fn pathbuf_proofs_non_empty() {
    assert_proofs_non_empty::<std::path::PathBuf>("PathBuf");
    assert_proofs_non_empty::<PathBufExists>("PathBufExists");
    assert_proofs_non_empty::<PathBufIsDir>("PathBufIsDir");
    assert_proofs_non_empty::<PathBufIsFile>("PathBufIsFile");
    assert_proofs_non_empty::<PathBufReadable>("PathBufReadable");
}

#[test]
fn duration_proofs_non_empty() {
    assert_proofs_non_empty::<std::time::Duration>("Duration");
    assert_proofs_non_empty::<DurationPositive>("DurationPositive");
}

#[test]
fn systemtime_proofs_non_empty() {
    assert_proofs_non_empty::<std::time::SystemTime>("SystemTime");
}

// ============================================================================
// Primitives — network
// ============================================================================

use elicitation::verification::types::{
    IpPrivate, IpPublic, IpV4, IpV6, Ipv4Loopback, Ipv6Loopback,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[test]
fn network_proofs_non_empty() {
    assert_proofs_non_empty::<IpAddr>("IpAddr");
    assert_proofs_non_empty::<Ipv4Addr>("Ipv4Addr");
    assert_proofs_non_empty::<Ipv6Addr>("Ipv6Addr");
    assert_proofs_non_empty::<SocketAddr>("SocketAddr");
    assert_proofs_non_empty::<SocketAddrV4>("SocketAddrV4");
    assert_proofs_non_empty::<SocketAddrV6>("SocketAddrV6");
    assert_proofs_non_empty::<IpV4>("IpV4");
    assert_proofs_non_empty::<IpV6>("IpV6");
    assert_proofs_non_empty::<IpPrivate>("IpPrivate");
    assert_proofs_non_empty::<IpPublic>("IpPublic");
    assert_proofs_non_empty::<Ipv4Loopback>("Ipv4Loopback");
    assert_proofs_non_empty::<Ipv6Loopback>("Ipv6Loopback");
}

// ============================================================================
// Generic stdlib containers
// ============================================================================

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

#[test]
fn generic_container_proofs_non_empty() {
    assert_proofs_non_empty::<Vec<bool>>("Vec<bool>");
    assert_proofs_non_empty::<Option<bool>>("Option<bool>");
    assert_proofs_non_empty::<Result<bool, String>>("Result<bool,String>");
    assert_proofs_non_empty::<Box<bool>>("Box<bool>");
    assert_proofs_non_empty::<std::sync::Arc<bool>>("Arc<bool>");
    assert_proofs_non_empty::<std::rc::Rc<bool>>("Rc<bool>");
    assert_proofs_non_empty::<HashMap<String, bool>>("HashMap<String,bool>");
    assert_proofs_non_empty::<BTreeMap<String, bool>>("BTreeMap<String,bool>");
    assert_proofs_non_empty::<HashSet<bool>>("HashSet<bool>");
    assert_proofs_non_empty::<BTreeSet<bool>>("BTreeSet<bool>");
    assert_proofs_non_empty::<VecDeque<bool>>("VecDeque<bool>");
    assert_proofs_non_empty::<LinkedList<bool>>("LinkedList<bool>");
    assert_proofs_non_empty::<[bool; 4]>("[bool; 4]");
}

// ============================================================================
// Verification wrapper types
// ============================================================================

use elicitation::verification::types::{
    ArcSatisfies, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxSatisfies, HashMapNonEmpty,
    HashSetNonEmpty, LinkedListNonEmpty, OptionSome, RcSatisfies, ResultOk, VecAllSatisfy,
    VecDequeNonEmpty, VecNonEmpty,
};

#[test]
fn verification_wrapper_proofs_non_empty() {
    assert_proofs_non_empty::<VecNonEmpty<bool>>("VecNonEmpty<bool>");
    assert_proofs_non_empty::<VecAllSatisfy<bool>>("VecAllSatisfy<bool>");
    assert_proofs_non_empty::<OptionSome<bool>>("OptionSome<bool>");
    assert_proofs_non_empty::<ResultOk<bool>>("ResultOk<bool>");
    assert_proofs_non_empty::<BoxSatisfies<bool>>("BoxSatisfies<bool>");
    assert_proofs_non_empty::<ArcSatisfies<bool>>("ArcSatisfies<bool>");
    assert_proofs_non_empty::<RcSatisfies<bool>>("RcSatisfies<bool>");
    assert_proofs_non_empty::<HashMapNonEmpty<String, bool>>("HashMapNonEmpty<String,bool>");
    assert_proofs_non_empty::<BTreeMapNonEmpty<String, bool>>("BTreeMapNonEmpty<String,bool>");
    assert_proofs_non_empty::<HashSetNonEmpty<bool>>("HashSetNonEmpty<bool>");
    assert_proofs_non_empty::<BTreeSetNonEmpty<bool>>("BTreeSetNonEmpty<bool>");
    assert_proofs_non_empty::<VecDequeNonEmpty<bool>>("VecDequeNonEmpty<bool>");
    assert_proofs_non_empty::<LinkedListNonEmpty<bool>>("LinkedListNonEmpty<bool>");
}

// ============================================================================
// URL
// ============================================================================

#[cfg(feature = "url")]
mod url_tests {
    use super::assert_proofs_non_empty;
    use elicitation::verification::types::{
        UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost,
    };

    #[test]
    fn url_proofs_non_empty() {
        assert_proofs_non_empty::<url::Url>("url::Url");
        assert_proofs_non_empty::<UrlValid>("UrlValid");
        assert_proofs_non_empty::<UrlHttp>("UrlHttp");
        assert_proofs_non_empty::<UrlHttps>("UrlHttps");
        assert_proofs_non_empty::<UrlWithHost>("UrlWithHost");
        assert_proofs_non_empty::<UrlCanBeBase>("UrlCanBeBase");
    }
}

// ============================================================================
// UUID
// ============================================================================

#[cfg(feature = "uuid")]
mod uuid_tests {
    use super::assert_proofs_non_empty;
    use elicitation::verification::types::{UuidNonNil, UuidV4};

    #[test]
    fn uuid_proofs_non_empty() {
        assert_proofs_non_empty::<uuid::Uuid>("uuid::Uuid");
        assert_proofs_non_empty::<UuidNonNil>("UuidNonNil");
        assert_proofs_non_empty::<UuidV4>("UuidV4");
    }
}

// ============================================================================
// WKT
// ============================================================================

#[cfg(feature = "geojson-types")]
mod geojson_tests {
    use super::assert_proofs_non_empty;

    #[test]
    fn geojson_proofs_non_empty() {
        assert_proofs_non_empty::<geojson::GeoJson>("geojson::GeoJson");
        assert_proofs_non_empty::<geojson::Geometry>("geojson::Geometry");
        assert_proofs_non_empty::<geojson::Value>("geojson::Value");
        assert_proofs_non_empty::<geojson::Feature>("geojson::Feature");
        assert_proofs_non_empty::<geojson::FeatureCollection>("geojson::FeatureCollection");
        assert_proofs_non_empty::<geojson::feature::Id>("geojson::feature::Id");
    }
}

// ============================================================================
// WKT
// ============================================================================

#[cfg(feature = "wkt-types")]
mod wkt_tests {
    use super::assert_proofs_non_empty;

    #[test]
    fn wkt_proofs_non_empty() {
        assert_proofs_non_empty::<elicitation::WktCoord>("WktCoord");
        assert_proofs_non_empty::<elicitation::WktPoint>("WktPoint");
        assert_proofs_non_empty::<elicitation::WktLineString>("WktLineString");
        assert_proofs_non_empty::<elicitation::WktPolygon>("WktPolygon");
        assert_proofs_non_empty::<elicitation::WktMultiPoint>("WktMultiPoint");
        assert_proofs_non_empty::<elicitation::WktMultiLineString>("WktMultiLineString");
        assert_proofs_non_empty::<elicitation::WktMultiPolygon>("WktMultiPolygon");
        assert_proofs_non_empty::<elicitation::WktGeometryCollection>("WktGeometryCollection");
        assert_proofs_non_empty::<elicitation::WktGeom>("WktGeom");
        assert_proofs_non_empty::<elicitation::WktString>("WktString");
    }
}

// ============================================================================
// WKB
// ============================================================================

#[cfg(feature = "wkb-types")]
mod wkb_tests {
    use super::assert_proofs_non_empty;

    #[test]
    fn wkb_proofs_non_empty() {
        assert_proofs_non_empty::<elicitation::WkbEndianness>("WkbEndianness");
        assert_proofs_non_empty::<elicitation::WkbDimension>("WkbDimension");
        assert_proofs_non_empty::<elicitation::WkbGeometryType>("WkbGeometryType");
        assert_proofs_non_empty::<elicitation::WkbBytes>("WkbBytes");
        assert_proofs_non_empty::<elicitation::WkbWriteOptions>("WkbWriteOptions");
    }
}

// ============================================================================
// Unit / trivial types
// ============================================================================

#[test]
fn unit_proofs_non_empty() {
    assert_proofs_non_empty::<()>("()");
}

// ============================================================================
// Derived unit-variant enums (regression: previously produced empty proofs)
// ============================================================================

use elicitation::{Elicit, Prompt, Select};

/// Unit-variant enum with two states — the TicTacToe `Player` case.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Elicit,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
enum TwoState {
    A,
    B,
}

/// Unit-variant enum with no `Default` derive — exercises `kani_first_variant_constructible`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Elicit,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
enum ThreeState {
    Alpha,
    Beta,
    Gamma,
}

/// Enum that wraps a unit-variant enum — exercises the cascading-emptiness case.
#[derive(
    Debug, Clone, PartialEq, Eq, Elicit, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
enum Wrapper {
    Empty,
    Occupied(TwoState),
}

#[test]
fn derived_unit_variant_enum_proofs_non_empty() {
    assert_proofs_non_empty::<TwoState>("TwoState (unit-variant enum regression)");
    assert_proofs_non_empty::<ThreeState>("ThreeState (no Default regression)");
}

#[test]
fn cascading_unit_variant_enum_proofs_non_empty() {
    // Wrapper delegates to TwoState; since TwoState now has a non-empty proof,
    // Wrapper's delegation loop extends by something non-empty.
    assert_proofs_non_empty::<Wrapper>("Wrapper (cascading delegation regression)");
}

// ============================================================================
// Third-party — accesskit types
// ============================================================================

#[cfg(feature = "accesskit")]
mod accesskit_proofs {
    use elicitation::Elicitation;

    #[track_caller]
    fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
        assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
        assert!(
            !T::verus_proof().is_empty(),
            "{label}: verus_proof is empty"
        );
        assert!(
            !T::creusot_proof().is_empty(),
            "{label}: creusot_proof is empty"
        );
    }

    #[test]
    fn accesskit_enum_proofs_non_empty() {
        assert_proofs_non_empty::<accesskit::Role>("accesskit::Role");
        assert_proofs_non_empty::<accesskit::Action>("accesskit::Action");
        assert_proofs_non_empty::<accesskit::Invalid>("accesskit::Invalid");
        assert_proofs_non_empty::<accesskit::Toggled>("accesskit::Toggled");
        assert_proofs_non_empty::<accesskit::Orientation>("accesskit::Orientation");
        assert_proofs_non_empty::<accesskit::TextDirection>("accesskit::TextDirection");
        assert_proofs_non_empty::<accesskit::SortDirection>("accesskit::SortDirection");
        assert_proofs_non_empty::<accesskit::AriaCurrent>("accesskit::AriaCurrent");
        assert_proofs_non_empty::<accesskit::AutoComplete>("accesskit::AutoComplete");
        assert_proofs_non_empty::<accesskit::Live>("accesskit::Live");
        assert_proofs_non_empty::<accesskit::HasPopup>("accesskit::HasPopup");
        assert_proofs_non_empty::<accesskit::ListStyle>("accesskit::ListStyle");
        assert_proofs_non_empty::<accesskit::TextAlign>("accesskit::TextAlign");
        assert_proofs_non_empty::<accesskit::VerticalOffset>("accesskit::VerticalOffset");
        assert_proofs_non_empty::<accesskit::TextDecorationStyle>("accesskit::TextDecorationStyle");
        assert_proofs_non_empty::<accesskit::ScrollUnit>("accesskit::ScrollUnit");
        assert_proofs_non_empty::<accesskit::ScrollHint>("accesskit::ScrollHint");
    }
}

// ============================================================================
// egui enum proof token-stream tests
// ============================================================================

#[cfg(feature = "egui-types")]
mod egui_proofs {
    use elicitation::Elicitation;

    #[track_caller]
    fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
        assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
        assert!(
            !T::verus_proof().is_empty(),
            "{label}: verus_proof is empty"
        );
        assert!(
            !T::creusot_proof().is_empty(),
            "{label}: creusot_proof is empty"
        );
    }

    #[test]
    fn egui_enum_proofs_non_empty() {
        assert_proofs_non_empty::<egui::Align>("egui::Align");
        assert_proofs_non_empty::<egui::CursorIcon>("egui::CursorIcon");
        assert_proofs_non_empty::<egui::Direction>("egui::Direction");
        assert_proofs_non_empty::<egui::FontFamily>("egui::FontFamily");
        assert_proofs_non_empty::<egui::Key>("egui::Key");
        assert_proofs_non_empty::<egui::Order>("egui::Order");
        assert_proofs_non_empty::<egui::PointerButton>("egui::PointerButton");
        assert_proofs_non_empty::<egui::TextStyle>("egui::TextStyle");
        assert_proofs_non_empty::<egui::TextWrapMode>("egui::TextWrapMode");
        assert_proofs_non_empty::<egui::epaint::textures::TextureFilter>("egui::TextureFilter");
        assert_proofs_non_empty::<egui::epaint::textures::TextureWrapMode>("egui::TextureWrapMode");
        assert_proofs_non_empty::<egui::Theme>("egui::Theme");
        assert_proofs_non_empty::<egui::ThemePreference>("egui::ThemePreference");
        assert_proofs_non_empty::<egui::TouchPhase>("egui::TouchPhase");
        assert_proofs_non_empty::<egui::UiKind>("egui::UiKind");
        assert_proofs_non_empty::<egui::WidgetType>("egui::WidgetType");
    }

    /// Compile-time assertion: all trenchcoat wrappers satisfy ElicitComplete.
    fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

    #[test]
    fn egui_trenchcoat_wrappers_elicit_complete() {
        assert_elicit_complete::<elicitation::AlignSelect>();
        assert_elicit_complete::<elicitation::CursorIconSelect>();
        assert_elicit_complete::<elicitation::DirectionSelect>();
        assert_elicit_complete::<elicitation::FontFamilySelect>();
        assert_elicit_complete::<elicitation::KeySelect>();
        assert_elicit_complete::<elicitation::OrderSelect>();
        assert_elicit_complete::<elicitation::PointerButtonSelect>();
        assert_elicit_complete::<elicitation::TextStyleSelect>();
        assert_elicit_complete::<elicitation::TextWrapModeSelect>();
        assert_elicit_complete::<elicitation::TextureFilterSelect>();
        assert_elicit_complete::<elicitation::TextureWrapModeSelect>();
        assert_elicit_complete::<elicitation::ThemeSelect>();
        assert_elicit_complete::<elicitation::ThemePreferenceSelect>();
        assert_elicit_complete::<elicitation::TouchPhaseSelect>();
        assert_elicit_complete::<elicitation::UiKindSelect>();
        assert_elicit_complete::<elicitation::WidgetTypeSelect>();
    }

    #[test]
    fn egui_trenchcoat_proofs_non_empty() {
        assert_proofs_non_empty::<elicitation::AlignSelect>("AlignSelect");
        assert_proofs_non_empty::<elicitation::CursorIconSelect>("CursorIconSelect");
        assert_proofs_non_empty::<elicitation::DirectionSelect>("DirectionSelect");
        assert_proofs_non_empty::<elicitation::FontFamilySelect>("FontFamilySelect");
        assert_proofs_non_empty::<elicitation::KeySelect>("KeySelect");
        assert_proofs_non_empty::<elicitation::OrderSelect>("OrderSelect");
        assert_proofs_non_empty::<elicitation::PointerButtonSelect>("PointerButtonSelect");
        assert_proofs_non_empty::<elicitation::TextStyleSelect>("TextStyleSelect");
        assert_proofs_non_empty::<elicitation::TextWrapModeSelect>("TextWrapModeSelect");
        assert_proofs_non_empty::<elicitation::TextureFilterSelect>("TextureFilterSelect");
        assert_proofs_non_empty::<elicitation::TextureWrapModeSelect>("TextureWrapModeSelect");
        assert_proofs_non_empty::<elicitation::ThemeSelect>("ThemeSelect");
        assert_proofs_non_empty::<elicitation::ThemePreferenceSelect>("ThemePreferenceSelect");
        assert_proofs_non_empty::<elicitation::TouchPhaseSelect>("TouchPhaseSelect");
        assert_proofs_non_empty::<elicitation::UiKindSelect>("UiKindSelect");
        assert_proofs_non_empty::<elicitation::WidgetTypeSelect>("WidgetTypeSelect");
    }

    // ── Composite struct wrapper tests ──────────────────────────────────

    #[test]
    fn egui_composite_proofs_non_empty() {
        assert_proofs_non_empty::<elicitation::EguiColor32>("EguiColor32");
        assert_proofs_non_empty::<elicitation::EguiPos2>("EguiPos2");
        assert_proofs_non_empty::<elicitation::EguiVec2>("EguiVec2");
        assert_proofs_non_empty::<elicitation::EguiRect>("EguiRect");
        assert_proofs_non_empty::<elicitation::EguiStroke>("EguiStroke");
        assert_proofs_non_empty::<elicitation::EguiCornerRadius>("EguiCornerRadius");
        assert_proofs_non_empty::<elicitation::EguiShadow>("EguiShadow");
        assert_proofs_non_empty::<elicitation::EguiMargin>("EguiMargin");
        assert_proofs_non_empty::<elicitation::EguiFontId>("EguiFontId");
    }

    #[test]
    fn egui_composite_wrappers_elicit_complete() {
        assert_elicit_complete::<elicitation::EguiColor32>();
        assert_elicit_complete::<elicitation::EguiPos2>();
        assert_elicit_complete::<elicitation::EguiVec2>();
        assert_elicit_complete::<elicitation::EguiRect>();
        assert_elicit_complete::<elicitation::EguiStroke>();
        assert_elicit_complete::<elicitation::EguiCornerRadius>();
        assert_elicit_complete::<elicitation::EguiShadow>();
        assert_elicit_complete::<elicitation::EguiMargin>();
        assert_elicit_complete::<elicitation::EguiFontId>();
    }
}

// ============================================================================
// Atomics — all 11 std::sync::atomic types
// ============================================================================

mod atomic_proofs {
    use super::assert_proofs_non_empty;
    use std::sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
        AtomicU32, AtomicU64, AtomicUsize,
    };

    fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

    #[test]
    fn atomic_proofs_non_empty() {
        assert_proofs_non_empty::<AtomicBool>("AtomicBool");
        assert_proofs_non_empty::<AtomicI8>("AtomicI8");
        assert_proofs_non_empty::<AtomicI16>("AtomicI16");
        assert_proofs_non_empty::<AtomicI32>("AtomicI32");
        assert_proofs_non_empty::<AtomicI64>("AtomicI64");
        assert_proofs_non_empty::<AtomicIsize>("AtomicIsize");
        assert_proofs_non_empty::<AtomicU8>("AtomicU8");
        assert_proofs_non_empty::<AtomicU16>("AtomicU16");
        assert_proofs_non_empty::<AtomicU32>("AtomicU32");
        assert_proofs_non_empty::<AtomicU64>("AtomicU64");
        assert_proofs_non_empty::<AtomicUsize>("AtomicUsize");
    }

    #[test]
    fn atomics_elicit_complete() {
        assert_elicit_complete::<AtomicBool>();
        assert_elicit_complete::<AtomicI8>();
        assert_elicit_complete::<AtomicI16>();
        assert_elicit_complete::<AtomicI32>();
        assert_elicit_complete::<AtomicI64>();
        assert_elicit_complete::<AtomicIsize>();
        assert_elicit_complete::<AtomicU8>();
        assert_elicit_complete::<AtomicU16>();
        assert_elicit_complete::<AtomicU32>();
        assert_elicit_complete::<AtomicU64>();
        assert_elicit_complete::<AtomicUsize>();
    }
}

// ============================================================================
// Derived struct harness content — non-vacuous constructibility proof
// ============================================================================

mod kani_struct_harness_tests {
    use elicitation::Elicitation;
    use elicitation_derive::Elicit;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    // A minimal data struct with two primitive fields.
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
    #[cfg_attr(kani, derive(kani::Arbitrary))]
    struct TestPoint {
        x: f64,
        y: f64,
    }

    // A zero-field marker struct — tautology is correct here.
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
    struct TestMarker;

    #[test]
    fn data_struct_kani_harness_uses_kani_any() {
        let proof = TestPoint::kani_proof().to_string();
        assert!(
            proof.contains("kani") && proof.contains("any"),
            "data struct kani_proof() should use kani::any(), got:\n{proof}"
        );
        assert!(
            proof.contains("TestPoint"),
            "data struct kani_proof() should reference the type name, got:\n{proof}"
        );
        assert!(
            !proof.contains("assert ! (established") && !proof.contains("assert!(established"),
            "data struct kani_proof() must not be a tautology, got:\n{proof}"
        );
    }

    #[test]
    fn marker_struct_kani_harness_is_tautology() {
        // Marker structs have no fields — assert!(true) is semantically correct.
        let proof = TestMarker::kani_proof().to_string();
        assert!(
            !proof.is_empty(),
            "marker struct kani_proof() must not be empty"
        );
    }
}
