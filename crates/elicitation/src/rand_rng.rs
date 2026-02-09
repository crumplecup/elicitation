//! Random number generator elicitation.
//!
//! Available with the `rand` feature.
//!
//! This module provides `Elicitation` implementations for common RNG types
//! from the `rand` ecosystem, allowing agents to conversationally construct
//! random number generators with specific seeds.
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::Elicitation;
//! use rand::rngs::StdRng;
//! use rand::Rng;
//!
//! async fn example(communicator: &impl ElicitCommunicator) {
//!     // Agent elicits seed for reproducibility
//!     let rng = StdRng::elicit(communicator).await?;
//!     
//!     // Use RNG
//!     let value: u32 = rng.gen();
//! }
//! ```

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt, default_style};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_chacha::ChaCha8Rng;

// Style types for RNG elicitation
default_style!(StdRng => StdRngStyle);
default_style!(ChaCha8Rng => ChaCha8RngStyle);

// ============================================================================
// StdRng - Standard, reproducible RNG
// ============================================================================

impl Prompt for StdRng {
    fn prompt() -> Option<&'static str> {
        Some("Enter seed for standard RNG (u64):")
    }
}

impl Elicitation for StdRng {
    type Style = StdRngStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // Elicit seed from agent
        let seed: u64 = u64::elicit(communicator).await?;

        // Construct RNG (trust rand's implementation)
        Ok(StdRng::seed_from_u64(seed))
    }
}

// ============================================================================
// ChaCha8Rng - Cryptographically secure RNG
// ============================================================================

impl Prompt for ChaCha8Rng {
    fn prompt() -> Option<&'static str> {
        Some("Enter seed for cryptographic RNG (u64):")
    }
}

impl Elicitation for ChaCha8Rng {
    type Style = ChaCha8RngStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let seed: u64 = u64::elicit(communicator).await?;
        Ok(ChaCha8Rng::seed_from_u64(seed))
    }
}
