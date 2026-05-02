//! Synthetic diagnostic harnesses for isolating BTreeMap/HashMap drop-glue issues.
//!
//! These theories use local enum types (no workspace dependencies) to pinpoint
//! whether enum union drop-glue causes unbounded CBMC unwinding when one variant
//! holds a BTreeMap but a *different* variant is constructed.
//!
//! Theories:
//! - BC: 2-variant enum, one arm has `BTreeMap`, construct the *other* arm → hang?
//! - BD: Same but `Box<BTreeMap>` in the BTree arm → does boxing fix it?
//! - BE: 2-variant enum, one arm has `HashMap`, construct the other arm → hang?
//! - BF: Same but `Box<HashMap>` → does boxing fix HashMap too?

use std::collections::{BTreeMap, HashMap};

// ── Theory BC ─────────────────────────────────────────────────────────────────
// 2-variant enum: variant A holds a BTreeMap, variant B holds a plain u32.
// We construct and drop B only — B has no BTreeMap.
//
// If this hangs: CBMC's static reachability pulls in A's drop-glue (BTree
// traversal) even when B is the live variant → union drop-glue is the culprit.
// If this passes: the problem is specific to the size/shape of ArchivePanelState.
enum SmallBTree {
    A { m: BTreeMap<String, f32> },
    B { x: u32 },
}

#[kani::proof]
fn theory_bc_btree_enum_other_variant() {
    let _v = SmallBTree::B { x: 42 };
}

// ── Theory BD ─────────────────────────────────────────────────────────────────
// Same as BC but variant A holds `Box<BTreeMap<String, f32>>`.
//
// If BC hangs but BD passes: boxing the BTree field cuts the reachable drop path
// (pointer dereference can be pruned; tree traversal cannot).
enum SmallBoxedBTree {
    A { m: Box<BTreeMap<String, f32>> },
    B { x: u32 },
}

#[kani::proof]
fn theory_bd_boxed_btree_enum_other_variant() {
    let _v = SmallBoxedBTree::B { x: 42 };
}

// ── Theory BE ─────────────────────────────────────────────────────────────────
// Same structure but with HashMap instead of BTreeMap.
// HashMap::new() is fine (no getrandom in Kani); question is drop-glue.
enum SmallHashMap {
    A { m: HashMap<String, f32> },
    B { x: u32 },
}

#[kani::proof]
fn theory_be_hashmap_enum_other_variant() {
    let _v = SmallHashMap::B { x: 42 };
}

// ── Theory BF ─────────────────────────────────────────────────────────────────
// HashMap behind Box — does boxing fix HashMap drop-glue too?
enum SmallBoxedHashMap {
    A { m: Box<HashMap<String, f32>> },
    B { x: u32 },
}

#[kani::proof]
fn theory_bf_boxed_hashmap_enum_other_variant() {
    let _v = SmallBoxedHashMap::B { x: 42 };
}

// ── Theory BG ─────────────────────────────────────────────────────────────────
// Scale test: 20-variant enum where only variant A0 has a BTreeMap, all others
// hold a plain u32.  Construct variant A19.
//
// BC (2-variant) passed in 0.21s.  If BG (20-variant) hangs, the problem is
// CBMC's per-variant drop-path analysis scaling with the number of variants.
enum BigBTree {
    A0 { m: BTreeMap<String, f32> },
    A1 { x: u32 },
    A2 { x: u32 },
    A3 { x: u32 },
    A4 { x: u32 },
    A5 { x: u32 },
    A6 { x: u32 },
    A7 { x: u32 },
    A8 { x: u32 },
    A9 { x: u32 },
    A10 { x: u32 },
    A11 { x: u32 },
    A12 { x: u32 },
    A13 { x: u32 },
    A14 { x: u32 },
    A15 { x: u32 },
    A16 { x: u32 },
    A17 { x: u32 },
    A18 { x: u32 },
    A19 { x: u32 },
}

#[kani::proof]
fn theory_bg_scale_20_variants_btree() {
    let _v = BigBTree::A19 { x: 42 };
}

// ── Theory BH ─────────────────────────────────────────────────────────────────
// Recursive type in one arm: variant A holds a locally-recursive enum, variant
// B holds a u32.  Construct B.
//
// serde_json::Value and DbValue::Json are recursive types present in
// ArchivePanelState::DataGrid.  If CBMC can't bound the drop of a recursive
// type even when we never construct it, this hangs.
enum RecursiveInner {
    Leaf(u32),
    Node(Box<RecursiveInner>),
}

