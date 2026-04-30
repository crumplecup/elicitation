//! Compositional depth-bounded construction for Kani proofs.
//!
//! Types that contain collection fields (`Vec<T>`, `Option<T>`, `BTreeMap`,
//! `HashMap`) or recursive fields (`Vec<Self>`) cause CBMC to generate
//! infinite destructor models when constructed symbolically via
//! `kani::any::<T>()`.  `KaniCompose` breaks this by providing concrete
//! bounded instances at each depth level.
//!
//! # Depth semantics
//!
//! | Depth | Meaning |
//! |-------|---------|
//! | 0     | Base case: all collections empty / `None`. Proves single-node soundness. |
//! | 1     | Inductive step: collections have one element populated with `kani_depth0()`. |
//! | 2     | Inductive step: collections have two elements populated with `kani_depth0()`. |
//!
//! Composing depths 0 → 1 → 2 proves soundness for any finite collection
//! length: the inductive step holds, so by induction any size is covered.
//!
//! # Usage in the derive pipeline
//!
//! `#[derive(VerifiedStateMachine)]` generates three Kani harnesses per
//! `(transition × state_variant)` pair — one per depth.  State variant field
//! expressions use `<T as KaniCompose>::kani_depth{n}()` instead of
//! `kani::any::<T>()` for all non-primitive types.
//!
//! # Implementing for custom types
//!
//! Use `#[derive(KaniCompose)]` from `elicitation_derive` for structs.
//! The derive inspects field types and generates depth-0/1/2 methods
//! automatically.  For recursive structs (e.g. `children: Vec<Self>`),
//! depth-1 populates children with one depth-0 instance; depth-2 uses two.

#[cfg(kani)]
pub trait KaniCompose: Sized {
    /// Depth-0: base case.
    ///
    /// All `Vec<T>` fields are empty, all `Option<T>` fields are `None`,
    /// all `BTreeMap`/`HashMap` fields are empty.  Scalar fields use
    /// `kani::any::<ScalarType>()`.
    fn kani_depth0() -> Self;

    /// Depth-1: inductive step.
    ///
    /// Each `Vec<T>` field contains one element (via `T::kani_depth0()`),
    /// each `Option<T>` field is `Some(T::kani_depth0())`.
    ///
    /// The default delegates to `kani_depth0()`, which is correct for
    /// types without collection fields.
    fn kani_depth1() -> Self {
        Self::kani_depth0()
    }

    /// Depth-2: second inductive step.
    ///
    /// Each `Vec<T>` field contains two elements (both via `T::kani_depth0()`).
    ///
    /// The default delegates to `kani_depth1()`.
    fn kani_depth2() -> Self {
        Self::kani_depth1()
    }
}

// ── Primitive impls ───────────────────────────────────────────────────────────

macro_rules! impl_kani_compose_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(kani)]
            impl KaniCompose for $t {
                fn kani_depth0() -> Self { kani::any::<Self>() }
            }
        )*
    };
}

impl_kani_compose_primitive!(
    bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize, char,
);

// ── Standard library impls ────────────────────────────────────────────────────

/// `String` is bounded by using an empty string at all depths.
///
/// Symbolic strings (`kani::any::<String>()`) create unbounded byte arrays
/// in CBMC, causing path explosion.  String content does not affect
/// structural invariant preservation in VSM transitions.
#[cfg(kani)]
impl KaniCompose for String {
    fn kani_depth0() -> Self {
        String::new()
    }
}

/// `Vec<T>`: depth-0 is empty; each depth adds one more `T::kani_depth0()` element.
#[cfg(kani)]
impl<T: KaniCompose> KaniCompose for Vec<T> {
    fn kani_depth0() -> Self {
        Vec::new()
    }

    fn kani_depth1() -> Self {
        vec![T::kani_depth0()]
    }

    fn kani_depth2() -> Self {
        vec![T::kani_depth0(), T::kani_depth0()]
    }
}

/// `[T; N]`: each element is constructed with the appropriate depth method.
///
/// `std::array::from_fn` (stable since 1.63) calls the closure `N` times without
/// requiring `T: Copy`, so only `KaniCompose` is needed.  Fixed-size arrays are
/// used in game-state structs (e.g. `[Hand; MAX_PLAYER_HANDS]`) and other
/// phase structs where the length is a compile-time constant.
#[cfg(kani)]
impl<T: KaniCompose, const N: usize> KaniCompose for [T; N] {
    fn kani_depth0() -> Self {
        std::array::from_fn(|_| T::kani_depth0())
    }

