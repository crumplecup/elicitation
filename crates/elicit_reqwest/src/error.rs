//! Error wrapper for reqwest errors.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Error.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::Error, as Error);
