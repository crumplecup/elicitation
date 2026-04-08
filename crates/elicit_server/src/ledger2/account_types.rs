//! GAAP account type system - the foundational IR for financial accounting.
//!
//! This module defines the five fundamental GAAP account classes and their
//! hierarchical sub-types. Account types are **primitive** - they encode the
//! chart of accounts structure at the type level.
//!
//! # GAAP Account Equation
//!
//! ```text
//! Assets = Liabilities + Equity
//! ```
//!
//! # Normal Balances
//!
//! - **Debit:** Assets, Expenses (increases with debit, decreases with credit)
//! - **Credit:** Liabilities, Equity, Revenue (increases with credit, decreases with debit)
//!
//! # Account Hierarchy
//!
//! ```text
//! Asset
//!   ├── CurrentAsset (cash, receivables, inventory, prepaid)
//!   ├── FixedAsset (land, building, equipment, vehicle)
//!   └── IntangibleAsset (goodwill, patents, trademarks)
//!
//! Liability
//!   ├── CurrentLiability (payables, accrued expenses, short-term debt)
//!   └── LongTermLiability (bonds, notes, leases)
//!
//! Equity
//!   ├── CapitalStock (common, preferred)
//!   ├── RetainedEarnings
//!   ├── Dividends
//!   └── TreasuryStock
//!
//! Revenue
//!   ├── Sales
//!   ├── ServiceRevenue
//!   └── OtherIncome (interest, gains)
//!
//! Expense
//!   ├── CostOfGoodsSold
//!   ├── OperatingExpense (salaries, rent, utilities)
//!   ├── Depreciation
//!   ├── InterestExpense
//!   └── TaxExpense
//! ```

use std::fmt;

use derive_more::Display;
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────
//  Core Types
// ─────────────────────────────────────────────────────────────

/// Normal balance for an account (debit or credit).
///
/// Determines how increases/decreases affect the account:
/// - **Debit normal:** Increases with debit, decreases with credit
/// - **Credit normal:** Increases with credit, decreases with debit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum NormalBalance {
    /// Normal balance is debit (Assets, Expenses).
    #[display("Debit")]
    Debit,

    /// Normal balance is credit (Liabilities, Equity, Revenue).
    #[display("Credit")]
    Credit,
}

/// Debit or credit side of a journal entry line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum DebitCredit {
    /// Debit side (left).
    #[display("Debit")]
    Debit,

    /// Credit side (right).
    #[display("Credit")]
    Credit,
}

// ─────────────────────────────────────────────────────────────
//  Account Class - Five Fundamental GAAP Types
// ─────────────────────────────────────────────────────────────

/// The five fundamental GAAP account classes.
///
/// Every account in a chart of accounts belongs to one of these classes.
/// The class determines the account's normal balance and where it appears
/// in financial statements.
///
/// # GAAP Account Equation
///
/// ```text
/// Assets = Liabilities + Equity
/// Revenue - Expenses = Net Income → Retained Earnings (Equity)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountClass {
    /// Asset account (normal balance: debit).
    ///
    /// Resources owned or controlled by the entity with future economic benefit.
    /// Examples: Cash, Inventory, Equipment, Accounts Receivable.
    Asset(AssetType),

    /// Liability account (normal balance: credit).
    ///
    /// Obligations to transfer resources to other entities.
    /// Examples: Accounts Payable, Notes Payable, Accrued Expenses.
    Liability(LiabilityType),

    /// Equity account (normal balance: credit).
    ///
    /// Residual interest in assets after deducting liabilities.
    /// Examples: Common Stock, Retained Earnings, Additional Paid-In Capital.
    Equity(EquityType),

    /// Revenue account (normal balance: credit).
    ///
    /// Increases in economic benefits from ordinary business activities.
    /// Examples: Sales, Service Revenue, Interest Income.
    Revenue(RevenueType),

    /// Expense account (normal balance: debit).
    ///
    /// Decreases in economic benefits from ordinary business activities.
    /// Examples: Cost of Goods Sold, Salaries, Rent, Depreciation.
    Expense(ExpenseType),
}

impl AccountClass {
    /// Returns the normal balance for this account class.
    ///
    /// # GAAP Rules
    ///
    /// - **Debit normal:** Assets, Expenses
    /// - **Credit normal:** Liabilities, Equity, Revenue
    pub fn normal_balance(&self) -> NormalBalance {
        match self {
            AccountClass::Asset(_) | AccountClass::Expense(_) => NormalBalance::Debit,
            AccountClass::Liability(_) | AccountClass::Equity(_) | AccountClass::Revenue(_) => {
                NormalBalance::Credit
            }
        }
    }

