//! GAAP-compliant proof markers for financial operations.
//!
//! This module defines propositions (zero-cost proof tokens) covering the
//! Generally Accepted Accounting Principles, organized by FASB ASC topic.
//!
//! # Structure
//!
//! - `principles` — 15 core GAAP principles and assumptions (ASC 105 + related)
//! - `asc_200` — ASC 205–280: presentation, income statement, cash flows, EPS, segment, interim
//! - `asc_300` — ASC 310–360: receivables, investments, inventory, intangibles, PP&E
//! - `asc_400` — ASC 405–480: liabilities, contingencies, debt, equity classification
//! - `asc_500` — ASC 505: stockholders' equity, treasury stock, OCI, dividends
//! - `asc_606` — ASC 606: revenue from contracts (full 5-step model)
//! - `asc_700` — ASC 718/740: stock compensation and income taxes
//! - `asc_800` — ASC 805/810/815/820/825/830/835/842: business combinations, derivatives, FV, leases
//! - `disclosure` — All §50 footnote and supplemental disclosure requirements
//! - `internal_controls` — SOX 302/404, ICFR, audit trail, segregation of duties
//! - `mathematical` — Accounting equation invariants and rollforward identities
//! - `temporal` — Period cut-off, accrual/deferral timing, and commencement date props
//!
//! # References
//!
//! - FASB Accounting Standards Codification: <https://asc.fasb.org/>

mod asc_200;
mod asc_300;
mod asc_400;
mod asc_500;
mod asc_606;
mod asc_700;
mod asc_800;
mod disclosure;
pub mod errors;
mod internal_controls;
mod mathematical;
mod principles;
mod proof_composition;
mod temporal;
pub mod traits;
pub mod types;

