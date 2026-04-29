//! Diagnostic micro-harnesses for isolating Kani failures.
//!
//! These are NOT part of the VSM suite — they exist to confirm or deny
//! specific theories about which types cause unbounded unwinding.

#[cfg(kani)]
use elicit_server::archive::{display::*, types::*, vsm::*};

/// Theory A: BTreeMap<String, (f32,f32,f32,f32)> drop causes unbounded unwinding.
#[cfg(kani)]
#[kani::proof]
fn diag_btreemap_drop() {
    let m: std::collections::BTreeMap<String, (f32, f32, f32, f32)> =
        std::collections::BTreeMap::new();
    let _ = m;
}

/// Theory B: kani::any::<Option<ErdLayout>>() hangs.
#[cfg(kani)]
#[kani::proof]
fn diag_option_erd_layout() {
    let _layout: Option<ErdLayout> = kani::any();
}

/// Theory C: kani::any::<ErdDiagramMode>() hangs.
#[cfg(kani)]
#[kani::proof]
fn diag_erd_diagram_mode() {
    let _mode: ErdDiagramMode = kani::any();
}

/// Theory E: Is kani::any::<f32>() itself slow?
#[cfg(kani)]
#[kani::proof]
fn diag_symbolic_f32() {
    let _x: f32 = kani::any();
}

/// Theory F: ErdLayout with concrete floats (no symbolic f32).
#[cfg(kani)]
#[kani::proof]
fn diag_erd_layout_concrete_floats() {
    let _layout = ErdLayout {
        canvas_w: 0.0_f32,
        canvas_h: 0.0_f32,
        boxes: std::collections::BTreeMap::new(),
    };
}

/// Theory G: ErdLayout with ONE symbolic f32.
#[cfg(kani)]
#[kani::proof]
fn diag_erd_layout_one_symbolic_f32() {
    let _layout = ErdLayout {
        canvas_w: kani::any::<f32>(),
        canvas_h: 0.0_f32,
        boxes: std::collections::BTreeMap::new(),
    };
}

/// Theory H: Option<ErdLayout> with manually constructed inner (isolates kani::Arbitrary for Option<T>).
#[cfg(kani)]
#[kani::proof]
fn diag_option_erd_layout_manual() {
    let inner = ErdLayout {
        canvas_w: kani::any::<f32>(),
        canvas_h: kani::any::<f32>(),
        boxes: std::collections::BTreeMap::new(),
    };
    let _layout: Option<ErdLayout> = if kani::any::<bool>() { Some(inner) } else { None };
}

/// Theory I: Option<f32> via kani::any() — does kani::Arbitrary for Option<primitive> hang?
#[cfg(kani)]
#[kani::proof]
fn diag_option_f32_arbitrary() {
    let _x: Option<f32> = kani::any();
}


/// Theory D: fully concrete erd_ready (None layout, concrete diagram, symbolic mode).
#[cfg(kani)]
#[kani::proof]
fn diag_erd_ready_concrete() {
    let state = ArchivePanelState::ColumnDetail;
    let proof = elicitation::contracts::Established::<ArchivePanelConsistent>::assert();
    let diagram = ErdDiagram { schema: String::new(), nodes: vec![], edges: vec![] };
    let layout: Option<ErdLayout> = None;
    let mode: ErdDiagramMode = kani::any();
    let _ = erd_ready(state, proof, String::new(), diagram, layout, mode);
}
