//! `JsonWorkflowPlugin` — phrase-level JSON tool compositions.
//!
//! While the atomic plugins (`JsonValue`, `JsonNumber`) are the **letters** of the
//! alphabet, this plugin provides **words**: each tool composes 2-4 primitives into a
//! meaningful operation with explicit contract documentation.
//!
//! # Typestate Design
//!
//! The internal implementation uses a typestate machine with proof-carrying transitions,
//! mirroring the tic-tac-toe `GameSetup → GameInProgress → GameFinished` pattern:
//!
//! ```text
//! RawJson ──parse()──→ ParsedJson ──assert_object()──→ ObjectJson ──validate_required()──→ ObjectJson
//!                           │                                                                    │
//!                           │ focus(ptr)                                                   .into_value()
//!                           ↓                                                            (no Option!)
//!                       FocusedJson
//!                           │
//!                       .extract()  ← returns Value, NOT Option<Value>
//!                                     (proof guarantees pointer resolved)
//! ```
//!
//! **Key invariant**: `FocusedJson::extract()` returns `serde_json::Value`, never
//! `Option<serde_json::Value>`. This mirrors `GameFinished::outcome()` returning
//! `Outcome` (not `Option<Outcome>`): the type carries the proof that the pointer
//! resolved, so we never need to check again.
//!
//! # Propositions and Contracts
//!
//! Every workflow tool documents **assumptions** and **establishes** propositions:
//!
//! ```text
//! parse_and_focus:     JsonParsed ∧ PointerResolved
//! validate_object:     JsonParsed ∧ IsObject ∧ RequiredKeysPresent
//! safe_merge:          IsObject(base) ∧ IsObject(patch)  ⟹  IsObject(result)
//! pointer_update:      JsonParsed ∧ PointerResolved  ⟹  UpdateApplied
//! field_chain:         ∀ key ∈ path. PointerResolved(root, key)
//! ```
//!
//! Registered under the `"json_workflow"` namespace.

use elicitation::contracts::{And, Established, both};
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is syntactically valid JSON.
#[derive(Prop)]
pub struct JsonParsed;
impl VerifiedWorkflow for JsonParsed {}

/// Proposition: the JSON value is an object (`serde_json::Map`), not array/string/etc.
#[derive(Prop)]
pub struct IsObject;
impl VerifiedWorkflow for IsObject {}

/// Proposition: a JSON Pointer (RFC 6901) path was resolved successfully in the document.
#[derive(Prop)]
pub struct PointerResolved;
impl VerifiedWorkflow for PointerResolved {}

/// Proposition: all specified required keys are present in a JSON object.
#[derive(Prop)]
pub struct RequiredKeysPresent;
impl VerifiedWorkflow for RequiredKeysPresent {}

/// Proposition: a pointer-targeted update was applied to the document.
#[derive(Prop)]
pub struct UpdateApplied;
impl VerifiedWorkflow for UpdateApplied {}

/// Composite: document is parsed AND its type is a JSON object.
pub type ParsedObject = And<JsonParsed, IsObject>;

/// Composite: parsed object with all required keys confirmed present.
pub type ValidatedObject = And<ParsedObject, RequiredKeysPresent>;

/// Composite: document parsed AND a pointer path was found within it.
pub type LocatedValue = And<JsonParsed, PointerResolved>;

// ── Select-pattern enums ──────────────────────────────────────────────────────

/// Merge strategy for two JSON objects.
///
/// Implements the Select pattern: the JSON schema restricts the caller to
/// exactly these variants, preventing ad-hoc strings.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ObjectMergeMode {
    /// RFC 7396 merge patch: null values in the patch **delete** the key in the base.
    MergePatch,
    /// Deep recursive merge: null values **overwrite** (no deletion semantics).
    DeepMerge,
}

