//! Formal verification proof harnesses for the elicitation ecosystem.
//!
//! Proof harnesses are generated at build time by `build.rs`.
//! Each verifier's harnesses live in their own feature-gated module:
//!
//! | Feature   | Module     | Verifier                     |
//! |-----------|------------|------------------------------|
//! | `kani`    | [`kani`]   | [Kani model checker]         |
//! | `creusot` | [`creusot`]| [Creusot deductive verifier] |
//! | `verus`   | [`verus`]  | [Verus verifier]             |
//!
//! # Usage
//!
//! ```bash
//! # Kani
//! cargo build -p elicit_proofs --features kani
//! cargo kani -p elicit_proofs --features kani -Z function-contracts --harness disconnect__kani
//!
//! # Creusot (once implemented)
//! cargo build -p elicit_proofs --features creusot
//!
//! # Verus (once implemented)
//! cargo build -p elicit_proofs --features verus
//! ```
//!
//! [Kani model checker]: https://model-checking.github.io/kani/
//! [Creusot deductive verifier]: https://github.com/creusot-rs/creusot
//! [Verus verifier]: https://github.com/verus-lang/verus

#[cfg(kani)]
pub mod kani;

#[cfg(any(feature = "creusot", creusot))]
pub mod creusot;

#[cfg(feature = "verus")]
pub mod verus;

#[cfg(feature = "runner")]
pub mod cli;

#[cfg(feature = "runner")]
pub mod vsm;
