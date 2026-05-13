# GAAP — Compile-Time Financial Statement Verification

Formally verified US GAAP accounting with proof-carrying factories, anchored
to the FASB Accounting Standards Codification (ASC) and foundational
double-entry bookkeeping invariants.

## Overview

The GAAP module models financial statement construction as a
proof-carrying pipeline. Every journal entry, asset measurement, and
financial statement section is built through a factory trait method that
performs a standards-compliance check and, on success, returns both the
constructed descriptor **and** a typed proof token — an `Established<P>` —
that records which accounting contract was satisfied.

Proof tokens compose upward through evidence bundles into aggregate proofs
(`TrialBalanceBalances`, `AccountingEquationHolds`, `FinancialStatementsComplete`),
which are the only legal way to assert compound financial statement validity.

The compiler enforces the chain. There is no way to produce
`Established<FinancialStatementsComplete>` without first assembling the
balance sheet, income statement, and cash-flow proofs from which it derives.

This module is an **interface definition** — not an implementation. Accounting
backends (`elicit_db` drivers, in-memory testing backends, stub backends for
formal verification) implement the traits; consumers depend only on these
contracts.

---

## Architecture

```text
  ┌──────────────────────────────────────────────────────────────────────────┐
  │                 Foundational Layer                                        │
  │  GaapBookkeeping · GaapLedgerMeta                                        │
  │  (double-entry recording; DebitEqualsCreditPerEntry → TrialBalanceBalances)│
  └──────────────────────────────┬───────────────────────────────────────────┘
                                 │
                                 ▼
  ┌──────────────────────────────────────────────────────────────────────────┐
  │              Balance-Sheet Domain                                         │
  │  GaapAssetFactory · GaapAssetMeta                                        │
  │  GaapLiabilityFactory · GaapEquityFactory                                │
  │  GaapPeriodFactory · GaapPeriodReporter                                  │
  └──────────────────────────────┬───────────────────────────────────────────┘
                                 │
                                 ▼
  ┌──────────────────────────────────────────────────────────────────────────┐
  │              Income-Statement Domain                                      │
  │  GaapRevenueFactory (ASC 606 five-step chain)                            │
  │  GaapRevenueMeta                                                          │
  │  GaapTaxFactory · GaapFairValueFactory                                   │
  │  GaapLeaseFactory · GaapDerivativeFactory                                │
  └──────────────────────────────┬───────────────────────────────────────────┘
                                 │
                                 ▼
  ┌──────────────────────────────────────────────────────────────────────────┐
  │              Disclosure & Controls Layer                                  │
  │  GaapDisclosureFactory · GaapDisclosureMeta                              │
  │  GaapIcfrFactory · GaapIcfrMeta                                          │
  └──────────────────────────────┬───────────────────────────────────────────┘
                                 │
                                 ▼
  ┌──────────────────────────────────────────────────────────────────────────┐
  │          Presentation Layer (Role 1b — Section Factories)                 │
  │  GaapPresentationFactory                                                  │
  │  BalanceSheetEvidence → BalanceSheetClassified                           │
  │  IncomeStatementEvidence → ComprehensiveIncomeReported                   │
  │  CashFlowEvidence → CashFlowStatementPresented                           │
  │  FullFinancialStatementsEvidence → FinancialStatementsComplete           │
  └──────────────────────────────┬───────────────────────────────────────────┘
                                 │
               ┌─────────────────┼──────────────────┐
               ▼                 ▼                  ▼
       elicit_db            in-memory           stub/proof
       (SqlxDbBackend)      (TestBackend)       (KaniBackend)
```

### Three-role taxonomy

| Role | Description | Return type | Traits |
|------|-------------|-------------|--------|
| **1a** (leaf factory) | Takes a raw descriptor; asserts one specific GAAP invariant; no prior proof required | `GaapResult<(Descriptor, Established<P>)>` | `GaapBookkeeping`, `GaapRevenueFactory`, `GaapAssetFactory`, `GaapLiabilityFactory`, `GaapEquityFactory`, `GaapTaxFactory`, `GaapFairValueFactory`, `GaapLeaseFactory`, `GaapDerivativeFactory`, `GaapDisclosureFactory`, `GaapIcfrFactory`, `GaapPeriodFactory` |
| **1b** (section factory) | Takes an evidence bundle of upstream proof tokens; enforces sequential composition at the type level; mints aggregate proofs | `GaapResult<(Descriptor, Established<P>)>` | `GaapPresentationFactory` |
| **2** (reporter) | Queries backend state; no proof tokens consumed or produced; orthogonal to validity | `BoxFuture<'_, GaapResult<T>>` | `GaapLedgerMeta`, `GaapRevenueMeta`, `GaapAssetMeta`, `GaapPeriodReporter`, `GaapDisclosureMeta`, `GaapIcfrMeta` |

---

## Proof Architecture

### Proposition types

Every verifiable GAAP contract has a corresponding Rust type — a
*proposition* — that implements `elicitation::contracts::Prop`. These are
zero-cost phantoms that exist only at the type level.

```rust
pub struct DoubleEntryBookkeeping;          // Accrual-basis double entry enforced
pub struct DebitEqualsCreditPerEntry;       // Single journal entry is balanced
pub struct TrialBalanceBalances;            // All entries in aggregate balance
pub struct AccountingEquationHolds;         // Assets = Liabilities + Equity
pub struct ContractIdentified;             // ASC 606-10-25-1 criteria met
pub struct TransactionPriceAllocated;      // ASC 606-10-32-28 allocation done
pub struct RevenueRecognizedOverTime;      // ASC 606-10-25-27 satisfied
pub struct FinancialStatementsComplete;    // ASC 205-10-45-1 full set present
```

### Proof tokens

`Established<P>` is the proof that proposition `P` holds. It is a zero-sized
type that carries no runtime data — only type-level evidence.

### The `ProvableFrom<C>` evidence path

The evidence-bundle minting path is `Established::prove`:

```rust
impl Established<P> {
    pub fn prove<C>(_: &C) -> Self  where P: ProvableFrom<C> { … }
}
```

`ProvableFrom<C>` declares "evidence bundle `C` proves proposition `P`". The
proof_composition module defines the evidence bundles and their `ProvableFrom`
impls. For example:

```rust
pub struct AccountingEquationEvidence {
    pub double_entry:  Established<DoubleEntryBookkeeping>,
    pub trial_balance: Established<TrialBalanceBalances>,
    pub net_income:    Established<NetIncomeAggregation>,
}
impl ProvableFrom<AccountingEquationEvidence> for AccountingEquationHolds {}
```

---

## Proof Composition

### ASC 606 five-step chain

The revenue recognition five-step model is encoded as a sequential type-level
dependency. Each step requires the token from the previous step; the compiler
rejects any out-of-order call.

```rust
// Step 1 — Identify the contract (ASC 606-10-25-1)
// No precondition; just raw data.
let (contract, step1) = backend.identify_contract(raw_contract)?;
// step1: Established<ContractIdentified>

// Step 2 — Identify performance obligations (ASC 606-10-25-14)
// Requires Step 1 token.
let (obligations, step2) =
    backend.identify_performance_obligations(step1.clone(), candidate_obs)?;
// step2: Established<PerformanceObligationsIdentified>

// Step 3 — Determine transaction price (ASC 606-10-32-2)
// Requires Step 1 token (contract context).
let evidence_1 = ContractIdentificationEvidence {
    criteria: step1.clone(),
    collectibility: step1.clone(), // backend asserted both
};
let (price, step3) = backend.determine_transaction_price(evidence_1, price_desc)?;
// step3: Established<TransactionPriceDetermined>

// Step 4 — Allocate transaction price (ASC 606-10-32-28)
// Requires Steps 1–3 + SSP.
let (_, ssp_token) = backend.determine_standalone_selling_price(ssp_desc)?;
let (allocation, step4) = backend.allocate_transaction_price(
    Asc606Steps1To3Evidence {
        contract:    step1,
        obligations: step2,
        price:       step3,
        ssp:         ssp_token,
    },
    alloc_desc,
)?;
// step4: Established<TransactionPriceAllocated>

// Step 5 — Recognize revenue over time (ASC 606-10-25-27)
// Requires Steps 1–4 + over-time criteria + progress method.
let (_, ot_criteria) = backend.assess_over_time_criteria(criteria_desc)?;
let (_, progress_method) = backend.select_progress_method(method_desc)?;
let (recognition, step5) = backend.recognize_revenue_over_time(
    Asc606OverTimeEvidence {
        contract:        step1_clone,
        obligations:     step2_clone,
        price:           step3_clone,
        allocation:      step4,
        over_time_criteria: ot_criteria,
        progress_method,
    },
    recognition_desc,
)?;
// step5: Established<RevenueRecognizedOverTime>
```

### Bookkeeping invariant chain

```rust
// Record balanced journal entries
let (_, entry_proof) = backend.record_journal_entry(entry)?;
// entry_proof: Established<DebitEqualsCreditPerEntry>

// Compile the trial balance (all entries must have balanced)
let (trial_balance, tb_proof) = backend.compile_trial_balance(period)?;
// tb_proof: Established<TrialBalanceBalances>
// — internally: ProvableFrom<TrialBalanceEvidence> for TrialBalanceBalances

// Verify the accounting equation
let ae_proof = Established::prove(&AccountingEquationEvidence {
    double_entry:  de_proof,
    trial_balance: tb_proof,
    net_income:    ni_proof,
});
// ae_proof: Established<AccountingEquationHolds>

// Retained earnings rollforward
let re_proof = Established::prove(&RetainedEarningsEvidence {
    accounting_equation: ae_proof,
    net_income:          ni_proof_clone,
});
// re_proof: Established<RetainedEarningsRollforward>
```

### Financial statement presentation chain

```rust
// Balance sheet requires equation + trial balance + current classifications
let (bs, bs_proof) = backend.present_balance_sheet(
    BalanceSheetEvidence {
        accounting_equation:        ae_proof,
        trial_balance:              tb_proof,
        current_assets_classified:  ca_proof,
        current_liabilities_classified: cl_proof,
        comparative_period:         cp_proof,
    },
    bs_descriptor,
)?;
// bs_proof: Established<BalanceSheetClassified>

// Income statement requires net income + OCI + continuing ops disclosure
let (is, is_proof) = backend.present_income_statement(
    IncomeStatementEvidence {
        net_income:             ni_proof,
        oci_separate:           oci_proof,
        continuing_ops_disclosed: co_proof,
    },
    is_descriptor,
)?;
// is_proof: Established<ComprehensiveIncomeReported>

// Cash flow statement
let (cf, cf_proof) = backend.present_cash_flows(
    CashFlowEvidence { operating_classified, investing_classified,
                       financing_classified, method_disclosed,
                       noncash_disclosed, reconciles },
    cf_descriptor,
)?;
// cf_proof: Established<CashFlowStatementPresented>

// Full set — type system enforces all three are proven before this compiles
let complete = backend.assert_financial_statements_complete(
    FullFinancialStatementsEvidence {
        balance_sheet:    bs_proof,
        income_statement: is_proof,
        cash_flows:       cf_proof,
    },
)?;
// complete: Established<FinancialStatementsComplete>
```

---

## Trait Interface

### Core bookkeeping

```rust
pub trait GaapBookkeeping: Send + Sync {
    fn record_journal_entry(&self, entry: JournalEntryDescriptor)
        -> GaapResult<(JournalEntryDescriptor, Established<DebitEqualsCreditPerEntry>)>;

    fn compile_trial_balance(&self, period: FinancialPeriod)
        -> GaapResult<(TrialBalanceDescriptor, Established<TrialBalanceBalances>)>;

    fn verify_accounting_equation(&self, total_assets: MonetaryAmount,
        total_liabilities: MonetaryAmount, total_equity: MonetaryAmount)
        -> GaapResult<Established<AccountingEquationHolds>>;
}

pub trait GaapLedgerMeta: Send + Sync {
    fn chart_of_accounts(&self) -> BoxFuture<'_, GaapResult<Vec<AccountDescriptor>>>;
    fn current_period(&self) -> BoxFuture<'_, GaapResult<FinancialPeriod>>;
    fn journal_entries_for_period(&self, period: FinancialPeriod)
        -> BoxFuture<'_, GaapResult<Vec<JournalEntryDescriptor>>>;
}
```

### Revenue (ASC 606 five-step)

