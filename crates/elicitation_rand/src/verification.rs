//! Kani verification for rand integration.
//!
//! Verifies that our wrapper types correctly store configuration and call
//! rand functions. We trust rand's implementations (castle on cloud) and
//! only verify our wrapper logic.

#[cfg(all(kani, feature = "verification"))]
pub mod kani_proofs;

#[cfg(all(kani, feature = "verification"))]
pub mod runner;
