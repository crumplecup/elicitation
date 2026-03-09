# Contract-Carrying Param Types

## Problem

Tool handler bodies currently perform explicit typestate ceremony:

```rust
async fn secure_fetch(ctx: Arc<PluginContext>, p: SecureFetchParams) -> Result<...> {
    let (parsed, url_proof) = UnvalidatedUrl::new(p.url.clone())
        .parse()
        .map_err(|e| ErrorData::invalid_params(...))?;
    let (_secure, _https_proof) = parsed
        .assert_https(url_proof)
        .map_err(|e| ErrorData::invalid_params(...))?;
    ctx.http.get(p.url.as_str()).send().await?   // uses raw string, not _secure
}
```

Three problems with this:

1. **Validation is duplicated**. The json schema says `url: string`; the handler then re-validates it.
   Every new tool that needs HTTPS validation writes the same ceremony.
2. **The proof leaks nowhere**. `_secure` and `_https_proof` are named with underscores because
   the handler doesn't actually use the proven type — it reverts to the raw string.
3. **Phase 7 (guard attributes) only makes it worse** — `#[requires_https(url)]` would hide
   the proof tokens, depriving us of the typed `SecureUrl` while proliferating bespoke attributes.

## Solution: The Newtype IS the Proof

Move the proof chain into the type constructor. A type that can only be deserialized by
traversing the full typestate chain *carries the proof in its existence*:

```rust
pub struct HttpsUrl(SecureUrlState);   // can only exist if https was proven

impl<'de> Deserialize<'de> for HttpsUrl {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let (parsed, proof) = UnvalidatedUrl::new(s)
            .parse()
            .map_err(de::Error::custom)?;
        let (secure, _) = parsed
            .assert_https(proof)
            .map_err(de::Error::custom)?;
        Ok(HttpsUrl(secure))
    }
}
```

The tool body shrinks to its actual intent:

```rust
async fn secure_fetch(ctx: Arc<PluginContext>, p: SecureFetchParams) -> Result<...> {
    ctx.http.get(p.url.as_str()).send().await?
}
```

No ceremony. No hidden proof tokens. The type is the contract.

## Design Principles

**One validation site.** The proof chain runs once, in `Deserialize`. Downstream code cannot
receive an `HttpsUrl` without the chain having executed successfully.

**No new attributes or macros.** These are plain Rust types. `make_descriptor`'s existing
`serde_json::from_value::<T>()` path handles validation and converts failures to
`ErrorData::invalid_params` automatically. No framework changes needed.

**Schema transparency.** `impl JsonSchema for HttpsUrl` exposes `{ "type": "string" }`
with a `description` that tells the agent the constraint. Agents see clean primitive schemas
with human-readable contract summaries. The JSON Schema `format` and `description` fields
carry the contract information where tooling can use it.

**Ergonomic access.** `Deref<Target = str>` (or `.as_str()`) lets the inner proven value
be used directly without unwrapping.

**Formal verification preserved.** The proof chain still exists; it now lives in
`HttpsUrl::deserialize`. Kani/Verus can prove properties about the constructor.
`Established<HttpsRequired>` is established inside the impl; the existence of the value
is the evidence.

## Existing Type Inventory (Already Implemented)

> **Key finding:** All the constrained types we need already exist in
> `crates/elicitation/src/verification/types/`. They have validated `new()` constructors,
> Kani/Verus proof instrumentation, `anodized::spec` annotations, and `Elicitation` impls.
> **The only missing bridge is `Serialize + Deserialize + JsonSchema`**, which lets them
> be used directly as fields in MCP params structs.

### `verification/types/floats.rs`

```
F64Positive      — f64 > 0.0 and finite
F64NonNegative   — f64 >= 0.0 and finite
F64Finite        — f64 finite (no NaN/inf)
F32Positive, F32NonNegative, F32Finite    — f32 variants
```

### `verification/types/integers.rs`

```
I8/I16/I32/I64/U8/U16/U32/U64/USize + Positive, NonNegative, NonZero variants
I8/I16/I32/I64/U8/U16/U32/U64/USize + Range<MIN, MAX>  (const-generic bounds)
```

### `verification/types/strings.rs`

```
StringNonEmpty<const MAX_LEN: usize = 4096>   — len > 0, len <= MAX_LEN
StringDefault                                  — unconstrained (already has Deserialize)
```