    /// Returns true if this is an Asset account.
    pub fn is_asset(&self) -> bool {
        matches!(self, AccountClass::Asset(_))
    }

    /// Returns true if this is a Liability account.
    pub fn is_liability(&self) -> bool {
        matches!(self, AccountClass::Liability(_))
    }

    /// Returns true if this is an Equity account.
    pub fn is_equity(&self) -> bool {
        matches!(self, AccountClass::Equity(_))
    }

    /// Returns true if this is a Revenue account.
    pub fn is_revenue(&self) -> bool {
        matches!(self, AccountClass::Revenue(_))
    }

    /// Returns true if this is an Expense account.
    pub fn is_expense(&self) -> bool {
        matches!(self, AccountClass::Expense(_))
    }

    /// Returns true if this is a temporary account (Revenue, Expense).
    ///
    /// Temporary accounts are closed to retained earnings at period end.
    pub fn is_temporary(&self) -> bool {
        matches!(self, AccountClass::Revenue(_) | AccountClass::Expense(_))
    }

    /// Returns true if this is a permanent account (Asset, Liability, Equity).
    ///
    /// Permanent accounts carry balances across periods.
    pub fn is_permanent(&self) -> bool {
        !self.is_temporary()
    }

    /// Returns the class name (Asset, Liability, Equity, Revenue, Expense).
    pub fn class_name(&self) -> &'static str {
        match self {
            AccountClass::Asset(_) => "Asset",
            AccountClass::Liability(_) => "Liability",
            AccountClass::Equity(_) => "Equity",
            AccountClass::Revenue(_) => "Revenue",
            AccountClass::Expense(_) => "Expense",
        }
    }
}

impl fmt::Display for AccountClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.class_name(), self.type_name())
    }
}

impl AccountClass {
    fn type_name(&self) -> &str {
        match self {
            AccountClass::Asset(t) => t.type_name(),
            AccountClass::Liability(t) => t.type_name(),
            AccountClass::Equity(t) => t.type_name(),
            AccountClass::Revenue(t) => t.type_name(),
            AccountClass::Expense(t) => t.type_name(),
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Asset Types
// ─────────────────────────────────────────────────────────────

/// Asset account types.
///
/// Assets are resources with future economic benefit. Classified by liquidity
/// (how quickly converted to cash) and nature (tangible vs. intangible).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    /// Current asset (convertible to cash within one year).
    CurrentAsset(CurrentAsset),

    /// Fixed asset (long-term tangible asset).
    FixedAsset(FixedAsset),

    /// Intangible asset (long-term non-physical asset).
    IntangibleAsset(IntangibleAsset),
}

impl AssetType {
    fn type_name(&self) -> &str {
        match self {
            AssetType::CurrentAsset(t) => t.type_name(),
            AssetType::FixedAsset(t) => t.type_name(),
            AssetType::IntangibleAsset(t) => t.type_name(),
        }
    }
}

/// Current asset types (cash and near-cash assets).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrentAsset {
    /// Cash and cash equivalents.
    Cash,

    /// Accounts receivable (amounts owed by customers).
    AccountsReceivable,

    /// Inventory (goods for sale).
    Inventory,

    /// Prepaid expenses (expenses paid in advance).
    PrepaidExpense,

    /// Short-term investments (marketable securities).
    ShortTermInvestment,

    /// Other current assets.
    Other(String),
}

impl CurrentAsset {
    fn type_name(&self) -> &str {
        match self {
            CurrentAsset::Cash => "Cash",
            CurrentAsset::AccountsReceivable => "Accounts Receivable",
            CurrentAsset::Inventory => "Inventory",
            CurrentAsset::PrepaidExpense => "Prepaid Expense",
            CurrentAsset::ShortTermInvestment => "Short-Term Investment",
            CurrentAsset::Other(name) => name,
        }
    }
}

/// Fixed asset types (long-term tangible assets).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FixedAsset {
    /// Land (not depreciable).
    Land,

    /// Building (depreciable).
    Building,

    /// Equipment (depreciable).
    Equipment,

    /// Vehicle (depreciable).
    Vehicle,

    /// Furniture and fixtures (depreciable).
    Furniture,

    /// Accumulated depreciation (contra-asset).
    AccumulatedDepreciation,

    /// Other fixed assets.
    Other(String),
}

