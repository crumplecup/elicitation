# Ledger Workflow Test — Implementation Plan

> **Premise:** End-to-end validation that agent workflow composition → emit_binary → compiled executable works correctly. Uses a double-entry ledger as the test domain because it requires transactions, demonstrates contract composition, and has meaningful error cases.

---

## Why a Ledger?

A todo app tests basic CRUD but misses the hard parts:

| What We Need to Validate | Todo App | Ledger |
|---|---|---|
| Transactions (atomicity) | ❌ Not required | ✅ Mandatory (debit + credit must balance) |
| Contract composition (`FullCommit`) | ❌ Single operations | ✅ `And<DbConnected, And<TransactionOpen, TransactionCommitted>>` |
| Constraint violations | ❌ Trivial | ✅ Insufficient funds, negative amounts |
| Concurrent access | ❌ No meaningful races | ✅ Two withdrawals from same account |
| Complex queries | ❌ Simple SELECT | ✅ Balance requires `SUM(amount)` |
| Rollback on failure | ❌ Not critical | ✅ Partial transfer = corrupted books |

The ledger is pedagogically relatable (everyone understands money) while being technically rigorous.

---

## Test Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ Phase 1: Agent composes workflow interactively             │
│   - sqlx_workflow__connect (SQLite in-memory)              │
│   - sqlx_workflow__execute (CREATE TABLE)                  │
│   - tokio_net__tcp_listener_bind (localhost:8080)          │
│   - tokio_net__tcp_listener_accept                         │
│   - sqlx_workflow__begin                                   │
│   - sqlx_workflow__tx_execute (INSERT ledger entries)      │
│   - sqlx_workflow__commit                                  │
│   - tokio_net__tcp_stream_write (HTTP response)            │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ Phase 2: emit_binary generates code                        │
│   - Calls elicit_server::emit_dispatch for each step       │
│   - BinaryScaffold assembles into main.rs                  │
│   - Writes Cargo.toml with deps                            │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ Phase 3: cargo build --release                             │
│   - Compiles generated code                                │
│   - Links elicit_sqlx, elicit_tokio dependencies           │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│ Phase 4: Run binary + validate with reqwest                │
│   - Binary listens on :8080                                │
│   - Agent uses elicit_reqwest to POST transfer             │
│   - Agent queries balance with GET                         │
│   - Validates response correctness                         │
└─────────────────────────────────────────────────────────────┘
```

---

## Phased Implementation

### Phase 1: Smoke Test (Minimal Viable Emit)

**Goal:** Prove emit pipeline works end-to-end with hardcoded transfer

**Scope:**
- Single hardcoded transfer: Alice -100, Bob +100
- No HTTP parsing (just respond to first request)
- No JSON parsing (hardcoded values)
- Manual HTTP response strings
- SQLite in-memory database

**Schema:**
```sql
CREATE TABLE ledger_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    transfer_id TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
```

**Workflow steps (13 total):**

1. `sqlx_workflow__connect`
   - Params: `{ database_url: "sqlite::memory:", max_connections: 1 }`
   - Returns: `{ pool_id: "..." }`

2. `sqlx_workflow__execute` (CREATE TABLE)
   - Params: `{ pool_id, sql: "CREATE TABLE ledger_entries ..." }`
   - Returns: `{ rows_affected: 0 }`

3. `sqlx_workflow__execute` (INSERT Alice account marker)
   - Params: `{ pool_id, sql: "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', 0, 'init', 0)" }`

4. `sqlx_workflow__execute` (INSERT Bob account marker)
   - Params: `{ pool_id, sql: "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 0, 'init', 0)" }`

5. `tokio_net__tcp_listener_bind`
   - Params: `{ addr: "127.0.0.1:8080" }`
   - Returns: `{ listener_id: "..." }`

6. `tokio_net__tcp_listener_accept`
   - Params: `{ listener_id }`
   - Returns: `{ stream_id: "...", peer_addr: "..." }`

7. `tokio_net__tcp_stream_read`
   - Params: `{ stream_id, max_bytes: 1024 }`
   - Returns: `{ data: "...", bytes_read: ..., eof: false }`
   - Note: We ignore the request content in Phase 1

8. `sqlx_workflow__begin`
   - Params: `{ pool_id }`
   - Returns: `{ tx_id: "..." }`

9. `sqlx_workflow__tx_execute` (INSERT Alice debit)
   - Params: `{ tx_id, sql: "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Alice', -100, 'tx1', ...)" }`

10. `sqlx_workflow__tx_execute` (INSERT Bob credit)
    - Params: `{ tx_id, sql: "INSERT INTO ledger_entries (account_name, amount, transfer_id, created_at) VALUES ('Bob', 100, 'tx1', ...)" }`

11. `sqlx_workflow__commit`
    - Params: `{ tx_id }`
    - Returns: `{ ok: true, contract: "TransactionCommitted" }`

12. `tokio_net__tcp_stream_write`
    - Params: `{ stream_id, data: "HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}" }`
    - Returns: `{ bytes_written: ... }`

13. `tokio_net__tcp_stream_close`
    - Params: `{ stream_id }`

**Generated code structure:**
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Connect
    let pool = sqlx::any::AnyPoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    // Step 2: CREATE TABLE
    sqlx::query("CREATE TABLE ledger_entries ...").execute(&pool).await?;

    // Steps 3-4: Initialize accounts
    sqlx::query("INSERT INTO ledger_entries ...").execute(&pool).await?;
    sqlx::query("INSERT INTO ledger_entries ...").execute(&pool).await?;

    // Step 5: Bind listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    // Step 6: Accept connection
    let (mut stream, _peer) = listener.accept().await?;

    // Step 7: Read request (ignored in Phase 1)
    let mut buffer = vec![0u8; 1024];
    let _n = stream.read(&mut buffer).await?;

    // Step 8: Begin transaction
    let mut tx = pool.begin().await?;

    // Steps 9-10: Insert ledger entries
    sqlx::query("INSERT INTO ledger_entries ...").execute(&mut *tx).await?;
    sqlx::query("INSERT INTO ledger_entries ...").execute(&mut *tx).await?;

    // Step 11: Commit
    tx.commit().await?;

    // Step 12: Write HTTP response
    stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}").await?;

    // Step 13: Close stream
    drop(stream);

    Ok(())
}
```

