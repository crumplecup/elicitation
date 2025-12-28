# Elicitation Library Roadmap

This document outlines planned extensions to support more Rust standard library types and advanced features.

## Version 0.2.0 - Collections & Standard Types

**Focus**: Extend to common std collections and path types

### Collections

#### ✅ HashMap<K, V> & BTreeMap<K, V> - COMPLETED

**Status**: Implemented in commit a5f3c6b

**Pattern**: Loop-based key-value elicitation

**Implementation**:
- `src/collections/hashmap.rs` - HashMap<K,V> with duplicate key handling
- `src/collections/btreemap.rs` - BTreeMap<K,V> with ordered keys
- `tests/collections_test.rs` - Test coverage
- `examples/collections.rs` - Usage examples

**Trait bounds**:
- `HashMap`: `K: Elicitation + Hash + Eq + Send`, `V: Elicitation + Send`
- `BTreeMap`: `K: Elicitation + Ord + Send`, `V: Elicitation + Send`

---

#### ✅ HashSet<T> & BTreeSet<T> - COMPLETED

**Status**: Implemented in commit a5f3c6b

**Pattern**: Loop-based item elicitation with automatic duplicate handling

**Implementation**:
- `src/collections/hashset.rs` - HashSet<T> with deduplication
- `src/collections/btreeset.rs` - BTreeSet<T> with ordered items
- `tests/collections_test.rs` - Test coverage
- `examples/collections.rs` - Usage examples

**Trait bounds**:
- `HashSet`: `T: Elicitation + Hash + Eq + Send`
- `BTreeSet`: `T: Elicitation + Ord + Send`

---

#### ✅ VecDeque<T> & LinkedList<T> - COMPLETED

**Status**: Implemented

**Pattern**: Identical to Vec<T> - loop-based sequential elicitation

**Implementation**:
- `src/collections/vecdeque.rs` - VecDeque<T> double-ended queue
- `src/collections/linkedlist.rs` - LinkedList<T> doubly-linked list
- `tests/collections_test.rs` - Test coverage
- `examples/collections.rs` - Usage examples

**Trait bounds**:
- `T: Elicitation + Send`

---

### Path & Filesystem Types


#### ✅ PathBuf - COMPLETED

**Status**: Implemented

**Pattern**: String-based elicitation with automatic conversion

**Implementation**:
- `src/primitives/pathbuf.rs` - PathBuf implementation
- `tests/pathbuf_test.rs` - Test coverage
- `examples/pathbuf.rs` - Usage example

**Details**:
- Elicits as String then converts to PathBuf
- Accepts any valid UTF-8 path string
- Works with Unix, Windows, and relative paths
- Supports Option<PathBuf> for optional paths

---

### Network Types

#### ✅ IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr - COMPLETED

**Status**: Implemented

**Pattern**: String elicitation with parsing validation

**Implementation**:
- `src/primitives/network.rs` - All network type implementations
- `tests/network_test.rs` - 15 tests covering all network types
- `examples/network.rs` - Comprehensive usage example

**Types implemented**:
- `IpAddr` - Generic IP address (IPv4 or IPv6)
- `Ipv4Addr` - Specific IPv4 address
- `Ipv6Addr` - Specific IPv6 address
- `SocketAddr` - Socket address (IP + port)
- `SocketAddrV4` - IPv4 socket address
- `SocketAddrV6` - IPv6 socket address

**Details**:
- String-based elicitation with automatic parsing
- Validation returns InvalidFormat error on parse failure
- Full tracing of validation attempts
- Helpful error messages with format examples

---

### Time & Duration Types

#### ✅ Duration - COMPLETED

**Status**: Implemented

**Pattern**: Numeric elicitation (f64 seconds) with validation

**Implementation**:
- `src/primitives/duration.rs` - Duration implementation
- `tests/duration_test.rs` - 4 tests covering Duration
- `examples/duration.rs` - Usage example with timeouts and intervals

**Details**:
- Elicits as f64 (supports decimal seconds)
- Validates non-negative (returns OutOfRange error)
- Converts using Duration::from_secs_f64()
- Works with Option<Duration> and Vec<Duration>