/// Policy when a pointer path does not exist during an update.
///
/// Implements the Select pattern.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MissingKeyPolicy {
    /// Return an error — the path must already exist (strict mode).
    Error,
    /// Create intermediate objects as needed (permissive mode).
    CreatePath,
}

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated JSON input string — the initial state.
///
/// Like `GameSetup`: nothing has been proven yet. The only transition is `parse()`.
pub struct RawJson {
    src: String,
}

/// A successfully parsed JSON value with proof of syntactic validity.
///
/// Like `GameInProgress`: we have entered the game and can make moves (operations).
/// Can transition to `ObjectJson` (via `assert_object`) or `FocusedJson` (via `focus`).
pub struct ParsedJson {
    value: serde_json::Value,
}

/// A JSON value proven to be an object (`Map<String, Value>`).
///
/// Can only be constructed from `ParsedJson::assert_object()` — you cannot
/// construct an `ObjectJson` without proof that parsing succeeded first.
pub struct ObjectJson {
    map: serde_json::Map<String, serde_json::Value>,
}

/// A JSON value with a pointer path that has been proven to resolve.
///
/// Like `GameFinished`: the outcome is guaranteed.
/// **Key invariant**: `FocusedJson::extract()` returns `Value`, never `Option<Value>`.
/// The `PointerResolved` proof means we **never** need to check for absence again.
pub struct FocusedJson {
    root: serde_json::Value,
    /// Guaranteed to exist — the proof says so.
    focus: serde_json::Value,
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl RawJson {
    /// Wrap a raw string as unvalidated JSON input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input, establishing `JsonParsed` proof on success.
    ///
    /// This is the only way to enter the validated document space.
    /// Analogous to `GameSetup::start()` — you can't make moves until you call this.
    pub fn parse(self) -> Result<(ParsedJson, Established<JsonParsed>), String> {
        match serde_json::from_str::<serde_json::Value>(&self.src) {
            Ok(value) => Ok((ParsedJson { value }, Established::assert())),
            Err(e) => Err(format!("JsonParsed not established: {e}")),
        }
    }
}

impl ParsedJson {
    /// Consume and return the inner `serde_json::Value`.
    ///
    /// Use when you need to operate on the raw value directly after parsing.
    pub fn into_value(self) -> serde_json::Value {
        self.value
    }

    /// Apply a pointer update in one step: resolve the pointer and write the new value.
    ///
    /// Combines `focus` + `FocusedJson::update` into a single ergonomic call.
    /// `MissingKeyPolicy::CreatePath` will create intermediate nodes; `Error` returns `Err`.
    pub fn pointer_update(
        self,
        ptr: &str,
        new_value: serde_json::Value,
        policy: MissingKeyPolicy,
        proof: Established<JsonParsed>,
    ) -> Result<(serde_json::Value, Established<UpdateApplied>), String> {
        // For CreatePath: keep a copy before focus consumes self.
        let value_copy = if matches!(policy, MissingKeyPolicy::CreatePath) {
            Some(self.value.clone())
        } else {
            None
        };
        match self.focus(ptr, proof) {
            Ok((focused, _located_proof)) => {
                let update_proof: Established<UpdateApplied> = Established::assert();
                let updated = focused.update(new_value, ptr, update_proof);
                Ok((updated, Established::assert()))
            }
            Err(focus_err) => match policy {
                MissingKeyPolicy::Error => Err(focus_err),
                MissingKeyPolicy::CreatePath => {
                    let mut doc = value_copy.unwrap_or_default();
                    set_pointer(&mut doc, ptr, new_value);
                    Ok((doc, Established::assert()))
                }
            },
        }
    }

    /// Assert that this value is a JSON object, establishing `ParsedObject` proof.
    ///
    /// Like `validate_square_empty()` in tic-tac-toe: establishes a precondition
    /// that downstream operations depend on.
    pub fn assert_object(
        self,
        proof: Established<JsonParsed>,
    ) -> Result<(ObjectJson, Established<ParsedObject>), String> {
        match self.value {
            serde_json::Value::Object(map) => {
                let obj_proof: Established<IsObject> = Established::assert();
                Ok((ObjectJson { map }, both(proof, obj_proof)))
            }
            other => Err(format!(
                "IsObject not established: expected object, got {}",
                json_type_name(&other)
            )),
        }
    }

