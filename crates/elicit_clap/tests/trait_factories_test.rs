//! Integration tests for `elicit_clap`'s `#[reflect_trait]` factories.
//!
//! Covers all four clap derive traits:
//! - [`clap::ValueEnum`] — exercises `type_map(PossibleValue)`, `&[T]` slice return, `&str` param
//! - [`clap::CommandFactory`] — exercises `type_map(Command)`, static method dispatch
//! - [`clap::Args`] — exercises `type_map(Command, Id)`, Command round-trip
//! - [`clap::Subcommand`] — exercises augment methods, `has_subcommand` with `&str`
//!
//! Each factory is tested at three levels:
//! 1. Inventory registration (link-time submission)
//! 2. Prime → register_type → instantiate lifecycle
//! 3. Handler invocation via `DynamicToolRegistry::invoke_dynamic`

use elicit_clap::trait_factories::{
    prime_clap__args, prime_clap__command_factory, prime_clap__subcommand, prime_clap__value_enum,
};
use elicitation::{
    DynamicToolRegistry, Elicit, ElicitPlugin, Prompt, Select, ToolFactoryRegistration,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Test types ────────────────────────────────────────────────────────────────

/// Minimal [`clap::ValueEnum`] type for tests.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit, clap::ValueEnum,
)]
pub enum OutputFormat {
    /// Plain text output
    Text,
    /// JSON output
    Json,
    /// YAML output
    Yaml,
}

/// Minimal [`clap::Parser`] type (implies [`clap::CommandFactory`] + [`clap::Args`]).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, clap::Parser)]
pub struct TestCli {
    /// Target name
    #[arg(long, default_value = "world")]
    pub name: String,
}

/// Minimal [`clap::Subcommand`] type for tests.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, clap::Subcommand)]
pub enum TestSubcmd {
    /// Run the operation
    Run,
    /// List available items
    List,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract the text content from a `CallToolResult`.
fn result_text(result: rmcp::model::CallToolResult) -> String {
    result
        .content
        .first()
        .and_then(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

// ── Inventory registration ────────────────────────────────────────────────────

#[test]
fn value_enum_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "clap::ValueEnum");
    assert!(found, "ValueEnumFactory not found in inventory");
}

#[test]
fn command_factory_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "clap::CommandFactory");
    assert!(found, "CommandFactoryFactory not found in inventory");
}

#[test]
fn subcommand_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "clap::Subcommand");
    assert!(found, "SubcommandFactory not found in inventory");
}

#[test]
fn args_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "clap::Args");
    assert!(found, "ArgsFactory not found in inventory");
}

// ── Meta-tools visible before instantiation ───────────────────────────────────

#[test]
fn clap_meta_tools_visible_in_fresh_registry() {
    let registry = DynamicToolRegistry::new();
    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    assert!(
        names.iter().any(|n| n.contains("value_enum")),
        "expected value_enum meta-tool, got: {names:?}"
    );
    assert!(
        names.iter().any(|n| n.contains("command_factory")),
        "expected command_factory meta-tool, got: {names:?}"
    );
    assert!(
        names.iter().any(|n| n.contains("subcommand")),
        "expected subcommand meta-tool, got: {names:?}"
    );
    assert!(
        names.iter().any(|n| n.contains("args")),
        "expected args meta-tool, got: {names:?}"
    );
}

// ── ValueEnum: full lifecycle ─────────────────────────────────────────────────

#[tokio::test]
async fn value_enum_instantiate_creates_method_tools() {
    prime_clap__value_enum::<OutputFormat>();

    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt");
    registry
        .instantiate("clap::ValueEnum", "fmt")
        .await
        .expect("ValueEnum instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    assert!(
        names.contains(&"fmt__value_variants".to_string()),
        "expected fmt__value_variants, got: {names:?}"
    );
    assert!(
        names.contains(&"fmt__to_possible_value".to_string()),
        "expected fmt__to_possible_value, got: {names:?}"
    );
    assert!(
        names.contains(&"fmt__from_str".to_string()),
        "expected fmt__from_str, got: {names:?}"
    );
}

#[tokio::test]
async fn value_enum_value_variants_returns_all_variants() {
    prime_clap__value_enum::<OutputFormat>();

    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt2");
    registry
        .instantiate("clap::ValueEnum", "fmt2")
        .await
        .unwrap();

    let result = registry
        .invoke_dynamic("fmt2__value_variants", serde_json::json!({}))
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // The result is a JSON array of serialized OutputFormat variants.
    // Verify it contains all three.
    assert!(text.contains("Text"), "expected 'Text' in variants: {text}");
    assert!(text.contains("Json"), "expected 'Json' in variants: {text}");
    assert!(text.contains("Yaml"), "expected 'Yaml' in variants: {text}");
}

