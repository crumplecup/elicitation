//! Elicitation-specific adapters for the constitutional `amenable` traits.

use crate::amenable::{
    Amenable, AsObjective, AsStandard, CreusotVerifier, Establish, Evidence, KaniVerifier,
    MetadataEntry, ProofToken, Provenance, RustStdType, Standard, StateMachine, VerusVerifier,
    WitnessSource,
};
use crate::{Established, Is, Prop, ProvableFrom, VerifiedStateMachine};

const RUST_STD_AUDIT_SURFACE: &[&str] = &[
    "Rust standard library source and documentation",
    "Rust compiler type checker",
    "Rust literal and binding semantics",
];

impl<P: Prop> WitnessSource<KaniVerifier> for P {
    type ProofArtifact = proc_macro2::TokenStream;

    fn proof() -> Self::ProofArtifact {
        P::kani_proof()
    }
}

impl<P: Prop> WitnessSource<CreusotVerifier> for P {
    type ProofArtifact = proc_macro2::TokenStream;

    fn proof() -> Self::ProofArtifact {
        P::creusot_proof()
    }
}

impl<P: Prop> WitnessSource<VerusVerifier> for P {
    type ProofArtifact = proc_macro2::TokenStream;

    fn proof() -> Self::ProofArtifact {
        P::verus_proof()
    }
}

impl<P: Prop> Prop for AsStandard<P> {
    fn kani_proof() -> proc_macro2::TokenStream {
        P::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        P::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        P::creusot_proof()
    }

    fn creusot_invariant_fn_name() -> &'static str {
        P::creusot_invariant_fn_name()
    }

    fn kani_invariant_fn_name() -> &'static str {
        P::kani_invariant_fn_name()
    }

    fn verus_invariant_fn_name() -> &'static str {
        P::verus_invariant_fn_name()
    }
}

impl<P: Prop> Prop for AsObjective<P> {
    fn kani_proof() -> proc_macro2::TokenStream {
        P::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        P::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        P::creusot_proof()
    }

    fn creusot_invariant_fn_name() -> &'static str {
        P::creusot_invariant_fn_name()
    }

    fn kani_invariant_fn_name() -> &'static str {
        P::kani_invariant_fn_name()
    }

    fn verus_invariant_fn_name() -> &'static str {
        P::verus_invariant_fn_name()
    }
}

impl<P> ProofToken for Established<P>
where
    P: Evidence + Prop,
{
    type Proposition = P;
}

impl<P, C> Establish<C> for P
where
    P: Evidence + Prop + ProvableFrom<C>,
{
    type Token = Established<Self>;

    #[track_caller]
    fn establish(credential: &C) -> Self::Token {
        Established::prove(credential)
    }
}

impl<T: VerifiedStateMachine> StateMachine for T {
    type State = T::State;
    type Invariant = T::Invariant;
}

impl<T: VerifiedStateMachine> Amenable for T {
    type ProofSurface = proc_macro2::TokenStream;

    fn kani_surface() -> Self::ProofSurface {
        T::vsm_kani_proof()
    }

    fn creusot_surface() -> Self::ProofSurface {
        T::vsm_creusot_proof()
    }

    fn verus_surface() -> Self::ProofSurface {
        T::vsm_verus_proof()
    }

    fn audit_surface() -> &'static [&'static str] {
        &[]
    }
}

impl<T> Evidence for AsStandard<Is<T>>
where
    T: RustStdType,
{
    type WitnessType = Self;

    fn lineage_summary() -> &'static str {
        "Rooted in the Rust standard library definition of the inhabited type."
    }

    fn audit_surface() -> &'static [&'static str] {
        RUST_STD_AUDIT_SURFACE
    }
}

