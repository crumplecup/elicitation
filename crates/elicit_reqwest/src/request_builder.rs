//! RequestBuilder wrapper for reqwest request builder.
//!
//! Demonstrates consuming method support with non-Clone builder types.
//!
//! NOTE: This is a SHADOW CRATE demonstrating macro usage only.
//! All Elicitation/JsonSchema/Prompt impls belong in elicitation crate.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder);

// TODO: Builder methods demonstration
// Blocked pending reqwest feature support in elicitation crate:
// - RequestBuilder needs Elicitation + JsonSchema + Prompt impls
// - Response needs Elicitation + JsonSchema + Prompt impls
//
// #[reflect_methods]
// impl RequestBuilder {
//     pub fn timeout(self, duration: Duration) -> Self { ... }
//     pub fn json<T>(self, json: &T) -> Self { ... }
//     pub async fn send(self) -> Result<Response, String> { ... }
// }
