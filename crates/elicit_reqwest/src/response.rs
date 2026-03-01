//! Response wrapper for reqwest HTTP responses.
//!
//! Demonstrates async consuming generic methods.
//!
//! NOTE: This is a SHADOW CRATE demonstrating macro usage only.
//! All Elicitation/JsonSchema/Prompt impls belong in elicitation crate.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::Response, as Response);

// TODO: Response methods demonstration
// Blocked pending reqwest feature support in elicitation crate:
// - Response needs Elicitation + JsonSchema + Prompt impls
//
// #[reflect_methods]
// impl Response {
//     pub fn status(&self) -> u16 { ... }
//     pub async fn json<T>(self) -> Result<T, String> { ... }
//     pub async fn text(self) -> Result<String, String> { ... }
// }
