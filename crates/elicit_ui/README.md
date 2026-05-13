# elicit_ui

Formally verified UI construction with compile-time WCAG 2.2 compliance
guarantees, built on AccessKit as a universal intermediate representation.

## Overview

`elicit_ui` models UI construction as a proof-carrying pipeline. Every
interactive element is built through a factory trait that performs a WCAG
runtime check and, on success, returns both the constructed element **and** a
typed proof token — an `Established<P>` — that records what was verified.
Proof tokens compose upward through section aggregates into a final
`Established<WcagLevelAAValid>`, which is the only legal way to assert that a
surface is WCAG 2.2 Level AA compliant.

The compiler enforces this chain. There is no way to produce
`Established<WcagLevelAAValid>` without assembling all the leaf proofs from
which it is derived.

---

## Architecture

```text
  ┌─────────────────────────────────────────────────────────────────┐
  │                     WCAG Factory Traits                         │
  │  WcagContrastFactory · WcagLabelFactory · WcagFocusFactory      │
  │  WcagKeyboardFactory · WcagTimingFactory · WcagTargetFactory     │
  │  WcagStructureFactory · WcagMediaFactory · WcagLanguageFactory   │
  │  WcagErrorFactory                                               │
  └────────────────────────────┬────────────────────────────────────┘
                               │ each method returns (Element, Established<Leaf>)
                               ▼
  ┌─────────────────────────────────────────────────────────────────┐
  │               Section Aggregate Factories                        │
  │   WcagPerceivedFactory   →  Established<WcagPerceivedValid>     │
  │   WcagOperableFactory    →  Established<WcagOperableValid>      │
  │   WcagUnderstandableFactory → Established<WcagUnderstandableValid>│
  │   WcagRobustFactory      →  Established<WcagRobustValid>        │
  └────────────────────────────┬────────────────────────────────────┘
                               │ assemble LevelAaEvidence
                               ▼
  ┌─────────────────────────────────────────────────────────────────┐
  │                     WcagBackend supertrait                       │
  │      build_level_aa(evidence) → Established<WcagLevelAAValid>   │
  └────────────────────────────┬────────────────────────────────────┘
                               │
               ┌───────────────┼───────────────┐
               ▼               ▼               ▼
         elicit_egui     elicit_ratatui   elicit_leptos
       (AccessKit IR)   (Ratatui cells)  (Leptos SSR/WASM)
```

The reference implementation of all factory traits is `AccessKitUiBackend`,
which builds AccessKit `Node` trees and issues proof tokens. Renderer crates
receive proof tokens through the IR but cannot forge them.

---

## Proof Architecture

### Proposition types

Every verifiable WCAG invariant has a corresponding Rust type — a
*proposition* — that implements `elicitation::contracts::Prop`. These types
are zero-cost phantoms that exist only at the type level.

```rust
pub struct WcagContrastMinimumNormalText;  // SC 1.4.3, normal text ≥ 4.5:1
pub struct WcagFocusVisibleKeyboard;       // SC 2.4.7
pub struct WcagLevelAAValid;               // the full Level AA composite
```

### Proof tokens

`Established<P>` is the proof that proposition `P` holds. It is a zero-sized
type that carries no runtime data — only type-level evidence.

### The `ProvableFrom<C>` trait

The credential-gated minting path is `Established::prove`:

```rust
impl Established<P> {
    pub fn prove<C>(_: &C) -> Self  where P: ProvableFrom<C> { … }
}
```

`ProvableFrom<C>` declares "credential `C` proves proposition `P`". The
46 impls live in `elicit_ui::contracts::wcag_proofs`:

```rust
impl ProvableFrom<NormalTextContrastVerified> for WcagContrastMinimumNormalText {}
impl ProvableFrom<FocusVisibleVerified>       for WcagFocusVisibleKeyboard {}
// … 44 more
```

### Credential types

Each credential is a `pub(crate)` ZST in `proof_credentials`. External code
cannot construct one:

```rust
// pub(crate) — only factory methods inside elicit_ui can build this
pub(crate) struct NormalTextContrastVerified;
```

The factory method constructs the credential *after* performing the runtime
check, then passes it to `prove`:

```rust
fn build_contrast_minimum(&self, input: ContrastDescriptor)
    -> UiResult<(ContrastPair, Established<WcagContrastMinimumNormalText>)>
{
    let ratio = contrast_ratio(&input.foreground, &input.background);
    if ratio < 4.5 {
        return Err(UiError::new(UiErrorKind::InsufficientContrast(…)));
    }
    let pair = ContrastPair { … };
    Ok((pair, Established::prove(&NormalTextContrastVerified)))
    //                                  ^^^^^^^^^^^^^^^^^^^
    //  credential constructed here, after the check, and nowhere else
}
```

### Why this is a compile-time guarantee

- `NormalTextContrastVerified` is `pub(crate)` — external crates cannot write
  `Established::prove(&NormalTextContrastVerified)`.
- `ProvableFrom<NormalTextContrastVerified>` is only implemented for
  `WcagContrastMinimumNormalText` — no other proposition can be proved with
  that credential.
- `Established::assert()` remains `pub` as a general escape hatch, but any
  call to `assert()` on a WCAG proposition is immediately visible in code
  review and audit tooling as an explicit bypass.

---

## WCAG 2.2 Contract Parity

### Leaf propositions (41)

| Factory trait | Factory method | Proposition | WCAG SC | Level |
|---|---|---|---|---|
| `WcagContrastFactory` | `build_contrast_minimum` | `WcagContrastMinimumNormalText` | 1.4.3 | AA |
| | `build_contrast_minimum_large` | `WcagContrastMinimumLargeText` | 1.4.3 | AA |
| | `build_contrast_enhanced` | `WcagContrastEnhancedNormalText` | 1.4.6 | AAA |
| | `build_contrast_enhanced_large` | `WcagContrastEnhancedLargeText` | 1.4.6 | AAA |
| | `build_non_text_contrast` | `WcagNonTextContrastMinimum` | 1.4.11 | AA |
| `WcagLabelFactory` | `build_labeled_element` | `WcagNamePresent` | 4.1.2 | A |
| | `build_labeled_form_field` | `WcagFormLabelsProgrammatic` | 1.3.1 / 3.3.2 | A |
| | `build_label_in_name` | `WcagLabelInNameMatch` | 2.5.3 | A |
| `WcagFocusFactory` | `build_focus_visible` | `WcagFocusVisibleKeyboard` | 2.4.7 | AA |
| | `build_focus_appearance_minimum` | `WcagFocusAppearanceMinimumArea` | 2.4.11 | AA |
| | `build_focus_appearance_enhanced` | `WcagFocusAppearanceEnhancedArea` | 2.4.12 | AAA |
| `WcagKeyboardFactory` | `build_keyboard_accessible` | `WcagKeyboardOperable` | 2.1.1 | A |
| | `build_keyboard_escape` | `WcagKeyboardNotTrapped` | 2.1.2 | A |
| | `build_remappable_shortcut` | `WcagCharacterShortcutsRemappable` | 2.1.4 | A |
| `WcagTimingFactory` | `build_timed_element` | `WcagTimingAdjustable` | 2.2.1 | A |
| `WcagTargetFactory` | `build_target_minimum` | `WcagTargetSizeMinimum` | 2.5.8 | AA |
| | `build_target_enhanced` | `WcagTargetSizeEnhanced` | 2.5.5 | AAA |
| | `build_pointer_gesture_alternative` | `WcagPointerGesturesSimpleAlternative` | 2.5.1 | A |
| | `build_pointer_cancellation` | `WcagPointerCancellationUpEvent` | 2.5.2 | A |
| `WcagStructureFactory` | `build_heading` | `WcagHeadingStructureProgrammatic` | 1.3.1 | A |
| | `build_list` | `WcagListStructureProgrammatic` | 1.3.1 | A |
| | `build_table` | `WcagTableHeadersProgrammatic` | 1.3.1 | A |
| | `build_resizable_text` | `WcagTextResizable` | 1.4.4 | AA |
| `WcagMediaFactory` | `build_captioned_media` | `WcagCaptionsSynchronized` | 1.2.2 | A |
| | `build_audio_described_media` | `WcagAudioDescriptionPrerecorded` | 1.2.5 | AA |
| `WcagLanguageFactory` | `build_language_page` | `WcagPageLanguageIdentified` | 3.1.1 | A |
| | `build_language_element` | `WcagPartLanguageIdentified` | 3.1.2 | AA |
| `WcagErrorFactory` | `build_identified_error` | `WcagErrorIdentificationDescriptive` | 3.3.1 | A |
| | `build_labeled_field` | `WcagLabelsOrInstructionsPresent` | 3.3.2 | A |
| | `build_error_suggestion` | `WcagErrorSuggestionProvided` | 3.3.3 | AA |
| | `build_error_prevention` | `WcagErrorPreventionLegal` | 3.3.4 | AA |

