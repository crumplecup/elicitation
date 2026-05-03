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

/// C2b: increment preserves the positivity invariant.
///
/// Verifies: `count@ > 0 ==> count@ + 1 > 0` is expressible and provable.

#[requires(c2_positive(&c))]
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
