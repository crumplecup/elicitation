//! GAAP accounting types module.
//!
//! All descriptor types are `Debug + Clone + PartialEq + Serialize + Deserialize + JsonSchema`.
//! They are the raw material consumed by the GAAP trait factories.

mod assets;
mod common;
mod complex;
mod disclosure;
mod equity;
mod icfr;
mod journal;
mod liabilities;
mod period;
mod revenue;
mod statements;
mod tax;

pub use assets::{
    CostFlowMethod, DepreciationMethod, InventoryDescriptor, PpeDescriptor, ReceivableDescriptor,
    SecurityClassification, SecurityDescriptor,
};
pub use common::{
    AccountDescriptor, AccountId, AccountType, CurrencyCode, EntityId, MonetaryAmount,
    NormalBalance, PeriodDate,
};
pub use complex::{
    DerivativeDescriptor, FairValueDescriptor, FairValueLevel, HedgeDesignation,
    LeaseClassification, LeaseDescriptor,
};
pub use disclosure::{DisclosureRequirement, FootnoteDescriptor};
pub use equity::{
    EquityDescriptor, OciDescriptor, OciItem, TreasuryStockDescriptor, TreasuryStockMethod,
};
pub use icfr::{ControlTestDescriptor, IcfrDescriptor, ManagementAssertionDescriptor};
pub use journal::{
    BalanceSheetTotals, CreditEntry, DebitEntry, JournalEntryDescriptor, TrialBalanceDescriptor,
};
pub use liabilities::{
    ContingencyDescriptor, ContingencyProbability, DebtDescriptor, LiabilityDescriptor,
};
pub use period::{AccrualDescriptor, DeferralDescriptor, FinancialPeriod, PeriodType};
pub use revenue::{
    AllocationDescriptor, ContractStatus, PerformanceObligationDescriptor,
    RevenueContractDescriptor, RevenueRecognitionDescriptor, TransactionPriceDescriptor,
};
pub use statements::{
    BalanceSheetDescriptor, CashFlowDescriptor, CashFlowMethod, EpsDescriptor,
    IncomeStatementDescriptor,
};
pub use tax::{DeferredTaxDescriptor, TaxRateDescriptor, TaxReconciliationItem};
