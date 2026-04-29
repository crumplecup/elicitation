//! Empirical Vec trust-boundary investigation.
//!
//! Each harness tests one Vec construction strategy inside a struct or enum
//! that mimics the VSM pattern (a state variant carrying a `Vec<T>` field,
//! dropped at the end of the harness).  Run WITHOUT any `#[kani::unwind]`
//! annotation.
//!
//! **A harness that terminates** = CBMC can see `len == 0` as concrete.
//! **A harness that spins**     = CBMC treats `len` as symbolic.
//!
//! # Running individual harnesses
//!
//! ```bash
//! cargo kani -p elicitation_kani --lib --harness <NAME>
//! ```
//!
//! # Strategies under test
//!
//! | Harness                         | Construction                         |
//! |---------------------------------|--------------------------------------|
//! | `vec_literal`                   | `vec![]`                             |
//! | `vec_new`                       | `Vec::<u8>::new()`                   |
//! | `vec_from_zero_array`           | `Vec::from([0u8; 0])`                |
//! | `vec_bounded_arbitrary_0`       | `<Vec<u8>>::bounded_any::<0>()`      |
//! | `vec_any_vec_0`                 | `kani::vec::any_vec::<u8, 0>()`      |
//! | `struct_with_vec_literal`       | struct field `vec![]`, then dropped  |
//! | `struct_with_bounded_arbitrary` | struct field `bounded_any::<0>()`    |
//! | `struct_with_any_vec_0`         | struct field `any_vec::<0>()`        |
//! | `enum_drop_vec_literal`         | enum Arbitrary uses `vec![]`         |
//! | `enum_drop_bounded_arbitrary`   | enum Arbitrary uses `bounded_any`    |
//! | `enum_drop_any_vec_0`           | enum Arbitrary uses `any_vec::<0>()` |
//! | `enum_drop_kani_vec`            | enum Arbitrary uses `kani_vec()`     |

use kani::BoundedArbitrary;

// ── Test types ───────────────────────────────────────────────────────────────

/// Struct with one Vec<u8> field — dropped at end of harness.
struct StateStruct {
    items: Vec<u8>,
}

/// Enum with two variants; CBMC explores both.  Drop of `WithItems` triggers
/// `drop_in_place::<[u8]>` — this is the problematic path in the VSM harnesses.
enum StateEnum {
    Empty,
    WithItems { items: Vec<u8> },
}

// ── Vec construction strategies (no drop) ────────────────────────────────────

#[kani::proof]
fn vec_literal() {
    let v: Vec<u8> = vec![];
    assert!(v.is_empty());
}

#[kani::proof]
fn vec_new() {
    let v = Vec::<u8>::new();
    assert!(v.is_empty());
}

#[kani::proof]
fn vec_from_zero_array() {
    let v = Vec::from([0u8; 0]);
    assert!(v.is_empty());
}

/// `BoundedArbitrary::bounded_any::<0>()` — uses `[T; N]` internally.
/// With N=0 the array is zero-size; `Vec::from([T; 0])` should give concrete len=0.
#[kani::proof]
fn vec_bounded_arbitrary_0() {
    let v = Vec::<u8>::bounded_any::<0>();
    assert!(v.is_empty());
}

/// `kani::vec::any_vec::<T, 0>()` — current `kani_vec` implementation.
/// Uses `any_where(|sz| *sz <= 0)` + `match`, which may leave len symbolic.
#[kani::proof]
fn vec_any_vec_0() {
    let v = kani::vec::any_vec::<u8, 0>();
    assert!(v.is_empty());
}

// ── Struct drop ───────────────────────────────────────────────────────────────

#[kani::proof]
fn struct_with_vec_literal() {
    let s = StateStruct { items: vec![] };
    assert!(s.items.is_empty());
    // implicit drop(s) — drop_in_place::<[u8]> must terminate
}

#[kani::proof]
fn struct_with_bounded_arbitrary() {
    let s = StateStruct {
        items: Vec::<u8>::bounded_any::<0>(),
    };
    assert!(s.items.is_empty());
}

#[kani::proof]
fn struct_with_any_vec_0() {
    let s = StateStruct {
        items: kani::vec::any_vec::<u8, 0>(),
    };
    assert!(s.items.is_empty());
}