pub use asc_200::{
    AccountingChangeJustified, BalanceSheetClassified, BasicEpsDeclared, CashFlowMethodDisclosed,
    CashFlowStatementPresented, ComparativePeriodPresented, ComprehensiveIncomeReported,
    CurrentAssetClassification, CurrentLiabilityClassification, DilutedEpsDeclared,
    DiscontinuedOperationsSeparated, EpsWeightedAverageSharesCorrect, ErrorCorrectionRestated,
    FinancialStatementsComplete, FinancingActivitiesClassified, GoingConcernEvaluated,
    IncomeFromContinuingOperationsDisclosed, InterimPeriodIntegral, InterimTaxRateAnnualized,
    InvestingActivitiesClassified, ManagementApproachApplied, NonCashActivitiesDisclosed,
    OciPresentedSeparately, OffsettingProhibited, OperatingActivitiesClassified,
    ProspectiveApplicationApplied, RetrospectiveApplicationApplied, SeasonalRevenueDisclosed,
    SegmentIdentificationComplete, SegmentReconcilesTotal, UnusualItemsInContinuingOperations,
};
pub use asc_300::{
    AfsSecurityAtFairValue, AllowanceForCreditLossEstimated, CostFlowAssumptionDisclosed,
    DebtSecurityClassified, DepreciationMethodDisclosed, DisposalGainLossRecognized,
    EquityMethodApplied, EquityMethodImpairmentAssessed, EquityMethodInvesteeIdentified,
    EquitySecurityAtFairValue, FactoredReceivableSaleAccountedFor, FiniteLifeIntangibleAmortized,
    GoodwillAnnuallyTested, GoodwillImpairmentRecognized, HtmSecurityAtAmortizedCost,
    IndefiniteLifeIntangibleTested, IntangibleUsefulLifeReassessed,
    InternalUseSoftwareCostCapitalized, InventoryAtLowerOfCostOrNrv, InventoryWriteDownRecognized,
    InvestmentImpairmentReviewed, LifoReserveDisclosed, LoanOriginationFeesDeferred,
    LongLivedAssetImpairmentTested, PpeAroRecognized, PpeCarriedAtCost,
    ReceivableRecordedAtAmortizedCost, TradingSecurityAtFairValue,
    TroubledDebtRestructuringIdentified, UsefulLifeEstimated,
};
pub use asc_400::{
    AroAccretionExpenseRecognized, AssetRetirementObligationRecognized,
    ContingentLiabilityReassessed, DebtClassifiedCorrectly, DebtCovenantComplianceDisclosed,
    DebtIssuanceCostsDeferred, DebtorTroubledDebtRestructuringAccounted,
    DeferredRevenueRecordedUntilEarned, EffectiveInterestMethodUsed,
    EnvironmentalLiabilityRecognized, ExitCostRecognizedWhenLiabilityIncurred,
    FreestandingInstrumentClassified, GainContingencyNotRecorded, GuaranteeDisclosed,
    GuaranteeObligationRecognized, LiabilityRecognitionCriteriaMet, LossContingencyAssessed,
    MandatorilyRedeemableInstrumentInLiabilities, MandatoryRedemptionDisclosed,
    ProbableLossAccrued, ReasonablyPossibleLossDisclosed, SeveranceLiabilityMeasured,
    ShortTermDebtRefinancingClassified, TradeAccountsPayableAccrued,
};
pub use asc_500::{
    CommonStockParValueDisclosed, DividendsDeclaredRecorded, NoncontrollingInterestPresented,
    OciAccumulatedSeparately, PreferredStockTermsDisclosed, RetainedEarningsAppropriationDisclosed,
    RetainedEarningsReconciled, StockSplitAccountedFor, StockSubscriptionReceivableAsContraEquity,
    StockholdersEquityPresented, TreasuryStockAccountedFor,
};
pub use asc_606::{
    AllocationAdjustedForModification, CollectibilityProbable,
    ConsiderationPayableToCustomerDeducted, ContractBalanceRecordedCorrectly,
    ContractBalancesDisclosed, ContractCombinationAssessed, ContractCriteriaMet,
    ContractIdentified, ContractModificationAccountedFor, DistinctGoodOrServiceDetermined,
    NonCashConsiderationMeasured, OverTimeCriteriaMet, PerformanceObligationsIdentified,
    PrincipalVsAgentDetermined, PriorPeriodPerformanceObligationRevenueDisclosed,
    ProgressMeasurementMethodSelected, RemainingPerformanceObligationsDisclosed,
    ResidualApproachApplied, RevenueDisaggregated, RevenueRecognitionPolicyDisclosed,
    RevenueRecognizedAtPointInTime, RevenueRecognizedOverTime,
    SeriesPerformanceObligationAccountedFor, SignificantFinancingComponentAssessed,
    StandaloneSellingPriceDetermined, TransactionPriceAllocated, TransactionPriceDetermined,
    VariableConsiderationConstraintApplied, VariableConsiderationEstimated,
};
pub use asc_700::{
    AwardModificationAccountedFor, DeferredTaxAssetRecognized, DeferredTaxLiabilityRecognized,
    DeferredTaxNoncurrentClassified, EffectiveTaxRateDisclosed, ForfeitureAccountingApplied,
    GradedVestingApplied, IntraperiodTaxAllocationApplied, MarketConditionIncludedInFairValue,
    MoreLikelyThanNotStandardApplied, PerformanceConditionAssessed, StockAwardTaxEffectInEarnings,
    StockCompensationFairValueMeasured, StockCompensationRecognizedOverVesting,
    UncertainTaxPositionEvaluated, ValuationAllowanceAssessed,
};
pub use asc_800::{
    AcquisitionDateIdentified, AcquisitionMethodApplied, CashFlowHedgeAccountedFor,
    ConsolidationCriteriaEvaluated, ContingentConsiderationMeasured,
    DerivativeRecognizedAtFairValue, EffectiveInterestMethodApplied, FairValueExitPriceApplied,
    FairValueHedgeAccountedFor, FairValueHierarchyApplied, FairValueOnNonrecurringBasis,
    FairValueOnRecurringBasis, FairValueOptionElectionDocumented,
    FinancialInstrumentFairValueDisclosed, ForeignCurrencyTransactionGainLossRecognized,
    FunctionalCurrencyDetermined, GoodwillOrBargainPurchaseRecognized, HedgeDesignationDocumented,
    HedgeEffectivenessAssessed, IdentifiableAssetsRecognized, InterestCapitalized, LeaseClassified,
    LeaseDiscountRateDetermined, LeaseIdentified, LeaseLiabilityRecognized, LeaseTermDetermined,
    Level3InputsDisclosed, MeasurementPeriodAdjustmentsRecorded, NetInvestmentHedgeApplied,
    NoncontrollingInterestRecognized, RemeasurementApplied, RouAssetRecognized,
    ShortTermLeaseExemptionApplied, TranslationAdjustmentInOci, VariableLeasePmtAccountedFor,
    VieConsolidationAssessed, VotingInterestModelApplied,
};
pub use disclosure::{
    CommitmentsAndContingenciesDisclosed, ConcentrationRisksDisclosed, ContractBalanceNote,
    DebtCovenantsDisclosed, DebtMaturityScheduleDisclosed, DeferredTaxComponentsDisclosed,
    DerivativeAndHedgingDisclosed, DerivativeFairValueTableDisclosed,
    EffectiveTaxRateReconciliationDisclosed, EntityWideDisclosuresIncluded,
    FairValueMeasurementMethodsDisclosed, GoodwillRollforwardDisclosed, IntangibleAssetsDisclosed,
    InterimSignificantChangesDisclosed, LeaseLiabilityMaturityDisclosed,
    LeaseQuantitativeDisclosed, Level3FairValueRollforwardDisclosed, LiquidityRisksDisclosed,
    NetPeriodicBenefitCostDisclosed, NewAccountingStandardsDisclosed, PensionObligationDisclosed,
    PpeDepreciationPolicyDisclosed, PreferredStockDisclosures, RelatedPartyTransactionsDisclosed,
    RemainingPerformanceObligationNote, RevenueDisaggregationNote, RevenueRecognitionPolicyNote,
    SegmentInformationDisclosed, SignificantAccountingPoliciesDisclosed,
    StockCompensationPlanDisclosed, SubsequentEventsDisclosed, TaxJurisdictionsDisclosed,
    UncertainTaxBenefitsDisclosed, UnrecognizedCompensationCostDisclosed,
};
pub use internal_controls::{
    AccessControlsImplemented, AuditTrailComplete, AuditTrailTamperEvident,
    AuditorIcfrOpinionIssued, ChangeManagementControlsApplied, DisclosureControlsEffective,
    IcfrDesignAdequate, IcfrOperatingEffective, JournalEntryReviewCompleted,
    ManagementIcfrAssessmentCompleted, MaterialWeaknessIdentified, PeriodEndCloseControlsApplied,
    PrivilegedAccessMonitored, ReconciliationPerformed, RecordsRetentionCompliant,
    SegregationOfDutiesEnforced, SignificantDeficiencyEvaluated, TransactionAuthorizationRequired,
};
pub use mathematical::{
    AccountingEquationHolds, AllowanceForCreditLossRollforward, AmortizationScheduleCorrect,
    CashFlowReconciles, DebitEqualsCreditPerEntry, DeferredTaxNetPresentable,
    DepreciationAccumulatesCorrectly, DilutedEpsNoMoreThanBasic, EpsDenominatorCorrect,
    EpsNumeratorCorrect, GoodwillRollforward, InventoryRollforward, LeaseLiabilityPvCorrect,
    NetIncomeAggregation, OciRollforward, ReceivablesRollforward, RetainedEarningsRollforward,
    SegmentRevenueSumsToConsolidated, TaxRateReconciles, TrialBalanceBalances,
};
pub use principles::{
    AccrualBasis, ConservatismPrinciple, ConsistencyPrinciple, DoubleEntryBookkeeping,
    EconomicEntityAssumption, FullDisclosurePrinciple, GoingConcernAssumption,
    HistoricalCostPrinciple, MatchingPrinciple, MaterialityPrinciple, MonetaryUnitAssumption,
    NeutralityPrinciple, RevenueRecognitionPrinciple, SubstanceOverFormPrinciple,
    TimePeriodAssumption,
};
pub use proof_composition::{
    AccountingEquationEvidence, Asc606OverTimeEvidence, Asc606PointInTimeEvidence,
    Asc606Steps1To3Evidence, ContractIdentificationEvidence, PerformanceObligationsEvidence,
    RetainedEarningsEvidence, TransactionPriceEvidence, TrialBalanceEvidence,
};
pub use temporal::{
    AccrualRecordedAtPeriodEnd, DeferralReleasedInEarnedPeriod, DepreciationComputedForFullPeriod,
    DividendsDeclaredInCorrectPeriod, ExpenseMatchedToPeriod, FxTranslationAtAverageRate,
    FxTranslationAtClosingRate, InterestAccruedThroughPeriodEnd, InterimAccrualMethodConsistent,
    LeaseCommencementDateCorrect, RevenueEarnedInPeriod, RevenueTransferDateCorrect,
    StockOptionGrantDateCorrect, SubsequentEventDateBound, TaxPeriodAligned,
    TemporaryDifferenceTimingCorrect, TransactionCutoffRespected,
};