```rust
pub trait GaapRevenueFactory: Send + Sync {
    // Step 1
    fn identify_contract(&self, contract: RevenueContractDescriptor)
        -> GaapResult<(RevenueContractDescriptor, Established<ContractIdentified>)>;
    // Step 2 — requires Step 1 token
    fn identify_performance_obligations(&self,
        contract_token: Established<ContractIdentified>,
        obligations: Vec<PerformanceObligationDescriptor>)
        -> GaapResult<(Vec<PerformanceObligationDescriptor>,
                       Established<PerformanceObligationsIdentified>)>;
    // Step 3 — requires contract evidence
    fn determine_transaction_price(&self, evidence: ContractIdentificationEvidence,
        price: TransactionPriceDescriptor)
        -> GaapResult<(TransactionPriceDescriptor, Established<TransactionPriceDetermined>)>;
    // Step 4 — requires Steps 1–3
    fn allocate_transaction_price(&self, evidence: Asc606Steps1To3Evidence,
        allocation: AllocationDescriptor)
        -> GaapResult<(AllocationDescriptor, Established<TransactionPriceAllocated>)>;
    // Step 5a — point-in-time
    fn recognize_revenue_at_point_in_time(&self, evidence: Asc606PointInTimeEvidence,
        recognition: RevenueRecognitionDescriptor)
        -> GaapResult<(RevenueRecognitionDescriptor, Established<RevenueRecognizedAtPointInTime>)>;
    // Step 5b — over time
    fn recognize_revenue_over_time(&self, evidence: Asc606OverTimeEvidence,
        recognition: RevenueRecognitionDescriptor)
        -> GaapResult<(RevenueRecognitionDescriptor, Established<RevenueRecognizedOverTime>)>;
}
```

### Asset measurement

```rust
pub trait GaapAssetFactory: Send + Sync {
    fn record_receivable(&self, r: ReceivableDescriptor)
        -> GaapResult<(ReceivableDescriptor, Established<ReceivableRecordedAtAmortizedCost>)>;
    fn record_debt_security(&self, s: SecurityDescriptor)
        -> GaapResult<(SecurityDescriptor, Established<DebtSecurityClassified>)>;
    fn record_equity_security(&self, s: SecurityDescriptor)
        -> GaapResult<(SecurityDescriptor, Established<EquitySecurityAtFairValue>)>;
    fn measure_inventory(&self, i: InventoryDescriptor)
        -> GaapResult<(InventoryDescriptor, Established<InventoryAtLowerOfCostOrNrv>)>;
    fn recognize_ppe(&self, a: PpeDescriptor)
        -> GaapResult<(PpeDescriptor, Established<PpeCarriedAtCost>)>;
    fn test_ppe_impairment(&self, a: PpeDescriptor, recoverable: MonetaryAmount)
        -> GaapResult<PpeDescriptor>;
}
```

### Financial statement presentation (Role 1b)

```rust
pub trait GaapPresentationFactory: Send + Sync {
    fn present_balance_sheet(&self, evidence: BalanceSheetEvidence,
        bs: BalanceSheetDescriptor)
        -> GaapResult<(BalanceSheetDescriptor, Established<BalanceSheetClassified>)>;

    fn present_income_statement(&self, evidence: IncomeStatementEvidence,
        is: IncomeStatementDescriptor)
        -> GaapResult<(IncomeStatementDescriptor, Established<ComprehensiveIncomeReported>)>;

    fn present_cash_flows(&self, evidence: CashFlowEvidence, cf: CashFlowDescriptor)
        -> GaapResult<(CashFlowDescriptor, Established<CashFlowStatementPresented>)>;

    fn present_eps(&self, eps: EpsDescriptor)
        -> GaapResult<(EpsDescriptor, Established<BasicEpsDeclared>)>;

    fn assert_financial_statements_complete(&self,
        evidence: FullFinancialStatementsEvidence)
        -> GaapResult<Established<FinancialStatementsComplete>>;
}
```

### `GaapBackend` supertrait

```rust
pub trait GaapBackend:
    GaapBookkeeping + GaapLedgerMeta
    + GaapPeriodFactory + GaapPeriodReporter
    + GaapRevenueFactory + GaapRevenueMeta
    + GaapAssetFactory + GaapAssetMeta
    + GaapLiabilityFactory + GaapEquityFactory
    + GaapTaxFactory
    + GaapFairValueFactory + GaapLeaseFactory + GaapDerivativeFactory
    + GaapDisclosureFactory + GaapDisclosureMeta
    + GaapIcfrFactory + GaapIcfrMeta
    + GaapPresentationFactory
    + Send + Sync
{}
```

A blanket impl is provided: any type satisfying all 19 sub-traits automatically
implements `GaapBackend`. Each sub-trait is individually object-safe; use
`dyn GaapRevenueFactory`, `dyn GaapPresentationFactory`, etc. for dynamic
dispatch at architectural boundaries.

---

## Contract Module Reference (282 propositions)

### `principles` — GAAP foundational principles (15 propositions)

Encodes the qualitative characteristics and basic assumptions that underlie
all of US GAAP.

| Representative propositions | Standard anchor |
|---|---|
| `DoubleEntryBookkeeping` | GAAP accrual basis; ASC 105-10-05-2 |
| `AccrualBasis` | ASC 105-10-05-1 |
| `MonetaryUnitAssumption` | GAAP conceptual framework |
| `TimePeriodAssumption` | ASC 270 — Interim Reporting |
| `GoingConcernAssumption` | ASC 205-40 |
| `MatchingPrinciple` | ASC 420 — Exit or Disposal Cost Obligations |
| `HistoricalCostPrinciple` | ASC 360-10-30 |
| `FullDisclosurePrinciple` | ASC 235 — Notes to Financial Statements |
| `RevenueRecognitionPrinciple` | ASC 606 |
| `ConservatismPrinciple` | FASB CON 2 |
| `MaterialityPrinciple`, `ConsistencyPrinciple`, `NeutralityPrinciple`, `SubstanceOverFormPrinciple` | FASB CON 8 |

### `mathematical` — Arithmetic invariants (20 propositions)

Proves the numerical invariants that formal verification tools can check
directly (debit/credit balance, equation balance, rollforward arithmetic).

