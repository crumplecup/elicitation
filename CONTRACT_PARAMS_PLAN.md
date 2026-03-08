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

## Type Inventory

### `elicit_url` — URL contract types

```
HttpsUrl       — UnvalidatedUrl → UrlParsed → HttpsRequired
               wraps SecureUrlState; as_str() returns the proven URL
               schema: { type: string, description: "HTTPS URL, validated" }

ParsedUrl      — UnvalidatedUrl → UrlParsed
               wraps ParsedUrl (the existing typestate type); as_str(), scheme()
               schema: { type: string, description: "Any valid URL" }

AllowedSchemeUrl { allowed: &'static [&'static str] }
               — tricky because the allowed set is a parameter
               Option A: const-generic `AllowedSchemeUrl<const SCHEMES: &[&str]>`
                         won't work — const slices aren't stable yet
               Option B: specific aliases `WssUrl`, `HttpOrHttpsUrl`
               Option C: runtime field on the struct (loses const-ness)
               → Defer. Start with HttpsUrl and ParsedUrl, revisit when a
                 concrete need for AllowedSchemeUrl appears.
```

### `elicitation` (or a new `elicitation_params` crate) — general primitives

```
PositiveF64    — f64 > 0.0
               schema: { type: number, exclusiveMinimum: 0 }

NonNegativeF64 — f64 >= 0.0
               schema: { type: number, minimum: 0 }

PositiveU32    — u32 > 0  (u32 is already non-negative; > 0 is the useful constraint)
               schema: { type: integer, minimum: 1 }

NonEmptyString — String with len() > 0 (after trim)
               schema: { type: string, minLength: 1 }

BoundedUsize<const MIN: usize, const MAX: usize>
               — const-generic range check; const generics for usize are stable
               schema: { type: integer, minimum: MIN, maximum: MAX }
               → Good candidate for timeout values, page sizes, retry counts
```

Where to put the primitives: in `elicitation` itself, under `elicitation::params`.
Keeping them in the main crate avoids adding a dependency on a new micro-crate for
something every downstream crate needs.

## The `HttpsUrl` Canary

Start here. It has the most concrete benefit:

1. Add `elicitation::params` module with primitives first (they have no external deps)
2. Add `elicitation_url::HttpsUrl` and `ParsedUrl` to `elicit_url`
3. Update `SecureFetchParams.url: String` → `url: HttpsUrl`
4. Verify `secure_fetch` body simplifies and still compiles
5. Update `EmitCode for SecureFetchParams` to call `p.url.as_str()` instead of `p.url`
6. Run existing smoke tests

## Schema Considerations

Agent-facing JSON schemas should be self-documenting. The `JsonSchema` impl for each
contract type should include a `description` explaining the contract:

```rust
impl JsonSchema for HttpsUrl {
    fn schema_name() -> String { "HttpsUrl".to_string() }
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
        let url: &str = self.url.as_str();  // extract proven string
        // emit code that constructs HttpsUrl from the string (via serde or directly)
        quote! { ... elicit_url::HttpsUrl::try_from(#url.to_string())? ... }
    }
}
```

Alternatively, emit the full proof chain explicitly (identical to current behavior,
just driven from the value):

```rust
quote! {
    let _url = elicit_url::HttpsUrl::try_from(#url.to_string())
        .map_err(|e| format!("HTTPS required: {e}"))?;
}
```

This is actually *better* than the current EmitCode: we emit a typed construction
rather than raw typestate ceremony, and the proof chain is guaranteed correct
because `HttpsUrl::try_from` is the same proof path that ran at tool-call time.

## Formal Verification Tie-In

Nothing changes for the formal verification story — it improves:

- The proof chain runs in `HttpsUrl::deserialize` / `HttpsUrl::try_from`
- Kani harnesses can target this function directly: "does `HttpsUrl::from_str`
  only succeed when the URL has scheme `https`?"
- Previously the proof was scattered across each tool handler; now there is one
  canonical proof site per contract type

