//! Shadows for `reqwest::redirect::{Action, Policy}`.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Redirect action decision.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a redirect decision:")]
pub enum Action {
    /// Follow the redirect target.
    Follow,
    /// Stop following and return the current response.
    Stop,
    /// Fail the redirect with a structured error message.
    Error {
        /// Reason for failing the redirect.
        #[prompt("Reason for failing the redirect:")]
        message: String,
    },
}

/// Redirect policy recipe.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a redirect policy:")]
pub enum Policy {
    /// Follow at most `max` redirects.
    Limited {
        /// Maximum redirect hops to follow.
        #[prompt("Maximum redirect hops to follow:")]
        max: usize,
    },
    /// Disable redirect following.
    None,
    /// Human-readable description of a custom redirect closure.
    Custom {
        /// Description of the custom redirect policy.
        #[prompt("Description of the custom redirect policy:")]
        description: String,
    },
}

impl Policy {
    /// Construct a limited redirect policy.
    pub fn limited(max: usize) -> Self {
        Self::Limited { max }
    }

    /// Construct a no-redirect policy.
    pub fn none() -> Self {
        Self::None
    }

    /// Record a custom redirect policy description.
    pub fn custom(description: impl Into<String>) -> Self {
        Self::Custom {
            description: description.into(),
        }
    }

    /// Rebuild a raw `reqwest::redirect::Policy` when expressible with the public API.
    pub fn build_raw(&self) -> Result<reqwest::redirect::Policy, Error> {
        match self {
            Self::Limited { max } => Ok(reqwest::redirect::Policy::limited(*max)),
            Self::None => Ok(reqwest::redirect::Policy::none()),
            Self::Custom { description } => Err(Error::builder(format!(
                "custom redirect policies are not rebuildable from shadow recipe: {description}"
            ))),
        }
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self::Limited { max: 10 }
    }
}
