//! Equity descriptor types — stockholders' equity, OCI, treasury stock.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount};

// ── Stockholders' equity ──────────────────────────────────────────────────────

/// Descriptor for a class of stock or a specific equity account.
///
/// The factory asserts `StockholdersEquityPresented` when all required equity
/// components are present.
///
/// Source: ASC 505 — Equity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EquityDescriptor {
    /// Ledger account for this equity component.
    pub account: AccountId,
    /// Par value per share, if applicable.
    pub par_value: Option<MonetaryAmount>,
    /// Shares authorized, if applicable.
    pub shares_authorized: Option<u64>,
    /// Shares issued.
    pub shares_issued: Option<u64>,
    /// Carrying amount of this equity component.
    pub amount: MonetaryAmount,
    /// Free-text description (e.g. `"Common Stock, $0.001 par"`).
    pub description: String,
}

// ── Other comprehensive income ────────────────────────────────────────────────

/// A single OCI item (one reclassification or unrealized adjustment).
///
/// Source: ASC 220 — Comprehensive Income.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OciItem {
    /// Description of the OCI component (e.g. `"AFS unrealized gain, net of tax"`).
    pub description: String,
    /// Amount of the OCI item (positive = income, negative = loss).
    pub amount: MonetaryAmount,
    /// Whether this is a reclassification out of AOCI into net income.
    pub is_reclassification: bool,
}

/// Descriptor for accumulated other comprehensive income/(loss) (AOCI).
///
/// The factory asserts `OciAccumulatedSeparately` when AOCI is carried as a
/// separate component of stockholders' equity.
///
/// Source: ASC 220-10-45 — Classification in Other Comprehensive Income.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct OciDescriptor {
    /// Ledger account for AOCI.
    pub account: AccountId,
    /// Individual OCI items for the current period.
    pub period_items: Vec<OciItem>,
    /// Ending accumulated OCI balance.
    pub accumulated_oci: MonetaryAmount,
}

// ── Treasury stock ────────────────────────────────────────────────────────────

/// Method used to record treasury stock repurchases.
///
/// Source: ASC 505-30 — Treasury Stock.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum TreasuryStockMethod {
    /// Treasury shares carried at total cost; most common U.S. practice.
    CostMethod,
    /// Treasury shares carried at par value; difference charged to APIC.
    ParValueMethod,
}

/// Descriptor for treasury stock repurchases.
///
/// The factory asserts `TreasuryStockAccountedFor` when the repurchase is
/// recorded as a reduction of equity at the appropriate cost.
///
/// Source: ASC 505-30 — Treasury Stock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TreasuryStockDescriptor {
    /// Ledger account for treasury stock (contra-equity).
    pub account: AccountId,
    /// Number of shares repurchased.
    pub shares_repurchased: u64,
    /// Total cost of the repurchased shares.
    pub cost: MonetaryAmount,
    /// Measurement method.
    pub method: TreasuryStockMethod,
}