enum ContainsRecursive {
    A { v: RecursiveInner },
    B { x: u32 },
}

#[kani::proof]
fn theory_bh_recursive_type_other_variant() {
    let _v = ContainsRecursive::B { x: 42 };
}

// ── Theory BI ─────────────────────────────────────────────────────────────────
// Same as BG (20-variant, BTree in A0) but the constructed arm A1 uses a
// SYMBOLIC value via kani::any() instead of a concrete 42.
//
// Result: PASS 0.13s — symbolic primitive + BTree in other arm is fine.

#[kani::proof]
fn theory_bi_symbolic_value_btree_other_variant() {
    let _v = BigBTree::A1 {
        x: kani::any::<u32>(),
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Ladder: incrementally approach AZ to find exactly what causes the hang.
//
// AZ: ArchivePanelState::ConnectionEdit { profile: ConnectionProfile::kani_depth0(),
//                                         display_mode: ... }  →  HANGS
//
// Known passing: BC (2-var, BTree dead, u32 live concrete)
//                BI (20-var, BTree dead, u32 live symbolic)
//
// We flip one variable per rung:
//   - What is in the LIVE arm (constructed variant)?
//   - What is in the DEAD arm (the non-constructed variant with complex drop)?
// ─────────────────────────────────────────────────────────────────────────────

// ── Rung 1 / Theory BJ ───────────────────────────────────────────────────────
// Live arm: String::new() — heap allocation, non-trivial drop but bounded.
// Dead arm: BTreeMap<String,f32>
// Question: does a String in the live arm matter?
enum LadderBJ {
    Dead { m: BTreeMap<String, f32> },
    Live { s: String },
}

#[kani::proof]
fn theory_bj_string_live_btree_dead() {
    let _v = LadderBJ::Live { s: String::new() };
}

// ── Rung 2 / Theory BK ───────────────────────────────────────────────────────
// Live arm: bool + u16 + String — mimics a minimal ConnectionProfile
// Dead arm: BTreeMap<String,f32>
// Question: does a multi-field live arm (some symbolic) + BTree dead matter?
enum LadderBK {
    Dead { m: BTreeMap<String, f32> },
    Live { b: bool, n: u16, s: String },
}

#[kani::proof]
fn theory_bk_connprofile_like_live_btree_dead() {
    let _v = LadderBK::Live {
        b: kani::any::<bool>(),
        n: kani::any::<u16>(),
        s: String::new(),
    };
}

// ── Rung 3 / Theory BL ───────────────────────────────────────────────────────
// Live arm: concrete u32
// Dead arm: Vec<String> — heap collection whose drop iterates over elements
// Question: does Vec<T-with-drop> in the dead arm (vs BTree) cause a hang?
enum LadderBL {
    Dead { v: Vec<String> },
    Live { x: u32 },
}

#[kani::proof]
fn theory_bl_vec_string_dead_concrete_live() {
    let _v = LadderBL::Live { x: 42 };
}

// ── Rung 4 / Theory BM ───────────────────────────────────────────────────────
// Live arm: symbolic kani::any::<u32>()
// Dead arm: Vec<String>
// Question: does symbolic live arm + Vec<String> dead matter?
enum LadderBM {
    Dead { v: Vec<String> },
    Live { x: u32 },
}

#[kani::proof]
fn theory_bm_vec_string_dead_symbolic_live() {
    let _v = LadderBM::Live {
        x: kani::any::<u32>(),
    };
}

// ── Rung 5 / Theory BN ───────────────────────────────────────────────────────
// Live arm: bool + u16 + String (ConnectionProfile-like, some symbolic)
// Dead arm: Vec<String>
// Question: ConnectionProfile-like complexity in live arm + Vec dead — hang?
enum LadderBN {
    Dead { v: Vec<String> },
    Live { b: bool, n: u16, s: String },
}

#[kani::proof]
fn theory_bn_connprofile_like_live_vec_dead() {
    let _v = LadderBN::Live {
        b: kani::any::<bool>(),
        n: kani::any::<u16>(),
        s: String::new(),
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Ladder 2: isolate the ConnectionEdit / ConnectionProfile hang
//
// BG (elicit_proofs): SqlEditor{ 2 Option fields } PASSES
// BE/BF (elicit_proofs): ConnectionEdit{ ConnectionProfile{ 7 Option<String> } } HANGS
//
// Hypothesis: 7 Option<String> fields cause the hang, either because of count
// or because of nested-struct wrapping.  Theories below flip each variable.
// ─────────────────────────────────────────────────────────────────────────────

// ── Theory BO ─────────────────────────────────────────────────────────────────
// 2-variant enum; LIVE arm has 7 Option<String> as DIRECT fields (no wrapper).
// Dead arm is a plain u32 (trivial drop).
// Pass → the count alone isn't the trigger; wrapping matters.
// Hang → 7 Option<String> direct fields is itself enough to cause the hang.
enum LadderBO {
    Dead {
        x: u32,
    },
    Live {
        a: Option<String>,
        b: Option<String>,
        c: Option<String>,
        d: Option<String>,
        e: Option<String>,
        f: Option<String>,
        g: Option<String>,
    },
}

#[kani::proof]
fn theory_bo_7_option_string_direct() {
    let _v = LadderBO::Live {
        a: None,
        b: None,
        c: None,
        d: None,
        e: None,
        f: None,
        g: None,
    };
}

// ── Theory BP ─────────────────────────────────────────────────────────────────
// 2-variant enum; LIVE arm has a NESTED struct with 7 Option<String> fields.
// This directly mirrors ConnectionProfile inside ConnectionEdit.
// Dead arm is a plain u32 (trivial drop).
// Pass → nesting isn't the trigger.
// Hang → wrapping in a nested struct is the trigger (vs BO's direct fields).
struct MockProfile {
    name: String,
    url: String,
    backend: u8,
    color: Option<String>,
    ssh_host: Option<String>,
    ssh_port: Option<u16>,
    ssh_user: Option<String>,
    ssh_key_env: Option<String>,
    ssl_mode: u8,
    ssl_cert_env: Option<String>,
    ssl_key_env: Option<String>,
    ssl_ca_env: Option<String>,
}

enum LadderBP {
    Dead { x: u32 },
    Live { profile: MockProfile, mode: u8 },
}

#[kani::proof]
fn theory_bp_nested_struct_7_options() {
    let _v = LadderBP::Live {
        profile: MockProfile {
            name: String::new(),
            url: String::new(),
            backend: 0,
            color: None,
            ssh_host: None,
            ssh_port: None,
            ssh_user: None,
            ssh_key_env: None,
            ssl_mode: 0,
            ssl_cert_env: None,
            ssl_key_env: None,
            ssl_ca_env: None,
        },
        mode: 0,
    };
}

// ── Theory BQ ─────────────────────────────────────────────────────────────────
// SAME as BP but dead arm has BTreeMap (like ErdView in ArchivePanelState has
// Option<ErdLayout> which contains a BTreeMap).
// RESULT: HANGS — BTree dead arm + complex live arm triggers unbounded loop.
// #[kani::proof]
// fn theory_bq_nested_profile_btree_dead() { ... }

// ── Theory BR ─────────────────────────────────────────────────────────────────
// Same as BQ but dead arm holds Box<BTreeMap> instead of BTreeMap directly.
// RESULT: HANGS — boxing the dead-arm BTree alone does not fix the interaction.
// #[kani::proof]
// fn theory_br_nested_profile_boxed_btree_dead() { ... }

// ── Theory BS ─────────────────────────────────────────────────────────────────
// Same setup as BQ (MockProfile live, BTree dead) but the live arm BOXES
// MockProfile, reducing the union footprint of the live arm to just a pointer.
//
// If this passes: boxing the LIVE arm's large struct is the fix.
//   → In ArchivePanelState, Box ConnectionProfile inside ConnectionEdit.
// If this hangs: we need to box both arms or use a different approach.
enum LadderBS {
    Dead { m: BTreeMap<String, f32> },
    Live { profile: Box<MockProfile>, mode: u8 },
}

#[kani::proof]
fn theory_bs_boxed_live_profile_btree_dead() {
    let _v = LadderBS::Live {
        profile: Box::new(MockProfile {
            name: String::new(),
            url: String::new(),
            backend: 0,
            color: None,
            ssh_host: None,
            ssh_port: None,
            ssh_user: None,
            ssh_key_env: None,
            ssl_mode: 0,
            ssl_cert_env: None,
            ssl_key_env: None,
            ssl_ca_env: None,
        }),
        mode: 0,
    };
}