    /// Focus on a JSON Pointer path, establishing `LocatedValue` proof.
    ///
    /// On success, the returned `FocusedJson` guarantees the pointer resolved —
    /// `FocusedJson::extract()` will never return `None`.
    pub fn focus(
        self,
        ptr: &str,
        proof: Established<JsonParsed>,
    ) -> Result<(FocusedJson, Established<LocatedValue>), String> {
        match self.value.pointer(ptr).cloned() {
            Some(focus) => {
                let ptr_proof: Established<PointerResolved> = Established::assert();
                Ok((
                    FocusedJson {
                        root: self.value,
                        focus,
                    },
                    both(proof, ptr_proof),
                ))
            }
            None => Err(format!(
                "PointerResolved not established: pointer '{ptr}' did not resolve"
            )),
        }
    }
}

impl ObjectJson {
    /// Validate that all required keys are present, establishing `ValidatedObject` proof.
    ///
    /// Like `validate_player_turn()` in tic-tac-toe: a second precondition check
    /// that must pass before executing the operation.
    pub fn validate_required(
        self,
        keys: &[&str],
        proof: Established<ParsedObject>,
    ) -> Result<(ObjectJson, Established<ValidatedObject>), String> {
        let missing: Vec<&str> = keys
            .iter()
            .copied()
            .filter(|k| !self.map.contains_key(*k))
            .collect();
        if missing.is_empty() {
            let keys_proof: Established<RequiredKeysPresent> = Established::assert();
            Ok((self, both(proof, keys_proof)))
        } else {
            Err(format!(
                "RequiredKeysPresent not established: missing keys {:?}",
                missing
            ))
        }
    }

    /// Consume and return as `serde_json::Value`.
    ///
    /// Note: no `Option` — the object is always valid by construction.
    pub fn into_value(self) -> serde_json::Value {
        serde_json::Value::Object(self.map)
    }

    /// Apply a merge patch to this object, consuming both.
    ///
    /// Requires proof that BOTH operands are objects, preventing merge
    /// against arrays, strings, or null values.
    pub fn merge(
        self,
        patch: ObjectJson,
        _proof: Established<And<ParsedObject, ParsedObject>>,
        mode: &ObjectMergeMode,
    ) -> serde_json::Value {
        // Proof guarantees: self is an object AND patch is an object.
        // No type checks needed — the contract enforces this.
        let mut base = self.map;
        match mode {
            ObjectMergeMode::MergePatch => {
                for (k, v) in patch.map {
                    if v.is_null() {
                        base.remove(&k);
                    } else {
                        base.insert(k, v);
                    }
                }
            }
            ObjectMergeMode::DeepMerge => {
                for (k, v) in patch.map {
                    let entry = base.entry(k).or_insert(serde_json::Value::Null);
                    deep_merge_value(entry, v);
                }
            }
        }
        serde_json::Value::Object(base)
    }
}

impl FocusedJson {
    /// Extract the focused value — **not** `Option<Value>`.
    ///
    /// This is the central invariant of `FocusedJson`:
    /// - `FocusedJson::extract()` returns `Value`
    /// - `GameFinished::outcome()` returns `Outcome`
    ///
    /// In both cases, the type carries the proof that the value exists.
    /// No runtime check, no unwrap, no panic possible.
    pub fn extract(self) -> serde_json::Value {
        self.focus
    }

    /// Get a reference to the root document.
    pub fn root(&self) -> &serde_json::Value {
        &self.root
    }

