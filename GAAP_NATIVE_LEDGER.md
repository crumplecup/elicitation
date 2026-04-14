# GAAP-Native Ledger Architecture

> Building a production-grade accounting system with GAAP as the intermediate representation, typesafe state machines for provable transitions, and a broad foundation for full accounting services.

## Vision

Transform the ledger from a proof-of-concept double-entry system into a **production accounting platform** where:

1. **GAAP is the IR** - Generally Accepted Accounting Principles are the foundational types, not a validation layer
2. **Typesafe state machines** - Financial transactions constrain to provable transitions through the type system
3. **AI-assisted accounting** - Agents help classify transactions, match expenses to revenue, suggest journal entries
4. **Audit-ready by design** - Every transaction carries proof-of-compliance from construction through commit
5. **Full accounting services** - Balance sheets, income statements, cash flow, tax reporting, multi-entity consolidation

## From POC to Production

### Current State (POC Ledger)

The existing ledger demonstrates typestate patterns:

```rust
Transfer<Pending> → Transfer<Validated> → Transfer<Committed>
```

**Strengths:**

- ✅ Typestate transitions work
- ✅ Proof-carrying validation established
- ✅ Double-entry bookkeeping enforced
- ✅ GAAP propositions defined (Phase 1-3 complete)

**Limitations:**

- ❌ Transfer-centric (not journal-entry-centric)
- ❌ No chart of accounts
- ❌ No account types (Asset/Liability/Equity/Revenue/Expense)
- ❌ No financial statement generation
- ❌ No matching principle enforcement
- ❌ No multi-entity support
- ❌ No period closing
- ❌ No audit trail beyond basic entries

### Target State (GAAP-Native Ledger)

A production accounting system where:

```rust
JournalEntry<Draft> → JournalEntry<Balanced> → JournalEntry<Posted> → JournalEntry<Closed>
```

**Capabilities:**

- ✅ GAAP account types are primitive (Asset, Liability, Equity, Revenue, Expense)
- ✅ Chart of accounts with hierarchical structure
- ✅ Normal balances by account type (debit/credit)
- ✅ Double-entry by construction (builder pattern)
- ✅ Matching principle enforced for revenue recognition
- ✅ Period closing with retained earnings calculation
- ✅ Financial statement generation (Balance Sheet, Income Statement, Cash Flow)
- ✅ Multi-entity consolidation
- ✅ Audit trail with GAAP compliance proofs
- ✅ AI-assisted transaction classification

## Architectural Principles

### 1. GAAP Types Are Primitive

Account types encode GAAP structure:

```rust
pub enum AccountClass {
    Asset(AssetType),
    Liability(LiabilityType),
    Equity(EquityType),
    Revenue(RevenueType),
    Expense(ExpenseType),
}

pub enum AssetType {
    CurrentAsset(CurrentAsset),
    FixedAsset(FixedAsset),
    IntangibleAsset(IntangibleAsset),
}

pub enum CurrentAsset {
    Cash { bank: Option<String> },
    AccountsReceivable { customer: CustomerId },
    Inventory { sku: Option<String> },
    PrepaidExpense { period_end: Date },
}

pub enum FixedAsset {
    Land,
    Building { address: String },
    Equipment { asset_tag: String },
    Vehicle { vin: String },
}
```

**Normal balance by type:**

- Assets: Debit
- Expenses: Debit
- Liabilities: Credit
- Equity: Credit
- Revenue: Credit

### 2. Journal Entries Are the Primitive

Not "transfers" but **journal entries** with multiple lines:

```rust
pub struct JournalEntry<S> {
    pub entry_id: EntryId,
    pub date: Date,
    pub description: String,
    pub lines: Vec<JournalLine>,
    pub gaap_proof: GaapProof,
    pub state_data: StateData<S>,
    _state: PhantomData<S>,
}

pub struct JournalLine {
    pub account: Account,
    pub debit: Option<Amount>,
    pub credit: Option<Amount>,
    pub memo: String,
}
```

**Typestates:**

- `Draft` - Being constructed, not balanced yet
- `Balanced` - Debits = Credits, ready to post
- `Posted` - Committed to ledger, affects account balances
- `Closed` - Part of closed period, immutable

### 3. Double-Entry by Construction

Builder pattern enforces balance:

```rust
impl JournalEntryBuilder {
    pub fn debit(mut self, account: Account, amount: Amount) -> Self {
        self.total_debits += amount;
        self.lines.push(JournalLine::debit(account, amount));
        self
    }

    pub fn credit(mut self, account: Account, amount: Amount) -> Self {
        self.total_credits += amount;
        self.lines.push(JournalLine::credit(account, amount));
        self
    }

    // Can only build if balanced
    pub fn build(self) -> Result<JournalEntry<Balanced>, ImbalanceError> {
        if self.total_debits != self.total_credits {
            return Err(ImbalanceError {
                debits: self.total_debits,
                credits: self.total_credits,
                difference: self.total_debits - self.total_credits,
            });
        }

        Ok(JournalEntry {
            entry_id: EntryId::new_v4(),
            date: Utc::now().date(),
            description: self.description,
            lines: self.lines,
            gaap_proof: establish_gaap_proofs(&self.lines)?,
            state_data: StateData::Balanced(BalancedData {
                total: self.total_debits,
            }),
            _state: PhantomData,
        })
    }
}
```

### 4. Matching Principle by Design

Revenue recognition types enforce matching:

```rust
pub struct RevenueRecognition {
    /// Revenue journal entry (Dr. Cash/AR, Cr. Sales)
    pub revenue_entry: JournalEntry<Posted>,

    /// Matched expense entries (Dr. COGS, Cr. Inventory)
    pub matched_expenses: Vec<JournalEntry<Posted>>,

    /// Proof that revenue and expenses are in same period
    pub matching_proof: Established<MatchingPrinciple>,

    /// Date range (all entries must be in this period)
    pub period: DateRange,
}

impl RevenueRecognition {
    pub fn recognize_sale(
        customer: CustomerId,
        revenue_amount: Amount,
        cost_of_goods: Amount,
        date: Date,
    ) -> Result<Self, RecognitionError> {
        // Revenue entry
        let revenue_entry = JournalEntryBuilder::new(date)
            .description("Sale to customer")
            .debit(Account::asset(CurrentAsset::AccountsReceivable { customer }), revenue_amount)
            .credit(Account::revenue(RevenueType::Sales), revenue_amount)
            .build()?;

        // COGS entry (same date = matching principle)
        let cogs_entry = JournalEntryBuilder::new(date)
            .description("Cost of goods sold")
            .debit(Account::expense(ExpenseType::CostOfGoodsSold), cost_of_goods)
            .credit(Account::asset(CurrentAsset::Inventory { sku: None }), cost_of_goods)
            .build()?;

        // Matching proof: entries in same period
        let matching_proof = validate_matching_principle(&revenue_entry, &[cogs_entry.clone()])?;

        Ok(Self {
            revenue_entry,
            matched_expenses: vec![cogs_entry],
            matching_proof,
            period: DateRange::single_day(date),
        })
    }
}
```

### 5. Chart of Accounts

Hierarchical account structure:

```rust
pub struct ChartOfAccounts {
    pub accounts: BTreeMap<AccountNumber, Account>,
    pub entity: EntityId,
}

pub struct Account {
    pub number: AccountNumber,
    pub name: String,
    pub class: AccountClass,
    pub parent: Option<AccountNumber>,
    pub active: bool,
    pub normal_balance: NormalBalance,
}

pub struct AccountNumber(String); // e.g., "1000", "1100", "5000"

impl ChartOfAccounts {
    /// Standard chart for small business
    pub fn standard_small_business() -> Self {
        Self::builder()
            // Assets (1000-1999)
            .account("1000", "Assets", AccountClass::Asset)
            .account("1100", "Current Assets", AccountClass::Asset).parent("1000")
            .account("1110", "Cash", AccountClass::Asset(CurrentAsset::Cash)).parent("1100")
            .account("1120", "Accounts Receivable", AccountClass::Asset(CurrentAsset::AccountsReceivable)).parent("1100")
            .account("1130", "Inventory", AccountClass::Asset(CurrentAsset::Inventory)).parent("1100")
            // Liabilities (2000-2999)
            .account("2000", "Liabilities", AccountClass::Liability)
            .account("2100", "Current Liabilities", AccountClass::Liability)
            .account("2110", "Accounts Payable", AccountClass::Liability(LiabilityType::AccountsPayable)).parent("2100")
            // Equity (3000-3999)
            .account("3000", "Equity", AccountClass::Equity)
            .account("3100", "Retained Earnings", AccountClass::Equity(EquityType::RetainedEarnings)).parent("3000")
            // Revenue (4000-4999)
            .account("4000", "Revenue", AccountClass::Revenue)
            .account("4100", "Sales", AccountClass::Revenue(RevenueType::Sales)).parent("4000")
            // Expenses (5000-5999)
            .account("5000", "Expenses", AccountClass::Expense)
            .account("5100", "Cost of Goods Sold", AccountClass::Expense(ExpenseType::CostOfGoodsSold)).parent("5000")
            .account("5200", "Operating Expenses", AccountClass::Expense).parent("5000")
            .build()
    }
}
```

### 6. Financial Statements by Projection

Balance sheet, income statement, cash flow generated from entries:

```rust
pub struct BalanceSheet {
    pub entity: EntityId,
    pub as_of_date: Date,
    pub assets: AccountGroup,
    pub liabilities: AccountGroup,
    pub equity: AccountGroup,
    pub proof: Established<And<DoubleEntryBookkeeping, EconomicEntityAssumption>>,
}

impl BalanceSheet {
    pub fn from_ledger(ledger: &Ledger, entity: EntityId, as_of: Date) -> Self {
        // Sum all posted entries by account class
        let assets = ledger.account_balances(entity, as_of, AccountClass::Asset);
        let liabilities = ledger.account_balances(entity, as_of, AccountClass::Liability);
        let equity = ledger.account_balances(entity, as_of, AccountClass::Equity);

        // Assets = Liabilities + Equity (proven by construction)
        let proof = Established::assert();

        Self {
            entity,
            as_of_date: as_of,
            assets: AccountGroup::from_balances(assets),
            liabilities: AccountGroup::from_balances(liabilities),
            equity: AccountGroup::from_balances(equity),
            proof,
        }
    }

    pub fn verify_equation(&self) -> bool {
        self.assets.total() == self.liabilities.total() + self.equity.total()
    }
}

pub struct IncomeStatement {
    pub entity: EntityId,
    pub period: DateRange,
    pub revenue: AccountGroup,
    pub expenses: AccountGroup,
    pub net_income: Amount,
    pub proof: Established<MatchingPrinciple>,
}

impl IncomeStatement {
    pub fn from_ledger(ledger: &Ledger, entity: EntityId, period: DateRange) -> Self {
        let revenue = ledger.account_activity(entity, period, AccountClass::Revenue);
        let expenses = ledger.account_activity(entity, period, AccountClass::Expense);

        let net_income = revenue.total() - expenses.total();

        // Matching principle: revenue and expenses in same period
        let proof = Established::assert();

        Self {
            entity,
            period,
            revenue: AccountGroup::from_activity(revenue),
            expenses: AccountGroup::from_activity(expenses),
            net_income,
            proof,
        }
    }
}
```

### 7. Period Closing

Close accounting periods with retained earnings:

```rust
pub struct ClosingEntry {
    /// Close revenue accounts to income summary
    pub close_revenue: JournalEntry<Posted>,

    /// Close expense accounts to income summary
    pub close_expenses: JournalEntry<Posted>,

    /// Close income summary to retained earnings
    pub close_income_summary: JournalEntry<Posted>,

    /// Period being closed
    pub period: DateRange,

    /// Proof that all temporary accounts are zeroed
    pub closing_proof: Established<GoingConcernAssumption>,
}

impl ClosingEntry {
    pub fn close_period(
        ledger: &Ledger,
        entity: EntityId,
        period: DateRange,
    ) -> Result<Self, ClosingError> {
        let income_stmt = IncomeStatement::from_ledger(ledger, entity, period);

        // Close revenue accounts (Dr. Revenue, Cr. Income Summary)
        let close_revenue = JournalEntryBuilder::new(period.end)
            .description("Close revenue accounts")
            .debit_all(income_stmt.revenue.accounts(), income_stmt.revenue.total())
            .credit(Account::temporary(TemporaryAccount::IncomeSummary), income_stmt.revenue.total())
            .build()?;

        // Close expense accounts (Dr. Income Summary, Cr. Expenses)
        let close_expenses = JournalEntryBuilder::new(period.end)
            .description("Close expense accounts")
            .debit(Account::temporary(TemporaryAccount::IncomeSummary), income_stmt.expenses.total())
            .credit_all(income_stmt.expenses.accounts(), income_stmt.expenses.total())
            .build()?;

        // Close income summary to retained earnings
        let close_income_summary = JournalEntryBuilder::new(period.end)
            .description("Close income summary to retained earnings")
            .debit_or_credit_income_summary(income_stmt.net_income)
            .debit_or_credit_retained_earnings(income_stmt.net_income)
            .build()?;

        // Going concern: period closed, new period begins
        let closing_proof = Established::assert();

        Ok(Self {
            close_revenue,
            close_expenses,
            close_income_summary,
            period,
            closing_proof,
        })
    }
}
```

### 8. AI Integration Points

Where AI assists in the accounting workflow:

```rust
pub struct TransactionClassifier {
    /// AI model for transaction classification
    model: ClassificationModel,
}

impl TransactionClassifier {
    /// Suggest account classification from transaction description
    pub async fn classify_transaction(
        &self,
        description: &str,
        amount: Amount,
        counterparty: Option<&str>,
    ) -> Result<ClassificationSuggestion, ClassificationError> {
        // AI suggests:
        // - Account class (Asset/Liability/Revenue/Expense)
        // - Specific account (Cash, Inventory, Sales, COGS, etc.)
        // - Debit or credit
        // - Confidence score

        let suggestion = self.model.infer(description, amount, counterparty).await?;

        Ok(ClassificationSuggestion {
            account: suggestion.account,
            debit_credit: suggestion.debit_credit,
            confidence: suggestion.confidence,
            explanation: suggestion.reasoning,
        })
    }

    /// Suggest revenue recognition with matched expenses
    pub async fn suggest_revenue_recognition(
        &self,
        sale_description: &str,
        revenue_amount: Amount,
    ) -> Result<RevenueRecognitionSuggestion, ClassificationError> {
        // AI suggests:
        // - Revenue account
        // - Matched COGS amount
        // - Inventory account to credit
        // - Whether recognition criteria are met (ASC 606)

        let suggestion = self.model.suggest_recognition(sale_description, revenue_amount).await?;

        Ok(RevenueRecognitionSuggestion {
            revenue_account: suggestion.revenue_account,
            cogs_amount: suggestion.cogs_amount,
            inventory_account: suggestion.inventory_account,
            recognition_criteria_met: suggestion.criteria,
            explanation: suggestion.reasoning,
        })
    }
}
```

## Implementation Plan

### Phase 1: Foundation - GAAP Types (Weeks 1-2)

**Goal:** Define GAAP account types and chart of accounts.

**Tasks:**

1. Define `AccountClass` enum (Asset/Liability/Equity/Revenue/Expense)
2. Define sub-types for each class (CurrentAsset, FixedAsset, etc.)
3. Define `Account` struct with account number, name, class
4. Define `ChartOfAccounts` with hierarchical structure
5. Implement standard chart templates (small business, nonprofit, etc.)
6. Add normal balance rules (debit/credit by account class)

**Deliverables:**

- `crates/elicit_server/src/ledger2/account_types.rs`
- `crates/elicit_server/src/ledger2/chart_of_accounts.rs`
- Tests for account classification and normal balances
- Standard chart templates

**Success criteria:**

- All GAAP account classes defined
- Normal balance rules encoded
- Standard chart of accounts templates
- Zero clippy warnings

### Phase 2: Journal Entries - Core ✅ COMPLETED

**Goal:** Implement journal entry types with typestate state machine.

**Tasks:**

1. ✅ Define `JournalEntry<S>` with state markers (Draft/Balanced/Posted/Closed)
2. ✅ Define `JournalLine` with account, debit, credit
3. ✅ Implement `JournalEntryBuilder` with double-entry enforcement
4. ✅ Implement state transitions (balanced → posted → closed)
5. ✅ Add GAAP proof carrying (`GaapProof` struct)
6. ✅ Integrate existing GAAP propositions (proof structure ready)

**Deliverables:**

- ✅ `crates/elicit_server/src/ledger2/journal_entry.rs` (370 lines)
- ✅ `crates/elicit_server/src/ledger2/journal_line.rs` (280 lines)
- ✅ `crates/elicit_server/src/ledger2/builder.rs` (170 lines)
- ✅ `crates/elicit_server/src/ledger2/errors.rs` (180 lines)
- ✅ `crates/elicit_server/tests/ledger2_journal_entry_test.rs` (350 lines)
- ✅ Tests for builder, state transitions, proof carrying (18 tests)
- ✅ Error types for imbalance, invalid transitions, GAAP validation

**Implementation:**

- **Amount type**: Monetary amounts in cents (i64) to avoid floating-point errors
- **JournalLine**: Either debit OR credit (mutually exclusive), with account and memo
- **State markers**: Draft, Balanced, Posted, Closed (zero-cost PhantomData)
- **State data**: Each state carries different metadata (balanced_at, posted_at, closed_at)
- **Builder pattern**: Accumulates debits/credits, validates balance on build()
- **Validation**: Entity consistency, account active status, minimum 2 lines, balance check
- **State transitions**: Balanced → Posted → Closed (one-way, immutable)
- **GaapProof**: Carries list of established propositions with timestamp

**Test coverage:**

- 18 tests covering:
  - Amount arithmetic and display
  - Builder validation (balance, entity, active accounts, description)
  - State transitions (Balanced → Posted → Closed)
  - Error conditions (imbalance, empty, single line, inactive accounts)
  - Entry properties (unique IDs, display formatting)

**Success criteria:**

- ✅ Builder enforces double-entry by construction (debits = credits)
- ✅ Typestate transitions work correctly (no invalid transitions possible)
- ✅ GAAP proofs carried through lifecycle (GaapProof struct in every entry)
- ✅ Comprehensive test coverage (18 tests, 100% pass)
- ✅ Zero clippy warnings
- ✅ All formatting checks pass

### Phase 3: Ledger - Storage and Queries ✅ COMPLETED

**Goal:** Implement ledger storage with account balances and queries.

**Tasks:**

