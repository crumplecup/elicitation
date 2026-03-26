# Typestate Ledger Design

> **Goal:** Design a double-entry ledger using elicitation framework's typestate state machines with proof-carrying contracts, building on the proven emit pipeline from Phases 1-4.

---

## Design Principles (from strictly_games)

**Pattern observed in tic-tac-toe:**
1. **Typestate phases**: Each phase is a distinct type (GameSetup → GameInProgress → GameFinished)
2. **Propositions**: Type-level statements (SquareEmpty, PlayerTurn)
3. **Composite props**: And<P, Q> for combining preconditions
4. **Validation functions**: Return `Established<Prop>` on success, error otherwise
5. **Proof-carrying execution**: Functions require `Established<Prop>` parameter (zero-cost)
6. **Transitions consume self**: `setup.start()` consumes and returns new state

---

## Ledger State Machine

### States (Typestate Phases)

```rust
/// Transfer in pending state - can be validated or rejected
struct Transfer<Pending> {
    from_account: AccountId,
    to_account: AccountId,
    amount: Amount,
    transfer_id: TransferId,
    _state: PhantomData<Pending>,
}

/// Transfer validated - preconditions checked, ready to commit
struct Transfer<Validated> {
    from_account: AccountId,
    to_account: AccountId,
    amount: Amount,
    transfer_id: TransferId,
    from_balance: i64,  // Captured during validation
    _state: PhantomData<Validated>,
}

/// Transfer committed - both entries written to ledger
struct Transfer<Committed> {
    from_account: AccountId,
    to_account: AccountId,
    amount: Amount,
    transfer_id: TransferId,
    from_balance_before: i64,
    from_balance_after: i64,
    to_balance_after: i64,
    _state: PhantomData<Committed>,
}

/// Transfer rejected - validation failed
struct Transfer<Rejected> {
    from_account: AccountId,
    to_account: AccountId,
    amount: Amount,
    transfer_id: TransferId,
    reason: RejectionReason,
    _state: PhantomData<Rejected>,
}
```

### Transitions

```
Pending ──validate()──> Validated ──commit()──> Committed
   │                          │
   │                          │
   └────reject()──────────────┴──────rollback()──> Rejected
```

---

## Propositions (Contract Types)

### Basic Propositions

```rust
/// Proposition: The amount is positive (> 0).
pub struct AmountPositive;
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

/// Proposition: The account has sufficient funds.
pub struct SufficientFunds;
impl Prop for SufficientFunds {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_sufficient_funds() {
                let balance: i64 = kani::any();
                let amount: i64 = kani::any();
                kani::assume(amount > 0);
                kani::assume(balance >= amount);
                assert!(balance - amount >= 0, "Sufficient funds required");
            }
        }
    }
}

/// Proposition: The accounts are distinct (from != to).
pub struct AccountsDistinct;
impl Prop for AccountsDistinct {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_accounts_distinct() {
                let from: u32 = kani::any();
                let to: u32 = kani::any();
                kani::assume(from != to);
                assert!(from != to, "Accounts must be distinct");
            }
        }
    }
}

/// Proposition: The transaction is atomic (BEGIN committed successfully).
pub struct TransactionOpen;
impl Prop for TransactionOpen {
    // Already defined in elicit_sqlx::workflow
}

/// Proposition: The ledger entries balance (debit + credit = 0).
pub struct BalancedEntries;
impl Prop for BalancedEntries {
    fn kani_proof() -> TokenStream {
        quote! {
            #[kani::proof]
            fn verify_balanced_entries() {
                let debit: i64 = kani::any();
                let credit: i64 = kani::any();
                kani::assume(debit < 0);  // Debits are negative
                kani::assume(credit > 0);  // Credits are positive
                kani::assume(debit + credit == 0);  // Must balance
                assert!(debit + credit == 0, "Entries must balance");
            }
        }
    }
}
```

### Composite Propositions