// ── Error, type, and trait re-exports ─────────────────────────────────────────

pub use errors::{GaapError, GaapErrorKind, GaapResult};
pub use traits::{
    BalanceSheetEvidence, CashFlowEvidence, FullFinancialStatementsEvidence, GaapAssetFactory,
    GaapAssetMeta, GaapBackend, GaapBookkeeping, GaapDerivativeFactory, GaapDisclosureFactory,
    GaapDisclosureMeta, GaapEquityFactory, GaapFairValueFactory, GaapIcfrFactory, GaapIcfrMeta,
    GaapLeaseFactory, GaapLedgerMeta, GaapLiabilityFactory, GaapPeriodFactory, GaapPeriodReporter,
    GaapPresentationFactory, GaapRevenueFactory, GaapRevenueMeta, GaapTaxFactory,
    IncomeStatementEvidence,
};
pub use types::{
    AccountDescriptor, AccountId, AccountType, AccrualDescriptor, AllocationDescriptor,
    BalanceSheetDescriptor, BalanceSheetTotals, CashFlowDescriptor, CashFlowMethod,
    ContingencyDescriptor, ContingencyProbability, ContractStatus, ControlTestDescriptor,
    CostFlowMethod, CreditEntry, CurrencyCode, DebitEntry, DebtDescriptor, DeferralDescriptor,
    DeferredTaxDescriptor, DepreciationMethod, DerivativeDescriptor, DisclosureRequirement,
    EntityId, EpsDescriptor, EquityDescriptor, FairValueDescriptor, FairValueLevel,
    FinancialPeriod, FootnoteDescriptor, HedgeDesignation, IcfrDescriptor,
    IncomeStatementDescriptor, InventoryDescriptor, JournalEntryDescriptor, LeaseClassification,
    LeaseDescriptor, LiabilityDescriptor, ManagementAssertionDescriptor, MonetaryAmount,
    NormalBalance, OciDescriptor, OciItem, PerformanceObligationDescriptor, PeriodDate, PeriodType,
    PpeDescriptor, ReceivableDescriptor, RevenueContractDescriptor, RevenueRecognitionDescriptor,
    SecurityClassification, SecurityDescriptor, TaxRateDescriptor, TaxReconciliationItem,
    TransactionPriceDescriptor, TreasuryStockDescriptor, TreasuryStockMethod,
};