// ── Enum Arbitrary impls (separate types to avoid impl conflicts) ─────────────

/// Enum Arbitrary backed by `vec![]`.
enum EnumLiteral {
    Empty,
    WithItems { items: Vec<u8> },
}

#[cfg(kani)]
impl kani::Arbitrary for EnumLiteral {
    fn any() -> Self {
        if kani::any::<bool>() {
            EnumLiteral::Empty
        } else {
            EnumLiteral::WithItems { items: vec![] }
        }
    }
}

/// Enum Arbitrary backed by `bounded_any::<0>()`.
enum EnumBounded {
    Empty,
    WithItems { items: Vec<u8> },
}

#[cfg(kani)]
impl kani::Arbitrary for EnumBounded {
    fn any() -> Self {
        if kani::any::<bool>() {
            EnumBounded::Empty
        } else {
            EnumBounded::WithItems {
                items: Vec::<u8>::bounded_any::<0>(),
            }
        }
    }
}

/// Enum Arbitrary backed by `any_vec::<0>()`.
enum EnumAnyVec {
    Empty,
    WithItems { items: Vec<u8> },
}

#[cfg(kani)]
impl kani::Arbitrary for EnumAnyVec {
    fn any() -> Self {
        if kani::any::<bool>() {
            EnumAnyVec::Empty
        } else {
            EnumAnyVec::WithItems {
                items: kani::vec::any_vec::<u8, 0>(),
            }
        }
    }
}

/// Enum Arbitrary backed by `elicitation::kani_vec()` (current production impl).
enum EnumKaniVec {
    Empty,
    WithItems { items: Vec<u8> },
}

#[cfg(kani)]
impl kani::Arbitrary for EnumKaniVec {
    fn any() -> Self {
        if kani::any::<bool>() {
            EnumKaniVec::Empty
        } else {
            EnumKaniVec::WithItems {
                items: elicitation::kani_vec::<u8>(),
            }
        }
    }
}

// ── Enum drop harnesses ───────────────────────────────────────────────────────

#[kani::proof]
fn enum_drop_vec_literal() {
    let s: EnumLiteral = kani::any();
    match &s {
        EnumLiteral::Empty => {}
        EnumLiteral::WithItems { items } => assert!(items.is_empty()),
    }
    // implicit drop(s)
}

#[kani::proof]
fn enum_drop_bounded_arbitrary() {
    let s: EnumBounded = kani::any();
    match &s {
        EnumBounded::Empty => {}
        EnumBounded::WithItems { items } => assert!(items.is_empty()),
    }
}

#[kani::proof]
fn enum_drop_any_vec_0() {
    let s: EnumAnyVec = kani::any();
    match &s {
        EnumAnyVec::Empty => {}
        EnumAnyVec::WithItems { items } => assert!(items.is_empty()),
    }
}

#[kani::proof]
fn enum_drop_kani_vec() {
    let s: EnumKaniVec = kani::any();
    match &s {
        EnumKaniVec::Empty => {}
        EnumKaniVec::WithItems { items } => assert!(items.is_empty()),
    }
}

// ── Incremental complexity: step from Vec<u8> toward Vec<ExportFormat> ───────
//
// Goal: find the smallest type that reproduces the unbounded drop loop.
//
// Level 1 — Vec<u8>              (covered above — all strategies pass)
// Level 2 — Vec<UnitEnum>        unit enum, kani::Arbitrary derived
// Level 3 — Vec<UnitEnumManual>  unit enum, Arbitrary hand-rolled
// Level 4 — Vec<StructNoHeap>    struct, all scalar fields
// Level 5 — Vec<StructWithString> struct with a String field (like SavedQuery)
// Level 6 — function call drops the Vec (crosses a call boundary)

// ── Level 2: unit enum with derived Arbitrary ─────────────────────────────────

#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Clone)]
enum FmtLike {
    Csv,
    Json,
    Tsv,
    Ndjson,
}

#[kani::proof]
fn vec_unit_enum_literal() {
    let v: Vec<FmtLike> = vec![];
    drop(v);
}