#[tokio::test]
async fn value_enum_from_str_parses_valid_variant() {
    prime_clap__value_enum::<OutputFormat>();

    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt3");
    registry
        .instantiate("clap::ValueEnum", "fmt3")
        .await
        .unwrap();

    // `from_str` params: { "input": String (mapped from &str), "ignore_case": bool }
    let result = registry
        .invoke_dynamic(
            "fmt3__from_str",
            serde_json::json!({"input": "json", "ignore_case": true}),
        )
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // Result<OutputFormat, String> success path — expect the serialized variant.
    assert!(
        text.contains("Json") || text.contains("json"),
        "expected parsed Json variant: {text}"
    );
}

#[tokio::test]
async fn value_enum_from_str_errors_on_invalid_variant() {
    prime_clap__value_enum::<OutputFormat>();

    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt4");
    registry
        .instantiate("clap::ValueEnum", "fmt4")
        .await
        .unwrap();

    let result = registry
        .invoke_dynamic(
            "fmt4__from_str",
            serde_json::json!({"input": "not_a_valid_variant", "ignore_case": false}),
        )
        .await
        .expect("tool should exist")
        .expect("handler should not hard-fail; Result::Err serializes fine");

    let text = result_text(result);
    // Result<T, String>: the Err variant serializes as {"Err": "...message..."}
    assert!(
        text.contains("Err") || text.contains("err") || text.contains("invalid"),
        "expected error message for unknown variant: {text}"
    );
}

#[tokio::test]
async fn value_enum_to_possible_value_returns_name() {
    prime_clap__value_enum::<OutputFormat>();

    let registry = DynamicToolRegistry::new().register_type::<OutputFormat>("fmt5");
    registry
        .instantiate("clap::ValueEnum", "fmt5")
        .await
        .unwrap();

    // `to_possible_value` has `&self` — params must include `"target"` with
    // the serialized OutputFormat instance.
    let result = registry
        .invoke_dynamic(
            "fmt5__to_possible_value",
            serde_json::json!({"target": "Json"}),
        )
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // Returns Option<PossibleValue> — Some({"name": "json"})
    assert!(
        text.contains("json") || text.contains("Json"),
        "expected possible value name for Json variant: {text}"
    );
}

// ── CommandFactory: full lifecycle ────────────────────────────────────────────

#[tokio::test]
async fn command_factory_instantiate_creates_method_tools() {
    prime_clap__command_factory::<TestCli>();

    let registry = DynamicToolRegistry::new().register_type::<TestCli>("cli");
    registry
        .instantiate("clap::CommandFactory", "cli")
        .await
        .expect("CommandFactory instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    assert!(
        names.contains(&"cli__command".to_string()),
        "expected cli__command, got: {names:?}"
    );
    assert!(
        names.contains(&"cli__command_for_update".to_string()),
        "expected cli__command_for_update, got: {names:?}"
    );
}

#[tokio::test]
async fn command_factory_command_returns_serialized_command() {
    prime_clap__command_factory::<TestCli>();

    let registry = DynamicToolRegistry::new().register_type::<TestCli>("cli2");
    registry
        .instantiate("clap::CommandFactory", "cli2")
        .await
        .unwrap();

    let result = registry
        .invoke_dynamic("cli2__command", serde_json::json!({}))
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // The Command serializes as {"name": "test-cli"} (clap derives the name from the struct)
    assert!(
        text.contains("name") || text.contains("test"),
        "expected a command name in result: {text}"
    );
}

// ── Args: full lifecycle ──────────────────────────────────────────────────────

#[tokio::test]
async fn args_instantiate_creates_method_tools() {
    prime_clap__args::<TestCli>();

    let registry = DynamicToolRegistry::new().register_type::<TestCli>("cli_args");
    registry
        .instantiate("clap::Args", "cli_args")
        .await
        .expect("Args instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    assert!(
        names.contains(&"cli_args__augment_args".to_string()),
        "expected cli_args__augment_args, got: {names:?}"
    );
    assert!(
        names.contains(&"cli_args__augment_args_for_update".to_string()),
        "expected cli_args__augment_args_for_update, got: {names:?}"
    );
    assert!(
        names.contains(&"cli_args__group_id".to_string()),
        "expected cli_args__group_id, got: {names:?}"
    );
}