    /// Update the focused path in the root and return the mutated document.
    ///
    /// Consumes self to return ownership of the root.
    pub fn update(
        mut self,
        new_value: serde_json::Value,
        ptr: &str,
        _proof: Established<UpdateApplied>,
    ) -> serde_json::Value {
        // Proof guarantees the pointer resolved — safe to write back.
        set_pointer(&mut self.root, ptr, new_value);
        self.root
    }
}

// ── Parameter types ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
struct ParseFocusParams {
    /// JSON string to parse. Assumes: syntactically valid JSON.
    json: String,
    /// RFC 6901 JSON Pointer (e.g. `"/user/address/city"`).
    /// Assumes: resolves to an existing path in the document.
    pointer: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ValidateObjectParams {
    /// JSON string to parse and validate. Assumes: valid JSON object.
    json: String,
    /// Keys that must all be present. If any are missing, the tool returns an error.
    required_keys: Vec<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MergeParams {
    /// Base JSON object. Assumes: syntactically valid JSON **object**.
    base: serde_json::Value,
    /// Patch JSON object. Assumes: syntactically valid JSON **object**.
    patch: serde_json::Value,
    /// Merge strategy.
    mode: ObjectMergeMode,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PointerUpdateParams {
    /// JSON document to update. Assumes: syntactically valid JSON.
    json: String,
    /// RFC 6901 JSON Pointer identifying the value to replace.
    /// Assumes: resolves to an existing path (or `missing_key_policy = create_path`).
    pointer: String,
    /// Replacement value.
    new_value: serde_json::Value,
    /// What to do if the pointer path does not exist.
    missing_key_policy: MissingKeyPolicy,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct FieldChainParams {
    /// JSON string to traverse. Assumes: syntactically valid JSON.
    json: String,
    /// Ordered list of object keys to descend through (e.g. `["user", "address", "city"]`).
    /// Each key is resolved in turn; the tool fails at the first missing key.
    path: Vec<String>,
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

/// Recursively merge `src` into `dst` (deep merge, no null-deletion semantics).
fn deep_merge_value(dst: &mut serde_json::Value, src: serde_json::Value) {
    match (dst, src) {
        (serde_json::Value::Object(d), serde_json::Value::Object(s)) => {
            for (k, v) in s {
                deep_merge_value(d.entry(k).or_insert(serde_json::Value::Null), v);
            }
        }
        (dst, src) => *dst = src,
    }
}

/// Write `value` at `pointer` path in `root`, creating intermediate objects if needed.
fn set_pointer(root: &mut serde_json::Value, pointer: &str, value: serde_json::Value) {
    if pointer.is_empty() {
        *root = value;
        return;
    }
    // Split RFC 6901 pointer into segments, unescape ~1 → / and ~0 → ~
    let segments: Vec<String> = pointer
        .trim_start_matches('/')
        .split('/')
        .map(|s| s.replace("~1", "/").replace("~0", "~"))
        .collect();

    let mut current = root;
    let last = segments.len().saturating_sub(1);
    for (i, seg) in segments.iter().enumerate() {
        if i == last {
            if let serde_json::Value::Object(map) = current {
                map.insert(seg.clone(), value);
            }
            return;
        }
        current = match current {
            serde_json::Value::Object(map) => map
                .entry(seg.clone())
                .or_insert_with(|| serde_json::Value::Object(Default::default())),
            _ => return,
        };
    }
}

/// Parse a value-as-object, returning `(ObjectJson, Established<ParsedObject>)`.
fn parse_as_object(
    value: serde_json::Value,
) -> Result<(ObjectJson, Established<ParsedObject>), String> {
    let parsed_proof: Established<JsonParsed> = Established::assert();
    let parsed = ParsedJson { value };
    parsed.assert_object(parsed_proof)
}

/// MCP plugin providing verified JSON workflow tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "json_workflow")]
pub struct JsonWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "json_workflow",
    name = "parse_and_focus",
    description = "Parse a JSON string and resolve a RFC 6901 JSON Pointer path in one atomic \
                   step. Establishes: JsonParsed ∧ PointerResolved. \
                   The focused value is guaranteed to exist — no null check needed.",
    emit = ParseAndFocusEmit
)]
#[instrument(skip_all)]
async fn wf_parse_and_focus(p: ParseFocusParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match RawJson::new(p.json).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (focused, _located_proof) = match parsed.focus(&p.pointer, parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let value = focused.extract();
    let result = serde_json::json!({
        "value": value,
        "contract": "JsonParsed ∧ PointerResolved",
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "json_workflow",
    name = "validate_object",
    description = "Parse a JSON string, assert it is an object, and verify all required keys \
                   are present. Establishes: JsonParsed ∧ IsObject ∧ RequiredKeysPresent. \
                   Returns the validated object with a contract summary.",
    emit = ValidateObjectEmit
)]
#[instrument(skip_all)]
async fn wf_validate_object(p: ValidateObjectParams) -> Result<CallToolResult, ErrorData> {
    let required: Vec<&str> = p.required_keys.iter().map(|s| s.as_str()).collect();
    let (parsed, parsed_proof) = match RawJson::new(p.json).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (obj, obj_proof) = match parsed.assert_object(parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (validated, _validated_proof) = match obj.validate_required(&required, obj_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let value = validated.into_value();
    let result = serde_json::json!({
        "value": value,
        "contract": "JsonParsed ∧ IsObject ∧ RequiredKeysPresent",
        "required_keys": p.required_keys,
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "json_workflow",
    name = "safe_merge",
    description = "Merge two JSON objects after proving BOTH are objects (not arrays or scalars). \
                   Establishes: IsObject(base) ∧ IsObject(patch) ⟹ IsObject(result). \
                   Choose merge_mode: 'merge_patch' (RFC 7396, nulls delete) or \
                   'deep_merge' (recursive, nulls overwrite).",
    emit = SafeMergeEmit
)]
#[instrument(skip_all)]
async fn wf_safe_merge(p: MergeParams) -> Result<CallToolResult, ErrorData> {
    let (base_obj, base_proof) = match parse_as_object(p.base) {
        Ok(r) => r,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "base: {e}"
            ))]));
        }
    };
    let (patch_obj, patch_proof) = match parse_as_object(p.patch) {
        Ok(r) => r,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "patch: {e}"
            ))]));
        }
    };
    let combined_proof = both(base_proof, patch_proof);
    let merged = base_obj.merge(patch_obj, combined_proof, &p.mode);
    let result = serde_json::json!({
        "value": merged,
        "contract": "IsObject(base) ∧ IsObject(patch) ⟹ IsObject(result)",
        "mode": format!("{:?}", p.mode),
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "json_workflow",
    name = "pointer_update",
    description = "Parse a JSON document, resolve a pointer path to prove it exists, then write \
                   a new value at that path. \
                   Establishes: JsonParsed ∧ PointerResolved ⟹ UpdateApplied. \
                   Use missing_key_policy to control behavior when the path is absent.",
    emit = PointerUpdateEmit
)]
#[instrument(skip_all)]
async fn wf_pointer_update(p: PointerUpdateParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match RawJson::new(&p.json).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let result = match parsed.focus(&p.pointer, parsed_proof) {
        Ok((focused, _located_proof)) => {
            let update_proof: Established<UpdateApplied> = Established::assert();
            let updated = focused.update(p.new_value, &p.pointer, update_proof);
            serde_json::json!({
                "value": updated,
                "contract": "JsonParsed ∧ PointerResolved ⟹ UpdateApplied",
                "pointer": p.pointer,
            })
        }
        Err(focus_err) => match p.missing_key_policy {
            MissingKeyPolicy::Error => {
                return Ok(CallToolResult::error(vec![Content::text(focus_err)]));
            }
            MissingKeyPolicy::CreatePath => {
                let mut doc: serde_json::Value = serde_json::from_str(&p.json).unwrap_or_default();
                set_pointer(&mut doc, &p.pointer, p.new_value);
                serde_json::json!({
                    "value": doc,
                    "contract": "JsonParsed ∧ UpdateApplied (path created)",
                    "pointer": p.pointer,
                })
            }
        },
    };
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "json_workflow",
    name = "field_chain",
    description = "Traverse a chain of object keys, proving each step exists before descending. \
                   Fails at the first missing key with the path consumed so far. \
                   Establishes: ∀ key ∈ path. PointerResolved(root, key). \
                   Returns the leaf value and the resolved path.",
    emit = FieldChainEmit
)]
#[instrument(skip_all)]
async fn wf_field_chain(p: FieldChainParams) -> Result<CallToolResult, ErrorData> {
    let (mut parsed, mut current_proof) = match RawJson::new(p.json).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let mut resolved_path = String::new();
    for key in &p.path {
        let ptr = format!("{resolved_path}/{key}");
        match parsed.focus(&ptr, current_proof) {
            Ok((focused, located_proof)) => {
                resolved_path = ptr;
                let next_value = focused.extract();
                parsed = ParsedJson { value: next_value };
                current_proof = elicitation::contracts::fst(located_proof);
            }
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "PointerResolved not established at '{ptr}': {e}"
                ))]));
            }
        }
    }
    let result = serde_json::json!({
        "value": parsed.value,
        "contract": format!("∀ key ∈ {:?}. PointerResolved", p.path),
        "resolved_path": resolved_path,
    });
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

