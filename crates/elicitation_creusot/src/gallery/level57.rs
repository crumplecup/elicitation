//! Gallery level C57: crate-local source/generated module tree.
//!
//! **Hypothesis**: the remaining downstream `valinoreth` ICE may require the
//! crate-local packaging shape rather than any single transition or wrapper:
//!
//! 1. source transition in `vsm::combat`,
//! 2. root re-export of that source item,
//! 3. sibling `proofs::creusot::generated::{elicitation_specs, combat}` modules,
//! 4. generated companion targeting the root re-export path,
//! 5. raw `#[instrument]` on the source transition plus a tracing-free local
//!    Creusot clone inside the generated companion.
//!
//! This mirrors the `valinoreth` structure as closely as possible inside one
//! gallery level while keeping the behavior minimal.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c57-crate-local-module-tree
//! ```

/// Source VSM tree mirroring the downstream crate layout.
pub mod vsm {
    /// Combat-like source module.
    pub mod combat {
        use elicitation::{Established, Prop, formal_method};

        /// Minimal lifecycle state.
        #[derive(Debug)]
        pub enum C57State {
            /// Initial state.
            Idle,
            /// Active state with a positive round counter.
            Active {
                /// Round number.
                round: u32,
            },
        }

        /// Shared consistency proposition.
        #[derive(Prop)]
        #[prop(creusot_invariant_fn = "c57_consistent", creusot_inv_body = "true")]
        pub struct C57Consistent;

        /// Dummy step credential.
        #[derive(Prop)]
        pub struct C57Step;

        /// Source-module exec predicate.
        pub fn c57_consistent(state: &C57State) -> bool {
            match state {
                C57State::Idle => true,
                C57State::Active { round } => *round > 0,
            }
        }

        /// Source transition with the same formal surface as downstream code.
        #[formal_method(contracts = [C57Consistent])]
        #[tracing::instrument(skip_all)]
        pub fn c57_begin(
            state: C57State,
            proof: Established<C57Consistent>,
            _step: Established<C57Step>,
        ) -> (C57State, Established<C57Consistent>) {
            (state, proof)
        }
    }
}

pub use vsm::combat::{C57Consistent, C57State, C57Step, c57_begin, c57_consistent};

/// Generated Creusot tree mirroring the downstream crate layout.
#[cfg(creusot)]
pub mod proofs {
    /// Creusot-specific proof tree.
    pub mod creusot {
        /// Generated companions.
        pub mod generated {
            /// Shared elicitation extern_specs sibling module.
            pub mod elicitation_specs {
                //! Placeholder sibling module matching the downstream generated tree.
            }

            /// Generated combat companion targeting the root re-export path.
            pub mod combat {
                use crate::gallery::level57::{C57Consistent, C57State, C57Step, c57_begin};
                use creusot_std::prelude::*;
                use elicitation::Established;

                #[logic]
                pub fn c57_consistent_creusot_logic(_state: &C57State) -> bool {
                    pearlite! { true }
                }

                extern_spec! {
                    #[requires(c57_consistent_creusot_logic(&state))]
                    #[ensures(c57_consistent_creusot_logic(&result.0))]
                    fn c57_begin(
                        state: C57State,
                        proof: Established<C57Consistent>,
                        _step: Established<C57Step>,
                    ) -> (C57State, Established<C57Consistent>);
                }

                fn c57_begin_creusot_local(
                    state: C57State,
                    proof: Established<C57Consistent>,
                    step: Established<C57Step>,
                ) -> (C57State, Established<C57Consistent>) {
                    c57_begin_creusot(state, proof, step)
                }

                #[requires(c57_consistent_creusot_logic(&state))]
                #[ensures(c57_consistent_creusot_logic(&result.0))]
                pub fn c57_begin_creusot(
                    state: C57State,
                    proof: Established<C57Consistent>,
                    _step: Established<C57Step>,
                ) -> (C57State, Established<C57Consistent>) {
                    c57_begin(state, proof, _step)
                }

                #[requires(c57_consistent_creusot_logic(&state))]
                #[ensures(c57_consistent_creusot_logic(&result.0))]
                pub fn c57_begin_generated(
                    state: C57State,
                    proof: Established<C57Consistent>,
                    step: Established<C57Step>,
                ) -> (C57State, Established<C57Consistent>) {
                    c57_begin_creusot_local(state, proof, step)
                }
            }
        }
    }
}