impl FixedAsset {
    fn type_name(&self) -> &str {
        match self {
            FixedAsset::Land => "Land",
            FixedAsset::Building => "Building",
            FixedAsset::Equipment => "Equipment",
            FixedAsset::Vehicle => "Vehicle",
            FixedAsset::Furniture => "Furniture",
            FixedAsset::AccumulatedDepreciation => "Accumulated Depreciation",
            FixedAsset::Other(name) => name,
        }
    }

    /// Returns true if this asset is depreciable.
    pub fn is_depreciable(&self) -> bool {
        !matches!(self, FixedAsset::Land | FixedAsset::AccumulatedDepreciation)
    }
}

/// Intangible asset types (long-term non-physical assets).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntangibleAsset {
    /// Goodwill (excess of purchase price over fair value).
    Goodwill,

    /// Patents (exclusive rights to inventions).
    Patent,

    /// Trademarks (brand names and logos).
    Trademark,

    /// Copyright (exclusive rights to creative works).
    Copyright,

    /// Software licenses.
    SoftwareLicense,

    /// Other intangible assets.
    Other(String),
}

impl IntangibleAsset {
    fn type_name(&self) -> &str {
        match self {
            IntangibleAsset::Goodwill => "Goodwill",
            IntangibleAsset::Patent => "Patent",
            IntangibleAsset::Trademark => "Trademark",
            IntangibleAsset::Copyright => "Copyright",
            IntangibleAsset::SoftwareLicense => "Software License",
            IntangibleAsset::Other(name) => name,
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Liability Types
// ─────────────────────────────────────────────────────────────

/// Liability account types.
///
/// Liabilities are obligations to transfer economic resources. Classified
/// by maturity (current vs. long-term).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LiabilityType {
    /// Current liability (due within one year).
    CurrentLiability(CurrentLiability),

    /// Long-term liability (due after one year).
    LongTermLiability(LongTermLiability),
}

impl LiabilityType {
    fn type_name(&self) -> &str {
        match self {
            LiabilityType::CurrentLiability(t) => t.type_name(),
            LiabilityType::LongTermLiability(t) => t.type_name(),
        }
    }
}

/// Current liability types (due within one year).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrentLiability {
    /// Accounts payable (amounts owed to suppliers).
    AccountsPayable,

    /// Accrued expenses (expenses incurred but not yet paid).
    AccruedExpense,

    /// Unearned revenue (advance payments from customers).
    UnearnedRevenue,

    /// Short-term notes payable.
    ShortTermNotesPayable,

    /// Current portion of long-term debt.
    CurrentPortionLongTermDebt,

    /// Payroll liabilities (wages, taxes).
    PayrollLiability,

    /// Other current liabilities.
    Other(String),
}

impl CurrentLiability {
    fn type_name(&self) -> &str {
        match self {
            CurrentLiability::AccountsPayable => "Accounts Payable",
            CurrentLiability::AccruedExpense => "Accrued Expense",
            CurrentLiability::UnearnedRevenue => "Unearned Revenue",
            CurrentLiability::ShortTermNotesPayable => "Short-Term Notes Payable",
            CurrentLiability::CurrentPortionLongTermDebt => "Current Portion of Long-Term Debt",
            CurrentLiability::PayrollLiability => "Payroll Liability",
            CurrentLiability::Other(name) => name,
        }
    }
}

/// Long-term liability types (due after one year).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LongTermLiability {
    /// Bonds payable.
    BondsPayable,

    /// Long-term notes payable.
    LongTermNotesPayable,

    /// Mortgage payable.
    MortgagePayable,

    /// Lease liability (ASC 842).
    LeaseLiability,

    /// Deferred tax liability.
    DeferredTaxLiability,

    /// Other long-term liabilities.
    Other(String),
}