// ── CustomEmit ZSTs ───────────────────────────────────────────────────────────
// Each implements CustomEmit<P> to emit the full typestate sequence for its tool.
// The #[elicit_tool(emit = XxxEmit)] macro generates impl EmitCode for XxxParams
// delegating to these, plus register_emit! inventory submission.

/// Custom emit for `parse_and_focus`.
pub struct ParseAndFocusEmit;

impl elicitation::emit_code::CustomEmit<ParseFocusParams> for ParseAndFocusEmit {
    fn emit_code(params: &ParseFocusParams) -> elicitation::proc_macro2::TokenStream {
        let json = &params.json;
        let pointer = &params.pointer;
        quote::quote! {
            let _raw = elicit_serde_json::RawJson::new(#json.to_string());
            let (_parsed, _json_proof) = _raw.parse()
                .map_err(|e| format!("JSON parse failed: {}", e))?;
            let (_focused, _focus_proof) = _parsed.focus(#pointer, _json_proof)
                .map_err(|e| format!("Pointer resolution failed: {}", e))?;
            let _value = _focused.extract();
        }
    }
}

/// Custom emit for `validate_object`.
pub struct ValidateObjectEmit;

impl elicitation::emit_code::CustomEmit<ValidateObjectParams> for ValidateObjectEmit {
    fn emit_code(params: &ValidateObjectParams) -> elicitation::proc_macro2::TokenStream {
        let json = &params.json;
        let keys = &params.required_keys;
        quote::quote! {
            let _raw = elicit_serde_json::RawJson::new(#json.to_string());
            let (_parsed, _proof) = _raw.parse()
                .map_err(|e| format!("JSON parse failed: {}", e))?;
            let (_obj, _obj_proof) = _parsed.assert_object(_proof)
                .map_err(|e| format!("Not a JSON object: {}", e))?;
            let (_validated, _val_proof) = _obj.validate_required(
                &[ #( #keys ),* ],
                _obj_proof,
            ).map_err(|e| format!("Missing required keys: {}", e))?;
        }
    }
}

/// Custom emit for `safe_merge`.
pub struct SafeMergeEmit;

impl elicitation::emit_code::CustomEmit<MergeParams> for SafeMergeEmit {
    fn emit_code(params: &MergeParams) -> elicitation::proc_macro2::TokenStream {
        let base = params.base.to_string();
        let patch = params.patch.to_string();
        let mode_expr = match params.mode {
            ObjectMergeMode::MergePatch => {
                quote::quote! { elicit_serde_json::ObjectMergeMode::MergePatch }
            }
            ObjectMergeMode::DeepMerge => {
                quote::quote! { elicit_serde_json::ObjectMergeMode::DeepMerge }
            }
        };
        quote::quote! {
            let _base_raw = elicit_serde_json::RawJson::new(#base.to_string());
            let (_base_parsed, _base_proof) = _base_raw.parse()
                .map_err(|e| format!("Base JSON parse failed: {}", e))?;
            let (_base_obj, _base_obj_proof) = _base_parsed.assert_object(_base_proof)
                .map_err(|e| format!("Base is not a JSON object: {}", e))?;

            let _patch_raw = elicit_serde_json::RawJson::new(#patch.to_string());
            let (_patch_parsed, _patch_proof) = _patch_raw.parse()
                .map_err(|e| format!("Patch JSON parse failed: {}", e))?;
            let (_patch_obj, _patch_obj_proof) = _patch_parsed.assert_object(_patch_proof)
                .map_err(|e| format!("Patch is not a JSON object: {}", e))?;

            let _both_proof = elicitation::contracts::both(_base_obj_proof, _patch_obj_proof);
            let _merged = _base_obj.merge(_patch_obj, _both_proof, &#mode_expr);
            println!("{}", serde_json::to_string_pretty(&_merged).unwrap_or_default());
        }
    }
}

/// Custom emit for `pointer_update`.
pub struct PointerUpdateEmit;

impl elicitation::emit_code::CustomEmit<PointerUpdateParams> for PointerUpdateEmit {
    fn emit_code(params: &PointerUpdateParams) -> elicitation::proc_macro2::TokenStream {
        let json = &params.json;
        let pointer = &params.pointer;
        let new_value = params.new_value.to_string();
        let policy_expr = match params.missing_key_policy {
            MissingKeyPolicy::Error => {
                quote::quote! { elicit_serde_json::MissingKeyPolicy::Error }
            }
            MissingKeyPolicy::CreatePath => {
                quote::quote! { elicit_serde_json::MissingKeyPolicy::CreatePath }
            }
        };
        quote::quote! {
            let _raw = elicit_serde_json::RawJson::new(#json.to_string());
            let (_parsed, _proof) = _raw.parse()
                .map_err(|e| format!("JSON parse failed: {}", e))?;
            let _new_val: serde_json::Value = serde_json::from_str(#new_value)
                .map_err(|e| format!("Invalid replacement value: {}", e))?;
            let (_updated, _update_proof) = _parsed.pointer_update(
                #pointer,
                _new_val,
                #policy_expr,
                _proof,
            ).map_err(|e| format!("Pointer update failed: {}", e))?;
            println!("{}", serde_json::to_string_pretty(&_updated).unwrap_or_default());
        }
    }
}

/// Custom emit for `field_chain`.
pub struct FieldChainEmit;

impl elicitation::emit_code::CustomEmit<FieldChainParams> for FieldChainEmit {
    fn emit_code(params: &FieldChainParams) -> elicitation::proc_macro2::TokenStream {
        let json = &params.json;
        let keys = &params.path;
        quote::quote! {
            let _raw = elicit_serde_json::RawJson::new(#json.to_string());
            let (_parsed, _proof) = _raw.parse()
                .map_err(|e| format!("JSON parse failed: {}", e))?;
            let mut _current = _parsed.into_value();
            for _key in &[ #( #keys ),* ] {
                let _key_ptr = format!("/{}", _key);
                let _step_raw = elicit_serde_json::RawJson::new(_current.to_string());
                let (_step_parsed, _step_proof) = _step_raw.parse()
                    .map_err(|e| format!("Parse failed at key '{}': {}", _key, e))?;
                let (_focused, _) = _step_parsed.focus(&_key_ptr, _step_proof)
                    .map_err(|e| format!("Key '{}' not found: {}", _key, e))?;
                _current = _focused.extract();
            }
            println!("{}", serde_json::to_string_pretty(&_current).unwrap_or_default());
        }
    }
}