#[kani::proof]
fn vec_unit_enum_any_vec() {
    let v: Vec<FmtLike> = kani::vec::any_vec::<FmtLike, 0>();
    drop(v);
}

#[kani::proof]
fn vec_unit_enum_bounded_any() {
    let v: Vec<FmtLike> = Vec::<FmtLike>::bounded_any::<0>();
    drop(v);
}

// ── Level 3: unit enum with hand-rolled Arbitrary ────────────────────────────

#[derive(Clone)]
enum FmtManual {
    Csv,
    Json,
    Tsv,
}

#[cfg(kani)]
impl kani::Arbitrary for FmtManual {
    fn any() -> Self {
        match kani::any::<u8>() % 3 {
            0 => FmtManual::Csv,
            1 => FmtManual::Json,
            _ => FmtManual::Tsv,
        }
    }
}

#[kani::proof]
fn vec_unit_enum_manual_arb_literal() {
    let v: Vec<FmtManual> = vec![];
    drop(v);
}

#[kani::proof]
fn vec_unit_enum_manual_arb_any_vec() {
    let v: Vec<FmtManual> = kani::vec::any_vec::<FmtManual, 0>();
    drop(v);
}

#[kani::proof]
fn vec_unit_enum_manual_arb_bounded_any() {
    let v: Vec<FmtManual> = Vec::<FmtManual>::bounded_any::<0>();
    drop(v);
}

// ── Level 4: struct, scalar fields only ───────────────────────────────────────

#[derive(Clone)]
struct ScalarRow {
    id: i64,
    count: u64,
}

#[cfg(kani)]
impl kani::Arbitrary for ScalarRow {
    fn any() -> Self {
        ScalarRow {
            id: kani::any(),
            count: kani::any(),
        }
    }
}

#[kani::proof]
fn vec_scalar_struct_literal() {
    let v: Vec<ScalarRow> = vec![];
    drop(v);
}

#[kani::proof]
fn vec_scalar_struct_any_vec() {
    let v: Vec<ScalarRow> = kani::vec::any_vec::<ScalarRow, 0>();
    drop(v);
}

#[kani::proof]
fn vec_scalar_struct_bounded_any() {
    let v: Vec<ScalarRow> = Vec::<ScalarRow>::bounded_any::<0>();
    drop(v);
}

// ── Level 5: struct with a String field ───────────────────────────────────────

#[derive(Clone)]
struct StringRow {
    id: i64,
    name: String,
    sql: String,
}

#[cfg(kani)]
impl kani::Arbitrary for StringRow {
    fn any() -> Self {
        StringRow {
            id: kani::any(),
            name: String::new(),
            sql: String::new(),
        }
    }
}

#[kani::proof]
fn vec_string_struct_literal() {
    let v: Vec<StringRow> = vec![];
    drop(v);
}

#[kani::proof]
fn vec_string_struct_any_vec() {
    let v: Vec<StringRow> = kani::vec::any_vec::<StringRow, 0>();
    drop(v);
}

#[kani::proof]
fn vec_string_struct_bounded_any() {
    let v: Vec<StringRow> = Vec::<StringRow>::bounded_any::<0>();
    drop(v);
}

// ── Level 6: function call drops the Vec ──────────────────────────────────────
//
// Mimics the VSM pattern: a state enum is passed into a function that drops it.

enum OverlayLike {
    None,
    Picker { formats: Vec<FmtLike>, idx: usize },
    Browser { entries: Vec<StringRow>, idx: usize },
}

#[cfg(kani)]
impl kani::Arbitrary for OverlayLike {
    fn any() -> Self {
        match kani::any::<u8>() % 3 {
            0 => OverlayLike::None,
            1 => OverlayLike::Picker {
                formats: Vec::<FmtLike>::bounded_any::<0>(),
                idx: kani::any(),
            },
            _ => OverlayLike::Browser {
                entries: Vec::<StringRow>::bounded_any::<0>(),
                idx: kani::any(),
            },
        }
    }
}

/// Mimics `close_overlay`: takes state by value, drops it, returns `None`.
fn consume_overlay(state: OverlayLike) -> OverlayLike {
    drop(state);
    OverlayLike::None
}