#[tokio::test]
async fn args_augment_args_accepts_command_and_returns_command() {
    prime_clap__args::<TestCli>();

    let registry = DynamicToolRegistry::new().register_type::<TestCli>("cli_args2");
    registry
        .instantiate("clap::Args", "cli_args2")
        .await
        .unwrap();

    // `augment_args` takes `cmd: clap::Command` (mapped to `crate::Command`).
    // The JSON deserializes Command from {"name": "base"}.
    let result = registry
        .invoke_dynamic(
            "cli_args2__augment_args",
            serde_json::json!({"cmd": {"name": "base"}}),
        )
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // The augmented Command serializes back to JSON with at least a "name" field.
    assert!(
        text.contains("name"),
        "expected Command JSON with 'name' field after augment: {text}"
    );
}

#[tokio::test]
async fn args_group_id_returns_option() {
    prime_clap__args::<TestCli>();

    let registry = DynamicToolRegistry::new().register_type::<TestCli>("cli_args3");
    registry
        .instantiate("clap::Args", "cli_args3")
        .await
        .unwrap();

    // `group_id()` returns `Option<clap::Id>` (mapped to `Option<crate::Id>`).
    let result = registry
        .invoke_dynamic("cli_args3__group_id", serde_json::json!({}))
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // TestCli has no explicit group_id — should serialize as `null` (None).
    // Just verify the tool returns valid JSON.
    let _: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("group_id result should be valid JSON: {e}\nGot: {text}"));
}

// ── Subcommand: full lifecycle ────────────────────────────────────────────────

#[tokio::test]
async fn subcommand_instantiate_creates_method_tools() {
    prime_clap__subcommand::<TestSubcmd>();

    let registry = DynamicToolRegistry::new().register_type::<TestSubcmd>("sub");
    registry
        .instantiate("clap::Subcommand", "sub")
        .await
        .expect("Subcommand instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    assert!(
        names.contains(&"sub__augment_subcommands".to_string()),
        "expected sub__augment_subcommands, got: {names:?}"
    );
    assert!(
        names.contains(&"sub__augment_subcommands_for_update".to_string()),
        "expected sub__augment_subcommands_for_update, got: {names:?}"
    );
    assert!(
        names.contains(&"sub__has_subcommand".to_string()),
        "expected sub__has_subcommand, got: {names:?}"
    );
}

#[tokio::test]
async fn subcommand_has_subcommand_returns_true_for_known() {
    prime_clap__subcommand::<TestSubcmd>();

    let registry = DynamicToolRegistry::new().register_type::<TestSubcmd>("sub2");
    registry
        .instantiate("clap::Subcommand", "sub2")
        .await
        .unwrap();

    // `has_subcommand(name: &str)` — param struct has `name: String` (mapped from &str).
    let result = registry
        .invoke_dynamic("sub2__has_subcommand", serde_json::json!({"name": "run"}))
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    assert!(
        text.contains("true"),
        "expected true for known subcommand 'run': {text}"
    );
}

#[tokio::test]
async fn subcommand_has_subcommand_returns_false_for_unknown() {
    prime_clap__subcommand::<TestSubcmd>();

    let registry = DynamicToolRegistry::new().register_type::<TestSubcmd>("sub3");
    registry
        .instantiate("clap::Subcommand", "sub3")
        .await
        .unwrap();

    let result = registry
        .invoke_dynamic(
            "sub3__has_subcommand",
            serde_json::json!({"name": "nonexistent_cmd"}),
        )
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    assert!(
        text.contains("false"),
        "expected false for unknown subcommand: {text}"
    );
}

#[tokio::test]
async fn subcommand_augment_subcommands_returns_augmented_command() {
    prime_clap__subcommand::<TestSubcmd>();

    let registry = DynamicToolRegistry::new().register_type::<TestSubcmd>("sub4");
    registry
        .instantiate("clap::Subcommand", "sub4")
        .await
        .unwrap();

    let result = registry
        .invoke_dynamic(
            "sub4__augment_subcommands",
            serde_json::json!({"cmd": {"name": "root"}}),
        )
        .await
        .expect("tool should exist")
        .expect("tool should succeed");

    let text = result_text(result);
    // The augmented Command serializes back with at least a name.
    assert!(
        text.contains("name"),
        "expected Command JSON with 'name' field after augment_subcommands: {text}"
    );
}

// ── Error cases ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn instantiate_fails_without_prime() {
    // Register a type but do NOT prime the factory for it.
    // The factory's vtable map won't have an entry for this TypeId.
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, clap::ValueEnum)]
    enum UnprimedEnum {
        A,
        B,
    }

    let registry = DynamicToolRegistry::new().register_type::<UnprimedEnum>("unprimed");
    let result = registry.instantiate("clap::ValueEnum", "unprimed").await;
    assert!(
        result.is_err(),
        "should error when factory not primed: {result:?}"
    );
}