| Group | Representative propositions |
|---|---|
| Core equation | `AccountingEquationHolds`, `DebitEqualsCreditPerEntry`, `TrialBalanceBalances` |
| Income / equity | `NetIncomeAggregation`, `OciRollforward`, `RetainedEarningsRollforward` |
| Cash reconciliation | `CashFlowReconciles` |
| EPS | `EpsNumeratorCorrect`, `EpsDenominatorCorrect`, `DilutedEpsNoMoreThanBasic` |
| Asset/liability rollforward | `InventoryRollforward`, `ReceivablesRollforward`, `AllowanceForCreditLossRollforward`, `GoodwillRollforward`, `LeaseLiabilityPvCorrect` |
| Depreciation / amortization | `AmortizationScheduleCorrect`, `DepreciationAccumulatesCorrectly` |
| Tax | `TaxRateReconciles`, `DeferredTaxNetPresentable` |
| Segment | `SegmentRevenueSumsToConsolidated` |

### `temporal` — Period-timing contracts (17 propositions)

Ensures that transactions are recorded in the correct accounting period
(cut-off, accrual vs. deferral, FX rate selection).

| Representative propositions | Standard anchor |
|---|---|
| `TransactionCutoffRespected`, `RevenueEarnedInPeriod`, `AccrualRecordedAtPeriodEnd` | ASC 105; ASC 606 |
| `DeferralReleasedInEarnedPeriod`, `ExpenseMatchedToPeriod` | Matching principle |
| `DepreciationComputedForFullPeriod`, `InterestAccruedThroughPeriodEnd` | ASC 360; ASC 835 |
| `FxTranslationAtClosingRate`, `FxTranslationAtAverageRate` | ASC 830 |
| `InterimAccrualMethodConsistent`, `SubsequentEventDateBound` | ASC 270; ASC 855 |
| `LeaseCommencementDateCorrect`, `RevenueTransferDateCorrect` | ASC 842; ASC 606 |

### `asc_200` — Presentation of Financial Statements (31 propositions)

Covers ASC 205–280: balance sheet structure, comprehensive income,
cash flows, EPS, interim reporting, and segment reporting.

| Group | Representative propositions |
|---|---|
| Balance sheet (ASC 205–210) | `BalanceSheetClassified`, `CurrentAssetClassification`, `CurrentLiabilityClassification`, `OffsettingProhibited`, `ComparativePeriodPresented`, `GoingConcernEvaluated` |
| Comprehensive income (ASC 220–225) | `OciPresentedSeparately`, `ComprehensiveIncomeReported`, `IncomeFromContinuingOperationsDisclosed`, `DiscontinuedOperationsSeparated` |
| Cash flows (ASC 230) | `CashFlowStatementPresented`, `OperatingActivitiesClassified`, `InvestingActivitiesClassified`, `FinancingActivitiesClassified`, `CashFlowMethodDisclosed`, `NonCashActivitiesDisclosed` |
| Accounting changes (ASC 250) | `AccountingChangeJustified`, `RetrospectiveApplicationApplied`, `ProspectiveApplicationApplied`, `ErrorCorrectionRestated` |
| EPS (ASC 260) | `BasicEpsDeclared`, `DilutedEpsDeclared`, `EpsWeightedAverageSharesCorrect` |
| Interim / segment (ASC 270–280) | `InterimPeriodIntegral`, `SeasonalRevenueDisclosed`, `InterimTaxRateAnnualized`, `SegmentIdentificationComplete`, `SegmentReconcilesTotal`, `ManagementApproachApplied` |
| Aggregate proof | `FinancialStatementsComplete` |

### `asc_300` — Assets (30 propositions)

Covers ASC 310–360: receivables, investments, equity method, inventory,
goodwill, intangibles, and PP&E.

| Group | Representative propositions |
|---|---|
| Receivables (ASC 310) | `ReceivableRecordedAtAmortizedCost`, `AllowanceForCreditLossEstimated`, `TroubledDebtRestructuringIdentified`, `LoanOriginationFeesDeferred` |
| Investments (ASC 320–321) | `DebtSecurityClassified`, `TradingSecurityAtFairValue`, `AfsSecurityAtFairValue`, `HtmSecurityAtAmortizedCost`, `InvestmentImpairmentReviewed`, `EquitySecurityAtFairValue` |
| Equity method (ASC 323) | `EquityMethodApplied`, `EquityMethodInvesteeIdentified`, `EquityMethodImpairmentAssessed` |
| Inventory (ASC 330) | `InventoryAtLowerOfCostOrNrv`, `CostFlowAssumptionDisclosed`, `InventoryWriteDownRecognized`, `LifoReserveDisclosed` |
| Goodwill / intangibles (ASC 350) | `GoodwillAnnuallyTested`, `GoodwillImpairmentRecognized`, `IndefiniteLifeIntangibleTested`, `FiniteLifeIntangibleAmortized`, `InternalUseSoftwareCostCapitalized` |
| PP&E (ASC 360) | `PpeCarriedAtCost`, `DepreciationMethodDisclosed`, `UsefulLifeEstimated`, `LongLivedAssetImpairmentTested`, `DisposalGainLossRecognized`, `PpeAroRecognized` |

### `asc_400` — Liabilities (24 propositions)

Covers ASC 405–480: payables, contingencies, debt, exit costs, mezzanine.

| Group | Representative propositions |
|---|---|
| Payables / ARO (ASC 405–410) | `LiabilityRecognitionCriteriaMet`, `TradeAccountsPayableAccrued`, `AssetRetirementObligationRecognized`, `AroAccretionExpenseRecognized`, `EnvironmentalLiabilityRecognized` |
| Exit costs (ASC 420) | `ExitCostRecognizedWhenLiabilityIncurred`, `SeveranceLiabilityMeasured`, `DeferredRevenueRecordedUntilEarned` |
| Contingencies (ASC 450) | `LossContingencyAssessed`, `ProbableLossAccrued`, `ReasonablyPossibleLossDisclosed`, `GainContingencyNotRecorded`, `ContingentLiabilityReassessed` |
| Guarantees (ASC 460) | `GuaranteeObligationRecognized`, `GuaranteeDisclosed` |
| Debt (ASC 470) | `DebtClassifiedCorrectly`, `DebtIssuanceCostsDeferred`, `EffectiveInterestMethodUsed`, `DebtCovenantComplianceDisclosed`, `ShortTermDebtRefinancingClassified` |
| Mezzanine (ASC 480) | `MandatorilyRedeemableInstrumentInLiabilities`, `FreestandingInstrumentClassified`, `MandatoryRedemptionDisclosed` |