### `verification/types/urls.rs`  _(feature = "url")_

```
UrlValid         — any parseable URL
UrlHttps         — HTTPS only; new(value: &str) → parse + scheme == "https"
UrlHttp          — HTTP only
UrlWithHost      — must have a hostname
UrlCanBeBase     — can-be-base flag
```

### The Gap

All constrained types (`F64Positive`, `UrlHttps`, `StringNonEmpty`, etc.) have:

- ✅ Validated `new()` constructors
- ✅ `get()` / `into_inner()` accessors
- ✅ `anodized::spec` annotations
- ✅ Kani proof instrumentation
- ✅ `Elicitation` impl
- ❌ `Serialize` (missing)
- ❌ `Deserialize` that routes through `new()` (missing)
- ❌ `JsonSchema` with constraint metadata (missing)

Adding these three bridges is the entirety of Phase A. No new types needed.

### URL Types: `UrlHttps` vs `SecureUrlState`

`verification/types/urls.rs::UrlHttps` validates identically to what Phase B originally
proposed (`parse + scheme == "https"`), but uses a direct `Url::parse()` call rather than
the `elicit_url` typestate chain. For params usage (serde boundary), `UrlHttps` is
sufficient and avoids the cross-crate dependency. The typestate chain in `elicit_url`
remains the correct place for *workflow tool* logic that composes proofs across steps.

For `SecureFetchParams.url`, we will use `UrlHttps` from `verification/types` directly.

## The `UrlHttps` Canary

Start here. It has the most concrete benefit:

1. Add serde bridge (`Serialize`, `Deserialize` via `try_from`, `JsonSchema`) to the
   constrained verification types — floats, integers, strings, URLs
2. Update `SecureFetchParams.url: String` → `url: UrlHttps`  (feature-gated on `"url"`)
3. Verify `secure_fetch` body simplifies and still compiles
4. Update `EmitCode for SecureFetchParams` to call `p.url.get().as_str()` instead of `p.url`
5. Run existing smoke tests

## Schema Considerations

Agent-facing JSON schemas should be self-documenting. The `JsonSchema` impl for each
contract type should include a `description` explaining the contract:

```rust
impl JsonSchema for UrlHttps {
    fn schema_name() -> String { "UrlHttps".to_string() }
    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("uri".to_string()),
            metadata: Some(Box::new(Metadata {
                description: Some(
                    "An HTTPS URL. Must use the https:// scheme. \
                     Validated at deserialization time.".to_string()
                ),
                ..Default::default()
            })),
            ..Default::default()
        }.into()
    }
}
```

For bounded numerics, the `minimum`/`maximum`/`exclusiveMinimum` fields give agents
machine-readable constraints they can respect without reading the description.

## EmitCode Impact

`EmitCode` for params structs containing contract types needs to emit the
*string value*, not the newtype — the reconstruction runs the proof chain again
at replay time, which is correct:

```rust
impl EmitCode for SecureFetchParams {
    fn emit_code(&self) -> TokenStream {
        let url: &str = self.url.get().as_str();  // extract proven string via get()
        quote! {
            let _url = elicitation::verification::types::UrlHttps::new(#url)
                .map_err(|e| format!("HTTPS required: {e}"))?;
        }
    }
}
```

This is *better* than the current EmitCode: we emit a typed construction
rather than raw typestate ceremony, and the proof chain is guaranteed correct
because `UrlHttps::new` is the same proof path that ran at tool-call time.

## Formal Verification Tie-In

Nothing changes for the formal verification story — it improves:

- The proof chain runs in `UrlHttps::deserialize` (via `TryFrom`) and `UrlHttps::new`
- Kani harnesses can target this directly: "does `UrlHttps::new` only succeed when
  the URL has scheme `https`?" — one proof covers all tools, not one per handler
- Previously the proof was scattered across each tool handler; now there is one
  canonical proof site per contract type

## Phases

### Phase A — Serde bridge for existing constrained types  _(was: new primitives)_

> **Revised:** no new types. The constrained types already exist. Add the missing
> serde/schema bridge to `verification/types/floats.rs`, `integers.rs`, `strings.rs`,
> `urls.rs`.

**Approach per type family:**