**Future enhancement (v0.3.0)**:
- Unit selection (seconds, minutes, hours, days)
- Human-readable format parsing

---

## Version 0.3.0 - Advanced Patterns & Validation

**Focus**: Custom validation, complex types, reference types

### Tuple Types

Support elicitation of tuples up to arity 12 (matching Rust std):

```rust
impl<T1: Elicit, T2: Elicit> Elicitation for (T1, T2) {
    async fn elicit<T: Transport>(client: &Client<T>) -> ElicitResult<Self> {
        let first = T1::elicit(client).await?;
        let second = T2::elicit(client).await?;
        Ok((first, second))
    }
}

// Similar for (T1, T2, T3), up to (T1, ..., T12)
```

**Files to create**:

- `src/primitives/tuples.rs` (macro-generated impls)

---

### Array Types

Fixed-size arrays `[T; N]`:

```rust
// Use const generics
impl<T: Elicitation + Send, const N: usize> Elicitation for [T; N] {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        let mut items = Vec::with_capacity(N);

        tracing::info!(size = N, "Eliciting fixed-size array");

        for i in 0..N {
            tracing::debug!(index = i, "Eliciting array element");
            let item = T::elicit(client).await?;
            items.push(item);
        }

        // Convert Vec to array (requires T: Default for try_into)
        items.try_into()
            .map_err(|_| ElicitError::new(ElicitErrorKind::InvalidFormat {
                expected: format!("array of size {}", N),
                received: "wrong size".to_string(),
            }))
    }
}
```

**Files to create**:

- `src/containers/array.rs`

---

### Result<T, E>

Elicit success/failure with value:

```rust
impl<T: Elicitation + Send, E: Elicitation + Send> Elicitation for Result<T, E> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        #[derive(Elicit)]
        enum ResultVariant {
            Ok,
            Err,
        }

        let variant = ResultVariant::elicit(client).await?;

        match variant {
            ResultVariant::Ok => {
                let value = T::elicit(client).await?;
                Ok(Ok(value))
            }
            ResultVariant::Err => {
                let error = E::elicit(client).await?;
                Ok(Err(error))
            }
        }
    }
}
```

**Files to create**:

- `src/containers/result.rs`

---

### Smart Pointers

#### Box<T>, Rc<T>, Arc<T>

Transparent wrappers around `T::elicit()`:

```rust
impl<T: Elicitation + Send> Elicitation for Box<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        T::elicit(client).await.map(Box::new)
    }
}

impl<T: Elicitation + Send> Elicitation for Rc<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        T::elicit(client).await.map(Rc::new)
    }
}

impl<T: Elicitation + Send> Elicitation for Arc<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        T::elicit(client).await.map(Arc::new)
    }
}
```

**Files to create**:

- `src/containers/smart_pointers.rs`

---

### Custom Validation

Add `#[validate]` attribute for field validation:

```rust
#[derive(Elicit)]
struct User {
    #[validate(email)]
    email: String,

    #[validate(range(min = 0, max = 120))]
    age: u8,

    #[validate(min_length = 8)]
    password: String,
}
```

**Implementation**:

- Proc macro generates validation calls after elicitation
- Built-in validators: `email`, `range`, `min_length`, `max_length`, `regex`
- Custom validator functions: `#[validate(function = "validate_username")]`

**Files to create**:

- `crates/elicitation_derive/src/validation.rs`
- `src/validation.rs` (runtime validation support)

---

## Version 0.4.0 - Authorize Paradigm & Permissions

**Focus**: Permission-based elicitation, security policies

### Authorize Trait Implementation

```rust
pub trait Authorize: Sized {
    /// Required permissions to elicit this type
    fn required_permissions() -> &'static [Permission];

    /// Check if user has permission
    async fn check_permission<T: Transport>(
        client: &Client<T>,
        permission: &Permission,
    ) -> ElicitResult<bool>;
}
```

**Use cases**:

- Eliciting sensitive data (SSN, credit cards)
- Admin-only configuration
- Rate-limited operations
- Conditional field elicitation

**Files to create**:

- `src/paradigm/authorize.rs`
- `src/permissions.rs`

---

## Version 0.5.0 - Multi-Select & Ranked Choice

**Focus**: Advanced selection paradigms

### Multi-Select

Allow selecting multiple enum variants:

```rust
#[derive(Elicit)]
#[multi_select] // New attribute
enum Features {
    DarkMode,
    Notifications,
    Analytics,
    Premium,
}

// Returns Vec<Features> instead of single Features
```

### Ranked Choice

Order options by preference:

```rust
#[derive(Elicit)]
#[ranked_choice] // New attribute
enum Priority {
    Feature1,
    Feature2,
    Feature3,
}

// Returns Vec<Priority> in ranked order
```

**Files to create**:

- `crates/elicitation_derive/src/multi_select.rs`
- `crates/elicitation_derive/src/ranked_choice.rs`

---

## Version 0.6.0 - Conditional & Dynamic Elicitation

**Focus**: Conditional fields, dynamic forms

### Conditional Fields

```rust
#[derive(Elicit)]
struct PaymentInfo {
    payment_method: PaymentMethod,

    #[elicit_if(payment_method = "PaymentMethod::CreditCard")]
    card_number: Option<String>,

    #[elicit_if(payment_method = "PaymentMethod::BankTransfer")]
    account_number: Option<String>,
}
```

### Dynamic Forms

Elicit number of items before eliciting collection:

```rust
// "How many items?" -> n
// Then elicit exactly n items (no loop)
```

---

## Future Considerations

### Async Iterators (v0.7.0)

Stream-based elicitation for large datasets.

### Custom Paradigms (v0.8.0)

Allow users to define custom interaction paradigms.

### Internationalization (v0.9.0)

Support multiple languages for prompts.

### UI Generation (v1.0.0)

Generate web forms from Elicit types.

---

## Implementation Priorities

### High Priority (v0.2.0)

1. HashMap<K, V> & BTreeMap<K, V>
2. HashSet<T> & BTreeSet<T>
3. PathBuf
4. Duration

### Medium Priority (v0.3.0)

1. Tuple types
2. Array types
3. Result<T, E>
4. Smart pointers (Box, Rc, Arc)
5. Network types (IpAddr, etc.)

### Low Priority (v0.4.0+)

1. Authorize paradigm
2. Custom validation
3. Multi-select
4. Conditional fields

---

## Testing Strategy

For each new type:

1. Unit tests in `tests/` directory
2. Doctests in type documentation
3. Integration example in `examples/`
4. Feature-gated API tests (`#[cfg(feature = "api")]`)

---

## Documentation Updates

For each version:

1. Update README.md with new types
2. Add examples for new patterns
3. Update CHANGELOG.md
4. Add migration guide if breaking changes
5. Update crate keywords/categories

---

## Backward Compatibility

- Maintain semantic versioning strictly
- Mark deprecated features with `#[deprecated]`
- Provide migration paths in docs
- Only break API in major versions

---

## Community Input

This roadmap is subject to community feedback. Please open issues or discussions for:

- New type suggestions
- Use case examples
- Priority adjustments
- Design feedback

---

## Version 0.2.0 Extensions - Result & Advanced Containers

### Result<T, E>

#### ✅ Result<T, E> - COMPLETED

**Status**: Implemented (moved from v0.3.0 to v0.2.0)

**Pattern**: Boolean choice for variant, then elicit value

**Implementation**:
- `src/containers/result.rs` - Result<T, E> implementation
- `tests/result_test.rs` - 6 tests covering Result operations
- `examples/result.rs` - Comprehensive usage with enums and complex types

**Details**:
- First elicits bool to choose Ok or Err variant
- Then elicits appropriate inner type (T for Ok, E for Err)
- Works with any T and E that implement Elicitation
- Supports complex nesting: Option<Result<T, E>>, Vec<Result<T, E>>

**Use cases demonstrated**:
- API responses (Result<StatusCode, ApiError>)
- Operation outcomes (Result<String, String>)
- Batch operations (Vec<Result<T, E>>)
- Optional results (Option<Result<T, E>>)

---