    fn kani_depth1() -> Self {
        std::array::from_fn(|_| T::kani_depth1())
    }

    fn kani_depth2() -> Self {
        std::array::from_fn(|_| T::kani_depth2())
    }
}

/// `Option<T>`: depth-0 is `None`; depth-1/2 are `Some(T::kani_depth0())`.
#[cfg(kani)]
impl<T: KaniCompose> KaniCompose for Option<T> {
    fn kani_depth0() -> Self {
        None
    }

    fn kani_depth1() -> Self {
        Some(T::kani_depth0())
    }
}

/// `BTreeMap<K, V>`: empty at all depths.
///
/// Non-empty BTreeMaps require concrete keys and values, making depth-1/2
/// harnesses for maps identical to depth-0 in the absence of user input.
/// The structure of the map does not affect VSM invariant preservation.
#[cfg(kani)]
impl<K, V> KaniCompose for std::collections::BTreeMap<K, V>
where
    K: KaniCompose + Ord,
    V: KaniCompose,
{
    fn kani_depth0() -> Self {
        std::collections::BTreeMap::new()
    }
}

/// `HashMap<K, V>`: empty at all depths.
///
/// Uses `HashMap::with_hasher(S::default())` to avoid `RandomState::new()` →
/// `getrandom` syscall that CBMC cannot model.
///
/// # Kani limitation
///
/// The default `S = RandomState` reads the system clock in `RandomState::new()`,
/// which CBMC cannot model.  Domain types should use `BTreeMap` for ordered
/// maps.  If `HashMap` is required, declare it with
/// `BuildHasherDefault<DefaultHasher>` as the hasher type.
#[cfg(kani)]
impl<K, V, S> KaniCompose for std::collections::HashMap<K, V, S>
where
    K: KaniCompose + Eq + std::hash::Hash,
    V: KaniCompose,
    S: Default + std::hash::BuildHasher,
{
    fn kani_depth0() -> Self {
        // Use new() not default() to avoid RandomState::new() → getrandom.
        std::collections::HashMap::with_hasher(S::default())
    }
}

/// Tuple (A, B): both elements use their respective `kani_depth{n}()`.
#[cfg(kani)]
impl<A: KaniCompose, B: KaniCompose> KaniCompose for (A, B) {
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0())
    }

    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1())
    }

    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2())
    }
}

/// Tuple (A, B, C).
#[cfg(kani)]
impl<A: KaniCompose, B: KaniCompose, C: KaniCompose> KaniCompose for (A, B, C) {
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0(), C::kani_depth0())
    }

    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1(), C::kani_depth1())
    }

    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2(), C::kani_depth2())
    }
}

/// Tuple (A, B, C, D).
#[cfg(kani)]
impl<A: KaniCompose, B: KaniCompose, C: KaniCompose, D: KaniCompose> KaniCompose for (A, B, C, D) {
    fn kani_depth0() -> Self {
        (
            A::kani_depth0(),
            B::kani_depth0(),
            C::kani_depth0(),
            D::kani_depth0(),
        )
    }

    fn kani_depth1() -> Self {
        (
            A::kani_depth1(),
            B::kani_depth1(),
            C::kani_depth1(),
            D::kani_depth1(),
        )
    }

    fn kani_depth2() -> Self {
        (
            A::kani_depth2(),
            B::kani_depth2(),
            C::kani_depth2(),
            D::kani_depth2(),
        )
    }
}

// ── chrono impls ──────────────────────────────────────────────────────────────

/// `Box<T>`: transparently delegates to `T`'s depth methods.
///
/// Boxing a large struct reduces an enum variant's union footprint to a single
/// pointer, which prevents the CBMC SAT formula explosion that occurs when a
/// complex live-arm type co-exists with a BTree-bearing dead-arm variant.
/// See `KANI_FOR_VSMS.md` for the root-cause analysis.
#[cfg(kani)]
impl<T: KaniCompose> KaniCompose for Box<T> {
    fn kani_depth0() -> Self {
        Box::new(T::kani_depth0())
    }

    fn kani_depth1() -> Self {
        Box::new(T::kani_depth1())
    }

    fn kani_depth2() -> Self {
        Box::new(T::kani_depth2())
    }
}

/// `DateTime<Utc>`: use the Unix epoch as a bounded stand-in at all depths.
#[cfg(all(kani, feature = "chrono"))]
impl KaniCompose for chrono::DateTime<chrono::Utc> {
    fn kani_depth0() -> Self {
        chrono::DateTime::from_timestamp(0, 0).unwrap_or_default()
    }
}