```rust
// Add to each constrained struct (e.g. F64Positive):
#[serde(try_from = "f64")]
#[serde(into = "f64")]           // for Serialize

impl TryFrom<f64> for F64Positive {
    type Error = ValidationError;
    fn try_from(v: f64) -> Result<Self, Self::Error> { Self::new(v) }
}

impl From<F64Positive> for f64 {
    fn from(v: F64Positive) -> f64 { v.into_inner() }
}

// JsonSchema: schemars range attribute already supported
#[schemars(range(min = 0.0))]  // on the inner field, or manual SchemaObject impl
```

For URLs (`UrlHttps`): `#[serde(try_from = "String")]` + `impl TryFrom<String>` calling `Self::new(&s)`.

A proc-macro can apply the pattern uniformly, but a declarative `impl_serde_bridge!` macro
may be sufficient. Check whether a blanket approach works before writing per-type impls.

Files: `verification/types/floats.rs`, `integers.rs`, `strings.rs`, `urls.rs`

Tests: in `tests/` — boundary conditions for each family (at limit, one below, one above)

### Phase B — Superseded

> **Superseded by Phase A.** `UrlHttps` and `UrlValid` already exist in
> `verification/types/urls.rs`. Once Phase A adds the serde bridge they are
> directly usable as params fields. No new `HttpsUrl`/`ParsedUrl` types needed.

### Phase C — Canary conversion

Files: `crates/elicit_server/src/secure_fetch.rs`

- `SecureFetchParams.url: String` → `url: UrlHttps`  (gated on `#[cfg(feature = "url")]`)
- Remove proof ceremony from `secure_fetch` body
- Remove proof ceremony from `validated_api_call` body
- Update `EmitCode for SecureFetchParams` to use `p.url.get().as_str()`
- Run smoke tests

### Phase D — Propagation (per-crate, independent)

Apply contract types wherever existing workflow params use raw primitives that
are immediately validated in the tool body. Candidates:

- `elicit_reqwest` params: `FetchParams.url`, `AuthFetchParams.url` → `UrlHttps`
- `timeout_secs` fields across multiple params → `F64Positive`
- Page size / limit fields → `USizeRange<1, 1000>` or similar

Each conversion is independent and can be a separate small commit.

### Phase E — Kani harnesses for contract types

Add targeted Kani harnesses in `crates/elicitation/tests/`
proving constructor correctness:


```rust
#[cfg(kani)]
#[kani::proof]
fn url_https_only_from_https() {
    let s: String = kani::any();
    if let Ok(url) = UrlHttps::new(&s) {
        assert!(s.starts_with("https://"));
    }
}
```

## Non-Goals

- **`AllowedSchemeUrl` with runtime scheme list** — defer until a concrete use case.
  `UrlHttps` and `UrlHttp` cover the common cases; specific aliases can be added later.
- **Const-generic string length bounds (`StringNonEmpty<MIN, MAX>`)** — `MAX_LEN` const
  generic is already supported; the existing `StringNonEmpty<4096>` default is sufficient.
  Add specific instances (`StringNonEmpty<256>`) as needed rather than proliferating params.
- **Phase 7 guard attributes** — explicitly replaced by this plan. Do not implement.
- **Auto-deriving contract types from spec annotations** — this is the anodized layer
  problem (see ANODIZED_SPEC_EXPLORER_PLAN.md) and is a separate, larger effort.

## Success Criteria

After Phase A:
- All constrained types in `verification/types/` have `Serialize + Deserialize + JsonSchema`
- Deserialization of an out-of-range value fails with a descriptive error at the serde boundary
- JsonSchema includes `minimum`/`maximum`/`exclusiveMinimum` or `description` per type family

After Phase C:
- `SecureFetchParams.url` is `UrlHttps`
- The proof ceremony is gone from both `secure_fetch` and `validated_api_call`
- Compilation still passes with all features
- Smoke tests pass
- An agent providing `http://` URL gets a well-formed `invalid_params` error
  (this is now testable without running the HTTP stack)

After Phase D:
- No `timeout_secs: f64` in any public params struct
- `F64Positive` or `USizeRange` used for all bounded numeric params
- `JsonSchema` for those params includes `minimum`/`exclusiveMinimum`
  giving agents machine-readable range information