### Section propositions (5)

Section factories accept *evidence bundles* — structs whose fields are
`Established<LeafProposition>` tokens. They compose leaf proofs into a
section-level proof and pass it upward.

| Factory | Evidence bundle | Section proposition |
|---|---|---|
| `WcagPerceivedFactory` | `PerceivedEvidence` | `WcagPerceivedValid` |
| `WcagOperableFactory` | `OperableEvidence` | `WcagOperableValid` |
| `WcagUnderstandableFactory` | `UnderstandableEvidence` | `WcagUnderstandableValid` |
| `WcagRobustFactory` | `RobustEvidence` | `WcagRobustValid` |
| `WcagBackend` | `LevelAaEvidence` | `WcagLevelAAValid` |

---

## Proof Composition

Proofs compose bottom-up. The compiler rejects any gap in the chain.

```rust
// 1. Leaf proofs — each factory method returns (Element, Established<LeafProp>)
let (_, contrast_proof) = backend
    .build_contrast_minimum(ContrastDescriptor { foreground, background })?;
let (_, focus_proof) = backend
    .build_focus_visible(FocusDescriptor { … })?;

// 2. Assemble a section evidence bundle — all fields are required proof tokens
let perceived_evidence = PerceivedEvidence {
    contrast_normal:   contrast_proof,
    non_text_contrast: non_text_proof,
    focus_contrast:    focus_contrast_proof,
    color_not_sole:    color_proof,
};

// 3. Section factory consumes the bundle, produces a section proof
let (_, perceived_proof) = backend.build_perceivable(perceived_evidence);

// 4. Repeat for all four WCAG principles, then assemble the top-level bundle
let level_aa_evidence = LevelAaEvidence {
    perceived:      perceived_proof,
    operable:       operable_proof,
    understandable: understandable_proof,
    robust:         robust_proof,
};

// 5. Mint the composite Level AA proof
let level_aa_proof: Established<WcagLevelAAValid> =
    backend.build_level_aa(level_aa_evidence);
```

If any leaf is missing the evidence bundle struct literal will not compile —
there is no API to skip a field.

---

## Trait Interface

### Factory traits (leaf level)

```rust
pub trait WcagContrastFactory: Send + Sync {
    fn build_contrast_minimum(&self, input: ContrastDescriptor)
        -> UiResult<(ContrastPair, Established<WcagContrastMinimumNormalText>)>;

    fn build_contrast_minimum_large(&self, input: ContrastDescriptor)
        -> UiResult<(ContrastPair, Established<WcagContrastMinimumLargeText>)>;

    fn build_contrast_enhanced(&self, input: ContrastDescriptor)
        -> UiResult<(ContrastPair, Established<WcagContrastEnhancedNormalText>)>;

    fn build_contrast_enhanced_large(&self, input: ContrastDescriptor)
        -> UiResult<(ContrastPair, Established<WcagContrastEnhancedLargeText>)>;

    fn build_non_text_contrast(&self, input: ContrastDescriptor)
        -> UiResult<(ContrastPair, Established<WcagNonTextContrastMinimum>)>;
}
// … WcagLabelFactory, WcagFocusFactory, WcagKeyboardFactory,
//   WcagTimingFactory, WcagTargetFactory, WcagStructureFactory,
//   WcagMediaFactory, WcagLanguageFactory, WcagErrorFactory
```

### Section aggregate traits

```rust
pub trait WcagPerceivedFactory: Send + Sync {
    fn build_perceivable(&self, evidence: PerceivedEvidence)
        -> (PerceivedSection, Established<WcagPerceivedValid>);
}
```

### Top-level supertrait

```rust
pub trait WcagBackend:
    WcagContrastFactory + WcagLabelFactory + WcagFocusFactory
    + WcagKeyboardFactory + WcagTimingFactory + WcagTargetFactory
    + WcagStructureFactory + WcagMediaFactory + WcagLanguageFactory
    + WcagErrorFactory + WcagPerceivedFactory + WcagOperableFactory
    + WcagUnderstandableFactory + WcagRobustFactory
    + Send + Sync
{
    fn build_level_aa(&self, evidence: LevelAaEvidence)
        -> Established<WcagLevelAAValid>;
}
```

