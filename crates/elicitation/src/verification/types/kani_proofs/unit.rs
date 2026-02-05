//! Kani proofs for unit type ().

// Unit Type Proofs
// ============================================================================

#[kani::proof]
fn verify_unit_type_unique_value() {
    // Unit type has exactly one value
    let unit1: () = ();
    let unit2: () = ();

    assert!(unit1 == unit2, "All unit values are equal");
}

#[kani::proof]
fn verify_unit_type_zero_sized() {
    let unit: () = ();

    assert!(std::mem::size_of_val(&unit) == 0, "Unit type has zero size");
}

#[kani::proof]
fn verify_unit_type_default() {
    let unit: () = Default::default();

    assert!(unit == (), "Default::default() returns ()");
}
