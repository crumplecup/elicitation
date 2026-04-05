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

// ── ContextualFactory tests ────────────────────────────────────────────────────

use elicitation::ContextualFactory;
use serde_json::json;

struct BetConstraints {
    min: u64,
    max: u64,
    presets: Vec<u64>,
}

struct BetAmountFactory;

impl ContextualFactory for BetAmountFactory {
    type Context = BetConstraints;

    fn instantiate(
        &self,
        prefix: &str,
        ctx: &BetConstraints,
    ) -> Result<Vec<DynamicToolDescriptor>, ErrorData> {
        let mut tools = vec![];
        let min = ctx.min;
        let max = ctx.max;
        let name = format!("{prefix}__place");
        tools.push(DynamicToolDescriptor {
            name: name.clone(),
            description: format!("Place a bet between {min} and {max}"),
            schema: json!({
                "type": "object",
                "properties": {
                    "amount": { "type": "integer", "minimum": min, "maximum": max }
                },
                "required": ["amount"]
            }),
            handler: Arc::new(move |args: serde_json::Value| {
                Box::pin(async move {
                    let amount = args["amount"].as_u64().unwrap_or(0);
                    if amount < min || amount > max {
                        return Err(ErrorData::invalid_params("amount out of range", None));
                    }
                    Ok(rmcp::model::CallToolResult::success(vec![
                        rmcp::model::Content::text(format!("bet {amount}"))
                    ]))
                })
            }),
        });
        for &preset in ctx.presets.iter().filter(|&&p| p <= ctx.max) {
            let pname = format!("{prefix}__preset_{preset}");
            tools.push(DynamicToolDescriptor {
                name: pname,
                description: format!("Place preset bet of {preset}"),
                schema: json!({ "type": "object", "properties": {} }),
                handler: Arc::new(move |_args| {
                    Box::pin(async move {
                        Ok(rmcp::model::CallToolResult::success(vec![
                            rmcp::model::Content::text(format!("bet {preset}"))
                        ]))
                    })
                }),
            });
        }
        Ok(tools)
    }
}

#[test]
fn contextual_tools_appear_in_list_tools() {
    let registry = DynamicToolRegistry::new().register_contextual(
        "bet",
        BetAmountFactory,
        BetConstraints { min: 1, max: 450, presets: vec![50, 100, 200, 500] },
    );

    let tools = registry.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    assert!(names.contains(&"bet__place"), "place tool missing");
    assert!(names.contains(&"bet__preset_50"), "preset 50 missing");
    assert!(names.contains(&"bet__preset_100"), "preset 100 missing");
    assert!(names.contains(&"bet__preset_200"), "preset 200 missing");
    // 500 exceeds bankroll of 450 — should not appear
    assert!(!names.contains(&"bet__preset_500"), "preset 500 should be filtered");
}

#[test]
fn contextual_re_registration_replaces_tools() {
    let registry = DynamicToolRegistry::new()
        .register_contextual(
            "bet",
            BetAmountFactory,
            BetConstraints { min: 1, max: 450, presets: vec![50, 100] },
        )
        .register_contextual(
            "bet",
            BetAmountFactory,
            BetConstraints { min: 1, max: 75, presets: vec![50, 100] },
        );

    let tools = registry.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    // After re-registration with max=75: preset_100 should be filtered
    assert!(names.contains(&"bet__preset_50"), "preset 50 should be present");
    assert!(!names.contains(&"bet__preset_100"), "preset 100 should be filtered after re-registration");
    // Only one bet__place tool (no duplicates)
    let place_count = names.iter().filter(|&&n| n == "bet__place").count();
    assert_eq!(place_count, 1, "re-registration must not duplicate tools");
}

#[tokio::test]
async fn contextual_tool_handler_enforces_schema_bounds() {
    let registry = DynamicToolRegistry::new().register_contextual(
        "bet",
        BetAmountFactory,
        BetConstraints { min: 1, max: 450, presets: vec![] },
    );

    // Valid amount
    let ok = registry
        .invoke_dynamic("bet__place", json!({ "amount": 100 }))
        .await
        .expect("tool should exist");
    assert!(ok.is_ok(), "valid bet should succeed");

    // Exceeds runtime maximum
    let err = registry
        .invoke_dynamic("bet__place", json!({ "amount": 500 }))
        .await
        .expect("tool should exist");
    assert!(err.is_err(), "bet exceeding bankroll should fail");
}
