//! Proof gallery: calibration harnesses of ascending complexity.
//!
//! Each level isolates one source of CBMC cost so we can find the solver
//! boundary before tackling the full `ArchivePanelState` machine.
//!
//! | Level | State type             | Fields       | Invariant          | Cost source      |
//! |-------|------------------------|--------------|--------------------|------------------|
//! | 0     | `GUnit` (unit enum)    | none         | `true`             | harness overhead |
//! | 1     | `GInt` (u32 fields)    | numeric      | `val < 1000`       | numeric SAT      |
//! | 2     | `GStr` (String field)  | String       | `s.len() <= 4`     | symbolic strings |
//! | 3     | `GVec` (Vec<u32>)      | Vec<u32>     | `v.len() <= 3`     | symbolic Vecs    |
//!
//! Run a single level:
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery0_unit_d0
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery1_int_d0
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery2_str_d0
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery3_vec_d0
//! ```

pub mod level0;
pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;
pub mod level10;