### `asc_500` — Equity (11 propositions)

Covers ASC 505: equity presentation, treasury stock, OCI, noncontrolling interests.

| Representative propositions | Standard anchor |
|---|---|
| `StockholdersEquityPresented`, `CommonStockParValueDisclosed` | ASC 505-10-45 |
| `TreasuryStockAccountedFor`, `StockSplitAccountedFor` | ASC 505-30 |
| `RetainedEarningsReconciled`, `OciAccumulatedSeparately` | ASC 220; ASC 505-10 |
| `NoncontrollingInterestPresented`, `DividendsDeclaredRecorded` | ASC 810 |
| `PreferredStockTermsDisclosed`, `StockSubscriptionReceivableAsContraEquity` | ASC 505-10-45 |
| `RetainedEarningsAppropriationDisclosed` | ASC 505-10-50 |

### `asc_606` — Revenue from Contracts with Customers (29 propositions)

Covers the complete five-step model: contract identification, performance
obligation identification, transaction price determination and allocation,
and revenue recognition (point-in-time and over-time).

| Step | Representative propositions |
|---|---|
| Step 1 — Identify contract | `ContractIdentified`, `ContractCriteriaMet`, `CollectibilityProbable`, `ContractCombinationAssessed`, `ContractModificationAccountedFor` |
| Step 2 — Identify POs | `PerformanceObligationsIdentified`, `DistinctGoodOrServiceDetermined`, `SeriesPerformanceObligationAccountedFor`, `PrincipalVsAgentDetermined` |
| Step 3 — Determine price | `TransactionPriceDetermined`, `VariableConsiderationEstimated`, `VariableConsiderationConstraintApplied`, `SignificantFinancingComponentAssessed`, `NonCashConsiderationMeasured`, `ConsiderationPayableToCustomerDeducted` |
| Step 4 — Allocate price | `TransactionPriceAllocated`, `StandaloneSellingPriceDetermined`, `ResidualApproachApplied`, `AllocationAdjustedForModification` |
| Step 5 — Recognize revenue | `RevenueRecognizedAtPointInTime`, `RevenueRecognizedOverTime`, `OverTimeCriteriaMet`, `ProgressMeasurementMethodSelected`, `ContractBalanceRecordedCorrectly` |
| Disclosures | `RevenueDisaggregated`, `ContractBalancesDisclosed`, `RemainingPerformanceObligationsDisclosed`, `RevenueRecognitionPolicyDisclosed` |

### `asc_700` — Stock Compensation & Tax (16 propositions)

Covers ASC 718–740: stock-based compensation recognition and tax accounting.

| Group | Representative propositions |
|---|---|
| Stock compensation (ASC 718) | `StockCompensationFairValueMeasured`, `StockCompensationRecognizedOverVesting`, `GradedVestingApplied`, `ForfeitureAccountingApplied`, `PerformanceConditionAssessed`, `MarketConditionIncludedInFairValue`, `AwardModificationAccountedFor`, `StockAwardTaxEffectInEarnings` |
| Income taxes (ASC 740) | `DeferredTaxAssetRecognized`, `DeferredTaxLiabilityRecognized`, `ValuationAllowanceAssessed`, `MoreLikelyThanNotStandardApplied`, `UncertainTaxPositionEvaluated`, `EffectiveTaxRateDisclosed`, `DeferredTaxNoncurrentClassified`, `IntraperiodTaxAllocationApplied` |

### `asc_800` — Business Combinations, Consolidation, Derivatives, Fair Value, FX, Leases (37 propositions)

| Group | Representative propositions |
|---|---|
| Business combinations (ASC 805) | `AcquisitionMethodApplied`, `AcquisitionDateIdentified`, `IdentifiableAssetsRecognized`, `GoodwillOrBargainPurchaseRecognized`, `ContingentConsiderationMeasured`, `MeasurementPeriodAdjustmentsRecorded` |
| Consolidation (ASC 810) | `ConsolidationCriteriaEvaluated`, `VieConsolidationAssessed`, `VotingInterestModelApplied`, `NoncontrollingInterestRecognized` |
| Derivatives / hedging (ASC 815) | `DerivativeRecognizedAtFairValue`, `HedgeDesignationDocumented`, `FairValueHedgeAccountedFor`, `CashFlowHedgeAccountedFor`, `HedgeEffectivenessAssessed`, `NetInvestmentHedgeApplied` |
| Fair value (ASC 820) | `FairValueExitPriceApplied`, `FairValueHierarchyApplied`, `Level3InputsDisclosed`, `FairValueOnRecurringBasis`, `FairValueOnNonrecurringBasis`, `FinancialInstrumentFairValueDisclosed`, `FairValueOptionElectionDocumented` |
| Foreign currency (ASC 830) | `FunctionalCurrencyDetermined`, `RemeasurementApplied`, `TranslationAdjustmentInOci`, `ForeignCurrencyTransactionGainLossRecognized` |
| Interest (ASC 835) | `InterestCapitalized`, `EffectiveInterestMethodApplied` |
| Leases (ASC 842) | `LeaseIdentified`, `LeaseClassified`, `RouAssetRecognized`, `LeaseLiabilityRecognized`, `LeaseDiscountRateDetermined`, `LeaseTermDetermined`, `VariableLeasePmtAccountedFor`, `ShortTermLeaseExemptionApplied` |

### `disclosure` — Note disclosure contracts (34 propositions)

Ensures that every required note is present, covering the topics mandated
by GAAP for each asset, liability, equity, and income-statement element.

