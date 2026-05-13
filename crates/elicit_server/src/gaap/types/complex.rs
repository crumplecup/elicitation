//! Complex-transaction descriptor types — fair value, leases, derivatives.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount, PeriodDate};

// ── Fair value ────────────────────────────────────────────────────────────────

/// ASC 820 fair value hierarchy level.
///
/// Source: ASC 820-10-35 — Fair Value Hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum FairValueLevel {
    /// Quoted prices in active markets for identical assets/liabilities.
    Level1,
    /// Observable inputs other than Level 1 quoted prices.
    Level2,
    /// Unobservable inputs significant to the measurement.
    Level3,
}

/// Descriptor for a fair value measurement.
///
/// The factory asserts `FairValueExitPriceApplied` when the exit-price
/// principle is applied, and `FairValueHierarchyApplied` when the appropriate
/// level is determined and disclosed.
///
/// Source: ASC 820 — Fair Value Measurement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FairValueDescriptor {
    /// Ledger account for the measured item.
    pub account: AccountId,
    /// Determined fair value (exit price in principal or most advantageous market).
    pub fair_value: MonetaryAmount,
    /// ASC 820 fair value hierarchy level.
    pub hierarchy_level: FairValueLevel,
    /// Valuation technique applied (e.g. `"market approach"`, `"income approach"`).
    pub valuation_technique: String,
    /// Significant inputs to the valuation.
    pub significant_inputs: Vec<String>,
    /// Whether measurement is on a recurring (each reporting date) basis.
    pub is_recurring: bool,
}

// ── Leases ────────────────────────────────────────────────────────────────────

/// ASC 842 lease classification.
///
/// Source: ASC 842-20-25 — Lessee Accounting.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum LeaseClassification {
    /// Finance lease (previously capital lease).
    Finance,
    /// Operating lease.
    Operating,
    /// Short-term lease (≤ 12 months, practical expedient applied).
    ShortTerm,
}

/// Descriptor for an ASC 842 lease.
///
/// The factory asserts `LeaseIdentified`, `LeaseClassified`,
/// `LeaseLiabilityRecognized`, and `RouAssetRecognized` through sequential
/// factory steps.
///
/// Source: ASC 842 — Leases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LeaseDescriptor {
    /// Unique lease reference.
    pub lease_id: String,
    /// Lessee account for the right-of-use asset.
    pub rou_asset_account: AccountId,
    /// Lessee account for the lease liability.
    pub lease_liability_account: AccountId,
    /// Lease classification as determined under ASC 842.
    pub classification: LeaseClassification,
    /// Lease commencement date (ISO 8601).
    pub commencement_date: PeriodDate,
    /// Determined lease term in months (including renewal options reasonably certain).
    pub lease_term_months: u32,
    /// Annual fixed lease payment.
    pub annual_payment: MonetaryAmount,
    /// Incremental borrowing rate (or implicit rate) used to discount payments.
    pub discount_rate: f64,
    /// Present value of remaining lease payments = lease liability at commencement.
    pub lease_liability_pv: MonetaryAmount,
    /// Right-of-use asset at commencement.
    pub rou_asset: MonetaryAmount,
}

// ── Derivatives and hedging ───────────────────────────────────────────────────

/// Hedge accounting designation for a derivative instrument.
///
/// Source: ASC 815-20 — Hedging—General.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum HedgeDesignation {
    /// Fair value hedge: hedges exposure to changes in fair value of a recognized asset/liability.
    FairValueHedge,
    /// Cash flow hedge: hedges exposure to variability in cash flows.
    CashFlowHedge,
    /// Net investment hedge: hedges foreign currency exposure of a net investment in a foreign operation.
    NetInvestmentHedge,
    /// Not designated as a hedge; changes in fair value go to earnings.
    NotDesignated,
}

/// Descriptor for a derivative financial instrument.
///
/// The factory asserts `DerivativeRecognizedAtFairValue` when the derivative
/// is recorded at fair value on the balance sheet, and
/// `HedgeDesignationDocumented` when hedge accounting is elected.
///
/// Source: ASC 815 — Derivatives and Hedging.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DerivativeDescriptor {
    /// Unique instrument identifier.
    pub instrument_id: String,
    /// Ledger account for the derivative (asset if positive fair value, liability if negative).
    pub account: AccountId,
    /// Notional amount.
    pub notional: MonetaryAmount,
    /// Current fair value (positive = asset, negative = liability).
    pub fair_value: MonetaryAmount,
    /// Hedge accounting designation.
    pub hedge_designation: HedgeDesignation,
    /// ISO 8601 designation date, if hedge accounting is elected.
    pub designation_date: Option<PeriodDate>,
}
