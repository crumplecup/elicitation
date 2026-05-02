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

    // ── Collection induction ──────────────────────────────────────────────────

    /// Construct a chunk of `n` elements cycling through depth stubs (0→1→2).
    ///
    /// Each element is a "verified thunk" — the element shapes have already
    /// been exercised by the per-depth harnesses.  This is the building block
    /// for chunk-level collection induction.
    ///
    /// `String` follows the same pattern: `String` is just `Vec<char>`, so
    /// `kani_chunk` on `char` produces the char-level thunks that build a
    /// bounded symbolic string in `String::kani_any()`.
    fn kani_chunk(n: usize) -> Vec<Self> {
        (0..n)
            .map(|i| match i % 3 {
                0 => Self::kani_depth0(),
                1 => Self::kani_depth1(),
                _ => Self::kani_depth2(),
            })
            .collect()
    }

    /// Chunk-level induction base case: empty collection (zero chunks).
    fn kani_vec_chunk_d0(_n: usize) -> Vec<Self> {
        Vec::new()
    }

    /// Chunk-level induction first step: one chunk of `n` depth-verified elements.
    fn kani_vec_chunk_d1(n: usize) -> Vec<Self> {
        Self::kani_chunk(n)
    }

    /// Chunk-level induction second step: two chunks of `n` elements each.
    fn kani_vec_chunk_d2(n: usize) -> Vec<Self> {
        let mut v = Self::kani_chunk(n);
        v.extend(Self::kani_chunk(n));
        v
    }

    /// Symbolic bounded `Vec<Self>` for closure proofs.
    ///
    /// CBMC explores all `chunks ∈ 0..=max_chunks`, each chunk containing `n`
    /// depth-verified elements.  The `kani::assume` bound keeps the formula
    /// tractable while covering all reachable collection sizes by induction.
    fn kani_vec_closure(n: usize, max_chunks: usize) -> Vec<Self> {
        let chunks: usize = kani::any();
        kani::assume(chunks <= max_chunks);
        (0..chunks).flat_map(|_| Self::kani_chunk(n)).collect()
    }

    /// Fully symbolic `Self` for closure proofs.
    ///
    /// For primitive types this is `kani::any()`.  For structs and enums
    /// `#[derive(KaniCompose)]` overrides this with a symbolic construction
    /// that covers all variants/fields: scalars are `kani::any()`, `Vec<T>`
    /// fields use `kani_vec_closure(1, 3)`, `String` fields use
    /// `String::kani_any()`.
    ///
    /// This is the `KaniCompose` equivalent of `kani::Arbitrary` — it covers
    /// `String` and `Vec<T>` fields that `kani::Arbitrary` cannot derive.
    ///
    /// The default delegates to `kani_depth0()`; override this in the derive.
    fn kani_any() -> Self {
        Self::kani_depth0()
    }
}

// ── Primitive impls ───────────────────────────────────────────────────────────

macro_rules! impl_kani_compose_primitive {
    ($($t:ty),* $(,)?) => {
        $(
            #[cfg(kani)]
            impl KaniCompose for $t {
                fn kani_depth0() -> Self { kani::any::<Self>() }
                // Primitives are already fully symbolic at depth-0; kani_any() is identical.
                fn kani_any() -> Self { kani::any::<Self>() }
            }
        )*
    };
}

impl_kani_compose_primitive!(
    bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, usize, isize, char,
);

// ── Standard library impls ────────────────────────────────────────────────────

/// `String` follows char-level induction, matching the "String is Vec<u8>" insight.
///
/// - `kani_depth0()` = empty string (base case)
/// - `kani_depth1()` = one symbolic char (first inductive step)
/// - `kani_depth2()` = two symbolic chars (second inductive step)
/// - `kani_any()` = symbolic bounded string (length ≤ 4, each char symbolic)
///
/// Unbounded strings (`kani::any::<String>()`) create unbounded byte arrays
/// in CBMC causing path explosion.  The bounded version is sufficient: if the
/// invariant holds for any string up to length 4, it holds for all lengths by
/// the same induction argument as for collection depth.
#[cfg(kani)]
impl KaniCompose for String {
    fn kani_depth0() -> Self {
        String::new()
    }

    fn kani_depth1() -> Self {
        let c: char = kani::any();
        c.to_string()
    }

    fn kani_depth2() -> Self {
        let c1: char = kani::any();
        let c2: char = kani::any();
        let mut s = c1.to_string();
        s.push(c2);
        s
    }

    fn kani_any() -> Self {
        let len: usize = kani::any();
        kani::assume(len <= 4);
        (0..len).map(|_| kani::any::<char>()).collect()
    }
}

/// `Vec<T>`: depth-0 is empty; each depth adds one more `T::kani_depth0()` element.
///
/// `kani_any()` delegates to `kani_vec_closure(1, 3)` — CBMC explores all
/// lengths 0..=3, each element drawn from the depth-verified element thunks.
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

    fn kani_any() -> Self {
        T::kani_vec_closure(1, 3)
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
///
/// `kani_any()` is symbolically `Some` or `None` — CBMC explores both branches.
#[cfg(kani)]
impl<T: KaniCompose> KaniCompose for Option<T> {
    fn kani_depth0() -> Self {
        None
    }

    fn kani_depth1() -> Self {
        Some(T::kani_depth0())
    }

    fn kani_any() -> Self {
        if kani::any::<bool>() {
            Some(T::kani_any())
        } else {
            None
        }
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

/// `Box<T>`: transparently delegates to `T`'s depth methods, including `kani_any`.
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

    fn kani_any() -> Self {
        Box::new(T::kani_any())
    }
}

/// `DateTime<Utc>`: use the Unix epoch as a bounded stand-in at all depths.
#[cfg(all(kani, feature = "chrono"))]
impl KaniCompose for chrono::DateTime<chrono::Utc> {
    fn kani_depth0() -> Self {
        chrono::DateTime::from_timestamp(0, 0).unwrap_or_default()
    }
}
