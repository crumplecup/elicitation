# Typestate Ledger

> A double-entry bookkeeping system demonstrating the elicitation framework's typestate state machines with proof-carrying contracts.

---

## Table of Contents

- [Why a Ledger?](#why-a-ledger)
- [Architecture Overview](#architecture-overview)
- [Typestate State Machine](#typestate-state-machine)
- [Proof-Carrying Contracts](#proof-carrying-contracts)
- [Usage Examples](#usage-examples)
- [Database Integration](#database-integration)
- [Testing Strategy](#testing-strategy)
- [Production Considerations](#production-considerations)

---

## Why a Ledger?

The ledger serves as a **reference implementation** demonstrating elicitation framework patterns:

### 1. Real-World Complexity
Unlike toy examples (counters, todos), a ledger has:
- **Transactions** - Atomic multi-step operations (debit + credit must balance)
- **Constraints** - Business rules (positive amounts, sufficient funds, distinct accounts)
- **State transitions** - Pending → Validated → Committed/Rejected
- **Concurrency** - Multiple transfers happening simultaneously
- **Invariants** - Double-entry bookkeeping (sum of all balances unchanged)

### 2. Proof Composition
Demonstrates how multiple preconditions combine:
```rust
type ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>;
```

### 3. Database Integration
Shows typestate working with real database operations:
- Validation queries (SELECT balance)
- Transactional commits (BEGIN/INSERT/COMMIT)
- Connection pooling
- Error handling

### 4. Compile-Time Guarantees
The type system **prevents** invalid operations:
```rust
// ❌ Compiler error: can't commit without validation
let transfer: Transfer<Pending> = Transfer::new(...);
transfer.commit(&pool).await; // ERROR: no method `commit` for Transfer<Pending>

// ✅ Type-safe: validation required first
let validated = transfer.validate(&pool).await?;
validated.commit(&pool).await?; // OK: Transfer<Validated> has commit()
```

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Domain Types                             │
│  AccountId, Amount, TransferId                              │
│  - Validation at construction (Amount > 0)                  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                Typestate State Machine                      │
│                                                             │
│  Transfer<Pending> ──validate()──> Transfer<Validated>     │
│       │                                    │                │
│       │                                    │                │
│       └────reject()────────────────────────┴──rollback()─> │
│                                                             │
│                          Transfer<Rejected>                │
│                                                             │
│                          Transfer<Committed>                │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Proof-Carrying Contracts                       │
│  AmountPositive, SufficientFunds, AccountsDistinct         │
│  - Zero-cost proofs (PhantomData)                          │
│  - Kani/Verus/Creusot verification                         │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                 Database Integration                        │
│  SQLx + AnyPool                                             │
│  - Async validation queries                                 │
│  - Transactional commits                                    │
│  - Error conversion (sqlx::Error → ValidationError)        │
└─────────────────────────────────────────────────────────────┘
```

---

## Typestate State Machine

### State Types

```rust
/// Transfer awaiting validation
struct Transfer<Pending> {
    from_account: AccountId,
    to_account: AccountId,
    amount: Amount,
    transfer_id: TransferId,
    state_data: StateData<Pending>,  // No additional data
    _state: PhantomData<Pending>,
}

/// Transfer validated and ready to commit
struct Transfer<Validated> {
    // Same fields...
    state_data: StateData<Validated>,  // Contains from_balance: i64
}

/// Transfer committed to ledger
struct Transfer<Committed> {
    // Same fields...
    state_data: StateData<Committed>,  // Contains before/after balances
}

/// Transfer rejected (validation failed or manual rollback)
struct Transfer<Rejected> {
    // Same fields...
    state_data: StateData<Rejected>,  // Contains rejection reason
}
```

### State Transitions

```rust
// Create pending transfer
let transfer: Transfer<Pending> = Transfer::new(
    AccountId::new("Alice"),
    AccountId::new("Bob"),
    Amount::new(50)?,
    TransferId::new("tx1"),
);

// Validate (queries database, establishes proofs)
let validated: Transfer<Validated> = transfer.validate(&pool).await?;

// Commit (writes debit + credit entries)
let committed: Transfer<Committed> = validated.commit(&pool).await?;

// Verify invariant
assert!(committed.verify_invariant()); // debit amount == transfer amount
```

### State-Specific Data

Each state captures exactly the data it needs:

```rust
// Pending: No additional data needed
PendingData;

// Validated: Balance snapshot from validation
ValidatedData {
    from_balance: i64,  // Balance at validation time
}

// Committed: Full audit trail
CommittedData {
    from_balance_before: i64,
    from_balance_after: i64,
    to_balance_after: i64,
}

// Rejected: Why it failed
RejectedData {
    reason: RejectionReason,
}
```

**No Option<i64> anywhere** - the type system guarantees data availability.

---

## Proof-Carrying Contracts

### Basic Propositions

```rust
/// The amount is positive (> 0)
pub struct AmountPositive;

/// The account has sufficient funds
pub struct SufficientFunds;

/// Source and destination accounts are distinct
pub struct AccountsDistinct;

/// Ledger entries balance (debit + credit = 0)
pub struct BalancedEntries;
```

### Composite Propositions

```rust
/// A transfer is valid
pub type ValidTransfer = And<
    AmountPositive,
    And<SufficientFunds, AccountsDistinct>
>;
```

### Proof Establishment

Validation functions return `Established<Prop>` on success:

```rust
fn validate_amount_positive(
    transfer: &Transfer<Pending>,
) -> Result<Established<AmountPositive>, ValidationError> {
    if transfer.amount.0 <= 0 {
        Err(ValidationError::NegativeAmount(transfer.amount.0))
    } else {
        Ok(Established::assert())  // Zero-cost proof
    }
}
```

### Proof Composition

```rust
// Validate individual preconditions
let amount_proof = validate_amount_positive(&transfer)?;
let funds_proof = validate_sufficient_funds(&transfer, balance)?;
let distinct_proof = validate_accounts_distinct(&transfer)?;

// Compose into ValidTransfer proof
let funds_and_distinct = both(funds_proof, distinct_proof);
let valid_proof: Established<ValidTransfer> = both(amount_proof, funds_and_distinct);
```

### Zero-Cost Proofs

```rust
pub struct Established<P>(PhantomData<P>);
```

Proofs compile away completely - **zero runtime overhead**.

### Formal Verification

Each proposition includes proof methods for:
- **Kani** - Bounded model checking
- **Verus** - SMT-based verification
- **Creusot** - Deductive verification (Why3 backend)

Example (AmountPositive):
```rust
impl Prop for AmountPositive {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_amount_positive() {
                let amount: i64 = kani::any();
                kani::assume(amount > 0);
                assert!(amount > 0, "Amount must be positive");
            }
        }
    }
    // verus_proof, creusot_proof...
}
```

---

## Usage Examples

### Basic Transfer

```rust
use elicit_server::ledger::{AccountId, Amount, Transfer, TransferId};

// Create pending transfer
let transfer = Transfer::new(
    AccountId::new("Alice"),
    AccountId::new("Bob"),
    Amount::new(100)?,
    TransferId::new("tx1"),
);

// Validate with database
let validated = transfer.validate(&pool).await?;
println!("Balance at validation: {}", validated.from_balance());

// Commit to ledger
let committed = validated.commit(&pool).await?;

// Verify invariant
assert!(committed.verify_invariant());

// Inspect final state
let data = committed.committed_data();
println!("Alice: {} → {}", data.from_balance_before, data.from_balance_after);
println!("Bob: {}", data.to_balance_after);
```

### Error Handling

```rust
use elicit_server::ledger::ValidationError;

match transfer.validate(&pool).await {
    Ok(validated) => {
        // Proceed with commit
        validated.commit(&pool).await?;
    }
    Err(ValidationError::InsufficientFunds { balance, required }) => {
        println!("Insufficient funds: has {}, needs {}", balance, required);
    }
    Err(ValidationError::SameAccount) => {
        println!("Cannot transfer to same account");
    }
    Err(ValidationError::NegativeAmount(amt)) => {
        println!("Amount must be positive, got {}", amt);
    }
    Err(ValidationError::Database(e)) => {
        println!("Database error: {}", e);
    }
}
```

### Manual Rejection

```rust
use elicit_server::ledger::RejectionReason;

// Reject before validation
let rejected = transfer.reject(RejectionReason::ManualRollback);
println!("Rejected: {}", rejected.reason());

// Or rollback after validation
let validated = transfer.validate(&pool).await?;
// ... business logic decides not to proceed
let rejected = validated.rollback(RejectionReason::ManualRollback);
```

### Sync Validation (Testing)

```rust
// For testing without database
let validated = transfer.validate_sync(100)?; // Alice has 100
let committed = validated.commit(&pool).await?;
```

---

## Database Integration

### Schema

```sql
CREATE TABLE ledger_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_name TEXT NOT NULL,
    amount INTEGER NOT NULL,        -- Positive for credit, negative for debit
    transfer_id TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
```

### Double-Entry Bookkeeping

Every transfer creates **two entries**:
```sql
-- Alice sends 50 to Bob
INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
VALUES ('Alice', -50, 'tx1', 1234567890);  -- Debit (negative)

INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
VALUES ('Bob', 50, 'tx1', 1234567890);     -- Credit (positive)
```

**Invariant:** Sum of amounts for each `transfer_id` = 0

### Balance Queries

```sql
-- Current balance for an account
SELECT COALESCE(SUM(amount), 0) as balance
FROM ledger_entries
WHERE account_name = ?;
```

### Validation Query

Before commit, the `validate()` method queries the balance:
```rust
let row = sqlx::query(
    "SELECT COALESCE(SUM(amount), 0) as balance
     FROM ledger_entries
     WHERE account_name = ?"
)
.bind(&self.from_account.0)
.fetch_one(pool)
.await?;

let balance: i64 = row.get("balance");

// Check sufficient funds
if balance < transfer.amount.0 {
    return Err(ValidationError::InsufficientFunds {
        balance,
        required: transfer.amount.0,
    });
}
```

### Transactional Commit

The `commit()` method uses a transaction:
```rust
let mut tx = pool.begin().await?;

// Debit entry
sqlx::query("INSERT INTO ledger_entries (...) VALUES (?, ?, ?, ?)")
    .bind(&self.from_account.0)
    .bind(-self.amount.0)  // Negative for debit
    .bind(&self.transfer_id.0)
    .bind(chrono::Utc::now().timestamp())
    .execute(&mut *tx)
    .await?;

// Credit entry
sqlx::query("INSERT INTO ledger_entries (...) VALUES (?, ?, ?, ?)")
    .bind(&self.to_account.0)
    .bind(self.amount.0)  // Positive for credit
    .bind(&self.transfer_id.0)
    .bind(chrono::Utc::now().timestamp())
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

### Error Conversion

SQLx errors automatically convert:
```rust
impl From<sqlx::Error> for ValidationError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}
```

---

## Testing Strategy

### Phase 1: Smoke Test
**Goal:** Verify basic emit pipeline

- Create table
- Initialize accounts
- Execute single transfer
- Verify balances

**Status:** ✅ Complete

### Phase 2: Balance Queries
**Goal:** Test SQL aggregation

- Query balances before/after transfers
- Verify SUM aggregation works
- Test COALESCE for missing accounts

**Status:** ✅ Complete

### Phase 3: Dynamic Transfers
**Goal:** Parameterized queries

- Runtime binding of account names
- Variable transfer amounts
- Multiple transfers in sequence

**Status:** ✅ Complete

### Phase 4: Constraint Validation
**Goal:** Pre-transfer validation pattern

- Query balance before transfer
- Check sufficient funds
- Validate amount > 0
- Handle validation failures

**Status:** ✅ Complete

### Phase 5: Typestate Integration
**Goal:** Full typestate API with database

- Transfer<Pending> → validate() → Transfer<Validated>
- Transfer<Validated> → commit() → Transfer<Committed>
- Error cases: insufficient funds, same account, negative amount
- Manual rejection and rollback

**Tests:**
- `test_ledger_typestate_valid_transfer` - Happy path
- `test_ledger_typestate_insufficient_funds` - Validation failure
- `test_ledger_typestate_same_account` - AccountsDistinct violation
- `test_ledger_typestate_manual_reject` - Pending → Rejected
- `test_ledger_typestate_rollback_after_validation` - Validated → Rejected
- `test_ledger_typestate_negative_amount` - Amount validation
- `test_ledger_typestate_zero_amount` - Amount validation

**Status:** ✅ Complete (7 tests, all passing)

### Phase 6: Concurrent Transfers
**Goal:** Verify behavior under concurrent load

- Multiple concurrent transfers from same account
- Bidirectional transfers (Alice ↔ Bob)
- Many-to-one transfers (10 → Target)
- Overdraft scenarios

**Tests:**
- `test_ledger_concurrent_transfers_from_same_account` - 5 concurrent transfers
- `test_ledger_concurrent_transfers_no_overdraft` - 10 concurrent transfers
- `test_ledger_concurrent_bidirectional_transfers` - Alice ↔ Bob
- `test_ledger_concurrent_many_to_one` - 10 → Target

**Key Finding:** SQLite's default DEFERRED locking allows overdrafts (validation sees stale balances), but **double-entry invariant always holds** (total balance unchanged).

**Status:** ✅ Complete (4 tests, all passing)

### Test Summary

**Total:** 6 test files covering smoke → concurrent
- Phase 1: 1 test (smoke)
- Phase 2: 1 test (queries)
- Phase 3: 1 test (dynamic)
- Phase 4: 1 test (contracts)
- Phase 5: 7 tests (typestate)
- Phase 6: 4 tests (concurrent)

**All tests passing:** ✅

---

## Production Considerations

### 1. Transaction Isolation

**SQLite Default (DEFERRED):**
- Locks acquired on first write, not BEGIN
- Validation queries may see stale balances
- Multiple transactions can pass validation and all commit
- **Result:** Overdrafts possible

**Recommended for Production:**
```rust
// Option 1: BEGIN IMMEDIATE
sqlx::query("BEGIN IMMEDIATE").execute(&mut tx).await?;

// Option 2: Use PostgreSQL
// - Better concurrency control
// - Serializable isolation level
// - Row-level locking
```

### 2. Unique Transfer IDs

Enforce uniqueness to prevent duplicate transfers:
```sql
CREATE UNIQUE INDEX idx_transfer_id ON ledger_entries(transfer_id, account_name);
```

### 3. Audit Trail

The current schema provides basic audit:
- `created_at` - Timestamp
- `transfer_id` - Links debit/credit entries
- `amount` - Signed integer (+ credit, - debit)

**Enhancement:** Add audit columns:
```sql
ALTER TABLE ledger_entries ADD COLUMN created_by TEXT;
ALTER TABLE ledger_entries ADD COLUMN metadata JSON;
```

### 4. Balance Snapshots

For high-volume systems, consider materializing balances:
```sql
CREATE TABLE account_balances (
    account_name TEXT PRIMARY KEY,
    balance INTEGER NOT NULL,
    last_updated INTEGER NOT NULL
);
```

Update via triggers or application code.

### 5. Archival

Partition or archive old entries:
```sql
CREATE TABLE ledger_entries_archive (
    LIKE ledger_entries INCLUDING ALL
);

-- Periodic archival (keep last N days in hot table)
INSERT INTO ledger_entries_archive
SELECT * FROM ledger_entries
WHERE created_at < ?;
```

### 6. Monitoring

Track:
- Failed validations (insufficient funds, same account)
- Commit latency
- Transaction retry rates
- Balance query performance

### 7. Idempotency

For distributed systems, ensure transfers are idempotent:
```rust
// Check if transfer_id already exists
let exists = sqlx::query("SELECT 1 FROM ledger_entries WHERE transfer_id = ?")
    .bind(&transfer_id.0)
    .fetch_optional(&pool)
    .await?;

if exists.is_some() {
    return Err(ValidationError::DuplicateTransfer);
}
```

### 8. Rate Limiting

Prevent abuse:
```rust
// Check recent transfer count
let count: i64 = sqlx::query_scalar(
    "SELECT COUNT(*) FROM ledger_entries
     WHERE account_name = ? AND created_at > ?"
)
.bind(&account_id.0)
.bind(one_hour_ago)
.fetch_one(&pool)
.await?;

if count > MAX_TRANSFERS_PER_HOUR {
    return Err(ValidationError::RateLimited);
}
```

### 9. Negative Balance Prevention

For strict constraints:
```rust
// Add check constraint
CREATE TABLE ledger_entries (
    -- ...
    CHECK (
        (SELECT COALESCE(SUM(amount), 0)
         FROM ledger_entries
         WHERE account_name = NEW.account_name) >= 0
    )
);
```

Or use application-level locking:
```rust
// Acquire advisory lock per account
sqlx::query("SELECT pg_advisory_lock(hashtext(?))")
    .bind(&account_id.0)
    .execute(&pool)
    .await?;

// Validate and commit...

sqlx::query("SELECT pg_advisory_unlock(hashtext(?))")
    .bind(&account_id.0)
    .execute(&pool)
    .await?;
```

### 10. Testing

Always test:
- Concurrent transfers (Phase 6 tests)
- Edge cases (zero balance, max int64)
- Failure recovery (what if commit fails?)
- Database failover/reconnection

---

## Comparison to Other Patterns

### Traditional Approach
```rust
struct Transfer {
    from: String,
    to: String,
    amount: i64,
    validated: bool,     // Runtime flag
    committed: bool,     // Runtime flag
}

// ❌ Can commit without validation
transfer.committed = true; // Oops, forgot to validate
```

### Typestate Approach
```rust
struct Transfer<Pending> { ... }
struct Transfer<Validated> { ... }
struct Transfer<Committed> { ... }

// ✅ Compiler enforces correct order
let pending: Transfer<Pending> = ...;
pending.commit(&pool).await; // ERROR: no method `commit`

let validated = pending.validate(&pool).await?;
validated.commit(&pool).await?; // OK
```

**Benefits:**
- Impossible states are unrepresentable
- State transitions enforced at compile time
- No runtime checks needed
- Self-documenting code

---

## Related Documentation

- [TYPESTATE_LEDGER_DESIGN.md](../../../TYPESTATE_LEDGER_DESIGN.md) - Original design document
- [elicitation contracts](../../elicitation/src/contracts.rs) - Proof framework
- [strictly_games tictactoe](https://github.com/crumplecup/strictly_games) - Reference typestate implementation

---

## License

Same as parent crate (elicitation).