impl LongTermLiability {
    fn type_name(&self) -> &str {
        match self {
            LongTermLiability::BondsPayable => "Bonds Payable",
            LongTermLiability::LongTermNotesPayable => "Long-Term Notes Payable",
            LongTermLiability::MortgagePayable => "Mortgage Payable",
            LongTermLiability::LeaseLiability => "Lease Liability",
            LongTermLiability::DeferredTaxLiability => "Deferred Tax Liability",
            LongTermLiability::Other(name) => name,
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Equity Types
// ─────────────────────────────────────────────────────────────

/// Equity account types.
///
/// Equity represents the residual interest in assets after deducting liabilities.
/// For corporations: stock + retained earnings. For partnerships: capital accounts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquityType {
    /// Common stock (basic ownership shares).
    CommonStock,

    /// Preferred stock (priority ownership shares).
    PreferredStock,

    /// Additional paid-in capital (excess over par value).
    AdditionalPaidInCapital,

    /// Retained earnings (accumulated net income).
    RetainedEarnings,

    /// Dividends (distributions to shareholders, temporary account).
    Dividends,

    /// Treasury stock (company's own stock repurchased, contra-equity).
    TreasuryStock,

    /// Partner capital account (for partnerships).
    PartnerCapital(String),

    /// Other equity.
    Other(String),
}

impl EquityType {
    fn type_name(&self) -> &str {
        match self {
            EquityType::CommonStock => "Common Stock",
            EquityType::PreferredStock => "Preferred Stock",
            EquityType::AdditionalPaidInCapital => "Additional Paid-In Capital",
            EquityType::RetainedEarnings => "Retained Earnings",
            EquityType::Dividends => "Dividends",
            EquityType::TreasuryStock => "Treasury Stock",
            EquityType::PartnerCapital(name) => name,
            EquityType::Other(name) => name,
        }
    }

    /// Returns true if this is a contra-equity account (reduces equity).
    pub fn is_contra(&self) -> bool {
        matches!(self, EquityType::Dividends | EquityType::TreasuryStock)
    }
}

// ─────────────────────────────────────────────────────────────
//  Revenue Types
// ─────────────────────────────────────────────────────────────

/// Revenue account types.
///
/// Revenue represents increases in economic benefits from ordinary business
/// activities. Temporary accounts closed to retained earnings at period end.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RevenueType {
    /// Sales revenue (goods sold).
    Sales,

    /// Service revenue (services provided).
    ServiceRevenue,

    /// Interest income (earned on investments).
    InterestIncome,

    /// Dividend income (received from investments).
    DividendIncome,

    /// Rental income (from leasing property).
    RentalIncome,

    /// Gain on sale of assets.
    GainOnSale,

    /// Other revenue.
    Other(String),
}

impl RevenueType {
    fn type_name(&self) -> &str {
        match self {
            RevenueType::Sales => "Sales",
            RevenueType::ServiceRevenue => "Service Revenue",
            RevenueType::InterestIncome => "Interest Income",
            RevenueType::DividendIncome => "Dividend Income",
            RevenueType::RentalIncome => "Rental Income",
            RevenueType::GainOnSale => "Gain on Sale",
            RevenueType::Other(name) => name,
        }
    }
}

// ─────────────────────────────────────────────────────────────
//  Expense Types
// ─────────────────────────────────────────────────────────────

/// Expense account types.
///
/// Expenses represent decreases in economic benefits from ordinary business
/// activities. Temporary accounts closed to retained earnings at period end.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExpenseType {
    /// Cost of goods sold (direct costs of producing goods).
    CostOfGoodsSold,

    /// Salaries and wages.
    Salaries,

    /// Rent expense.
    Rent,

    /// Utilities expense (electricity, water, gas).
    Utilities,

    /// Depreciation expense (fixed asset consumption).
    Depreciation,

    /// Amortization expense (intangible asset consumption).
    Amortization,

    /// Interest expense (cost of borrowed funds).
    InterestExpense,

    /// Tax expense (income taxes).
    TaxExpense,

    /// Insurance expense.
    Insurance,

    /// Advertising expense.
    Advertising,

    /// Supplies expense.
    Supplies,

    /// Repairs and maintenance expense.
    RepairsAndMaintenance,

    /// Loss on sale of assets.
    LossOnSale,

    /// Other operating expense.
    Other(String),
}

impl ExpenseType {
    fn type_name(&self) -> &str {
        match self {
            ExpenseType::CostOfGoodsSold => "Cost of Goods Sold",
            ExpenseType::Salaries => "Salaries",
            ExpenseType::Rent => "Rent",
            ExpenseType::Utilities => "Utilities",
            ExpenseType::Depreciation => "Depreciation",
            ExpenseType::Amortization => "Amortization",
            ExpenseType::InterestExpense => "Interest Expense",
            ExpenseType::TaxExpense => "Tax Expense",
            ExpenseType::Insurance => "Insurance",
            ExpenseType::Advertising => "Advertising",
            ExpenseType::Supplies => "Supplies",
            ExpenseType::RepairsAndMaintenance => "Repairs and Maintenance",
            ExpenseType::LossOnSale => "Loss on Sale",
            ExpenseType::Other(name) => name,
        }
    }

    /// Returns true if this is a cost of goods sold expense (matches with revenue).
    pub fn is_cogs(&self) -> bool {
        matches!(self, ExpenseType::CostOfGoodsSold)
    }

    /// Returns true if this is an operating expense.
    pub fn is_operating(&self) -> bool {
        !matches!(self, ExpenseType::InterestExpense | ExpenseType::TaxExpense)
    }
}
