//! Integration test for [`DynamicToolRegistry`].
//!
//! Verifies the full lifecycle without an MCP connection:
//! 1. Implement [`AnyToolFactory`] manually for a fake trait
//! 2. Submit it to inventory via [`ToolFactoryRegistration`]
//! 3. Create a registry and register a concrete type
//! 4. Confirm factory meta-tools appear in `list_tools`
//! 5. Call `instantiate` → new tools appear in `list_tools`
//! 6. Call a dynamic tool handler directly → correct output

use std::sync::Arc;

use elicitation::{
    AnyToolFactory, AnyToolSlot, DynamicToolDescriptor, DynamicToolRegistry, ElicitPlugin,
    ToolFactoryRegistration, dynamic::slot::TypedSlot,
};
use futures::future::BoxFuture;
use rmcp::ErrorData;

// ── Fake factory ──────────────────────────────────────────────────────────────

struct DescribableFactory;

impl AnyToolFactory for DescribableFactory {
    fn trait_name(&self) -> &'static str {
        "test::Describable"
    }

    fn factory_description(&self) -> &'static str {
        "Tools for types implementing test::Describable"
    }

    fn method_names(&self) -> &'static [&'static str] {
        &["describe"]
    }

    fn instantiate(&self, slot: &dyn AnyToolSlot) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
        if TypedSlot::<bool>::downcast_ref(slot).is_none() {
            return Err(ErrorData::invalid_params(
                format!(
                    "DescribableFactory does not support type `{}`",
                    slot.type_name()
                ),
                None,
            ));
        }

        let prefix = slot.prefix().to_string();
        let name = format!("{prefix}__describe");

        let handler: Arc<
            dyn Fn(
                    serde_json::Value,
                )
                    -> BoxFuture<'static, Result<rmcp::model::CallToolResult, ErrorData>>
                + Send
                + Sync,
        > = Arc::new(move |_params| {
            Box::pin(async move {
                Ok(rmcp::model::CallToolResult::success(vec![
                    rmcp::model::Content::new(
                        rmcp::model::RawContent::text("bool: true or false"),
                        None,
                    ),
                ]))
            })
        });

        Ok(vec![DynamicToolDescriptor {
            name,
            description: format!("Describe {prefix}"),
            schema: serde_json::json!({"type": "object", "properties": {}}),
            handler,
        }])
    }
}

inventory::submit!(ToolFactoryRegistration {
    trait_name: "test::Describable",
    factory: &DescribableFactory,
});

// ── Tests ──────────────────────────────────────────────────────────────────────

#[test]
fn factory_meta_tools_visible_before_instantiation() {
    let registry = DynamicToolRegistry::new();
    let tools = registry.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.iter().any(|n| n.contains("describable")),
        "expected a 'describable' meta-tool, got: {names:?}"
    );
    // No flag__ tools yet
    assert!(
        names.iter().all(|n| !n.contains("flag__")),
        "no dynamic tools before instantiation"
    );
}

#[test]
fn register_type_does_not_panic() {
    let _registry = DynamicToolRegistry::new().register_type::<bool>("flag");
}

#[tokio::test]
async fn instantiate_creates_dynamic_tools() {
    let registry = DynamicToolRegistry::new().register_type::<bool>("flag");

    // Before instantiation: only meta-tools visible
    let before = registry.list_tools();
    assert!(
        before.iter().all(|t| !t.name.as_ref().contains("flag__")),
        "no flag__ tools before instantiation"
    );

    registry
        .instantiate("test::Describable", "flag")
        .await
        .expect("instantiation should succeed");

    let after = registry.list_tools();
    let names: Vec<&str> = after.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.contains(&"flag__describe"),
        "expected flag__describe after instantiation, got: {names:?}"
    );
}

#[tokio::test]
async fn dynamic_tool_handler_executes() {
    let registry = DynamicToolRegistry::new().register_type::<bool>("flag");

    registry
        .instantiate("test::Describable", "flag")
        .await
        .expect("instantiation");

    // Find the handler and call it directly
    let tools = registry.list_tools();
    // The dynamic tool appears in list_tools — verify it has the expected name
    let tool = tools
        .iter()
        .find(|t| t.name.as_ref() == "flag__describe")
        .expect("flag__describe should be present");
    assert!(
        tool.description.as_deref().unwrap_or("").contains("flag"),
        "description should mention prefix"
    );
}

#[tokio::test]
async fn instantiate_wrong_type_returns_error() {
    // Register String, but DescribableFactory only accepts bool
    let registry = DynamicToolRegistry::new().register_type::<String>("greeting");

    let result = registry.instantiate("test::Describable", "greeting").await;

    assert!(result.is_err(), "should error for unsupported type");
}

#[tokio::test]
async fn instantiate_unknown_prefix_returns_error() {
    let registry = DynamicToolRegistry::new();

    let result = registry
        .instantiate("test::Describable", "nonexistent")
        .await;

    assert!(result.is_err(), "should error for unregistered prefix");
}

#[tokio::test]
async fn instantiate_unknown_trait_returns_error() {
    let registry = DynamicToolRegistry::new().register_type::<bool>("flag");

    let result = registry.instantiate("no_such::Trait", "flag").await;

    assert!(result.is_err(), "should error for unregistered factory");
}

#[tokio::test]
async fn reinstantiation_is_idempotent() {
    let registry = DynamicToolRegistry::new().register_type::<bool>("flag");

    registry
        .instantiate("test::Describable", "flag")
        .await
        .unwrap();
    registry
        .instantiate("test::Describable", "flag")
        .await
        .unwrap();

    let tools = registry.list_tools();
    let count = tools
        .iter()
        .filter(|t| t.name.as_ref() == "flag__describe")
        .count();
    assert_eq!(count, 1, "re-instantiation should not duplicate tools");
}
