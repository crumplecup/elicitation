//! Verus verification proofs for elicitation contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

// Contract type proofs (newtypes/wrappers)
pub mod bools;
pub mod chars;
pub mod collections;
pub mod datetimes;
pub mod durations;
pub mod floats;
pub mod integers;
pub mod networks;
pub mod paths;
pub mod regexes;
pub mod strings;
pub mod tuples;
pub mod urls;
pub mod utf8;
pub mod uuids;
pub mod values;

// Base type proofs (stdlib and external crates)
pub mod external_types;
pub mod primitives;
pub mod stdlib_collections;