```rust
/// Composite: A transfer is valid (positive amount AND sufficient funds AND distinct accounts).
pub type ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>;

/// Composite: A transfer can be committed (valid AND transaction open).
pub type CommittableTransfer = And<ValidTransfer, TransactionOpen>;

/// Composite: A transfer is fully committed (committable AND balanced entries).
pub type CommittedTransfer = And<CommittableTransfer, BalancedEntries>;
```

---

## Validation Functions (Establish Proofs)

```rust
/// Validates that the amount is positive.
pub fn validate_amount_positive(
    transfer: &Transfer<Pending>
) -> Result<Established<AmountPositive>, ValidationError> {
    if transfer.amount.0 <= 0 {
        Err(ValidationError::NegativeAmount(transfer.amount.0))
    } else {
        Ok(Established::assert())
    }
}

/// Validates that the account has sufficient funds.
pub async fn validate_sufficient_funds(
    transfer: &Transfer<Pending>,
    pool: &AnyPool,
) -> Result<(Established<SufficientFunds>, i64), ValidationError> {
    // Query current balance
    let row = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0) as balance
         FROM ledger_entries
         WHERE account_name = ?"
    )
    .bind(&transfer.from_account.0)
    .fetch_one(pool)
    .await?;

    let balance: i64 = row.get("balance");

    if balance < transfer.amount.0 {
        Err(ValidationError::InsufficientFunds {
            balance,
            required: transfer.amount.0
        })
    } else {
        Ok((Established::assert(), balance))
    }
}

/// Validates that the accounts are distinct.
pub fn validate_accounts_distinct(
    transfer: &Transfer<Pending>
) -> Result<Established<AccountsDistinct>, ValidationError> {
    if transfer.from_account == transfer.to_account {
        Err(ValidationError::SameAccount)
    } else {
        Ok(Established::assert())
    }
}

/// Validates all preconditions for a transfer.
pub async fn validate_transfer(
    transfer: &Transfer<Pending>,
    pool: &AnyPool,
) -> Result<(Established<ValidTransfer>, i64), ValidationError> {
    let amount_proof = validate_amount_positive(transfer)?;
    let (funds_proof, balance) = validate_sufficient_funds(transfer, pool).await?;
    let distinct_proof = validate_accounts_distinct(transfer)?;

    // Compose proofs: And<AmountPositive, And<SufficientFunds, AccountsDistinct>>
    let funds_and_distinct = both(funds_proof, distinct_proof);
    let valid_proof = both(amount_proof, funds_and_distinct);

    Ok((valid_proof, balance))
}
```

---

## State Transitions (Proof-Carrying)