`AccessKitUiBackend` implements `WcagBackend` and is the reference backend
shipped with this crate. Other implementations must satisfy the same trait
bounds — the factory signatures are the contract.

---

## Compile-Time Guarantee Summary

| What is guaranteed | Mechanism |
|---|---|
| Contrast ratio was checked before proof minted | `NormalTextContrastVerified` is `pub(crate)` — only the factory body can construct it |
| All WCAG SCs are covered before Level AA | `LevelAaEvidence` fields are `Established<…>` tokens — all must be present at the struct literal |
| No renderer can skip the factory | Renderer crates receive proof tokens via the AccessKit IR; they cannot call `prove()` on WCAG types because they cannot construct any WCAG credential |
| Section composition is total | Evidence bundle structs have no optional fields; every field is an `Established<…>` |
| `assert()` bypasses are audit-visible | `assert()` is `pub` but stands out immediately in code review; `prove()` + credential is the type-safe path |

---

## Evidence Bundles

Evidence bundles are the bridges between proof layers. Each field must be
filled with a token minted by the corresponding factory method.

```rust
pub struct PerceivedEvidence {
    /// Normal text meets SC 1.4.3 (≥ 4.5:1).
    pub contrast_normal:   Established<WcagContrastMinimumNormalText>,
    /// Non-text controls meet SC 1.4.11 (≥ 3:1).
    pub non_text_contrast: Established<WcagNonTextContrastMinimum>,
    /// Focus indicator meets SC 1.4.11 / 2.4.11 contrast.
    pub focus_contrast:    Established<WcagFocusIndicatorContrast>,
    /// Colour is not the sole means of conveying information (SC 1.4.1).
    pub color_not_sole:    Established<WcagColorNotSoleConveyor>,
}

pub struct LevelAaEvidence {
    pub perceived:      Established<WcagPerceivedValid>,
    pub operable:       Established<WcagOperableValid>,
    pub understandable: Established<WcagUnderstandableValid>,
    pub robust:         Established<WcagRobustValid>,
}
```

---

## Implementing a Custom Backend

To implement `WcagBackend` for a new rendering target:

1. Implement all ten leaf factory traits (`WcagContrastFactory` through
   `WcagErrorFactory`). Each method must perform its runtime check, then call
   `Established::prove(&Credential)` only after the check passes.

2. Implement the four section factories. These accept evidence bundles and
   call `prove` with the evidence value as the credential.

3. Implement `WcagBackend`. The `build_level_aa` default impl composes the
   four section proofs into `Established<WcagLevelAAValid>` — override only
   if the backend needs additional logic.

> **Note for external backends:** Credentials are `pub(crate)` inside
> `elicit_ui`. Implementations that live outside this crate can use
> `Established::assert()` at the leaf level — which is an explicit, auditable
> choice — or add their credential types and `ProvableFrom` impls inside
> `elicit_ui` as a new backend module.

---

## Formal Verification

The proof architecture is designed for downstream formal verification.
Each proposition type implements `elicitation::contracts::Prop`, which
exposes a `kani_proof()` method for generating verification harnesses.

- **Kani** — bounded model checking on credential construction paths
- **Creusot** — deductive verification that factory bodies satisfy their WCAG
  postconditions before calling `prove`
- **Verus** — SMT-based proofs of composition totality

---

## Crate Layout

```
src/
├── lib.rs                      pub use surface
├── accesskit_backend.rs        AccessKitUiBackend — reference WcagBackend impl
├── proof_credentials.rs        41 pub(crate) ZST credential types
├── contracts/
│   ├── mod.rs
│   ├── wcag_proofs.rs          46 ProvableFrom<C> for P impls
│   ├── node_roles.rs           AccessKit role proofs
│   └── ui.rs                   layout / navigation proofs
├── traits/
│   ├── wcag.rs                 14 factory traits + WcagBackend supertrait
│   └── renderer.rs             UiNodeBridge, UiLayoutManager, UiNavigationManager
├── wcag_types.rs               descriptor, element, evidence, section types
├── typestate.rs                Layout<Pending>, Layout<Verified>
├── constraints/                runtime WCAG constraint checking
├── color_contrast.rs           WCAG contrast ratio arithmetic
└── css_units.rs                CssLength, zoom invariance
```

---

## License

Licensed under Apache-2.0 OR MIT.