#[kani::proof]
fn fn_drop_overlay_bounded_any() {
    let state: OverlayLike = kani::any();
    let result = consume_overlay(state);
    assert!(matches!(result, OverlayLike::None));
}

/// Same but using any_vec inside the Arbitrary impl.
enum OverlayAnyVec {
    None,
    Picker { formats: Vec<FmtLike>, idx: usize },
    Browser { entries: Vec<StringRow>, idx: usize },
}

#[cfg(kani)]
impl kani::Arbitrary for OverlayAnyVec {
    fn any() -> Self {
        match kani::any::<u8>() % 3 {
            0 => OverlayAnyVec::None,
            1 => OverlayAnyVec::Picker {
                formats: kani::vec::any_vec::<FmtLike, 0>(),
                idx: kani::any(),
            },
            _ => OverlayAnyVec::Browser {
                entries: kani::vec::any_vec::<StringRow, 0>(),
                idx: kani::any(),
            },
        }
    }
}

fn consume_overlay_anyvec(state: OverlayAnyVec) -> OverlayAnyVec {
    drop(state);
    OverlayAnyVec::None
}

#[kani::proof]
fn fn_drop_overlay_any_vec() {
    let state: OverlayAnyVec = kani::any();
    let result = consume_overlay_anyvec(state);
    assert!(matches!(result, OverlayAnyVec::None));
}

// ── Level 7: String isolation — pin down the exact failure mode ──────────────
//
// Summary of findings from Level 5:
//
//   `vec_string_struct_any_vec`      → TIMEOUT (hangs)
//   `vec_string_struct_bounded_any`  → TIMEOUT (hangs)
//   `vec_string_struct_literal`      → PASS  0.07s
//
// Root cause (traced from Kani source at ~/.kani/kani-0.67.0/library/kani/src/):
//
//   any_vec::<T, 0>():
//     routes through exact_vec → `<[T]>::into_vec(Box::new(any::<[T;0]>()))`.
//     `into_vec` loses the compile-time N=0; the resulting Vec carries a
//     runtime-symbolic `len` field in CBMC's model.
//
//   bounded_any::<0>():
//     calls `vec.truncate(any_where(|s| *s <= 0))`.  Even though constrained
//     to ≤0, CBMC does not reduce `symbolic_0` to the concrete value `0`.
//     `truncate` computes `remaining_len = self.len - symbolic_0` (symbolic),
//     then calls `drop_in_place` on a slice of symbolic length → unbounded.
//
//   For types with trivial drop (unit enums, scalar structs) the loop body is
//   cheap enough that CBMC terminates.  For StringRow (String destructor →
//   dealloc), CBMC diverges.
//
//   String: !kani::Arbitrary — `kani::any::<String>()` does not compile.
//   `String::bounded_any::<N>()` exists and works, but StringRow::any() uses
//   `String::new()` (concrete), which is fine.
//
// The Level 7 harnesses below confirm:
//   - String::new() is fine
//   - StringRow::any() (uses String::new) is fine
//   - Vec::new::<StringRow>() is fine
//   - kani::any::<[StringRow; 0]>() is fine
//   - Vec::from(kani::any::<[StringRow; 0]>()) is fine  ← what bounded_any does
//
// The issue is specifically the SYMBOLIC len produced by any_vec/bounded_any
// for element types that have non-trivial destructors.

/// Directly drop a concrete String::new().
#[kani::proof]
fn string_new_drop() {
    let s = String::new();
    drop(s);
}

/// Construct StringRow using String::new() directly (not via Arbitrary), drop it.
#[kani::proof]
fn string_row_direct_drop() {
    let row = StringRow {
        id: kani::any(),
        name: String::new(),
        sql: String::new(),
    };
    drop(row);
}

/// Construct StringRow via its Arbitrary impl, drop it.
#[kani::proof]
fn string_row_arbitrary_drop() {
    let row: StringRow = kani::any();
    drop(row);
}

/// Construct Vec<StringRow> via Vec::new() (concrete empty), drop it.
#[kani::proof]
fn vec_string_row_new_drop() {
    let v: Vec<StringRow> = Vec::new();
    drop(v);
}