**Test validation:**
```rust
// Integration test in elicit_server/tests/ledger_smoke_test.rs
#[tokio::test]
async fn test_ledger_workflow_smoke() {
    // 1. Agent composes workflow (13 tool calls via MCP)
    let workflow = compose_ledger_workflow().await;

    // 2. emit_binary generates code
    let output_dir = tempdir().unwrap();
    let result = emit_binary(EmitBinaryParams {
        steps: workflow,
        with_tracing: true,
        output_dir: output_dir.path().to_str().unwrap().to_string(),
        package_name: "ledger_test".to_string(),
        compile: true,
        workspace_root: Some(env!("CARGO_MANIFEST_DIR").to_string()),
    }).await;

    assert!(result.is_ok(), "emit_binary failed");

    // 3. Verify compilation succeeded
    let binary_path = output_dir.path().join("target/release/ledger_test");
    assert!(binary_path.exists(), "Binary not found");

    // 4. Run binary in background
    let mut child = Command::new(&binary_path)
        .spawn()
        .expect("Failed to start binary");

    // Wait for server to be ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // 5. Validate with HTTP client
    let client = reqwest::Client::new();
    let response = client.post("http://127.0.0.1:8080/")
        .send()
        .await
        .expect("Request failed");

    assert_eq!(response.status(), 200);
    let body = response.text().await.unwrap();
    assert_eq!(body, r#"{"status":"ok"}"#);

    // 6. Cleanup
    child.kill().await.ok();
}
```

