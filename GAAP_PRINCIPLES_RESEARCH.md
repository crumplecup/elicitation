# GAAP Principles Research for Ledger Integration

> Research document identifying applicable Generally Accepted Accounting Principles (GAAP) for the typestate ledger system, with formal FASB Accounting Standards Codification (ASC) references.

**Status:** Phase 1 Complete
**Date:** 2026-04-06

---

## Table of Contents

- [Executive Summary](#executive-summary)
- [ASC Codification Overview](#asc-codification-overview)
- [Core GAAP Principles](#core-gaap-principles)
- [Applicable ASC Topics](#applicable-asc-topics)
- [GAAP Principles Mapping](#gaap-principles-mapping)
- [Applicability to Ledger Operations](#applicability-to-ledger-operations)
- [Implementation Recommendations](#implementation-recommendations)
- [Sources](#sources)

---

## Executive Summary

This document identifies **8 core GAAP principles** directly applicable to double-entry ledger operations, with formal ASC references for audit-traceable compliance. The research focuses on principles that can be verified at transaction time through typestate proofs.

**Key finding:** While GAAP encompasses many principles, only a subset apply to basic ledger operations (transfers, debits, credits). Advanced principles (revenue recognition, fair value) apply to higher-level financial reporting built *on top of* the ledger.

---

## ASC Codification Overview

The [FASB Accounting Standards Codification](https://asc.fasb.org/) (ASC) is the single source of authoritative U.S. GAAP. It reorganizes thousands of GAAP pronouncements into ~90 accounting topics using a consistent structure:

**Structure:**
- **Topics** - Three-digit numbers (e.g., 606, 820)
- **Subtopics** - Two-digit suffixes (e.g., 606-10)
- **Sections** - Two-digit codes:
  - `05` - Overview and Background
  - `15` - Scope and Scope Exceptions
  - `25` - Recognition
  - `30` - Initial Measurement
  - `35` - Subsequent Measurement
  - `45` - Other Presentation Matters
  - `50` - Disclosure

**Example:** ASC 606-10-25-1 = Topic 606 (Revenue), Subtopic 10 (Overall), Section 25 (Recognition), Paragraph 1

**Key topics for ledger operations:**
- **ASC 105** - Generally Accepted Accounting Principles
- **ASC 606** - Revenue from Contracts with Customers
- **ASC 820** - Fair Value Measurement
- **ASC 250** - Accounting Changes and Error Corrections (includes materiality)

---

## Core GAAP Principles

### 1. Double-Entry Bookkeeping

**Not explicitly codified** - foundational requirement predating ASC

**Principle:** Every transaction records equal and opposite entries (debits and credits) to maintain the accounting equation:

```
Assets = Liabilities + Equity
```

**Requirements:**
- Total debits = Total credits for every transaction
- At least two accounts affected per transaction
- Self-balancing general ledger
- Required by FASB for GAAP compliance

**Applicability:** **CRITICAL** - Core ledger invariant

Public companies must use double-entry bookkeeping and follow GAAP/IFRS rules. The Financial Accounting Standards Board (FASB) decides on GAAP, which are the official rules for double-entry bookkeeping.

---

### 2. Accrual Basis Accounting

**ASC Reference:** Related to ASC 606-10-25-1 (Revenue Recognition)

**Principle:** Transactions are recorded when they occur, not when cash changes hands. Combines revenue recognition principle and matching principle.

**Requirements:**
- Record transactions at occurrence time
- Recognize revenue when earned
- Recognize expenses when incurred
- Independent of cash flow timing

**Applicability:** **HIGH** - Transfer timing verification

The accrual basis is the only method allowed under GAAP and is required by the SEC for publicly traded companies. It ensures financial statements accurately reflect a company's financial position at a specific point in time.

---

### 3. Matching Principle (Expense Recognition)

**ASC Reference:** Related to ASC 606-10-25-23 (Revenue Recognition timing)

**Principle:** Expenses should be recognized in the same period as the revenue they help generate. Part of accrual accounting that ensures accurate profit tracking.

**Requirements:**
- Associated costs recorded in same period as revenues
- Maintains consistency in profitability tracking
- Prevents artificial inflation/deflation of profits

**Applicability:** **MEDIUM** - Related transaction linking

Developed jointly with FASB's ASC Topic 606, the matching principle traces back to the development of double-entry bookkeeping during the Italian Renaissance, formalized by Luca Pacioli's 1494 treatise.

---

### 4. Historical Cost Principle

**ASC Reference:** ASC 820-10 (Fair Value - by contrast/exception)

**Principle:** Assets recorded at their original purchase price (cost), not current market value. Provides conservative, reliable, easily calculated basis for asset valuation.

**Requirements:**
- Assets recorded at acquisition cost
- No revaluation to market value (unless impaired or specific rules apply)
- Depreciation/amortization applied over time
- Conservative approach to asset valuation

**Applicability:** **MEDIUM** - Initial transaction amounts

Under U.S. GAAP, historical cost is a conservative and reliable way to account for capital expenditures. The cost principle aligns with the conservatism principle by preventing companies from overstating the value of an asset.

**Note:** ASC 820 defines *fair value* measurement for exceptions to historical cost (e.g., financial instruments, crypto assets).

---

### 5. Conservatism Principle (Prudence)

**ASC Reference:** Implicit in ASC 250 (Error Corrections), ASC 450 (Contingencies)

**Principle:** When faced with uncertainty, choose the option that results in lower net income or smaller asset values. "Recognize losses when probable, gains when realized."

**Requirements:**
- Loss contingencies accrued if probable and estimable
- Gain contingencies not recognized until realized
- If two acceptable methods exist, choose the more conservative
- Prevents overstatement of financial position

**Applicability:** **LOW** - Error handling bias

Losses must be recognized when their occurrence becomes probable, whether or not they have actually occurred. The conservatism principle influences both recognition and measurement.

---

### 6. Economic Entity Assumption

**ASC Reference:** ASC 105 (General Principles)

**Principle:** The business is separate and distinct from its owners, managers, and employees. Owner's personal transactions excluded from company books.

**Requirements:**
- Business transactions recorded separately
- Personal expenses excluded
- Clear boundary between entity and owners
- Maintains integrity of financial statements

**Applicability:** **MEDIUM** - Account ownership verification

The Economic Entity Assumption states that the company or business entity is separate and distinct from its owners, managers and employees. This validates keeping business and personal finances separate.

---

### 7. Monetary Unit Assumption

**ASC Reference:** ASC 105 (General Principles)

**Principle:** All business transactions measured and recorded in a common monetary unit (e.g., USD). Assumes stable currency as unit of record, unadjusted for inflation.

**Requirements:**
- Transactions recorded in single currency
- Monetary unit is stable and consistent
- Non-monetary items excluded (e.g., employee morale)
- FASB accepts nominal USD value without inflation adjustment

**Applicability:** **HIGH** - Transaction amount representation

The Monetary Unit Assumption states that all business transactions must be measured and recorded only in terms of a common unit of measurement which is money. The FASB accepts the nominal value of the US dollar as the monetary unit of record.

---

### 8. Going Concern Assumption

**ASC Reference:** ASC 105 (General Principles), ASC 205-40 (Going Concern)

**Principle:** Financial statements prepared assuming the business will continue operating indefinitely. Validates capitalization, depreciation, and amortization methods.

**Requirements:**
- Business expected to continue operations
- Assets valued under ongoing use, not liquidation
- Liabilities paid in normal course
- Management must disclose substantial doubt if exists

**Applicability:** **LOW** - System-level assumption

Going Concern assumes that the business will be in operation indefinitely, which validates the methods of asset capitalization, depreciation, and amortization. Financial statements are required to be prepared under the "going concern" basis unless management sees major risks.

---

### 9. Materiality Principle

**ASC Reference:** ASC 250 (Accounting Changes and Error Corrections), SEC Staff Accounting Bulletin No. 99

**Principle:** Significant items affecting financial decisions must be reported; insignificant items may be treated more flexibly. Based on professional judgment, not mechanical rules.

**Requirements:**
- Quantitative benchmarks (common: 5% pre-tax income, 0.5% total assets)
- Qualitative factors (fraud, covenant compliance, analyst expectations)
- Total mix of information evaluated
- No bright-line rules or absolute thresholds

**Applicability:** **LOW** - Error threshold policy

FASB has intentionally avoided creating a rigid, universal formula for materiality, emphasizing that it is a concept based on professional judgment. The SEC cautions against using mechanical rules of thumb. Even small errors can be qualitatively material if they involve fraud or affect key metrics.

---

## Applicable ASC Topics

### ASC 105 - Generally Accepted Accounting Principles

**Scope:** Establishes FASB Codification as authoritative source of GAAP

**Key sections:**
- ASC 105-10-05: Overview of GAAP hierarchy
- Contains fundamental assumptions (economic entity, monetary unit, going concern)

**Ledger applicability:** Foundational framework

ASC Topic 105 establishes the FASB Accounting Standards Codification as the source of authoritative GAAP recognized by the FASB to be applied by nongovernmental entities.

---

### ASC 606 - Revenue from Contracts with Customers

**Scope:** Revenue recognition framework

**Core principle:** Recognize revenue to depict the transfer of promised goods or services in an amount reflecting the consideration expected.

**Five-step model:**
1. Identify the contract(s) with customer
2. Identify performance obligations
3. Determine transaction price
4. Allocate transaction price to performance obligations
5. Recognize revenue when performance obligation satisfied

**Key sections:**
- ASC 606-10-25: Recognition criteria
- ASC 606-10-25-23: Timing of revenue recognition (related to matching principle)

**Ledger applicability:** HIGH-LEVEL - Revenue transactions built on ledger

ASC 606 replaced almost all pre-existing revenue recognition guidance. FASB's 2025 Post-Implementation Review found the standard meets its intended purpose without significant unintended consequences.

**Note:** Relevant for applications that track revenue transactions, not basic ledger operations.

---

### ASC 820 - Fair Value Measurement

**Scope:** Framework for defining, measuring, and reporting fair value

**Core principle:** Fair value is an exit price - the price to sell an asset or transfer a liability in an orderly transaction.

**Key sections:**
- ASC 820-10-05: Definition of fair value
- ASC 820-10-35: Measurement framework

**Ledger applicability:** EXCEPTION CASE - Fair value vs. historical cost

ASC 820 is the American accounting standard for Fair Value Measurements issued by FASB in 2011. The definition is based on an exit price notion, representing the price to sell an asset or transfer a liability.

**Recent updates:**
- ASU 2022-03: Equity securities with contractual sale restrictions
- 2023: Joint ventures required to measure assets/liabilities at fair value
- 2023: Crypto assets measured at fair value each reporting period

**Note:** Historical cost is default; ASC 820 defines exceptions (financial instruments, crypto, certain equity securities).

---

### ASC 250 - Accounting Changes and Error Corrections

**Scope:** Accounting for changes in principles, estimates, and error corrections

**Key sections:**
- Contains guidance on materiality
- Cross-referenced with SEC SAB 99

**Ledger applicability:** ERROR HANDLING - Correction procedures

ASC 250 provides guidance on evaluating and correcting accounting errors, incorporating the materiality principle for determining significance.

---

## GAAP Principles Mapping

| GAAP Principle | ASC Reference | Applicability | Priority | Verification Point |
|----------------|---------------|---------------|----------|-------------------|
| **Double-Entry** | Pre-ASC foundational | **CRITICAL** | P0 | Every transaction |
| **Accrual Basis** | ASC 606-10-25-1 | **HIGH** | P0 | Transaction timestamp |
| **Matching Principle** | ASC 606-10-25-23 | **MEDIUM** | P1 | Related entries |
| **Monetary Unit** | ASC 105 | **HIGH** | P0 | Amount representation |
| **Economic Entity** | ASC 105 | **MEDIUM** | P1 | Account ownership |
| **Historical Cost** | ASC 820-10 (contrast) | **MEDIUM** | P1 | Initial valuation |
| **Conservatism** | ASC 250, 450 | **LOW** | P2 | Error handling |
| **Going Concern** | ASC 105, 205-40 | **LOW** | P2 | System assumption |
| **Materiality** | ASC 250, SAB 99 | **LOW** | P2 | Error thresholds |

**Priority levels:**
- **P0** - Core ledger operations (implement first)
- **P1** - Enhanced compliance (implement second)
- **P2** - Policy/configuration (implement later)

---

## Applicability to Ledger Operations

### Directly Verifiable (P0 - Critical)

These principles can be verified at transaction time through typestate proofs:

#### 1. Double-Entry Bookkeeping
```rust
/// GAAP Double-Entry Requirement
///
/// Every transaction must have equal debits and credits, maintaining
/// the accounting equation: Assets = Liabilities + Equity
///
/// # ASC Reference
///
/// Pre-ASC foundational requirement. Required by FASB for GAAP compliance.
/// Public companies must use double-entry bookkeeping per GAAP/IFRS.
///
/// # Verification
///
/// - Debit amount == Credit amount (within same transfer_id)
/// - Sum of all account balances unchanged after transaction
/// - At least two ledger entries per transaction
#[derive(elicitation::Prop)]
pub struct DoubleEntryBookkeeping;
```

**Validation:**
- Check that debit entry amount = credit entry amount
- Verify both entries share same `transfer_id`
- Confirm total balance sum unchanged (invariant preservation)

---

#### 2. Accrual Basis
```rust
/// GAAP Accrual Basis Accounting
///
/// Transactions recorded when they occur, not when cash changes hands.
/// Ensures financial statements reflect actual financial position.
///
/// # ASC Reference
///
/// ASC 606-10-25-1: Revenue recognition when performance obligation satisfied
///
/// # GAAP Requirement
///
/// Only method allowed under GAAP. Required by SEC for public companies.
///
/// # Verification
///
/// - Transaction timestamp = occurrence time
/// - Created_at field accurately records transaction date
/// - Independent of cash settlement timing
#[derive(elicitation::Prop)]
pub struct AccrualBasis;
```

**Validation:**
- Verify `created_at` timestamp is set at transaction time
- Check that transaction is atomic (both entries recorded simultaneously)
- Confirm no dependency on cash settlement status

---

#### 3. Monetary Unit Assumption
```rust
/// GAAP Monetary Unit Assumption
///
/// All transactions measured in common monetary unit (USD).
/// Assumes stable currency, nominal value unadjusted for inflation.
///
/// # ASC Reference
///
/// ASC 105: Generally Accepted Accounting Principles
///
/// # GAAP Requirement
///
/// FASB accepts nominal USD value as monetary unit of record.
///
/// # Verification
///
/// - Amount stored as integer (cents) or decimal
/// - Single currency unit (no foreign exchange)
/// - No inflation adjustment applied
#[derive(elicitation::Prop)]
pub struct MonetaryUnitAssumption;
```

**Validation:**
- Verify amount is expressed in consistent unit (e.g., cents as i64)
- Check no currency conversion applied
- Confirm simple arithmetic operations valid

---

### Enhanced Compliance (P1)

#### 4. Matching Principle
```rust
/// GAAP Matching Principle
///
/// Related revenues and expenses recognized in same period.
/// Maintains consistency in profitability tracking.
///
/// # ASC Reference
///
/// ASC 606-10-25-23: Timing of revenue recognition
///
/// # Historical Context
///
/// Developed jointly with ASC 606. Traces to Luca Pacioli's 1494
/// double-entry bookkeeping treatise.
///
/// # Verification
///
/// - Related entries share same transfer_id
/// - Identical created_at timestamps
/// - Both entries atomic (succeed/fail together)
#[derive(elicitation::Prop)]
pub struct MatchingPrinciple;
```

**Validation:**
- Verify debit and credit entries have same `transfer_id`
- Check timestamps are identical
- Confirm transactional atomicity (database transaction)

---

#### 5. Economic Entity Assumption
```rust
/// GAAP Economic Entity Assumption
///
/// Business is separate and distinct from owners/managers/employees.
/// Owner's personal transactions excluded from company books.
///
/// # ASC Reference
///
/// ASC 105: Generally Accepted Accounting Principles
///
/// # Verification
///
/// - Account ownership clearly defined
/// - No mixing of personal/business transactions
/// - Account names represent business entities
#[derive(elicitation::Prop)]
pub struct EconomicEntityAssumption;
```

**Validation:**
- Verify accounts belong to defined entities
- Check account naming conventions enforce separation
- Confirm no cross-entity contamination

---

#### 6. Historical Cost Principle
```rust
/// GAAP Historical Cost Principle
///
/// Assets recorded at original acquisition cost, not current market value.
/// Provides conservative, reliable, easily calculated basis.
///
/// # ASC Reference
///
/// ASC 820-10: Fair Value Measurement (defines exceptions to historical cost)
///
/// # GAAP Requirement
///
/// Default under U.S. GAAP. Conservative approach preventing overstatement.
///
/// # Verification
///
/// - Initial transaction amount = acquisition cost
/// - No subsequent revaluation to market value
/// - Amount unchanged unless depreciation/impairment
#[derive(elicitation::Prop)]
pub struct HistoricalCostPrinciple;
```

**Validation:**
- Verify initial transfer amount reflects original cost
- Check no fair value adjustments applied
- Confirm amounts are immutable (entries never updated)

---

### Policy/Configuration (P2)

#### 7. Conservatism Principle
```rust
/// GAAP Conservatism Principle
///
/// When uncertain, choose option resulting in lower net income
/// or smaller asset values. Recognize losses when probable,
/// gains when realized.
///
/// # ASC Reference
///
/// Implicit in ASC 250 (Error Corrections), ASC 450 (Contingencies)
///
/// # Verification
///
/// - Error corrections reduce assets / increase liabilities
/// - Validation failures reject transactions (conservative)
/// - No speculative gains recorded
#[derive(elicitation::Prop)]
pub struct ConservatismPrinciple;
```

**Validation:**
- Prefer rejection over questionable acceptance
- Round down for asset increases, round up for liability increases
- Log warnings for near-threshold scenarios

---

#### 8. Going Concern Assumption
```rust
/// GAAP Going Concern Assumption
///
/// Financial statements prepared assuming business continues
/// operating indefinitely. Validates capitalization, depreciation,
/// amortization methods.
///
/// # ASC Reference
///
/// ASC 105: General Principles
/// ASC 205-40: Presentation of Financial Statements—Going Concern
///
/// # Verification
///
/// - System operational and accepting transactions
/// - No liquidation mode active
/// - Normal business operations assumed
#[derive(elicitation::Prop)]
pub struct GoingConcernAssumption;
```

**Validation:**
- System-level flag (not per-transaction)
- Check system not in shutdown/liquidation mode
- Primarily documentation/disclosure requirement

---

#### 9. Materiality Principle
```rust
/// GAAP Materiality Principle
///
/// Significant items affecting financial decisions must be reported.
/// Insignificant items may be treated flexibly. Based on professional
/// judgment, not mechanical rules.
///
/// # ASC Reference
///
/// ASC 250: Accounting Changes and Error Corrections
/// SEC Staff Accounting Bulletin No. 99
///
/// # Thresholds
///
/// Quantitative (common): 5% pre-tax income, 0.5% total assets
/// Qualitative: Fraud, covenants, analyst expectations
///
/// # Verification
///
/// - Configurable thresholds
/// - Small amounts may use simplified handling
/// - Errors below threshold may defer correction
#[derive(elicitation::Prop)]
pub struct MaterialityPrinciple;
```

**Validation:**
- Configuration-driven thresholds
- Apply to error correction decisions
- Document threshold rationale

---

## Implementation Recommendations

### Phase 2: Proposition Types

**Create 9 proposition types** (P0-P2 priorities):

**P0 - Critical (Implement first):**
1. `DoubleEntryBookkeeping` - Debit = Credit invariant
2. `AccrualBasis` - Transaction timing
3. `MonetaryUnitAssumption` - Amount representation

**P1 - Enhanced Compliance:**
4. `MatchingPrinciple` - Related entry linking
5. `EconomicEntityAssumption` - Account ownership
6. `HistoricalCostPrinciple` - Original cost

**P2 - Policy/Configuration:**
7. `ConservatismPrinciple` - Error handling bias
8. `GoingConcernAssumption` - System-level flag
9. `MaterialityPrinciple` - Threshold configuration

### Phase 3: Validation Functions

Each proposition gets a validation function:

```rust
fn validate_double_entry_bookkeeping(
    transfer: &Transfer<Pending>,
) -> Result<Established<DoubleEntryBookkeeping>, ValidationError> {
    // Verify debit amount == credit amount
    // Check both entries share transfer_id
    // Confirm balance invariant
    Ok(Established::assert())
}
```

### Phase 4: Composite Proofs

**Option A: Full GAAP Compliance (All 9)**
```rust
pub type GaapCompliant = And<
    DoubleEntryBookkeeping,
    And<AccrualBasis,
    And<MonetaryUnitAssumption,
    And<MatchingPrinciple,
    And<EconomicEntityAssumption,
    And<HistoricalCostPrinciple,
    And<ConservatismPrinciple,
    And<GoingConcernAssumption,
    MaterialityPrinciple>>>>>>>
>;
```

**Option B: Core Compliance (P0 only)**
```rust
pub type GaapCoreCompliant = And<
    DoubleEntryBookkeeping,
    And<AccrualBasis, MonetaryUnitAssumption>
>;
```

**Option C: Enhanced Compliance (P0 + P1)**
```rust
pub type GaapEnhancedCompliant = And<
    GaapCoreCompliant,
    And<MatchingPrinciple,
    And<EconomicEntityAssumption, HistoricalCostPrinciple>>
>;
```

**Recommendation:** Start with Option B (Core), extend to Option C (Enhanced), make Option A (Full) optional.

---

## Key Insights

### 1. Not All GAAP Applies to Basic Ledger

Many GAAP principles (revenue recognition, fair value, lease accounting) apply to *financial reporting* built on top of the ledger, not to individual ledger entries.

**Ledger scope:** Recording transactions with proper debit/credit entries
**Financial reporting scope:** Aggregating, classifying, and presenting ledger data in financial statements

### 2. Priority-Based Implementation

**P0 principles** are non-negotiable for GAAP compliance:
- Double-entry bookkeeping (foundational)
- Accrual basis (required by SEC)
- Monetary unit (data representation)

**P1 principles** enhance audit-readiness but aren't verified per-transaction in traditional systems. We can provide compile-time guarantees that traditional systems can't.

**P2 principles** are primarily policy/configuration, not transactional proofs.

### 3. Typestate Advantage

Traditional accounting systems verify GAAP compliance through:
- Manual review
- Post-hoc audits
- Runtime validation
- Periodic reconciliation

**Typestate ledger provides:**
- Compile-time proof that violations are impossible
- Zero-cost runtime enforcement (proofs compile away)
- Audit trail baked into type system
- Machine-checkable compliance

### 4. ASC References Are Documentation

ASC codification doesn't provide formulas or algorithms - it provides:
- Definitions and concepts
- Recognition criteria
- Disclosure requirements
- Scope and applicability

**Our propositions** translate these conceptual requirements into:
- Concrete validation logic
- Type-safe state transitions
- Formally verified proofs
- Auditable trace through Established<P> tokens

---

## Next Steps (Phase 2)

1. **Create `src/ledger/gaap.rs`** with 9 proposition types
2. **Write comprehensive doc comments** using templates above
3. **Export publicly** from `ledger/mod.rs`
4. **Verify zero-cost** with `std::mem::size_of` tests
5. **Add to PLANNING_INDEX.md** when complete

---

## Sources

### FASB ASC Topic 606 (Revenue Recognition)

- [Revenue Recognition Under ASC 606: Addressing Organizational Pain Points with Strategic Foresight](https://www.bdo.com/insights/assurance/revenue-recognition-under-asc-606)
- [ASC 606 revenue recognition | New standard | Firm of the Future](https://www.firmofthefuture.com/accounting/timing-is-everything-with-asc-606-the-new-revenue-recognition-standard/)
- [A guide to revenue recognition](https://rsmus.com/insights/financial-reporting/a-guide-to-revenue-recognition.html)
- [ASC 606 Revenue Recognition Guide: Methods How-To](https://www.rightrev.com/asc-606-revenue-recognition-guide/)
- [Revenue Recognition](https://fasb.org/page/PageContent?pageId=/projects/recentlycompleted/revenue-recognition-summary.html)
- [ASC 606 Revenue Recognition | 5-Step Model + Examples](https://www.wallstreetprep.com/knowledge/asc-606/)

### GAAP Principles (Matching, Accrual)

- [Matching principle — Grokipedia](https://grokipedia.com/page/Matching_principle)
- [What is Matching Principle Accounting? Significance & Impact](https://suozziforny.com/matching-principle-accounting/)
- [GAAP Accrual Accounting: a Comprehensive Guide](https://www.indinero.com/blog/gaap-what-it-is-why-your-investors-expect-it/)
- [Accrual Accounting Explained: Examples, Journal Entries, & More](https://finquery.com/blog/accrual-accounting-explained/)
- [Matching principle - Wikipedia](https://en.wikipedia.org/wiki/Matching_principle)
- [Accrual-Based Accounting Explained: What It Is, Advantages & Examples | NetSuite](https://www.netsuite.com/portal/resource/articles/accounting/accrual-basis-accounting.shtml)
- [Matching Principle | Definition + Concept Examples](https://www.wallstreetprep.com/knowledge/matching-principle/)

### FASB ASC Codification

- [FASB Accounting Standards Codification®](https://asc.fasb.org/)
- [Standards](https://fasb.org/standards)
- [What is the Codification? - FASB Accounting Standards Codification and Additional Resources](https://guides.newman.baruch.cuny.edu/FASBCodificationFall2020)
- [Accounting Standards Codification - Wikipedia](https://en.wikipedia.org/wiki/Accounting_Standards_Codification)
- [ASC Accounting Standards Codification: Complete Guide to FASB's Framework | CPCON](https://cpcongroup.com/insights/article/asc-accounting-standards-codification/)

### FASB ASC Topic 820 (Fair Value)

- [What is ASC 820? A Guide to Fair Value Measurements](https://carta.com/learn/private-funds/management/asc-820/)
- [ASC 820 on fair value measurement](https://fasb.org/page/PageContent?pageId=/projects/recentlycompleted/fair-value-measurement-topic-820-fair-value-measurement-of-equity-securities-subject-to-contractual-sale-restrictions.html)
- [What Is ASC 820? A Guide to Measuring Fair Value | Pulley](https://pulley.com/guides/asc-820)
- [ASC 820 - Wikipedia](https://en.wikipedia.org/wiki/ASC_820)
- [Fair Value Measurements - GAAP Dynamics](https://www.gaapdynamics.com/insights/accounting-topics/fair-value-measurements-accounting-resources-for-asc-820-and-ifrs-13/)

### Historical Cost & Conservatism

- [Historical Cost Principle | Definition + Concept Examples](https://www.wallstreetprep.com/knowledge/historical-cost-principle/)
- [Generally Accepted Accounting Principles (United States) - Wikipedia](https://en.wikipedia.org/wiki/Generally_Accepted_Accounting_Principles_(United_States))
- [Accounting Principles: In-Depth Explanation with Examples | AccountingCoach](https://www.accountingcoach.com/accounting-principles/explanation)
- [Historical Cost Principle: How It Works & Why It Matters | NetSuite](https://www.netsuite.com/portal/resource/articles/accounting/historical-cost.shtml)

### GAAP Assumptions (Economic Entity, Monetary Unit, Going Concern)

- [Accounting Principles: In-Depth Explanation with Examples | AccountingCoach](https://www.accountingcoach.com/accounting-principles/explanation)
- [7 Basic Accounting Principles You Must Know | Pass Accounting Class](https://passaccountingclass.com/7-basic-accounting-principles-to-pass-accounting-101)
- [The 4 Basic Accounting Assumptions | Accountdemy](https://accountdemy.com/4-basic-accounting-assumptions/)
- [What are the key accounting assumptions in financial statements?](https://www.universalcpareview.com/ask-joey/what-are-the-key-accounting-assumptions-in-financial-statements/)
- [What Are Generally Accepted Accounting Principles? | Business.org](https://www.business.org/finance/accounting/what-are-generally-accepted-accounting-principles/)

### ASC Topic 105 (General Principles)

- [Generally Accepted Accounting Principles (Topic 105)](https://viewpoint.pwc.com/dt/us/en/fasb_financial_accou/asus_fulltext/2009/asu_200901generally_/asu_200901generally__US/asu_200901generally__US.html)
- [ASC 105 GENERALLY ACCEPTED ACCOUNTING PRINCIPLES](https://www.researchgate.net/publication/350219797_ASC_105_GENERALLY_ACCEPTED_ACCOUNTING_PRINCIPLES)
- [Codification | DART – Deloitte Accounting Research Tool](https://dart.deloitte.com/USDART/home/codification)

### Double-Entry Bookkeeping

- [Double-entry bookkeeping - Wikipedia](https://en.wikipedia.org/wiki/Double-entry_bookkeeping)
- [What Is Double-Entry Bookkeeping? A Simple Guide for Small Businesses](https://www.freshbooks.com/hub/accounting/double-entry-bookkeeping)
- [Double-Entry Accounting: What It Is and How It Works | Coursera](https://www.coursera.org/articles/double-entry-accounting)
- [What is double-entry bookkeeping? How it works in 2026 | QuickBooks](https://quickbooks.intuit.com/r/bookkeeping/complete-guide-to-double-entry-bookkeeping/)
- [Double-entry bookkeeping: How it works and why your business needs it | Xero US](https://www.xero.com/us/guides/double-entry-bookkeeping/)

### Materiality

- [Minding the Gaps: How to Calculate Materiality Thresholds in Accounting | Numeric](https://www.numeric.io/blog/materiality-threshold)
- [GAAP Materiality: What It Is and How It's Determined - Accounting Insights](https://accountinginsights.org/gaap-materiality-what-it-is-and-how-its-determined/)
- [SEC Staff Accounting Bulletin No. 99: Materiality](https://www.sec.gov/interps/account/sab99.htm)
- [How to Determine Materiality Under GAAP - LegalClarity](https://legalclarity.org/how-to-determine-materiality-under-gaap/)
- [SEC.gov | Assessing Materiality: Focusing on the Reasonable Investor](https://www.sec.gov/newsroom/speeches-statements/munter-statement-assessing-materiality-030922)

---

**Document complete.** Ready for Phase 2: Proposition type definitions.