/// Construct [StringRow; 0] via kani::any(), drop it.
#[kani::proof]
fn array_string_row_zero_any_drop() {
    let a: [StringRow; 0] = kani::any();
    drop(a);
}

/// Construct Vec<StringRow> from [StringRow; 0] via kani::any(), drop it.
/// This is exactly what bounded_any does internally, without the truncate call.
/// It PASSES — confirming the truncate(symbolic) is the hang site in bounded_any.
#[kani::proof]
fn vec_from_array_string_row_zero_any() {
    let a: [StringRow; 0] = kani::any();
    let v: Vec<StringRow> = Vec::from(a);
    drop(v);
}

// ── Level 8: definitive fix — Vec::new() for the kani_vec trust boundary ─────
//
// The correct implementation of `kani_vec<T>()` is `Vec::new()`.
// It gives CBMC a concrete len=0; no loop body is generated at all.
// No element type restriction needed — T does not need to be Arbitrary.
//
// Proof scope: "for any state whose Vec fields are EMPTY, the structural
// transition is correct."  This is the correct scope for VSM invariants.

/// kani_vec fix: Vec::new() for StringRow inside an enum variant, dropped.
/// Mirrors the actual ArchiveOverlayState pattern.
enum OverlayFixed {
    None,
    Picker { formats: Vec<FmtLike>, idx: usize },
    Browser { entries: Vec<StringRow>, idx: usize },
}

#[cfg(kani)]
impl kani::Arbitrary for OverlayFixed {
    fn any() -> Self {
        match kani::any::<u8>() % 3 {
            0 => OverlayFixed::None,
            1 => OverlayFixed::Picker {
                formats: Vec::new(), // kani_vec() fix
                idx: kani::any(),
            },
            _ => OverlayFixed::Browser {
                entries: Vec::new(), // kani_vec() fix
                idx: kani::any(),
            },
        }
    }
}

fn consume_overlay_fixed(state: OverlayFixed) -> OverlayFixed {
    drop(state);
    OverlayFixed::None
}

/// Definitive harness: Vec::new() + struct-with-String element type + function boundary.
/// This is the exact pattern of close_overlay__kani.
#[kani::proof]
fn fn_drop_overlay_fixed() {
    let state: OverlayFixed = kani::any();
    let result = consume_overlay_fixed(state);
    assert!(matches!(result, OverlayFixed::None));
}

// ── Level 9: per-variant harnesses — avoid symbolic enum dispatch ─────────────
//
// Root cause of the fn_drop_overlay_fixed hang:
//
//   When kani::any::<OverlayFixed>() creates a SYMBOLIC enum via a match on
//   kani::any::<u8>() % 3, CBMC models the whole enum symbolically, including
//   ALL variant fields simultaneously.  The drop of a symbolic OverlayFixed
//   must reason about ALL variant destructors at once.  Even though Vec::new()
//   was used in each arm, CBMC does not propagate per-variant field invariants
//   through the symbolic discriminant — so Vec fields appear symbolic-length
//   globally, and drop_in_place hangs.
//
//   SOLUTION: prove each variant separately.  Construct the state CONCRETELY
//   (no match dispatch), call the transition, assert the result.  CBMC sees
//   a concrete variant and knows the Vec fields are empty.
//
// This is the correct harness pattern for VSM structural proofs.

/// Browser variant: concrete construction, no match dispatch.
#[kani::proof]
fn close_overlay_browser_variant() {
    let state = OverlayFixed::Browser {
        entries: Vec::new(),
        idx: kani::any(),
    };
    let result = consume_overlay_fixed(state);
    assert!(matches!(result, OverlayFixed::None));
}

/// Picker variant: concrete construction, no match dispatch.
#[kani::proof]
fn close_overlay_picker_variant() {
    let state = OverlayFixed::Picker {
        formats: Vec::new(),
        idx: kani::any(),
    };
    let result = consume_overlay_fixed(state);
    assert!(matches!(result, OverlayFixed::None));
}

/// None variant: trivial structural check.
#[kani::proof]
fn close_overlay_none_variant() {
    let state = OverlayFixed::None;
    let result = consume_overlay_fixed(state);
    assert!(matches!(result, OverlayFixed::None));
}
