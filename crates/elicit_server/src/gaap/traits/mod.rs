//! GAAP trait interface — three-role taxonomy for accounting backends.
//!
//! # Three-Role Taxonomy
//!
//! The GAAP trait interface partitions accounting operations into three
//! orthogonal roles, mirroring the design used in `elicit_gis` and `elicit_ui`:
//!
//! | Role | Description | Return type | Example traits |
//! |------|-------------|-------------|----------------|
//! | **1a** (leaf factory) | Takes a raw descriptor; asserts that the operation satisfies a specific GAAP invariant. | `GaapResult<(Descriptor, Established<P>)>` | `GaapBookkeeping`, `GaapRevenueFactory`, `GaapAssetFactory` |
//! | **1b** (section factory) | Takes an evidence bundle of upstream `Established<P>` tokens; mints an aggregate proof. Enforces sequential proof composition at the type level. | `GaapResult<(Descriptor, Established<P>)>` | `GaapPresentationFactory` |
//! | **2** (reporter) | Queries backend state; no proof tokens consumed or produced. Independent of validity assertions. | `BoxFuture<'_, GaapResult<T>>` | `GaapLedgerMeta`, `GaapRevenueMeta`, `GaapDisclosureMeta` |
//!
//! # `GaapBackend` supertrait
//!
//! [`GaapBackend`] is the aggregate supertrait that a crate implementing the
//! full GAAP surface must satisfy.  It has a blanket impl: any type that
//! implements all 18 sub-traits automatically implements `GaapBackend`.
//!
//! ```ignore
//! // Example: a toy in-memory backend
//! impl GaapBookkeeping for MyBackend { ... }
//! impl GaapLedgerMeta   for MyBackend { ... }
//! // ... all 18 sub-trait impls ...
//!
//! // GaapBackend is automatically satisfied
//! fn requires_backend(b: &dyn GaapBackend) { ... }
//! ```
//!
//! # Proof composition example (ASC 606)
//!
//! ```ignore
//! // Step 1: identify the contract — no precondition
//! let (contract, step1) = backend.identify_contract(raw_contract)?;
//!
//! // Step 2: identify obligations — requires Step 1 token
//! let (obligations, step2) =
//!     backend.identify_performance_obligations(step1, candidate_obligations)?;
//!
//! // Steps 3–4 via evidence bundle
//! let evidence = Asc606Steps1To3Evidence { contract_identified: step1_clone, ... };
//! let (allocation, step4) = backend.allocate_transaction_price(evidence, alloc)?;
//!
//! // Step 5 — recognize revenue; type system forbids calling this without step4
//! let ot_evidence = Asc606OverTimeEvidence { steps_1_to_4: step4, ... };
//! let (recognition, step5) = backend.recognize_revenue_over_time(ot_evidence, rec)?;
//! ```

mod assets;
mod bookkeeping;
mod complex;
mod disclosure;
mod equity;
mod icfr;
mod liabilities;
mod period;
mod presentation;
mod revenue;
mod tax;

pub use assets::{GaapAssetFactory, GaapAssetMeta};
pub use bookkeeping::{GaapBookkeeping, GaapLedgerMeta};
pub use complex::{GaapDerivativeFactory, GaapFairValueFactory, GaapLeaseFactory};
pub use disclosure::{GaapDisclosureFactory, GaapDisclosureMeta};
pub use equity::GaapEquityFactory;
pub use icfr::{GaapIcfrFactory, GaapIcfrMeta};
pub use liabilities::GaapLiabilityFactory;
pub use period::{GaapPeriodFactory, GaapPeriodReporter};
pub use presentation::{
    BalanceSheetEvidence, CashFlowEvidence, FullFinancialStatementsEvidence,
    GaapPresentationFactory, IncomeStatementEvidence,
};
pub use revenue::{GaapRevenueFactory, GaapRevenueMeta};
pub use tax::GaapTaxFactory;

// ── GaapBackend supertrait ────────────────────────────────────────────────────

/// Aggregate supertrait for a full GAAP accounting backend.
///
/// A blanket impl is provided: any type that satisfies all 18 sub-traits
/// (10 factory traits + 7 reporter traits + 1 presentation section factory)
/// automatically implements `GaapBackend`.
///
/// Multiple crates can implement `GaapBackend` against their own data stores —
/// `elicit_db` backends, in-memory backends for testing, stub backends for
/// property-based verification.  The proof token design means every backend
/// must satisfy the same compositional contract at compile time.
///
/// Source: FASB Accounting Standards Codification (all topics).
pub trait GaapBackend:
    GaapBookkeeping
    + GaapLedgerMeta
    + GaapPeriodFactory
    + GaapPeriodReporter
    + GaapRevenueFactory
    + GaapRevenueMeta
    + GaapAssetFactory
    + GaapAssetMeta
    + GaapLiabilityFactory
    + GaapEquityFactory
    + GaapTaxFactory
    + GaapFairValueFactory
    + GaapLeaseFactory
    + GaapDerivativeFactory
    + GaapDisclosureFactory
    + GaapDisclosureMeta
    + GaapIcfrFactory
    + GaapIcfrMeta
    + GaapPresentationFactory
    + Send
    + Sync
{
}

impl<T> GaapBackend for T where
    T: GaapBookkeeping
        + GaapLedgerMeta
        + GaapPeriodFactory
        + GaapPeriodReporter
        + GaapRevenueFactory
        + GaapRevenueMeta
        + GaapAssetFactory
        + GaapAssetMeta
        + GaapLiabilityFactory
        + GaapEquityFactory
        + GaapTaxFactory
        + GaapFairValueFactory
        + GaapLeaseFactory
        + GaapDerivativeFactory
        + GaapDisclosureFactory
        + GaapDisclosureMeta
        + GaapIcfrFactory
        + GaapIcfrMeta
        + GaapPresentationFactory
        + Send
        + Sync
{
}