impl<T> Provenance for AsStandard<Is<T>>
where
    T: RustStdType,
{
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("source_family", "rust_std"),
            MetadataEntry::new("authority", "Rust Project Developers"),
            MetadataEntry::new("source_crate", T::rust_source_crate()),
            MetadataEntry::new("source_module", T::rust_source_module()),
            MetadataEntry::new("source_type", std::any::type_name::<T>()),
            MetadataEntry::new("source_url", T::rust_doc_url()),
            MetadataEntry::new(
                "rustup_toolchain",
                option_env!("ELICITATION_RUSTUP_TOOLCHAIN").unwrap_or("unknown"),
            ),
            MetadataEntry::new(
                "rustc_version",
                option_env!("ELICITATION_RUSTC_VERSION").unwrap_or("unknown"),
            ),
        ]
    }
}

impl<T> Standard for AsStandard<Is<T>>
where
    T: RustStdType,
{
    fn authorizing_body() -> &'static str {
        "Rust Project Developers"
    }

    fn authoritative_source() -> &'static str {
        T::rust_doc_url()
    }

    fn source_scope() -> &'static str {
        std::any::type_name::<T>()
    }

    fn normative_summary() -> &'static str {
        T::rust_semantics_summary()
    }

    fn fidelity_rationale() -> &'static str {
        "This root treats the Rust standard-library definition of the carrier type as the trusted authority for what it means for a value to inhabit that type."
    }
}

macro_rules! impl_rust_std_primitive {
    ($ty:ty, $module:expr, $url:expr, $summary:expr) => {
        impl RustStdType for $ty {
            fn rust_source_crate() -> &'static str {
                "core"
            }

            fn rust_source_module() -> &'static str {
                $module
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

impl_rust_std_primitive!(
    bool,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.bool.html",
    "The boolean carrier admits exactly the truth values false and true."
);
impl_rust_std_primitive!(
    char,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.char.html",
    "The character carrier stores a Unicode scalar value."
);
impl_rust_std_primitive!(
    i8,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i8.html",
    "The signed 8-bit integer carrier stores values in the i8 range defined by Rust."
);
impl_rust_std_primitive!(
    i16,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i16.html",
    "The signed 16-bit integer carrier stores values in the i16 range defined by Rust."
);
impl_rust_std_primitive!(
    i32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i32.html",
    "The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
);
impl_rust_std_primitive!(
    i64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i64.html",
    "The signed 64-bit integer carrier stores values in the i64 range defined by Rust."
);
impl_rust_std_primitive!(
    i128,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i128.html",
    "The signed 128-bit integer carrier stores values in the i128 range defined by Rust."
);
impl_rust_std_primitive!(
    isize,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.isize.html",
    "The pointer-sized signed integer carrier stores values in the isize range defined by Rust."
);
impl_rust_std_primitive!(
    u8,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u8.html",
    "The unsigned 8-bit integer carrier stores values in the u8 range defined by Rust."
);
impl_rust_std_primitive!(
    u16,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u16.html",
    "The unsigned 16-bit integer carrier stores values in the u16 range defined by Rust."
);
impl_rust_std_primitive!(
    u32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u32.html",
    "The unsigned 32-bit integer carrier stores values in the u32 range defined by Rust."
);
impl_rust_std_primitive!(
    u64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u64.html",
    "The unsigned 64-bit integer carrier stores values in the u64 range defined by Rust."
);
impl_rust_std_primitive!(
    u128,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u128.html",
    "The unsigned 128-bit integer carrier stores values in the u128 range defined by Rust."
);
impl_rust_std_primitive!(
    usize,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.usize.html",
    "The pointer-sized unsigned integer carrier stores values in the usize range defined by Rust."
);
impl_rust_std_primitive!(
    f32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.f32.html",
    "The 32-bit floating-point carrier follows Rust's f32 semantics."
);
impl_rust_std_primitive!(
    f64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.f64.html",
    "The 64-bit floating-point carrier follows Rust's f64 semantics."
);

impl RustStdType for String {
    fn rust_source_crate() -> &'static str {
        "alloc"
    }

    fn rust_source_module() -> &'static str {
        "alloc::string"
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/string/struct.String.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The String carrier stores owned UTF-8 text as defined by Rust's standard library."
    }
}