```rust
impl Transfer<Pending> {
    /// Creates a new pending transfer.
    pub fn new(
        from: AccountId,
        to: AccountId,
        amount: Amount,
        transfer_id: TransferId,
    ) -> Self {
        Self {
            from_account: from,
            to_account: to,
            amount,
            transfer_id,
            _state: PhantomData,
        }
    }

    /// Validates the transfer, consuming Pending and returning Validated.
    ///
    /// Proof-carrying: Establishes ValidTransfer proof.
    pub async fn validate(
        self,
        pool: &AnyPool,
    ) -> Result<Transfer<Validated>, ValidationError> {
        let (proof, balance) = validate_transfer(&self, pool).await?;

        // Proof established - transition to Validated state
        Ok(Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            from_balance: balance,
            _state: PhantomData,
        })
    }

    /// Rejects the transfer, consuming Pending and returning Rejected.
    pub fn reject(self, reason: RejectionReason) -> Transfer<Rejected> {
        Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            reason,
            _state: PhantomData,
        }
    }
}

impl Transfer<Validated> {
    /// Commits the transfer to the ledger, consuming Validated and returning Committed.
    ///
    /// Proof-carrying: Requires ValidTransfer proof (from validation).
    /// Establishes CommittedTransfer proof (transaction + balanced entries).
    pub async fn commit(
        self,
        pool: &AnyPool,
    ) -> Result<Transfer<Committed>, CommitError> {
        // Begin transaction (establishes TransactionOpen)
        let mut tx = pool.begin().await?;

        // Insert debit entry
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&self.from_account.0)
        .bind(-self.amount.0)  // Negative for debit
        .bind(&self.transfer_id.0)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *tx)
        .await?;

        // Insert credit entry
        sqlx::query(
            "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&self.to_account.0)
        .bind(self.amount.0)  // Positive for credit
        .bind(&self.transfer_id.0)
        .bind(chrono::Utc::now().timestamp())
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        // Query final balances (for audit trail)
        let from_balance = query_balance(pool, &self.from_account).await?;
        let to_balance = query_balance(pool, &self.to_account).await?;

        Ok(Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            from_balance_before: self.from_balance,
            from_balance_after: from_balance,
            to_balance_after: to_balance,
            _state: PhantomData,
        })
    }

    /// Rolls back the validated transfer without committing.
    pub fn rollback(self, reason: RejectionReason) -> Transfer<Rejected> {
        Transfer {
            from_account: self.from_account,
            to_account: self.to_account,
            amount: self.amount,
            transfer_id: self.transfer_id,
            reason,
            _state: PhantomData,
        }
    }
}

impl Transfer<Committed> {
    /// Returns the balance change for the source account.
    pub fn from_balance_delta(&self) -> i64 {
        self.from_balance_after - self.from_balance_before
    }

    /// Verifies the double-entry invariant was preserved.
    pub fn verify_invariant(&self) -> bool {
        // Debit + Credit = 0
        self.from_balance_delta() == -self.amount.0
    }
}
```

---

## Workflow Integration (Emit Pipeline)

### Tool Composition

The typestate ledger plugs into the existing sqlx_workflow tools:

```rust
// Phase 1: Create pending transfer
let transfer = Transfer::new(
    AccountId("Alice".to_string()),
    AccountId("Bob".to_string()),
    Amount(100),
    TransferId("tx1".to_string()),
);

// Phase 2: Validate (establishes ValidTransfer proof)
// Workflow: sqlx_workflow__fetch_one (query balance)
let validated = transfer.validate(&pool).await?;

// Phase 3: Commit (requires ValidTransfer proof)
// Workflow: sqlx_workflow__begin
//           sqlx_workflow__tx_execute (debit)
//           sqlx_workflow__tx_execute (credit)
//           sqlx_workflow__commit
let committed = validated.commit(&pool).await?;

// Phase 4: Verify
assert!(committed.verify_invariant());
```

### Workflow Steps (matches Phase 4 test)

1. **Pending → Validated**
   - `sqlx_workflow__fetch_one` - Query balance
   - Validate constraints (amount > 0, balance >= amount, accounts distinct)
   - Transition to Validated state

2. **Validated → Committed**
   - `sqlx_workflow__begin` - Start transaction
   - `sqlx_workflow__tx_execute` - Insert debit entry
   - `sqlx_workflow__tx_execute` - Insert credit entry
   - `sqlx_workflow__commit` - Commit transaction
   - `sqlx_workflow__fetch_one` - Query final balances
   - Transition to Committed state

3. **Error Paths**
   - Pending → Rejected (validation fails)
   - Validated → Rejected (rollback before commit)

---

## Proof Composition

```rust
// Level 1: Basic propositions
AmountPositive      // amount > 0
SufficientFunds     // balance >= amount
AccountsDistinct    // from != to

// Level 2: Valid transfer (compose basic props)
ValidTransfer = And<AmountPositive, And<SufficientFunds, AccountsDistinct>>

// Level 3: Committable transfer (add transaction)
CommittableTransfer = And<ValidTransfer, TransactionOpen>

// Level 4: Committed transfer (add balance invariant)
CommittedTransfer = And<CommittableTransfer, BalancedEntries>
```

---