**Success criteria:**
- ✅ Generated code compiles without errors
- ✅ Binary runs and accepts TCP connection
- ✅ Transaction commits successfully (no rollback)
- ✅ HTTP response is sent and received
- ✅ Test passes in CI

---

### Phase 2: Balance Query Endpoint

**Goal:** Add read path with SQL aggregation

**Add:**
- Second endpoint: `GET /balance/:account`
- Parse URL path to extract account name
- Query: `SELECT SUM(amount) FROM ledger_entries WHERE account_name = ?`
- Return JSON: `{"account": "Alice", "balance": -100}`

**New tools needed:**
- String parsing for HTTP path extraction (can use stdlib split)
- `sqlx_workflow__fetch_one` with bind parameters

**Validation:**
```rust
// After transfer
let alice_balance = client.get("http://127.0.0.1:8080/balance/Alice")
    .send()
    .await?
    .json::<BalanceResponse>()
    .await?;

assert_eq!(alice_balance.balance, -100);

let bob_balance = client.get("http://127.0.0.1:8080/balance/Bob")
    .send()
    .await?
    .json::<BalanceResponse>()
    .await?;

assert_eq!(bob_balance.balance, 100);
```

---

### Phase 3: Dynamic Transfers

**Goal:** Parse POST body and execute parameterized transfers

**Add:**
- Parse JSON body: `{"from": "Alice", "to": "Bob", "amount": 50}`
- Extract parameters and bind to SQL
- Return transfer_id in response

**New tools needed:**
- `serde_json::from_str` for JSON parsing (from `elicit_serde_json`)
- Parameterized queries with `args` field in `WfTxSqlParams`

**Validation:**
```rust
// Transfer 1: Alice → Bob (50)
let response = client.post("http://127.0.0.1:8080/transfer")
    .json(&json!({"from": "Alice", "to": "Bob", "amount": 50}))
    .send()
    .await?;

assert_eq!(response.status(), 200);

// Verify balances updated
let alice = get_balance("Alice").await?;
let bob = get_balance("Bob").await?;
assert_eq!(alice.balance, -50);
assert_eq!(bob.balance, 50);
```

---

### Phase 4: Constraint Validation (Contract Types)

**Goal:** Demonstrate contract types rejecting invalid transfers

**Add contract types:**
```rust
#[contract_type(requires = "value > 0")]
pub struct TransferAmount(i64);

#[contract_type(
    requires = "balance + amount.0 >= 0",
    ensures = "result.balance >= 0"
)]
pub struct ValidatedTransfer {
    from_account: String,
    to_account: String,
    amount: TransferAmount,
    from_balance: i64,
}
```

**Validation cases:**
```rust
// Valid transfer
let ok = client.post("/transfer")
    .json(&json!({"from": "Alice", "to": "Bob", "amount": 10}))
    .send().await?;
assert_eq!(ok.status(), 200);

// Invalid: negative amount
let neg = client.post("/transfer")
    .json(&json!({"from": "Alice", "to": "Bob", "amount": -10}))
    .send().await?;
assert_eq!(neg.status(), 400);

// Invalid: insufficient funds
let overdraft = client.post("/transfer")
    .json(&json!({"from": "Alice", "to": "Bob", "amount": 999999}))
    .send().await?;
assert_eq!(overdraft.status(), 400);
```

---

### Phase 5: Typestate State Machine

**Goal:** Showcase typestate transitions preventing invalid sequences

**Add typestate:**
```rust
struct Transfer<S> {
    from: AccountId,
    to: AccountId,
    amount: TransferAmount,
    _state: PhantomData<S>,
}

struct Pending;
struct Validated;
struct Committed;

impl Transfer<Pending> {
    fn validate(self, balances: &BalanceMap)
        -> Result<Transfer<Validated>, ValidationError>
    {
        // Check amount > 0, sufficient funds
        // Can only be called once
    }
}

impl Transfer<Validated> {
    fn commit(self, tx: &mut Transaction)
        -> Result<Transfer<Committed>, DbError>
    {
        // Execute SQL
        // Can't be called without validate() first
    }
}
```

