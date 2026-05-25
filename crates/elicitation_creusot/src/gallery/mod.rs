//! Creusot proof gallery for VSM invariant expressibility.
//!
//! Each level tests a specific hypothesis about what Pearlite can express,
//! mirroring the methodology used in the Kani gallery.  Unlike Kani (which
//! runs CBMC and measures wall-clock time), Creusot gallery levels are
//! validated by:
//!
//! 1. **Compilation** (`cargo creusot`): annotations parse and type-check
//!    in Pearlite.  Failure here means the predicate is inexpressible.
//! 2. **Goal discharge** (Why3 + Alt-Ergo/Z3): the generated WhyML proof
//!    obligations close.  For trivial goals this takes < 1 s.
//!
//! ## Gallery levels
//!
//! | Level        | Subject                              | Key question                                    |
//! |--------------|--------------------------------------|-------------------------------------------------|
//! | [`level1`]  | Unit type, trivial invariant         | Does basic `#[logic]` / `#[requires]` work?    |
//! | [`level2`]  | Integer bounds (`@` model)           | Can Pearlite do arithmetic?                     |
//! | [`level3`]  | Unit enum, `match` in `#[logic]`    | Can predicates match on enum variants?          |
//! | [`level4`]  | String length via `@.len()`          | Can Pearlite reason about String?               |
//! | [`level5`]  | Data-carrying enum (String payload)  | Invariant over data variant field?              |
//! | [`level6`]  | Composition (postcond → precond)    | Is composition free (no `stub_verified`)?       |
//! | [`level7`]  | Named struct fields in variants      | Can predicates access named enum struct fields? |
//! | [`level8`]  | Machine wrapper struct               | Invariant over state enum + numeric metadata?   |
//! | [`level9`]  | Contract chains vs proof tokens      | Is `Established<P>` unnecessary in Creusot?     |
//! | [`level10`] | Full mini connection machine         | Complete 4-state VSM lifecycle provable?        |
//! | [`level11`] | Panel machine with nested enum       | Nested enum field access in pearlite?           |
//! | [`level12`] | Two-machine composition + gating     | Cross-machine invariant (panel gates on conn)?  |
//! | [`level13`] | Machine wrapper + transition counter | Exact counter postconditions chain `below_max`? |
//! | [`level14`] | Two counters + relational invariant  | Can `error_count ≤ transition_count` be proved? |
//! | [`level15`] | Six-variant state, two-field variant | Two-field `Connecting`, tag propagation through `match`? |
//! | [`level16`] | Nested struct field in enum variant  | Struct consistency predicate, field propagation across variants? |
//! | [`level17`] | Nested enum field + backend routing  | Nested enum match in `#[logic]`; routing guard chains through lifecycle? |
//! | [`level18`] | Full `ArchiveConnectionState` replica | All C15–C17 patterns combined; production-identical structure? |
//! | [`level19`] | `ArchiveConnectionMachine` wrapper    | Counter + routing guards + struct propagation simultaneously?  |
//! | [`level20`] | `Option<T>` fields + bool implication | `Option<String>`/`Option<Struct>` in `#[logic]`; `*running ==>` invariant? |
//! | [`level21`] | `Vec<T>` sequence + `usize` cursor    | `forall` quantifier over Vec; cursor-in-bounds after saturating move?      |
//! | [`level22`] | `Box<T>` field access in `#[logic]`   | Pearlite deref of boxed struct fields; invariant on boxed content?         |
//! | [`level23`] | 18-variant scale test (panel machine) | Alt-Ergo closes 18-arm match? All patterns combined at production scale?   |
//! | [`level24`] | Depth-bounded inductive closure       | `depth > 0 ==> Ok`? Well-foundedness of depth-decrement? Error at limit?  |
//! | [`level25`] | Harness delegation vs inlining        | Does a delegated `__creusot` harness prove if the callee spec is visible? |
//! | [`level26`] | `#[instrument]` delegation + `format!` pitfalls | `{ f(args) }` body rewritten to `__creusot`? `String::new()` safe for labels? |
//! | [`level31`] | `formal_method` vs raw `#[instrument]` | Is the current Creusot ICE caused by raw tracing, `formal_method`, or their combination? |
//! | [`level32`] | source invariant fn vs generated logic wrapper | Does the remaining downstream ICE require the valinoreth-style split between exec predicate and generated `#[logic]` wrapper? |
//! | [`level33`] | tiny combat VSM with Vec state | Does the remaining downstream ICE require a real `VerifiedStateMachine` over `Vec`-backed enum state with shared `formal_method` contracts, and if so which ingredient is decisive? |
//! | [`level34`] | simplest real VSM | If C33 still ICEs with one transition, does the crash persist when the state is reduced to the smallest real ElicitComplete enum with no `Vec` fields? |
//! | [`level35`] | same formal surface, no VSM derive | If C34 still ICEs, is `VerifiedStateMachine` itself required, or is the tiny same-crate `formal_method` + `ElicitComplete` surface already enough? |
//! | [`level36`] | same state/props, no formal_method | If C35 still ICEs, does removing `#[formal_method]` eliminate the crash, or is the tiny same-crate state+prop packaging already sufficient? |
//! | [`level37`] | same state only | If C36 still ICEs, does the tiny `Elicit`/`ElicitComplete` state surface alone trigger the crash, or are the proposition items required? |
//! | [`level38`] | `Elicit` only state | If C37 still ICEs after proper feature isolation, is `#[derive(Elicit)]` alone enough to trigger the crash? |
//! | [`level39`] | `KaniVariantState` only state | Or is `#[derive(KaniVariantState)]` the decisive state-side ingredient instead? |
//! | [`level40`] | manual `Elicitation`, empty proofs | If C38 ICEs, does a hand-written `Elicitation` impl with concrete futures and no helper-backed `creusot_proof()` compile cleanly? |
//! | [`level41`] | manual `Elicitation`, helper-backed Creusot proof | If C40 is clean, does adding only the `creusot_single_variant_enum` / `creusot_multi_variant_enum` helpers reintroduce the ICE? |
//! | [`level42`] | manual traced async `Elicitation` | If C40/C41 are clean, do the traced `async fn elicit` methods from `derive(Elicit)` reintroduce the Creusot ICE even with trivial proof methods? |
//! | [`level43`] | `Vec::new()` + `push` | If `vec![..]` is the bad MIR shape, can Creusot still handle manual `Vec` construction for options/labels? |
//! | [`level44`] | `Vec::from([..])` | If not, does array-to-`Vec` conversion preserve the `Vec` model while avoiding the bad lowering? |
//! | [`level45`] | label matching on `&str` | If `Vec` construction is fine, is the next Creusot blocker the `from_label` string-match shape itself? |
//! | [`level46`] | label comparison via bytes | If `&str` pattern matching is the problem, does comparing `label.as_bytes()` to byte literals preserve behavior without the bad shape? |
//! | [`level47`] | generic params with `ElicitComplete` | Does the `reflect_methods` failure reduce to a generic parameter struct using `#[serde(bound = \"\")]` plus only `ElicitComplete` under Creusot? |
//! | [`level48`] | generic params with explicit wire bounds | If C47 fails, do explicit `Serialize + Deserialize + JsonSchema` bounds remove the local error? |
//! | [`level49`] | generic serde with empty bound | If C48 still fails, is `#[serde(bound = \"\")]` itself the bound-erasing shape, even without `#[derive(Elicit)]`? |
//! | [`level50`] | generic serde with inferred bounds | If so, does removing the empty serde bound restore the local generic derive without touching `ElicitComplete`? |
//! | [`level51`] | serde derives with explicit bounds | If C49/C50 fail the same way, do explicit serde derive bounds satisfy the generic wire-format surface under Creusot? |
//! | [`level52`] | generic `JsonSchema` only | If serde is fixed, is generic `JsonSchema` derive a separate Creusot blocker? |
//! | [`level53`] | generated extern wrapper + `Option<String>` | Does the remaining downstream ICE reduce to the generated Creusot wrapper shape once an `Option<String>` argument is present? |
//! | [`level54`] | combat-like generated module, no tracing | If C53 is clean, does a larger valinoreth-shaped companion without `#[instrument]` reproduce the crash? |
//! | [`level55`] | generated wrapper through re-exported function path | If C54 is clean, does targeting a re-exported source function reproduce the remaining downstream crash? |
//! | [`level56`] | nested `Prop` evidence bundles | If combat is exonerated, does the remaining crash reduce to valinoreth-style non-VSM `#[derive(Prop)]` bundles over `Established<P>` fields and nested bundle structs? |
//! | [`level57`] | crate-local source/generated module tree | If workspace-local shapes are all clean, does the crash require valinoreth-style crate-local packaging: `vsm::combat`, root re-export, and `proofs::creusot::generated::{elicitation_specs, combat}` together? |
//! | [`level58`] | split-file crate-local module tree | If C57 is still clean, does the remaining crash require the same packaging spread across real files/modules rather than inline nested modules? |
//!
//! ## Run all levels
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```
//!
//! The output is WhyML in `verif/elicitation_creusot_rlib/`.
//! Each level's functions appear as separate Why3 modules.
//!
//! ## Prove gallery
//!
//! ```bash
//! just verify-creusot-gallery
//! ```