| Group | Representative propositions |
|---|---|
| General notes | `SignificantAccountingPoliciesDisclosed`, `RelatedPartyTransactionsDisclosed`, `SubsequentEventsDisclosed`, `CommitmentsAndContingenciesDisclosed`, `NewAccountingStandardsDisclosed` |
| Risk disclosures | `ConcentrationRisksDisclosed`, `LiquidityRisksDisclosed` |
| Revenue notes | `RevenueRecognitionPolicyNote`, `RevenueDisaggregationNote`, `ContractBalanceNote`, `RemainingPerformanceObligationNote` |
| Asset notes | `GoodwillRollforwardDisclosed`, `IntangibleAssetsDisclosed`, `PpeDepreciationPolicyDisclosed` |
| Debt notes | `DebtCovenantsDisclosed`, `DebtMaturityScheduleDisclosed` |
| Tax notes | `DeferredTaxComponentsDisclosed`, `EffectiveTaxRateReconciliationDisclosed`, `UncertainTaxBenefitsDisclosed`, `TaxJurisdictionsDisclosed` |
| Benefit plan notes | `PensionObligationDisclosed`, `NetPeriodicBenefitCostDisclosed` |
| Financial instrument notes | `DerivativeAndHedgingDisclosed`, `DerivativeFairValueTableDisclosed`, `FairValueMeasurementMethodsDisclosed`, `Level3FairValueRollforwardDisclosed` |
| Lease notes | `LeaseQuantitativeDisclosed`, `LeaseLiabilityMaturityDisclosed` |
| Compensation notes | `StockCompensationPlanDisclosed`, `UnrecognizedCompensationCostDisclosed` |
| Segment / interim notes | `SegmentInformationDisclosed`, `EntityWideDisclosuresIncluded`, `InterimSignificantChangesDisclosed` |
| Equity notes | `PreferredStockDisclosures` |

### `internal_controls` — ICFR (18 propositions)

Covers SOX Section 302/404 internal controls over financial reporting
and IT general controls.

| Group | Representative propositions |
|---|---|
| Design / operating effectiveness | `IcfrDesignAdequate`, `IcfrOperatingEffective`, `MaterialWeaknessIdentified`, `SignificantDeficiencyEvaluated`, `ManagementIcfrAssessmentCompleted`, `AuditorIcfrOpinionIssued` |
| Disclosure controls | `DisclosureControlsEffective` |
| Audit trail | `AuditTrailComplete`, `AuditTrailTamperEvident`, `RecordsRetentionCompliant` |
| Access / authorization | `SegregationOfDutiesEnforced`, `TransactionAuthorizationRequired`, `AccessControlsImplemented`, `PrivilegedAccessMonitored` |
| Close procedures | `ReconciliationPerformed`, `JournalEntryReviewCompleted`, `PeriodEndCloseControlsApplied` |
| IT general controls | `ChangeManagementControlsApplied` |

---

## Proof Composition Reference

### ASC 606 evidence bundles

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `ContractIdentificationEvidence` | `criteria: Established<ContractCriteriaMet>`, `collectibility: Established<CollectibilityProbable>` | `ContractIdentified` |
| `PerformanceObligationsEvidence` | `contract: Established<ContractIdentified>` | `PerformanceObligationsIdentified` |
| `TransactionPriceEvidence` | `contract: Established<ContractIdentified>` | `TransactionPriceDetermined` |
| `Asc606Steps1To3Evidence` | `contract`, `obligations`, `price`, `ssp` | `TransactionPriceAllocated` |
| `Asc606PointInTimeEvidence` | Steps 1–4 all required | `RevenueRecognizedAtPointInTime` |
| `Asc606OverTimeEvidence` | Steps 1–4 + `over_time_criteria` + `progress_method` | `RevenueRecognizedOverTime` |

### Bookkeeping invariant chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `TrialBalanceEvidence` | `all_entries_balance: Established<DebitEqualsCreditPerEntry>` | `TrialBalanceBalances` |
| `AccountingEquationEvidence` | `double_entry`, `trial_balance`, `net_income` | `AccountingEquationHolds` |
| `RetainedEarningsEvidence` | `accounting_equation`, `net_income` | `RetainedEarningsRollforward` |

### Financial statement presentation bundles

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `BalanceSheetEvidence` | `accounting_equation`, `trial_balance`, `current_assets_classified`, `current_liabilities_classified`, `comparative_period` | `BalanceSheetClassified` |
| `IncomeStatementEvidence` | `net_income`, `oci_separate`, `continuing_ops_disclosed` | `ComprehensiveIncomeReported` |
| `CashFlowEvidence` | `operating_classified`, `investing_classified`, `financing_classified`, `method_disclosed`, `noncash_disclosed`, `reconciles` | `CashFlowStatementPresented` |
| `FullFinancialStatementsEvidence` | `balance_sheet`, `income_statement`, `cash_flows` | `FinancialStatementsComplete` |

---

## Descriptor Types

`gaap::types` provides the data-carrying companions to proof tokens:

### Common primitives

| Type | Purpose |
|------|---------|
| `MonetaryAmount` | `i64` in smallest currency unit (e.g. cents); may be negative for contra-accounts |
| `CurrencyCode` | ISO 4217 code string (e.g. `"USD"`, `"EUR"`) |
| `AccountId` | Chart-of-accounts identifier |
| `EntityId` | Legal-entity identifier |
| `PeriodDate` | ISO 8601 calendar date string (`YYYY-MM-DD`) |
| `AccountType` | `Asset`, `Liability`, `Equity`, `Revenue`, `Expense`, `Contra` |
| `NormalBalance` | `Debit` or `Credit` |
| `AccountDescriptor` | Account id, name, type, normal balance, current balance |

### Journal and ledger

| Type | Purpose |
|------|---------|
| `JournalEntryDescriptor` | Debit entries, credit entries, date, description, entity |
| `DebitEntry` / `CreditEntry` | Account id + monetary amount |
| `TrialBalanceDescriptor` | Period + list of account balances |
| `BalanceSheetTotals` | Period-end totals: total assets, liabilities, equity |
| `FinancialPeriod` | Start date, end date, period type |
| `PeriodType` | `Annual`, `SemiAnnual`, `Quarterly`, `Monthly`, `Custom` |

### Revenue and contracts

| Type | Purpose |
|------|---------|
| `RevenueContractDescriptor` | Contract id, customer, criteria flags, collectibility assessment |
| `PerformanceObligationDescriptor` | PO id, description, distinct flag, stand-alone selling price |
| `TransactionPriceDescriptor` | Gross price, variable consideration estimate, financing component flag |
| `AllocationDescriptor` | Allocation method, per-PO amounts |
| `RevenueRecognitionDescriptor` | PO id, recognized amount, recognition date, method |
| `ContractStatus` | `Draft`, `Active`, `Modified`, `Completed`, `Cancelled` |

### Assets

