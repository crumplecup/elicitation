//! Gallery level 9: Find the variant-count scaling threshold.
//!
//! Level 8e showed a 2-variant enum (Nothing + MonitorSnapshot) completes in ~1.4s.
//! ArchivePanelState has 18 variants and times out (>1 hour).
//! This level escalates variant count to find where CBMC blows up.
//!
//! All variants carry MonitorSnapshot (the heaviest real field type proven in 8e)
//! so the per-variant cost is constant and we isolate COUNT as the variable.
//!
//! ### 9a — 4 variants with MonitorSnapshot
//! ### 9b — 8 variants with MonitorSnapshot
//! ### 9c — 12 variants with MonitorSnapshot
//! ### 9d — 18 variants with MonitorSnapshot (mirrors ArchivePanelState count)

use elicitation::KaniCompose;
use elicit_server::archive::types::MonitorSnapshot;

// ── 9a: 4 variants ───────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D9Enum4 {
    V0,
    V1(MonitorSnapshot),
    V2(MonitorSnapshot),
    V3(MonitorSnapshot),
}

#[cfg(kani)]
#[kani::proof]
fn gallery9a_4variant_any() {
    let e = D9Enum4::kani_any();
    std::mem::forget(e);
}

// ── 9b: 8 variants ───────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D9Enum8 {
    V0,
    V1(MonitorSnapshot),
    V2(MonitorSnapshot),
    V3(MonitorSnapshot),
    V4(MonitorSnapshot),
    V5(MonitorSnapshot),
    V6(MonitorSnapshot),
    V7(MonitorSnapshot),
}

#[cfg(kani)]
#[kani::proof]
fn gallery9b_8variant_any() {
    let e = D9Enum8::kani_any();
    std::mem::forget(e);
}

// ── 9c: 12 variants ──────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D9Enum12 {
    V0,
    V1(MonitorSnapshot),
    V2(MonitorSnapshot),
    V3(MonitorSnapshot),
    V4(MonitorSnapshot),
    V5(MonitorSnapshot),
    V6(MonitorSnapshot),
    V7(MonitorSnapshot),
    V8(MonitorSnapshot),
    V9(MonitorSnapshot),
    V10(MonitorSnapshot),
    V11(MonitorSnapshot),
}

#[cfg(kani)]
#[kani::proof]
fn gallery9c_12variant_any() {
    let e = D9Enum12::kani_any();
    std::mem::forget(e);
}

// ── 9d: 18 variants (matches ArchivePanelState count) ────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D9Enum18 {
    V0,
    V1(MonitorSnapshot),
    V2(MonitorSnapshot),
    V3(MonitorSnapshot),
    V4(MonitorSnapshot),
    V5(MonitorSnapshot),
    V6(MonitorSnapshot),
    V7(MonitorSnapshot),
    V8(MonitorSnapshot),
    V9(MonitorSnapshot),
    V10(MonitorSnapshot),
    V11(MonitorSnapshot),
    V12(MonitorSnapshot),
    V13(MonitorSnapshot),
    V14(MonitorSnapshot),
    V15(MonitorSnapshot),
    V16(MonitorSnapshot),
    V17(MonitorSnapshot),
}

#[cfg(kani)]
#[kani::proof]
fn gallery9d_18variant_any() {
    let e = D9Enum18::kani_any();
    std::mem::forget(e);
}