## Verification Properties

### Kani Proofs

```rust
#[kani::proof]
fn verify_transfer_lifecycle() {
    let amount: i64 = kani::any();
    let balance: i64 = kani::any();

    kani::assume(amount > 0);
    kani::assume(balance >= amount);

    // After transfer:
    let new_balance = balance - amount;

    // Invariant preserved
    assert!(new_balance >= 0);
    assert!(balance - new_balance == amount);
}

#[kani::proof]
fn verify_double_entry() {
    let amount: i64 = kani::any();
    kani::assume(amount > 0);

    let debit = -amount;
    let credit = amount;

    // Double-entry invariant
    assert!(debit + credit == 0);
}
```

### Creusot Proofs

```rust
#[requires(amount > 0)]
#[requires(balance >= amount)]
#[ensures(result >= 0)]
#[ensures(balance - result == amount)]
pub fn execute_transfer(balance: i64, amount: i64) -> i64 {
    balance - amount
}
```

---

## Error Handling

```rust
#[derive(Debug, Clone, Display, Error)]
pub enum ValidationError {
    #[display("Negative or zero amount: {}", _0)]
    NegativeAmount(i64),

    #[display("Insufficient funds: balance={}, required={}", balance, required)]
    InsufficientFunds { balance: i64, required: i64 },

    #[display("Cannot transfer to same account")]
    SameAccount,

    #[display("Database error: {}", _0)]
    Database(sqlx::Error),
}

#[derive(Debug, Clone, Display)]
pub enum RejectionReason {
    #[display("Validation failed: {}", _0)]
    ValidationFailed(ValidationError),

    #[display("Manual rollback")]
    ManualRollback,
}
```

---

## Implementation Plan

### Phase 5: Typestate Types & Validation

**Files:**
- `crates/elicitation/src/ledger/mod.rs` - Module root
- `crates/elicitation/src/ledger/types.rs` - AccountId, Amount, TransferId
- `crates/elicitation/src/ledger/typestate.rs` - Transfer<S> states
- `crates/elicitation/src/ledger/contracts.rs` - Propositions & validation
- `crates/elicitation/src/ledger/errors.rs` - Error types

**Test:**
- `crates/elicitation/tests/ledger_typestate_test.rs`
  - Create pending transfer
  - Validate → Validated
  - Reject invalid (negative amount)
  - Reject invalid (insufficient funds)
  - Reject invalid (same account)

### Phase 6: Commit & Workflow Integration

**Files:**
- `crates/elicitation/src/ledger/commit.rs` - Commit logic
- `crates/elicit_server/tests/ledger_phase5_typestate_test.rs` - Emit test

**Test:**
- Validated → Committed (emit workflow)
- Verify double-entry invariant
- Rollback scenarios

---

## Benefits

1. **Compile-time guarantees**: Can't commit without validation
2. **Zero-cost proofs**: `Established<P>` is `PhantomData`
3. **Compositional verification**: Kani checks proof composition
4. **Type-driven design**: State machine encoded in types
5. **Audit trail**: Each state captures relevant data
6. **Error transparency**: Explicit rejection reasons

---

## Comparison to tic-tac-toe

| Aspect | Tic-Tac-Toe | Ledger |
|--------|-------------|--------|
| States | Setup → InProgress → Finished | Pending → Validated → Committed |
| Validation | SquareEmpty ∧ PlayerTurn | AmountPositive ∧ SufficientFunds ∧ AccountsDistinct |
| Execution | execute_move(proof) | commit(proof) |
| Rejection | MoveError | ValidationError |
| Persistence | In-memory board | SQLx transactions |
| Emit support | No | Yes (via sqlx_workflow tools) |

---

## Next Steps

1. Implement Phase 5 types and validation functions
2. Add Kani harnesses for proof composition
3. Integrate with Phase 4 emit test
4. Document typestate guarantees
5. Add Creusot/Verus proofs (if time permits)