**Compiler enforces:**
```rust
// ✅ Valid sequence
let transfer = Transfer::new(from, to, amount);
let validated = transfer.validate(&balances)?;
let committed = validated.commit(&mut tx)?;

// ❌ Compile error: can't commit without validate
let transfer = Transfer::new(from, to, amount);
let committed = transfer.commit(&mut tx)?; // ERROR: no method `commit` on Transfer<Pending>

// ❌ Compile error: can't validate twice
let transfer = Transfer::new(from, to, amount);
let validated = transfer.validate(&balances)?;
let validated_again = validated.validate(&balances)?; // ERROR: moved
```

---

### Phase 6: Concurrent Transfers

**Goal:** Validate correct behavior under concurrent access

**Add:**
- Multiple simultaneous transfers from same account
- Proper locking/serialization via transactions

**Test:**
```rust
// Spawn 10 concurrent transfers from Alice
let handles: Vec<_> = (0..10).map(|_| {
    tokio::spawn(async {
        client.post("/transfer")
            .json(&json!({"from": "Alice", "to": "Bob", "amount": 10}))
            .send()
            .await
    })
}).collect();

let results = join_all(handles).await;

// Count successes
let success_count = results.iter()
    .filter(|r| r.status() == 200)
    .count();

// Verify balance consistency
let alice = get_balance("Alice").await?;
let bob = get_balance("Bob").await?;
assert_eq!(alice.balance + bob.balance, 0); // Zero sum
assert_eq!(success_count * 10, -alice.balance); // Each success withdrew 10
```

---

## File Locations

```
crates/elicit_server/tests/
  ├── ledger_smoke_test.rs           # Phase 1
  ├── ledger_balance_query_test.rs   # Phase 2
  ├── ledger_dynamic_test.rs         # Phase 3
  └── ledger_concurrent_test.rs      # Phase 6

examples/
  ├── ledger_contracts.rs             # Phase 4 (contract type examples)
  └── ledger_typestate.rs             # Phase 5 (typestate machine example)

docs/
  └── ledger_workflow_guide.md       # End-to-end tutorial
```

---

## Dependencies Required

All already in workspace:

- ✅ `elicit_sqlx` - database operations
- ✅ `elicit_tokio` - TCP networking
- ✅ `elicit_reqwest` - HTTP client validation
- ✅ `elicit_serde_json` - JSON parsing (Phase 3+)
- ✅ `elicit_server` - emit_binary orchestration

---

## Success Metrics

**Phase 1 (Smoke):**
- Generated code compiles
- Binary runs and responds
- Test passes in CI
- Validates basic emit pipeline

**Phase 2 (Query):**
- Multiple endpoints work
- SQL aggregation emits correctly
- Balance queries return correct values

**Phase 3 (Dynamic):**
- JSON parsing works
- Parameterized queries work
- Arbitrary transfers succeed

**Phase 4 (Contracts):**
- Contract violations rejected
- Type-level guarantees preserved in emitted code
- Error messages reference contracts

**Phase 5 (Typestate):**
- Compiler enforces state transitions
- Can't commit without validate
- Can't validate twice

**Phase 6 (Concurrency):**
- No race conditions
- Balance consistency maintained
- Proper transaction isolation

---

## Known Limitations

**Phase 1:**
- Single hardcoded transfer only
- No HTTP routing
- No error handling
- SQLite in-memory (data lost on exit)

**Phase 2-3:**
- Brittle HTTP parsing (manual string splitting)
- No proper router (path matching by hand)
- No content negotiation

**Phase 4-6:**
- Requires significant refactoring of emitted code
- May need `elicit_http` shadow crate for proper request/response handling

---

## Next Steps

1. ✅ Create this plan document
2. ✅ Add to PLANNING_INDEX.md
3. Implement Phase 1 smoke test
4. Validate emit pipeline works
5. Iterate through phases 2-6 based on learnings