| Type | Purpose |
|------|---------|
| `ReceivableDescriptor` | Receivable id, gross amount, allowance, origination date |
| `SecurityDescriptor` | Security id, classification, cost, fair value, impairment flag |
| `SecurityClassification` | `Trading`, `AvailableForSale`, `HeldToMaturity` |
| `InventoryDescriptor` | Account, cost, NRV, cost flow method |
| `CostFlowMethod` | `Fifo`, `Lifo`, `WeightedAverage`, `SpecificIdentification` |
| `PpeDescriptor` | Asset id, cost, accumulated depreciation, useful life, method |
| `DepreciationMethod` | `StraightLine`, `DecliningBalance`, `UnitsOfProduction` |

### Liabilities, equity, tax

| Type | Purpose |
|------|---------|
| `LiabilityDescriptor` | Liability id, amount, maturity date, category |
| `ContingencyDescriptor` | Description, probability, estimated range |
| `ContingencyProbability` | `Probable`, `ReasonablyPossible`, `Remote` |
| `DebtDescriptor` | Face value, carrying amount, effective rate, covenant list |
| `EquityDescriptor` | Par value, shares authorized/issued/outstanding, APIC |
| `OciDescriptor` | OCI items list, accumulated OCI balance |
| `TreasuryStockDescriptor` | Shares repurchased, cost, method |
| `DeferredTaxDescriptor` | Temporary difference origin, asset/liability amount, rate used |
| `TaxRateDescriptor` | Statutory rate, effective rate, reconciling items |

### Complex financial instruments

| Type | Purpose |
|------|---------|
| `FairValueDescriptor` | Asset/liability id, valuation technique, level (1/2/3), amount |
| `FairValueLevel` | `Level1`, `Level2`, `Level3` |
| `LeaseDescriptor` | Lease id, classification, ROU asset, liability, discount rate |
| `LeaseClassification` | `Finance`, `Operating` |
| `DerivativeDescriptor` | Derivative id, notional, fair value, hedge designation |
| `HedgeDesignation` | `FairValue`, `CashFlow`, `NetInvestment`, `NotDesignated` |

### Statements and disclosures

| Type | Purpose |
|------|---------|
| `BalanceSheetDescriptor` | Assets, liabilities, equity totals + comparative period |
| `IncomeStatementDescriptor` | Revenue, expenses, OCI items, EPS |
| `CashFlowDescriptor` | Operating, investing, financing activities; reconciliation |
| `EpsDescriptor` | Basic and diluted EPS, weighted-average shares |
| `FootnoteDescriptor` | Topic, text, referenced standard |
| `DisclosureRequirement` | Requirement id, standard citation, present flag |
| `IcfrDescriptor` | Control assessment scope, conclusion, deficiency list |
| `ControlTestDescriptor` | Control id, test type, result, exception count |

---

## Compile-Time Guarantee Summary

| What is guaranteed | Mechanism |
|---|---|
| Journal entries are balanced before trial balance | `GaapBookkeeping::compile_trial_balance` requires a prior `record_journal_entry`; the trial balance method internally chains through `TrialBalanceEvidence` |
| ASC 606 Step 2 cannot precede Step 1 | `identify_performance_obligations` takes `Established<ContractIdentified>` as a parameter — not constructible without calling Step 1 |
| Transaction price allocation requires all three prior steps | `Asc606Steps1To3Evidence` struct fields `contract`, `obligations`, `price` are all required `Established<_>` tokens |
| Financial statements cannot be "complete" without all three sections | `FullFinancialStatementsEvidence` requires `balance_sheet`, `income_statement`, `cash_flows` — all required, no `Option` |
| Balance sheet requires accounting equation | `BalanceSheetEvidence.accounting_equation: Established<AccountingEquationHolds>` is a required field |
| `assert()` bypasses are audit-visible | Any `Established::assert()` on a GAAP proposition stands out immediately in code review and is searchable |

---

## Implementing a Custom Backend

To implement `GaapBackend` for a new accounting data store:

1. Implement the **leaf factory traits** (Role 1a): `GaapBookkeeping`,
   `GaapRevenueFactory`, `GaapAssetFactory`, `GaapLiabilityFactory`,
   `GaapEquityFactory`, `GaapTaxFactory`, `GaapFairValueFactory`,
   `GaapLeaseFactory`, `GaapDerivativeFactory`, `GaapDisclosureFactory`,
   `GaapIcfrFactory`, `GaapPeriodFactory`. Each method validates its input
   and then calls `Established::assert()` on success.

2. Implement the **reporter traits** (Role 2): `GaapLedgerMeta`,
   `GaapRevenueMeta`, `GaapAssetMeta`, `GaapPeriodReporter`,
   `GaapDisclosureMeta`, `GaapIcfrMeta`. These return plain data;
   no proof tokens required.

3. Implement the **section factory** (Role 1b): `GaapPresentationFactory`.
   Each method receives a pre-assembled evidence bundle — the evidence bundles
   are defined in `traits::presentation` and are the only way to call these
   methods.

4. `GaapBackend` is automatically satisfied by the blanket impl.

> **Note:** `Established::assert()` is the correct constructor for backend
> implementations — the factory method *is* the authority. The credential-gated
> `Established::prove()` path is reserved for evidence-bundle composition
> between already-established leaf tokens.

### Example: minimal in-memory backend (testing)

```rust
use crate::gaap::{GaapBookkeeping, GaapLedgerMeta};
use crate::gaap::types::{JournalEntryDescriptor, FinancialPeriod, TrialBalanceDescriptor};
use crate::gaap::mathematical::{DebitEqualsCreditPerEntry, TrialBalanceBalances};
use elicitation::Established;

pub struct InMemoryGaapBackend { /* ... */ }

impl GaapBookkeeping for InMemoryGaapBackend {
    fn record_journal_entry(&self, entry: JournalEntryDescriptor)
        -> GaapResult<(JournalEntryDescriptor, Established<DebitEqualsCreditPerEntry>)>
    {
        let debit_total:  i64 = entry.debits.iter().map(|d| d.amount.units).sum();
        let credit_total: i64 = entry.credits.iter().map(|c| c.amount.units).sum();
        if debit_total != credit_total {
            return Err(GaapError::unbalanced_entry(debit_total, credit_total));
        }
        // All validation passed — the factory is the authority.
        Ok((entry, Established::assert()))
    }
    // ...
}
```

---

## Standards Grounding

