//! Integration test for the `#[reflect_trait]` proc macro.
//!
//! Verifies the full lifecycle:
//! 1. Apply `#[reflect_trait(test_greet::Greetable)]` to a marker trait block
//! 2. The macro generates a factory, vtable, param structs, and inventory submission
//! 3. Prime the factory for a concrete type (`FlagType`)
//! 4. Register the type in a `DynamicToolRegistry`
//! 5. Instantiate tools for that type
//! 6. Confirm the generated tools appear in the tool list

use elicitation::{DynamicToolRegistry, Elicit, ElicitPlugin};
use elicitation_macros::reflect_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Fake third-party trait ────────────────────────────────────────────────────

mod test_greet {
    /// A minimal fake "third-party" trait whose methods we want as MCP tools.
    pub trait Greetable {
        /// Produce a greeting string for this value.
        fn greet(name: String) -> String;

        /// Check whether a name is valid for this value.
        fn is_valid_name(&self, name: String) -> bool;
    }
}

// ── reflect_trait macro application ───────────────────────────────────────────

// Apply the macro — this generates:
//  - `GreetParams`, `IsValidNameParams` param structs
//  - `GreetableVTable`
//  - `GreetableFactory` (implements AnyToolFactory) + inventory submission
//  - `prime_test_greet__greetable::<T>()` free function
#[reflect_trait(test_greet::Greetable)]
trait GreetableTools {
    fn greet(name: String) -> String;
    fn is_valid_name(&self, name: String) -> bool;
}

// ── Concrete type implementing the third-party trait ─────────────────────────

/// A simple bool-wrapper that implements Greetable.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct FlagType {
    pub value: bool,
}

impl test_greet::Greetable for FlagType {
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }

    fn is_valid_name(&self, name: String) -> bool {
        !name.is_empty() && self.value
    }
}

/// A second concrete type — used only in the "not primed" test to ensure
/// the vtable hasn't been populated by another test for this TypeId.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct UnprimedFlag {
    pub label: String,
}

impl test_greet::Greetable for UnprimedFlag {
    fn greet(name: String) -> String {
        format!("Hi {name}")
    }
    fn is_valid_name(&self, name: String) -> bool {
        name == self.label
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn reflect_trait_factory_registered_in_inventory() {
    // The factory should be registered at link time via inventory::submit!
    let found = inventory::iter::<elicitation::ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "test_greet::Greetable");
    assert!(found, "GreetableFactory not found in inventory");
}

#[test]
fn reflect_trait_meta_tool_visible_before_instantiation() {
    let registry = DynamicToolRegistry::new();

    let tools = registry.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    // A meta-tool for the Greetable factory should appear (from inventory)
    assert!(
        names.iter().any(|n| n.contains("greet")),
        "expected meta-tool containing 'greet', got: {names:?}"
    );
    // No flag__ tools yet
    assert!(
        names.iter().all(|n| !n.contains("flag__")),
        "no dynamic tools before instantiation"
    );
}

#[tokio::test]
async fn reflect_trait_instantiate_creates_method_tools() {
    // Prime the factory for FlagType — monomorphizes vtable closures
    prime_test_greet__greetable::<FlagType>();

    let registry = DynamicToolRegistry::new().register_type::<FlagType>("flag");

    // Instantiate Greetable tools for the "flag" slot
    registry
        .instantiate("test_greet::Greetable", "flag")
        .await
        .expect("instantiate should succeed");

    let tools = registry.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    assert!(
        names.iter().any(|n| *n == "flag__greet"),
        "expected flag__greet, got: {names:?}"
    );
    assert!(
        names.iter().any(|n| *n == "flag__is_valid_name"),
        "expected flag__is_valid_name, got: {names:?}"
    );
}

#[tokio::test]
async fn reflect_trait_instantiate_error_if_not_primed() {
    // Register UnprimedFlag WITHOUT priming the factory for its TypeId
    let registry = DynamicToolRegistry::new().register_type::<UnprimedFlag>("unprimed_flag");

    let result = registry
        .instantiate("test_greet::Greetable", "unprimed_flag")
        .await;

    assert!(
        result.is_err(),
        "expected error when factory not primed for type"
    );
}