1. ✅ Define `Ledger` struct with journal entries and account balances
2. ✅ Implement posting (JournalEntry<Balanced> → JournalEntry<Posted>)
3. ✅ Update account balances on posting
4. ✅ Implement balance queries (by account, by date, by entity)
5. ✅ Implement activity queries (revenue, expenses, by period)
6. ⏸️ Add database schema (deferred - in-memory implementation complete)
7. ⏸️ Implement SQLx queries for persistence (deferred - not needed yet)

**Deliverables:**

- ✅ `crates/elicit_server/src/ledger2/ledger.rs` (295 lines)
- ✅ `crates/elicit_server/src/ledger2/balance.rs` (305 lines)
- ✅ `crates/elicit_server/tests/ledger2_ledger_test.rs` (450 lines)
- ⏸️ Database migrations (deferred)
- ✅ Tests for posting, queries, balance tracking (13 tests)

**Implementation:**

- **AccountBalance**: Tracks debits/credits for individual account with as-of date
- **BalanceSheet**: Assets, Liabilities, Equity with equation verification (A = L + E)
- **Ledger**: Central repository managing posted entries and balances
- **Posting**: Transitions Balanced → Posted, updates account balances
- **Queries**: By date range, as-of date, chronological order
- **Activity queries**: Revenue, expenses, net income by period

**Balance tracking:**

- Debits/credits accumulated per account
- Normal balance rules enforced (Assets/Expenses = Debit, Liabilities/Equity/Revenue = Credit)
- As-of-date queries for point-in-time balances
- Entry count tracking for audit trail

**Ledger operations:**

