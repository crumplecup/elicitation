//! Unit tests for `elicitation::contracts` primitives.
//!
//! Extracted from the former inline `mod tests` block in `contracts.rs`.
//!
//! Note: tests requiring `impl Implies<Is<X>> for Is<Y>` are excluded here
//! because the orphan rule prevents implementing foreign-crate generics in
//! external test files.  Those impls were compile-time API-shape checks; the
//! reflexive cases below sufficiently exercise the runtime behaviour.

use elicitation::contracts::{
    And, Established, Implies, InVariant, Is, Prop, both, downcast, fst, snd,
};

#[test]
fn test_established_is_zero_sized() {
    let proof: Established<Is<String>> = Established::assert();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_established_is_copy() {
    let proof: Established<Is<String>> = Established::assert();
    let proof2 = proof;
    let _proof3 = proof;
    let _proof4 = proof2;
}

#[test]
fn test_can_construct_proof() {
    let _proof: Established<Is<String>> = Established::assert();
    let _proof: Established<Is<i32>> = Established::assert();
    let _proof: Established<Is<Vec<u8>>> = Established::assert();
}

#[test]
fn test_proof_requires_type() {
    fn requires_string_proof(_proof: Established<Is<String>>) {}
    let proof: Established<Is<String>> = Established::assert();
    requires_string_proof(proof);
}

#[test]
fn test_implies_reflexive() {
    let proof: Established<Is<String>> = Established::assert();
    let same_proof: Established<Is<String>> = proof.weaken();
    let _ = same_proof;
}

#[test]
fn test_weaken_with_custom_impl() {
    struct StrongProp;
    struct WeakProp;

    impl Prop for StrongProp {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for WeakProp {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Implies<WeakProp> for StrongProp {}

    let strong: Established<StrongProp> = Established::assert();
    let _weak: Established<WeakProp> = strong.weaken();
}

#[test]
fn test_conjunction_combine() {
    struct P;
    struct Q;
    impl Prop for P {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for Q {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }

    let p: Established<P> = Established::assert();
    let q: Established<Q> = Established::assert();
    let _pq: Established<And<P, Q>> = both(p, q);
}

#[test]
fn test_conjunction_project_left() {
    struct P;
    struct Q;
    impl Prop for P {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for Q {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }

    let pq: Established<And<P, Q>> = both(Established::assert(), Established::assert());
    let _p: Established<P> = fst(pq);
}

#[test]
fn test_conjunction_project_right() {
    struct P;
    struct Q;
    impl Prop for P {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for Q {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }

    let pq: Established<And<P, Q>> = both(Established::assert(), Established::assert());
    let _q: Established<Q> = snd(pq);
}

#[test]
fn test_conjunction_is_zero_sized() {
    struct P;
    struct Q;
    impl Prop for P {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for Q {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }

    let pq: Established<And<P, Q>> = both(Established::assert(), Established::assert());
    assert_eq!(std::mem::size_of_val(&pq), 0);
}

#[test]
fn test_conjunction_chain() {
    struct P;
    struct Q;
    struct R;
    impl Prop for P {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for Q {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }
    impl Prop for R {
        fn kani_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn verus_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
        fn creusot_proof() -> proc_macro2::TokenStream { proc_macro2::TokenStream::new() }
    }

    let pq = both(Established::<P>::assert(), Established::<Q>::assert());
    let _pqr: Established<And<And<P, Q>, R>> = both(pq, Established::<R>::assert());
}

// Reflexive downcast (no orphan-violating impl needed — the crate provides
// `impl<T> Implies<Is<T>> for Is<T>` via reflexivity).
#[test]
fn test_refinement_reflexive() {
    let proof: Established<Is<String>> = Established::assert();
    let _same: Established<Is<String>> = downcast(proof);
}

#[test]
fn test_refinement_zero_sized() {
    let refined: Established<Is<String>> = Established::assert();
    assert_eq!(std::mem::size_of_val(&refined), 0);
    let base: Established<Is<String>> = downcast(refined);
    assert_eq!(std::mem::size_of_val(&base), 0);
}

#[test]
fn test_invariant_zero_sized() {
    enum Status { _Active, _Inactive }
    struct ActiveVariant;

    let proof: Established<InVariant<Status, ActiveVariant>> = Established::assert();
    assert_eq!(std::mem::size_of_val(&proof), 0);
}

#[test]
fn test_invariant_type_safety() {
    enum Status { _Active, _Inactive }
    struct ActiveVariant;

    fn process_active(_status: Status, _proof: Established<InVariant<Status, ActiveVariant>>) {}

    let proof: Established<InVariant<Status, ActiveVariant>> = Established::assert();
    process_active(Status::_Active, proof);
}

#[test]
fn test_invariant_enum_branches() {
    enum State { _Loading, _Ready, _Error }
    struct LoadingVariant;
    struct ReadyVariant;
    struct ErrorVariant;

    fn handle_loading(_proof: Established<InVariant<State, LoadingVariant>>) {}
    fn handle_ready(_proof: Established<InVariant<State, ReadyVariant>>) {}
    fn handle_error(_proof: Established<InVariant<State, ErrorVariant>>) {}

    handle_loading(Established::assert());
    handle_ready(Established::assert());
    handle_error(Established::assert());
}

#[test]
fn test_invariant_with_inhabitation() {
    enum Color { _Red, _Green, _Blue }
    struct RedVariant;

    let _type_proof: Established<Is<Color>> = Established::assert();
    let _variant_proof: Established<InVariant<Color, RedVariant>> = Established::assert();
}
