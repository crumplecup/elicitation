//! Error types for `elicit_server::gaap`.

use derive_more::{Display, Error};

/// Specific error conditions for GAAP accounting operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum GaapErrorKind {
    /// Journal entry debits do not equal credits.
    #[display("Unbalanced journal entry: debit={}, credit={}", debit, credit)]
    UnbalancedEntry {
        /// Total debit amount in smallest currency unit.
        debit: i64,
        /// Total credit amount in smallest currency unit.
        credit: i64,
    },

    /// Trial balance does not balance across all accounts.
    #[display(
        "Trial balance out of balance: total_debit={}, total_credit={}",
        total_debit,
        total_credit
    )]
    TrialBalanceOutOfBalance {
        /// Sum of all debit balances.
        total_debit: i64,
        /// Sum of all credit balances.
        total_credit: i64,
    },

    /// The accounting equation (Assets = Liabilities + Equity) does not hold.
    #[display(
        "Accounting equation violated: assets={}, liabilities_plus_equity={}",
        assets,
        liabilities_plus_equity
    )]
    AccountingEquationViolated {
        /// Total asset balance.
        assets: i64,
        /// Sum of liabilities and equity.
        liabilities_plus_equity: i64,
    },

    /// A monetary amount is invalid (e.g., negative when positive is required).
    #[display("Invalid amount: {}", _0)]
    InvalidAmount(String),

    /// An account identifier could not be resolved.
    #[display("Account not found: {}", _0)]
    AccountNotFound(String),

    /// The specified accounting period is invalid or inconsistent.
    #[display("Invalid period: {}", _0)]
    InvalidPeriod(String),

    /// Transaction cut-off was violated (transaction recorded in wrong period).
    #[display("Cut-off violation: {}", _0)]
    CutoffViolation(String),

    /// ASC 606 contract criteria are not satisfied.
    #[display("Contract criteria not met: {}", _0)]
    ContractCriteriaNotMet(String),

    /// A performance obligation could not be identified or is not distinct.
    #[display("Performance obligation invalid: {}", _0)]
    PerformanceObligationInvalid(String),

    /// Transaction price could not be determined.
    #[display("Transaction price indeterminate: {}", _0)]
    TransactionPriceIndeterminate(String),

    /// Standalone selling price could not be determined for allocation.
    #[display("SSP unavailable: {}", _0)]
    StandaloneSellingPriceUnavailable(String),

    /// Revenue cannot be recognized (recognition criteria not met).
    #[display("Revenue recognition criteria not met: {}", _0)]
    RevenueNotRecognizable(String),

    /// A financial instrument classification is invalid or inconsistent.
    #[display("Invalid classification: {}", _0)]
    InvalidClassification(String),

    /// An asset carrying value fails the lower-of-cost-or-NRV or impairment test.
    #[display(
        "Impairment required: carrying={}, recoverable={}",
        carrying,
        recoverable
    )]
    ImpairmentRequired {
        /// Current carrying amount in smallest currency unit.
        carrying: i64,
        /// Recoverable amount or NRV.
        recoverable: i64,
    },

    /// A deferred tax computation is inconsistent.
    #[display("Deferred tax error: {}", _0)]
    DeferredTaxError(String),

    /// A fair-value measurement is missing required inputs or hierarchy classification.
    #[display("Fair value measurement error: {}", _0)]
    FairValueMeasurementError(String),

    /// A lease cannot be classified or recognized due to missing data.
    #[display("Lease error: {}", _0)]
    LeaseError(String),

    /// A derivative instrument lacks required designation documentation.
    #[display("Derivative designation error: {}", _0)]
    DerivativeDesignationError(String),

    /// An ICFR assessment or control test failed.
    #[display("ICFR failure: {}", _0)]
    IcfrFailure(String),

    /// A required footnote disclosure is missing or incomplete.
    #[display("Missing disclosure: {}", _0)]
    MissingDisclosure(String),

    /// A financial statement presentation requirement is not satisfied.
    #[display("Presentation error: {}", _0)]
    PresentationError(String),

    /// Operation not supported by this backend.
    #[display("Unsupported operation: {}", _0)]
    Unsupported(String),
}

/// GAAP accounting operation error with source location.
#[derive(Debug, Clone, Display, Error)]
#[display("{} at {}:{}", kind, file, line)]
pub struct GaapError {
    /// Specific error condition.
    pub kind: GaapErrorKind,
    /// Line number where the error was created.
    pub line: u32,
    /// File where the error was created.
    pub file: &'static str,
}

impl GaapError {
    /// Create a new [`GaapError`] capturing the call-site location.
    #[track_caller]
    pub fn new(kind: GaapErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Result type for GAAP accounting operations.
pub type GaapResult<T> = Result<T, GaapError>;