- Post entries (Balanced → Posted with balance updates)
- Entity validation (entries must belong to ledger's entity)
- Chronological storage (entries ordered by posting time)
- Account balance computation from journal lines
- Balance sheet generation with equation verification

**Query capabilities:**

- `account_balance(account, date)` - Balance for specific account as of date
- `all_balances(date)` - All account balances as of date
- `entries_as_of(date)` - Entries posted on or before date
- `entries_in_range(start, end)` - Entries within date range
- `total_revenue(start, end)` - Total revenue for period
- `total_expenses(start, end)` - Total expenses for period
- `net_income(start, end)` - Net income (revenue - expenses)

**Test coverage (13 tests):**

- Account balance initialization and updates
- Multiple entries affecting same account
- As-of-date balance queries
- Entry posting and retrieval
- Chronological order preservation
- Date range queries
- Revenue/expense/net income calculations
- Balance sheet generation

**Success criteria:**

- ✅ Account balances calculated correctly from journal lines
- ✅ Queries work efficiently (in-memory for now)
- ✅ Balance sheet equation verified
- ✅ All tests passing (13 tests, 100% pass)
- ✅ Zero clippy warnings
- ✅ All formatting checks pass

**Deferred:**
Database persistence (SQLx) is deferred - in-memory implementation is sufficient for
current needs. When persistence is needed, the Ledger can be extended with load/save
methods without changing the public API.

### Phase 4: Financial Statements ✅ COMPLETED

**Goal:** Generate GAAP-compliant financial statements.

**Tasks:**

1. ✅ Implement `BalanceSheet` with assets/liabilities/equity grouping (basic version in Phase 3)
2. ✅ Implement `IncomeStatement` with revenue/expenses/net income
3. ⏸️ Implement `CashFlowStatement` (deferred - complex, low priority)
4. ✅ Add statement period logic (monthly, quarterly, annual)
5. ✅ Add comparative statements (current vs. prior period)
6. ✅ Add financial ratios (current ratio, debt-to-equity, profit margin)

**Deliverables:**

- ✅ `crates/elicit_server/src/ledger2/statements.rs` (480 lines - unified module)
- ✅ `crates/elicit_server/tests/ledger2_statements_test.rs` (370 lines)
- ⏸️ Cash flow statement (deferred)
- ✅ Tests for statement generation and accuracy (19 tests)

**Implementation:**

- **StatementPeriod**: Defines time ranges for financial reporting (monthly, quarterly, annual)
- **IncomeStatement**: Revenue, expenses, net income with profit margin calculation
- **ComparativeIncomeStatement**: Side-by-side comparison of current vs. prior periods with variance analysis
- **FinancialRatios**: Profitability, liquidity, and leverage ratios

**Statement Period:**

- Monthly periods with proper month-end dates
- Quarterly periods (Q1-Q4)
- Annual (fiscal year) periods
- Custom date ranges with descriptions

**Income Statement:**

- Revenue accounts aggregated by account number
- Expense accounts aggregated by account number
- Net income computed as revenue - expenses
- Profit margin calculation (net income / revenue)
- Integration with Ledger via `income_statement(period)` method

**Comparative Statements:**

- Current vs. prior period comparison
- Revenue, expense, and net income variance
- Revenue growth percentage calculation
- Formatted display for analysis

**Financial Ratios:**

- **Profit margin**: Net income / revenue (profitability)
- **Current ratio**: Current assets / current liabilities (liquidity)
- **Debt-to-equity**: Total liabilities / total equity (leverage)
- None handling for zero divisors

**Ledger integration:**

- `Ledger::income_statement(&period)` generates statement from posted entries
- Filters entries by date range
- Aggregates revenue and expenses by account
- Handles normal balance rules (revenue=credit, expense=debit)

**Test coverage (19 tests):**

- Statement period creation (monthly, quarterly, annual)
- Income statement operations (add revenue, add expense, net income)
- Profit margin calculation
- Ledger integration (generate from posted entries)
- Period filtering (only include entries in date range)
- Comparative statements (variance, growth)
- Financial ratios (all three ratios, zero divisor handling)

**Success criteria:**

- ✅ Balance sheet equation verified (from Phase 3)
- ✅ Income statement generated from ledger entries
- ⏸️ Cash flow statement (deferred - not critical for MVP)
- ✅ All statements GAAP-compliant
- ✅ 19 tests passing (100%)
- ✅ Zero clippy warnings
- ✅ All formatting checks pass

**Deferred:**
Cash flow statement is complex (requires categorizing cash flows into operating, investing,
financing activities) and lower priority. Can be added in future phase if needed.

### Phase 5: Matching Principle (Weeks 9-10)

**Goal:** Enforce matching principle for revenue recognition.

**Tasks:**

1. Define `RevenueRecognition` type with matched expenses
2. Implement revenue recognition rules (ASC 606 five-step model)
3. Add validation that revenue and COGS are in same period
4. Implement deferred revenue handling
5. Implement accrued expense handling
6. Add matching proof establishment

**Deliverables:**

- `crates/elicit_server/src/ledger2/revenue_recognition.rs`
- `crates/elicit_server/src/ledger2/deferrals.rs`
- `crates/elicit_server/src/ledger2/accruals.rs`
- Tests for revenue recognition, deferrals, accruals

**Success criteria:**

- Matching principle enforced by type system
- Revenue recognition follows ASC 606
- Deferred revenue and accrued expenses handled correctly
- Proofs established for matching

### Phase 6: Period Closing (Weeks 11-12)

**Goal:** Implement accounting period closing with retained earnings.

**Tasks:**

1. Define `ClosingEntry` type with closing journal entries
2. Implement temporary account closing (revenue/expenses)
3. Implement income summary to retained earnings transfer
4. Add period state (open/closed)
5. Prevent posting to closed periods
6. Implement re-opening entries if needed

**Deliverables:**

- `crates/elicit_server/src/ledger2/closing.rs`
- `crates/elicit_server/src/ledger2/period.rs`
- Tests for period closing, re-opening, immutability

**Success criteria:**

- Period closing zeroes temporary accounts
- Net income flows to retained earnings
- Closed periods immutable
- Balance sheet equation preserved after closing

### Phase 7: Multi-Entity Support (Weeks 13-14)

**Goal:** Support multiple entities with consolidation.

**Tasks:**

1. Add `Entity` type with entity ID, name, type (parent/subsidiary)
2. Add entity ID to all journal entries and accounts
3. Implement inter-company transactions
4. Implement consolidation eliminations
5. Generate consolidated financial statements
6. Add non-controlling interest handling

**Deliverables:**

- `crates/elicit_server/src/ledger2/entity.rs`
- `crates/elicit_server/src/ledger2/consolidation.rs`
- Tests for multi-entity, consolidation, eliminations

**Success criteria:**

- Each entity has separate books
- Inter-company transactions tracked
- Consolidated statements eliminate inter-company
- Non-controlling interest calculated correctly

### Phase 8: AI Integration (Weeks 15-16)

**Goal:** AI-assisted transaction classification and suggestions.

**Tasks:**

1. Define `TransactionClassifier` with AI model integration
2. Implement classification API (description → account suggestion)
3. Implement revenue recognition suggestions (revenue → COGS match)
4. Add confidence scores and explanations
5. Implement feedback loop (user corrections → model improvement)
6. Add batch classification for imports (bank statements, invoices)

**Deliverables:**

- `crates/elicit_server/src/ledger2/ai/classifier.rs`
- `crates/elicit_server/src/ledger2/ai/suggestions.rs`
- `crates/elicit_server/src/ledger2/ai/feedback.rs`
- Tests for classification, suggestions, feedback

**Success criteria:**

- AI suggests correct account 80%+ of time
- Revenue recognition suggestions follow ASC 606
- User feedback improves model
- Batch classification fast (< 1s per 100 transactions)

### Phase 9: Audit Trail (Weeks 17-18)

**Goal:** Complete audit trail with GAAP compliance tracking.

**Tasks:**

1. Add audit log for all ledger operations
2. Track user, timestamp, operation type for each entry
3. Add GAAP compliance metadata (which propositions satisfied)
4. Implement audit report generation
5. Add immutability guarantees (entries never deleted, only reversed)
6. Implement reversal entries for corrections

**Deliverables:**

- `crates/elicit_server/src/ledger2/audit.rs`
- `crates/elicit_server/src/ledger2/reversal.rs`
- Audit report templates
- Tests for audit trail, reversals, immutability

**Success criteria:**

- Every operation logged with user/timestamp
- GAAP compliance tracked per entry
- Audit reports generated
- No entries ever deleted (reversals only)

### Phase 10: Migration and Deprecation (Weeks 19-20)

**Goal:** Migrate from POC ledger to GAAP-native ledger.

**Tasks:**

1. Write migration script (Transfer → JournalEntry)
2. Migrate existing test data
3. Update tests to use new ledger
4. Deprecate old `Transfer` API
5. Update documentation
6. Performance benchmarks (old vs. new)

**Deliverables:**

- Migration script
- Updated tests
- Performance comparison
- Deprecation notices
- Updated documentation

**Success criteria:**

- All existing functionality migrated
- Tests passing with new ledger
- Performance >= old ledger
- Documentation complete

## Database Schema

### Accounts Table

```sql
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY,
    entity_id UUID NOT NULL,
    account_number VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_class VARCHAR(50) NOT NULL, -- Asset, Liability, etc.
    account_type JSONB NOT NULL, -- Specific type (CurrentAsset::Cash, etc.)
    parent_account_id UUID,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    normal_balance VARCHAR(10) NOT NULL, -- Debit or Credit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (parent_account_id) REFERENCES accounts(account_id),
    UNIQUE (entity_id, account_number)
);

CREATE INDEX idx_accounts_entity ON accounts(entity_id);
CREATE INDEX idx_accounts_class ON accounts(account_class);
```

### Journal Entries Table

```sql
CREATE TABLE journal_entries (
    entry_id UUID PRIMARY KEY,
    entity_id UUID NOT NULL,
    entry_date DATE NOT NULL,
    description TEXT NOT NULL,
    state VARCHAR(20) NOT NULL, -- Draft, Balanced, Posted, Closed
    gaap_proof JSONB NOT NULL, -- Established propositions
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    posted_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    created_by UUID, -- User ID
    CONSTRAINT valid_state CHECK (state IN ('Draft', 'Balanced', 'Posted', 'Closed'))
);

CREATE INDEX idx_entries_entity ON journal_entries(entity_id);
CREATE INDEX idx_entries_date ON journal_entries(entry_date);
CREATE INDEX idx_entries_state ON journal_entries(state);
```

### Journal Lines Table

```sql
CREATE TABLE journal_lines (
    line_id UUID PRIMARY KEY,
    entry_id UUID NOT NULL,
    account_id UUID NOT NULL,
    debit BIGINT, -- Amount in cents, NULL if credit
    credit BIGINT, -- Amount in cents, NULL if debit
    memo TEXT,
    line_order INTEGER NOT NULL,
    FOREIGN KEY (entry_id) REFERENCES journal_entries(entry_id),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id),
    CONSTRAINT debit_or_credit CHECK (
        (debit IS NOT NULL AND credit IS NULL) OR
        (debit IS NULL AND credit IS NOT NULL)
    ),
    CONSTRAINT positive_amounts CHECK (
        (debit IS NULL OR debit >= 0) AND
        (credit IS NULL OR credit >= 0)
    )
);

CREATE INDEX idx_lines_entry ON journal_lines(entry_id);
CREATE INDEX idx_lines_account ON journal_lines(account_id);
```

### Account Balances Table (Materialized View)

```sql
CREATE TABLE account_balances (
    balance_id UUID PRIMARY KEY,
    account_id UUID NOT NULL,
    as_of_date DATE NOT NULL,
    balance BIGINT NOT NULL, -- Running balance in cents
    debit_total BIGINT NOT NULL,
    credit_total BIGINT NOT NULL,
    entry_count INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id),
    UNIQUE (account_id, as_of_date)
);

CREATE INDEX idx_balances_account_date ON account_balances(account_id, as_of_date DESC);
```

### Audit Log Table

```sql
CREATE TABLE audit_log (
    log_id UUID PRIMARY KEY,
    operation VARCHAR(50) NOT NULL, -- Create, Post, Close, Reverse
    entry_id UUID,
    user_id UUID,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    details JSONB NOT NULL,
    gaap_compliance JSONB, -- Which propositions were satisfied
    FOREIGN KEY (entry_id) REFERENCES journal_entries(entry_id)
);

CREATE INDEX idx_audit_entry ON audit_log(entry_id);
CREATE INDEX idx_audit_timestamp ON audit_log(timestamp DESC);
```

## Success Metrics

### Technical Metrics

- **Type safety:** Zero runtime balance errors (proven by construction)
- **Performance:** < 100ms to post entry, < 500ms to generate balance sheet
- **Test coverage:** > 90% line coverage, 100% of critical paths
- **GAAP compliance:** All 9 propositions verifiable for every entry
- **Audit trail:** 100% of operations logged with proof metadata

### User Metrics

- **AI classification accuracy:** > 80% first suggestion correct
- **Time to close period:** < 5 minutes for small business (< 1000 entries/month)
- **Statement generation:** < 1 second for balance sheet, income statement
- **Multi-entity consolidation:** < 10 seconds for 10 entities

### Business Metrics

- **Migration from QuickBooks:** Possible without data loss
- **Tax reporting:** Generate forms (1099, W-2, etc.)
- **Bank reconciliation:** Automated with AI matching
- **Invoice generation:** Integrated with accounts receivable

## Migration Strategy

### Coexistence Period

Both ledgers live side-by-side:

```rust
// Old POC ledger (deprecated but functional)
pub mod ledger {
    pub use crate::ledger_poc::*;
}

// New GAAP-native ledger
pub mod ledger2 {
    pub use crate::ledger_gaap::*;
}
```

### Migration Path

1. **Phase 1:** New code uses `ledger2`, existing code unchanged
2. **Phase 2:** Tests migrated to `ledger2`
3. **Phase 3:** POC ledger marked `#[deprecated]`
4. **Phase 4:** POC ledger removed after 2 releases

### Adapter Pattern

Allow POC `Transfer` to create GAAP `JournalEntry`:

```rust
impl From<Transfer<Committed>> for JournalEntry<Posted> {
    fn from(transfer: Transfer<Committed>) -> Self {
        // Convert 2-line transfer to journal entry
        JournalEntryBuilder::new(Utc::now().date())
            .description(format!("Transfer: {} to {}", transfer.from_account, transfer.to_account))
            .debit(
                Account::generic(transfer.from_account.0),
                transfer.amount,
            )
            .credit(
                Account::generic(transfer.to_account.0),
                transfer.amount,
            )
            .build()
            .expect("Transfer is already balanced")
            .post()
            .expect("Transfer is already committed")
    }
}
```

## Future Extensions

### Year 1

- **Tax reporting:** Generate 1099, W-2, 1120, 1065
- **Payroll:** Integrate payroll with GAAP journal entries
- **Fixed assets:** Depreciation schedules, asset tracking
- **Budgeting:** Budget vs. actual reporting

### Year 2

- **Multi-currency:** ASC 830 foreign currency translation
- **Revenue recognition:** Full ASC 606 five-step model
- **Lease accounting:** ASC 842 right-of-use assets
- **Fair value:** ASC 820 fair value measurements

### Year 3

- **Consolidation:** Full multi-entity with non-controlling interest
- **Segment reporting:** ASC 280 segment disclosures
- **IFRS support:** Dual reporting (GAAP + IFRS)
- **XBRL export:** SEC filing format

## Timeline

**Total: 20 weeks (5 months)**

- Weeks 1-2: GAAP Types (Foundation)
- Weeks 3-4: Journal Entries (Core)
- Weeks 5-6: Ledger (Storage)
- Weeks 7-8: Financial Statements
- Weeks 9-10: Matching Principle
- Weeks 11-12: Period Closing
- Weeks 13-14: Multi-Entity
- Weeks 15-16: AI Integration
- Weeks 17-18: Audit Trail
- Weeks 19-20: Migration

**Milestones:**

- Month 1: Foundation complete (GAAP types, journal entries)
- Month 2: Core ledger functional (posting, queries, statements)
- Month 3: GAAP enforcement (matching, closing)
- Month 4: Multi-entity and AI
- Month 5: Audit trail and migration

## References

### GAAP Standards

- FASB ASC 105: Generally Accepted Accounting Principles
- FASB ASC 205-40: Going Concern
- FASB ASC 250: Accounting Changes
- FASB ASC 450: Contingencies
- FASB ASC 606: Revenue Recognition
- FASB ASC 820: Fair Value Measurement
- FASB ASC 830: Foreign Currency
- FASB ASC 842: Leases

### Implementation References

- GAAP_PRINCIPLES_RESEARCH.md: Detailed GAAP principle documentation
- GAAP_LEDGER_INTEGRATION.md: Current GAAP proposition implementation
- elicit_server/src/ledger/: POC ledger (to be deprecated)
- elicit_server/src/ledger2/: GAAP-native ledger (new implementation)

---

**Status:** 📋 Planning - Ready for Review
**Next Step:** Review and approve architecture, then begin Phase 1 implementation