## Phases

### Phase A — `elicitation::params` primitives

Files: `crates/elicitation/src/params.rs` (new), `lib.rs` (add `pub mod params`)

Types: `PositiveF64`, `NonNegativeF64`, `PositiveU32`, `NonEmptyString`,
       `BoundedUsize<const MIN: usize, const MAX: usize>`

Each needs: `Deserialize`, `JsonSchema`, `Debug`, `Clone`, `Serialize`,
            `Deref<Target = inner>` or accessor method, `TryFrom<primitive>`

Tests: unit tests for boundary conditions (exactly at limit, one below, one above)

### Phase B — `elicit_url` contract types

Files: `crates/elicit_url/src/params.rs` (new), `lib.rs` re-exports

Types: `HttpsUrl` (wraps `SecureUrlState`), `ParsedUrl` (wraps existing `ParsedUrl`)

Each needs: `Deserialize`, `JsonSchema`, `Debug`, `Clone`, `Serialize`,
            `as_str() -> &str`, `as_url() -> &url::Url` (where relevant),
            `TryFrom<String>`

Tests: valid HTTPS → succeeds; HTTP → fails; malformed → fails

### Phase C — Canary conversion

Files: `crates/elicit_server/src/secure_fetch.rs`

- `SecureFetchParams.url: String` → `url: HttpsUrl`
- Remove proof ceremony from `secure_fetch` body
- Remove proof ceremony from `validated_api_call` body
- Update `EmitCode for SecureFetchParams` to use `p.url.as_str()`
- Run smoke tests

### Phase D — Propagation (per-crate, independent)

Apply contract types wherever existing workflow params use raw primitives that
are immediately validated in the tool body. Candidates:

- `elicit_url` workflow tools: `AssertHttpsParams.url` → `ParsedUrl` (already parsed
  by the time the tool runs — but the tool *is* the validation; this one stays raw)
- `elicit_reqwest` params: `FetchParams.url`, `AuthFetchParams.url` → `HttpsUrl`
- `timeout_secs` fields across multiple params → `PositiveF64`
- Page size / limit fields → `BoundedUsize<1, 1000>` or similar

Each conversion is independent and can be a separate small commit.

### Phase E — Kani harnesses for contract types

Add targeted Kani harnesses in `crates/elicitation/tests/` and `crates/elicit_url/tests/`
proving constructor correctness:

```rust
#[cfg(kani)]
#[kani::proof]
fn https_url_only_from_https() {
    let s: String = kani::any();
    if let Ok(url) = HttpsUrl::try_from(s.clone()) {
        assert!(s.starts_with("https://"));
    }
}
```

## Non-Goals

- **`AllowedSchemeUrl` with runtime scheme list** — defer until a concrete use case.
  Static aliases (`WssUrl = AllowedSchemeUrl<["wss"]>`) may be worth adding later.
- **Const-generic string length bounds** (`NonEmptyString<MIN_LEN, MAX_LEN>`) — the
  const generic approach for string bounds requires nightly. Use specific types instead.
- **Phase 7 guard attributes** — explicitly replaced by this plan. Do not implement.
- **Auto-deriving contract types from spec annotations** — this is the anodized layer
  problem (see ANODIZED_SPEC_EXPLORER_PLAN.md) and is a separate, larger effort.

## Success Criteria

After Phase C:
- `SecureFetchParams.url` is `HttpsUrl`
- The proof ceremony is gone from both `secure_fetch` and `validated_api_call`
- Compilation still passes with all features
- Smoke tests pass
- An agent providing `http://` URL gets a well-formed `invalid_params` error
  (this is now testable without running the HTTP stack)

After Phase D:
- No `timeout_secs: f64` in any public params struct
- `PositiveF64` or `BoundedUsize` used for all bounded numeric params
- `JsonSchema` for those params includes `minimum`/`exclusiveMinimum`
  giving agents machine-readable range information