| Standard | Coverage |
|----------|----------|
| FASB ASC 105 | Generally Accepted Accounting Principles — hierarchy, accrual basis |
| FASB ASC 205–280 | Presentation of Financial Statements — BS, IS, CF, EPS, interim, segments |
| FASB ASC 310–360 | Assets — receivables, investments, inventory, goodwill, PP&E |
| FASB ASC 405–480 | Liabilities — payables, contingencies, ARO, debt, mezzanine |
| FASB ASC 505 | Equity — components, treasury stock, OCI, noncontrolling interests |
| FASB ASC 606 | Revenue from Contracts with Customers — complete five-step model |
| FASB ASC 718 | Stock-Based Compensation — grant-date FV, vesting, modifications |
| FASB ASC 740 | Income Taxes — deferred tax, valuation allowance, uncertain positions |
| FASB ASC 805 | Business Combinations — acquisition method, contingent consideration |
| FASB ASC 810 | Consolidation — VIE model, voting-interest model |
| FASB ASC 815 | Derivatives and Hedging — recognition, hedge accounting |
| FASB ASC 820 | Fair Value Measurement — exit price, hierarchy, Level 3 disclosures |
| FASB ASC 830 | Foreign Currency — functional currency, remeasurement, translation |
| FASB ASC 835 | Interest — capitalization, effective-interest method |
| FASB ASC 842 | Leases — classification, ROU asset, lease liability |
| SOX §302/§404 | ICFR — management assessment, auditor attestation, disclosure controls |
| COSO 2013 Framework | Internal controls — design, operating effectiveness, deficiencies |

---

## Module Layout

```text
src/gaap/
├── mod.rs                     pub use surface + module declarations
├── errors.rs                  GaapError, GaapResult
│
├── principles.rs              15 props — foundational GAAP principles
├── mathematical.rs            20 props — arithmetic invariants (equation, rollforward, EPS)
├── temporal.rs                17 props — period-timing contracts (cut-off, accrual, FX rates)
├── asc_200.rs                 31 props — ASC 200: presentation (BS, IS, CF, EPS, segments)
├── asc_300.rs                 30 props — ASC 300: assets (receivables, securities, inventory, PPE)
├── asc_400.rs                 24 props — ASC 400: liabilities (payables, contingencies, debt)
├── asc_500.rs                 11 props — ASC 500: equity (components, treasury, OCI)
├── asc_606.rs                 29 props — ASC 606: revenue recognition (five-step model)
├── asc_700.rs                 16 props — ASC 700: stock compensation + income taxes
├── asc_800.rs                 37 props — ASC 800: combinations, consolidation, derivatives,
│                                         fair value, FX, interest, leases
├── disclosure.rs              34 props — note disclosure obligations per GAAP topic
├── internal_controls.rs       18 props — ICFR / SOX §302/404 / IT general controls
│
├── proof_composition.rs       ProvableFrom chains + evidence bundle types
│                              (ASC 606 Steps 1–5, trial balance, equation, retained earnings)
│
├── traits/
│   ├── mod.rs                 Re-exports + GaapBackend supertrait + blanket impl
│   ├── bookkeeping.rs         GaapBookkeeping (Role 1a) · GaapLedgerMeta (Role 2)
│   ├── revenue.rs             GaapRevenueFactory (Role 1a) · GaapRevenueMeta (Role 2)
│   ├── assets.rs              GaapAssetFactory (Role 1a) · GaapAssetMeta (Role 2)
│   ├── liabilities.rs         GaapLiabilityFactory (Role 1a)
│   ├── equity.rs              GaapEquityFactory (Role 1a)
│   ├── period.rs              GaapPeriodFactory (Role 1a) · GaapPeriodReporter (Role 2)
│   ├── tax.rs                 GaapTaxFactory (Role 1a)
│   ├── complex.rs             GaapFairValueFactory · GaapLeaseFactory · GaapDerivativeFactory (Role 1a)
│   ├── disclosure.rs          GaapDisclosureFactory (Role 1a) · GaapDisclosureMeta (Role 2)
│   ├── icfr.rs                GaapIcfrFactory (Role 1a) · GaapIcfrMeta (Role 2)
│   └── presentation.rs        GaapPresentationFactory (Role 1b) +
│                              BalanceSheetEvidence, IncomeStatementEvidence,
│                              CashFlowEvidence, FullFinancialStatementsEvidence
│
└── types/
    ├── mod.rs                 Re-exports
    ├── common.rs              MonetaryAmount, CurrencyCode, AccountId, PeriodDate, AccountType
    ├── journal.rs             JournalEntryDescriptor, DebitEntry, CreditEntry, TrialBalanceDescriptor
    ├── period.rs              FinancialPeriod, PeriodType, AccrualDescriptor, DeferralDescriptor
    ├── revenue.rs             RevenueContractDescriptor, PerformanceObligationDescriptor, AllocationDescriptor
    ├── assets.rs              ReceivableDescriptor, SecurityDescriptor, InventoryDescriptor, PpeDescriptor
    ├── liabilities.rs         LiabilityDescriptor, ContingencyDescriptor, DebtDescriptor
    ├── equity.rs              EquityDescriptor, OciDescriptor, TreasuryStockDescriptor
    ├── tax.rs                 DeferredTaxDescriptor, TaxRateDescriptor
    ├── complex.rs             FairValueDescriptor, LeaseDescriptor, DerivativeDescriptor
    ├── disclosure.rs          FootnoteDescriptor, DisclosureRequirement
    ├── icfr.rs                IcfrDescriptor, ControlTestDescriptor, ManagementAssertionDescriptor
    └── statements.rs          BalanceSheetDescriptor, IncomeStatementDescriptor, CashFlowDescriptor, EpsDescriptor
```

---

## Formal Verification

Each proposition type implements `elicitation::contracts::Prop`, which exposes
`kani_proof()`, `verus_proof()`, and `creusot_proof()` methods for generating
verification harnesses.

- **Kani** — bounded model checking on arithmetic invariants: debit/credit
  balance, accounting equation, EPS denominator bounds
- **Creusot** — deductive verification that factory methods check invariants
  before calling `Established::assert()`; proof that the ASC 606 five-step
  chain cannot be collapsed
- **Verus** — SMT-based proofs of evidence bundle composition totality;
  verification that `FullFinancialStatementsEvidence` requires all three
  section proofs

---

## License

Apache-2.0 OR MIT
