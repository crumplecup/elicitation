//! Gallery level C2: integer bounds invariant.
//!
//! **Hypothesis**: Pearlite's integer model (`@` operator) supports arithmetic
//! comparisons in `#[logic]` predicates and contract annotations.  This is the
//! foundation for any numeric invariant (e.g., non-negative counters, bounded
//! indices, port ranges).
//!
//! ## Experiment table
//!
//! | ID   | Predicate                  | Expected |
//! |------|----------------------------|----------|
//! | C2a  | `count@ > 0`               | ✓        |
//! | C2b  | Preserves positivity       | ✓        |
//! | C2c  | Bounded range `0..=255`    | ✓        |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// A counter that must stay positive.

pub struct Counter {
    count: i64,
}

/// C2a: integer must be strictly positive.

#[logic]
pub fn c2_positive(c: &Counter) -> bool {
    pearlite! { c.count@ > 0 }
}

/// C2a′: counter is below `i64::MAX` (guards overflow in `c2_increment`).

#[logic]
pub fn c2_below_max(c: &Counter) -> bool {
    pearlite! { c.count@ < 9223372036854775807i64@ }
}

/// C2b: increment preserves the positivity invariant.
///
/// Verifies: `count@ > 0 ==> count@ + 1 > 0` is expressible and provable.
///
/// The `c2_below_max` precondition guards against `i64` overflow — Creusot
/// generates an overflow verification condition for `+=` on machine integers.
/// Without bounding the top, the SMT solver cannot rule out `i64::MAX + 1`,
/// so the overflow goal would be unproved despite the arithmetic being trivial.

#[requires(c2_positive(&c))]
#[requires(c2_below_max(&c))]
#[ensures(c2_positive(&result))]
pub fn c2_increment(mut c: Counter) -> Counter {
    c.count += 1;
    c
}

/// C2c: bounded range predicate.
///
/// Verifies: multi-condition arithmetic bounds in Pearlite.

#[logic]
pub fn c2_in_byte_range(c: &Counter) -> bool {
    pearlite! { c.count@ >= 0 && c.count@ <= 255 }
}

/// Clamp to byte range — verifies `c2_in_byte_range` on output.

#[requires(true)]
#[ensures(c2_in_byte_range(&result))]
pub fn c2_clamp_to_byte(mut c: Counter) -> Counter {
    if c.count < 0 {
        c.count = 0;
    } else if c.count > 255 {
        c.count = 255;
    }
    c
}
