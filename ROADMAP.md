# Elicitation Library Roadmap

This document outlines planned extensions to support more Rust standard library types and advanced features.

## Version 0.2.0 - Collections & Standard Types

**Focus**: Extend to common std collections and path types

### Collections

#### HashMap<K, V> & BTreeMap<K, V>

**Pattern**: Loop-based key-value elicitation

```rust
impl<K: Elicit + Hash + Eq + Send, V: Elicit + Send> Elicit for HashMap<K, V> {
    async fn elicit<T: Transport>(client: &Client<T>) -> ElicitResult<Self> {
        let mut map = HashMap::new();

        loop {
            let add_more = if map.is_empty() {
                // "Add first entry to this map?"
                bool::elicit(client).await?
            } else {
                // "Add another entry? (current count: N)"
                bool::elicit(client).await?
            };

            if !add_more {
                break;
            }

            // Elicit key
            tracing::debug!("Eliciting key");
            let key = K::elicit(client).await?;

            // Check for duplicate keys
            if map.contains_key(&key) {
                // "Key already exists. Replace value?"
                let replace = bool::elicit(client).await?;
                if !replace {
                    continue; // Skip this entry
                }
            }

            // Elicit value
            tracing::debug!("Eliciting value for key");
            let value = V::elicit(client).await?;

            map.insert(key, value);
        }

        Ok(map)
    }
}
```

**BTreeMap**: Identical implementation, but `K: Ord` instead of `K: Hash + Eq`

**Files to create**:
- `src/collections/hashmap.rs`
- `src/collections/btreemap.rs`
- `tests/collections_test.rs`
- `examples/collections.rs`

**Trait bounds required**:
- `HashMap`: `K: Elicit + Hash + Eq + Send`, `V: Elicit + Send`
- `BTreeMap`: `K: Elicit + Ord + Send`, `V: Elicit + Send`

---

#### HashSet<T> & BTreeSet<T>

**Pattern**: Loop-based item elicitation with duplicate detection

```rust
impl<T: Elicit + Hash + Eq + Send> Elicit for HashSet<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        let mut set = HashSet::new();

        loop {
            let add_more = if set.is_empty() {
                bool::elicit(client).await?
            } else {
                bool::elicit(client).await?
            };

            if !add_more {
                break;
            }

            let item = T::elicit(client).await?;

            // Automatic duplicate handling (Sets ignore duplicates)
            if !set.insert(item) {
                tracing::debug!("Duplicate item ignored (already in set)");
            }
        }

        Ok(set)
    }
}
```

**BTreeSet**: Identical, but `T: Ord` instead of `T: Hash + Eq`

**Files to create**:
- `src/collections/hashset.rs`
- `src/collections/btreeset.rs`

**Trait bounds required**:
- `HashSet`: `T: Elicit + Hash + Eq + Send`
- `BTreeSet`: `T: Elicit + Ord + Send`

---

#### VecDeque<T> & LinkedList<T>

**Pattern**: Identical to Vec<T> - loop-based sequential elicitation

```rust
impl<T: Elicit + Send> Elicit for VecDeque<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        let mut deque = VecDeque::new();

        loop {
            let add_more = bool::elicit(client).await?;
            if !add_more { break; }

            let item = T::elicit(client).await?;
            deque.push_back(item);
        }

        Ok(deque)
    }
}
```

**LinkedList**: Identical implementation

**Files to create**:
- `src/collections/vecdeque.rs`
- `src/collections/linkedlist.rs`

**Trait bounds required**:
- `T: Elicit + Send`

---

### Path & Filesystem Types

#### PathBuf

**Pattern**: String-based elicitation with validation

```rust
impl Elicit for PathBuf {
    async fn elicit<T: Transport>(client: &Client<T>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBuf");

        // Elicit as string, then parse
        let path_str = String::elicit(client).await?;

        // Validate path
        let path = PathBuf::from(path_str);

        // Optional: Check if path exists, is valid, etc.
        // For now, accept any string

        Ok(path)
    }
}
```

**Related types**:
- `PathBuf` - Main type
- Consider: `Path` (via `&Path` reference types in v0.3.0)

**Files to create**:
- `src/primitives/pathbuf.rs`

---

### Network Types

#### IpAddr, Ipv4Addr, Ipv6Addr

**Pattern**: String elicitation with parsing validation

```rust
impl Elicit for IpAddr {
    async fn elicit<T: Transport>(client: &Client<T>) -> ElicitResult<Self> {
        loop {
            let ip_str = String::elicit(client).await?;

            match ip_str.parse::<IpAddr>() {
                Ok(addr) => return Ok(addr),
                Err(e) => {
                    tracing::warn!(error = ?e, "Invalid IP address format");
                    // Could prompt: "Invalid IP address. Try again?"
                    continue;
                }
            }
        }
    }
}
```

**Related types**:
- `IpAddr` (enum: V4 | V6)
- `Ipv4Addr`
- `Ipv6Addr`
- `SocketAddr` (IpAddr + port)
- `SocketAddrV4`, `SocketAddrV6`

**Files to create**:
- `src/primitives/network.rs`

---

### Time & Duration Types

#### Duration

**Pattern**: Numeric elicitation with unit selection

```rust
// Simple approach: elicit seconds as f64
impl Elicit for Duration {
    async fn elicit<T: Transport>(client: &Client<T>) -> ElicitResult<Self> {
        // "Enter duration in seconds:"
        let seconds = f64::elicit(client).await?;

        if seconds < 0.0 {
            return Err(ElicitError::new(ElicitErrorKind::OutOfRange {
                min: "0".to_string(),
                max: "positive".to_string(),
            }));
        }

        Ok(Duration::from_secs_f64(seconds))
    }
}
```

**Advanced approach** (v0.3.0): Elicit value + unit

```rust
#[derive(DeriveElicit)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}

// Then combine with numeric value
```

**Related types**:
- `Duration`
- `SystemTime` (Duration since UNIX_EPOCH)
- `Instant` (not serializable - skip)

**Files to create**:
- `src/primitives/duration.rs`

---

## Version 0.3.0 - Advanced Patterns & Validation

**Focus**: Custom validation, complex types, reference types

### Tuple Types

Support elicitation of tuples up to arity 12 (matching Rust std):

```rust
impl<T1: Elicit, T2: Elicit> Elicit for (T1, T2) {
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
impl<T: Elicit + Send, const N: usize> Elicit for [T; N] {
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
impl<T: Elicit + Send, E: Elicit + Send> Elicit for Result<T, E> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        #[derive(DeriveElicit)]
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
impl<T: Elicit + Send> Elicit for Box<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        T::elicit(client).await.map(Box::new)
    }
}

impl<T: Elicit + Send> Elicit for Rc<T> {
    async fn elicit<U: Transport>(client: &Client<U>) -> ElicitResult<Self> {
        T::elicit(client).await.map(Rc::new)
    }
}

impl<T: Elicit + Send> Elicit for Arc<T> {
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
#[derive(DeriveElicit)]
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
#[derive(DeriveElicit)]
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
#[derive(DeriveElicit)]
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
#[derive(DeriveElicit)]
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
