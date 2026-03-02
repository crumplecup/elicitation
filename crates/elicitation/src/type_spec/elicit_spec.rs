//! The [`ElicitSpec`] trait for types that expose agent-browsable specs.

use crate::TypeSpec;

/// Provides an agent-browsable spec for a type.
///
/// Complements [`ElicitIntrospect`](crate::ElicitIntrospect): where
/// `ElicitIntrospect` describes *structure* (fields, options, pattern),
/// `ElicitSpec` describes *contracts* (requires, ensures, bounds).
///
/// Implementations are registered globally via [`inventory`] so the
/// `describe_type` and `explore_type` MCP tools can look up any type by name.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{ElicitSpec, TypeSpec, SpecCategoryBuilder, SpecEntryBuilder};
///
/// impl ElicitSpec for I32Positive {
///     fn type_spec() -> TypeSpec {
///         let entry = SpecEntryBuilder::default()
///             .label("positive".to_string())
///             .description("value must be greater than zero".to_string())
///             .expression(Some("value > 0".to_string()))
///             .build()
///             .expect("valid entry");
///
///         let requires = SpecCategoryBuilder::default()
///             .name("requires".to_string())
///             .entries(vec![entry])
///             .build()
///             .expect("valid category");
///
///         TypeSpecBuilder::default()
///             .type_name("I32Positive".to_string())
///             .summary("A positive 32-bit integer (value > 0)".to_string())
///             .categories(vec![requires])
///             .build()
///             .expect("valid spec")
///     }
/// }
/// ```
pub trait ElicitSpec {
    /// Returns the complete spec for this type.
    fn type_spec() -> TypeSpec;
}