pub mod level1;
pub mod level10;
pub mod level11;
pub mod level12;
pub mod level13;
pub mod level14;
pub mod level15;
pub mod level16;
pub mod level17;
pub mod level18;
pub mod level19;
pub mod level2;
pub mod level20;
pub mod level21;
pub mod level22;
pub mod level23;
pub mod level24;
pub mod level25;
pub mod level26;
pub mod level27;
pub mod level28;
pub mod level29;
pub mod level3;
pub mod level30;
pub mod level31;
pub mod level32;
#[cfg(any(
    feature = "gallery-c33-vsm-combat",
    feature = "gallery-c33-one-transition",
    feature = "gallery-c33-three-transitions"
))]
pub mod level33;
#[cfg(feature = "gallery-c34-simple-vsm")]
pub mod level34;
#[cfg(feature = "gallery-c35-formal-no-vsm")]
pub mod level35;
#[cfg(feature = "gallery-c36-state-props-only")]
pub mod level36;
#[cfg(feature = "gallery-c37-state-only")]
pub mod level37;
#[cfg(feature = "gallery-c38-elicit-only")]
pub mod level38;
#[cfg(feature = "gallery-c39-kani-variant-only")]
pub mod level39;
#[cfg(feature = "gallery-c40-manual-empty-proof")]
pub mod level40;
#[cfg(feature = "gallery-c41-manual-helper-proof")]
pub mod level41;
#[cfg(feature = "gallery-c42-manual-async-traced")]
pub mod level42;
#[cfg(feature = "gallery-c43-vec-push")]
pub mod level43;
#[cfg(feature = "gallery-c44-vec-from-array")]
pub mod level44;
#[cfg(feature = "gallery-c45-label-match")]
pub mod level45;
#[cfg(feature = "gallery-c46-label-bytes")]
pub mod level46;
#[cfg(feature = "gallery-c47-generic-param-elicit-complete")]
pub mod level47;
#[cfg(feature = "gallery-c48-generic-param-wire-bounds")]
pub mod level48;
#[cfg(feature = "gallery-c49-serde-empty-bound")]
pub mod level49;
#[cfg(feature = "gallery-c50-serde-inferred-bound")]
pub mod level50;
#[cfg(feature = "gallery-c51-serde-explicit-bounds")]
pub mod level51;
#[cfg(feature = "gallery-c52-jsonschema-generic")]
pub mod level52;
pub mod level4;
pub mod level5;
#[cfg(feature = "gallery-c53-generated-option-string")]
pub mod level53;
#[cfg(feature = "gallery-c54-generated-combat-module")]
pub mod level54;
#[cfg(feature = "gallery-c55-generated-reexport-path")]
pub mod level55;
#[cfg(feature = "gallery-c56-nested-prop-bundles")]
pub mod level56;
#[cfg(feature = "gallery-c57-crate-local-module-tree")]
pub mod level57;
#[cfg(feature = "gallery-c58-split-module-tree")]
pub mod level58;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;
