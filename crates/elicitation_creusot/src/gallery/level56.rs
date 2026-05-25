//! Gallery level C56: nested non-VSM `Prop` evidence bundles.
//!
//! **Hypothesis**: if the downstream `valinoreth` ICE survives after excluding
//! combat, `#[formal_method]`, `VerifiedStateMachine`, and raw tracing, the
//! remaining trigger may be the large same-crate surface of plain
//! `#[derive(Prop)]` items used in `contracts/*` — especially nested evidence
//! bundles containing `Established<P>` leaves and other derived `Prop` bundles.
//!
//! This level mirrors that shape directly, but with no VSM, no invariant
//! function, no generated companion module, and no tracing. If it still ICEs,
//! the culprit is likely in `Prop` derive packaging itself; if it is clean, the
//! remaining downstream crash needs some other ingredient.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c56-nested-prop-bundles
//! ```

use elicitation::contracts::Established;
use elicitation_derive::Prop;

/// Leaf proposition: an attack roll was made.
#[derive(Prop)]
pub struct C56AttackRollMade;

/// Leaf proposition: an attack outcome was determined.
#[derive(Prop)]
pub struct C56AttackOutcomeDetermined;

/// Leaf proposition: the attack succeeded.
#[derive(Prop)]
pub struct C56AttackSucceeded;

/// Leaf proposition: the attack failed.
#[derive(Prop)]
pub struct C56AttackFailed;

/// Leaf proposition: a defense roll was made.
#[derive(Prop)]
pub struct C56DefenseRollMade;

/// Leaf proposition: a defense outcome was determined.
#[derive(Prop)]
pub struct C56DefenseOutcomeDetermined;

/// Leaf proposition: the defense succeeded.
#[derive(Prop)]
pub struct C56DefenseSucceeded;

/// Leaf proposition: damage was applied.
#[derive(Prop)]
pub struct C56DamageApplied;

/// Leaf proposition: a skill roll was made.
#[derive(Prop)]
pub struct C56SkillRollMade;

/// Leaf proposition: a skill outcome was determined.
#[derive(Prop)]
pub struct C56SkillOutcomeDetermined;

/// Attack resolution bundle.
#[derive(Prop)]
pub struct C56AttackResolutionEvidence {
    /// Proof that an attack roll occurred.
    pub roll_made: Established<C56AttackRollMade>,
    /// Proof that the attack result was evaluated.
    pub outcome: Established<C56AttackOutcomeDetermined>,
}

/// Attack success bundle.
#[derive(Prop)]
pub struct C56AttackSuccessEvidence {
    /// Nested attack resolution evidence.
    pub resolution: C56AttackResolutionEvidence,
    /// Proof that the attack hit.
    pub success: Established<C56AttackSucceeded>,
}

/// Attack failure bundle.
#[derive(Prop)]
pub struct C56AttackFailureEvidence {
    /// Nested attack resolution evidence.
    pub resolution: C56AttackResolutionEvidence,
    /// Proof that the attack missed.
    pub failure: Established<C56AttackFailed>,
}

/// Defense resolution bundle.
#[derive(Prop)]
pub struct C56DefenseResolutionEvidence {
    /// Proof that a defense roll occurred.
    pub roll_made: Established<C56DefenseRollMade>,
    /// Proof that the defense result was evaluated.
    pub outcome: Established<C56DefenseOutcomeDetermined>,
}

/// Defense success bundle.
#[derive(Prop)]
pub struct C56DefenseSuccessEvidence {
    /// Nested defense resolution evidence.
    pub resolution: C56DefenseResolutionEvidence,
    /// Proof that the defense avoided the attack.
    pub success: Established<C56DefenseSucceeded>,
}

/// Skill-check resolution bundle.
#[derive(Prop)]
pub struct C56SkillCheckEvidence {
    /// Proof that a skill roll occurred.
    pub roll_made: Established<C56SkillRollMade>,
    /// Proof that the skill result was evaluated.
    pub outcome: Established<C56SkillOutcomeDetermined>,
}

/// Damage application bundle composed from attack and defense outcomes.
#[derive(Prop)]
pub struct C56DamageApplicationEvidence {
    /// Attack must have succeeded first.
    pub attack: C56AttackSuccessEvidence,
    /// Defense must have failed or been bypassed; modelled here as a prior defense resolution.
    pub defense: C56DefenseResolutionEvidence,
    /// Proof that damage was actually applied.
    pub damage: Established<C56DamageApplied>,
}

/// High-level bundled action evidence combining combat and skill surfaces.
#[derive(Prop)]
pub struct C56CompositeActionEvidence {
    /// Damage path through combat resolution.
    pub damage: C56DamageApplicationEvidence,
    /// Independent nested skill-check evidence.
    pub skill: C56SkillCheckEvidence,
}
