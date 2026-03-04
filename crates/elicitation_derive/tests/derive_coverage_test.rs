//! Comprehensive regression coverage for `#[derive(Elicit)]`.
//!
//! Every supported type shape has at least one structural assertion so that
//! a silent regression in generated code fails a test, not just compilation.
//! Compilation itself is the primary regression gate: if any derive breaks,
//! this file fails to compile and *all* tests in the crate fail.
//!
//! Shapes covered
//! ──────────────
//! Structs
//!   - Named, plain fields
//!   - Named, custom `#[prompt]`
//!   - Named, `#[skip]` field excluded from survey
//!   - Named, nested type (struct field whose type also derives Elicit)
//!   - Named, `Option<T>` field
//!   - Tuple / newtype (single field)
//!   - Tuple, multi-field
//!   - Tuple, custom `#[prompt]`
//!   - Unit struct (no prompt)
//!   - Unit struct, custom `#[prompt]`
//!
//! Enums
//!   - Unit variants only
//!   - Unit variants, custom `#[prompt]`
//!   - Mixed variants (unit + tuple + struct)
//!   - All-tuple variants
//!   - All-struct variants
//!   - Nested (enum variant holds another Elicit enum)

use elicitation::{Elicit, ElicitIntrospect, ElicitationPattern, Prompt, Select, Survey};

// ── Named structs ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Elicit)]
struct PlainStruct {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Elicit)]
#[prompt("Configure the server:")]
struct PromptedStruct {
    host: String,
    port: u16,
}

#[derive(Debug, Clone, Elicit)]
struct SkipStruct {
    visible: String,
    #[skip]
    internal_id: u64,
}

#[derive(Debug, Clone, Elicit)]
struct NestedNamedStruct {
    label: String,
    inner: PlainStruct,
}

#[derive(Debug, Clone, Elicit)]
struct OptionFieldStruct {
    required: String,
    optional: Option<String>,
}

// ── Tuple structs ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Elicit)]
struct Newtype(String);

#[derive(Debug, Clone, Elicit)]
struct MultiField(f64, f64, f64);

#[derive(Debug, Clone, Elicit)]
#[prompt("Enter a comma-separated list:")]
struct PromptedTuple(Vec<String>);

// ── Unit structs ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Elicit)]
struct UnitNoPrompt;

#[derive(Debug, Clone, Copy, Elicit)]
#[prompt("Confirm action:")]
struct UnitWithPrompt;

// ── Enums: unit variants ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
enum UnitEnum {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
#[prompt("Choose a direction:")]
enum PromptedEnum {
    North,
    South,
    East,
    West,
}

// ── Enums: mixed variants ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Elicit)]
enum MixedEnum {
    Plain,
    WithTuple(String),
    WithStruct { x: i32, y: i32 },
}

// ── Enums: all-tuple variants ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Elicit)]
enum AllTupleEnum {
    Single(String),
    Pair(u32, u32),
}

// ── Enums: all-struct variants ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Elicit)]
enum AllStructEnum {
    Http { status: u16, body: String },
    Grpc { code: u32, message: String },
}

// ── Enums: nested (variant holds another Elicit enum) ────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
enum Inner {
    X,
    Y,
}

#[derive(Debug, Clone, PartialEq, Elicit)]
enum Outer {
    Leaf,
    Nested(Inner),
    Config { inner: Inner, label: String },
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Named structs
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn plain_struct_prompt_generated() {
    let p = PlainStruct::prompt();
    assert!(p.is_some());
    assert!(p.unwrap().contains("PlainStruct"));
}

#[test]
fn plain_struct_fields() {
    let fields = PlainStruct::fields();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[0].type_name, "String");
    assert_eq!(fields[1].name, "age");
    assert_eq!(fields[1].type_name, "u32");
}

#[test]
fn plain_struct_introspect_pattern() {
    assert_eq!(PlainStruct::pattern(), ElicitationPattern::Survey);
}

#[test]
fn plain_struct_metadata_type_name() {
    let meta = PlainStruct::metadata();
    assert_eq!(meta.type_name, "PlainStruct");
}

#[test]
fn prompted_struct_prompt() {
    assert_eq!(
        <PromptedStruct as Prompt>::prompt(),
        Some("Configure the server:")
    );
}

#[test]
fn prompted_struct_fields() {
    let fields = PromptedStruct::fields();
    assert_eq!(fields.len(), 2);
    assert!(fields.iter().any(|f| f.name == "host"));
    assert!(fields.iter().any(|f| f.name == "port"));
}

#[test]
fn skip_field_excluded_from_survey() {
    let fields = SkipStruct::fields();
    // Only `visible` should appear; `internal_id` is skipped
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "visible");
}

#[test]
fn nested_named_struct_fields() {
    let fields = NestedNamedStruct::fields();
    assert_eq!(fields.len(), 2);
    assert!(fields.iter().any(|f| f.name == "label"));
    assert!(fields.iter().any(|f| f.name == "inner"));
}

#[test]
fn option_field_struct_fields() {
    let fields = OptionFieldStruct::fields();
    assert_eq!(fields.len(), 2);
    assert!(fields.iter().any(|f| f.name == "required"));
    assert!(fields.iter().any(|f| f.name == "optional"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Tuple structs
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn newtype_prompt_none() {
    assert_eq!(<Newtype as Prompt>::prompt(), None);
}

#[test]
fn newtype_fields_index_zero() {
    let fields = Newtype::fields();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "0");
    assert_eq!(fields[0].type_name, "String");
}

#[test]
fn newtype_introspect_is_survey() {
    assert_eq!(Newtype::pattern(), ElicitationPattern::Survey);
}

#[test]
fn multi_field_tuple_fields() {
    let fields = MultiField::fields();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].name, "0");
    assert_eq!(fields[1].name, "1");
    assert_eq!(fields[2].name, "2");
    for f in &fields {
        assert_eq!(f.type_name, "f64");
    }
}

#[test]
fn prompted_tuple_prompt() {
    assert_eq!(
        <PromptedTuple as Prompt>::prompt(),
        Some("Enter a comma-separated list:")
    );
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Unit structs
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn unit_no_prompt_is_none() {
    assert_eq!(<UnitNoPrompt as Prompt>::prompt(), None);
}

#[test]
fn unit_no_prompt_empty_survey() {
    assert_eq!(UnitNoPrompt::fields().len(), 0);
}

#[test]
fn unit_no_prompt_is_survey_pattern() {
    assert_eq!(UnitNoPrompt::pattern(), ElicitationPattern::Survey);
}

#[test]
fn unit_with_prompt_has_prompt() {
    assert_eq!(
        <UnitWithPrompt as Prompt>::prompt(),
        Some("Confirm action:")
    );
}

#[test]
fn unit_with_prompt_empty_survey() {
    assert_eq!(UnitWithPrompt::fields().len(), 0);
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Enums — unit variants
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn unit_enum_prompt_generated() {
    let p = UnitEnum::prompt();
    assert!(p.is_some());
    assert!(p.unwrap().contains("UnitEnum"));
}

#[test]
fn unit_enum_options_count() {
    let opts = UnitEnum::options();
    assert_eq!(opts.len(), 3);
}

#[test]
fn unit_enum_options_values() {
    let opts = UnitEnum::options();
    assert!(opts.contains(&UnitEnum::Alpha));
    assert!(opts.contains(&UnitEnum::Beta));
    assert!(opts.contains(&UnitEnum::Gamma));
}

#[test]
fn unit_enum_labels() {
    let labels = UnitEnum::labels();
    assert_eq!(labels, &["Alpha", "Beta", "Gamma"]);
}

#[test]
fn unit_enum_from_label_round_trip() {
    assert_eq!(UnitEnum::from_label("Alpha"), Some(UnitEnum::Alpha));
    assert_eq!(UnitEnum::from_label("Beta"), Some(UnitEnum::Beta));
    assert_eq!(UnitEnum::from_label("Gamma"), Some(UnitEnum::Gamma));
    assert_eq!(UnitEnum::from_label("missing"), None);
    assert_eq!(UnitEnum::from_label("alpha"), None); // case-sensitive
}

#[test]
fn unit_enum_introspect_pattern() {
    assert_eq!(UnitEnum::pattern(), ElicitationPattern::Select);
}

#[test]
fn unit_enum_metadata_type_name() {
    let meta = UnitEnum::metadata();
    assert_eq!(meta.type_name, "UnitEnum");
}

#[test]
fn prompted_enum_prompt() {
    assert_eq!(
        <PromptedEnum as Prompt>::prompt(),
        Some("Choose a direction:")
    );
}

#[test]
fn prompted_enum_labels() {
    let labels = PromptedEnum::labels();
    assert_eq!(labels, &["North", "South", "East", "West"]);
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Enums — mixed variants
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn mixed_enum_labels_all_present() {
    let labels = MixedEnum::labels();
    assert_eq!(labels.len(), 3);
    assert!(labels.iter().any(|l| l == "Plain"));
    assert!(labels.iter().any(|l| l == "WithTuple"));
    assert!(labels.iter().any(|l| l == "WithStruct"));
}

#[test]
fn mixed_enum_from_label_unit_only() {
    // Only unit variants can be constructed from a label alone
    assert_eq!(MixedEnum::from_label("Plain"), Some(MixedEnum::Plain));
    assert_eq!(MixedEnum::from_label("WithTuple"), None);
    assert_eq!(MixedEnum::from_label("WithStruct"), None);
}

#[test]
fn mixed_enum_is_select_pattern() {
    assert_eq!(MixedEnum::pattern(), ElicitationPattern::Select);
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Enums — all-tuple variants
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn all_tuple_enum_labels() {
    let labels = AllTupleEnum::labels();
    assert_eq!(labels.len(), 2);
    assert!(labels.iter().any(|l| l == "Single"));
    assert!(labels.iter().any(|l| l == "Pair"));
}

#[test]
fn all_tuple_enum_no_unit_from_label() {
    assert_eq!(AllTupleEnum::from_label("Single"), None);
    assert_eq!(AllTupleEnum::from_label("Pair"), None);
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Enums — all-struct variants
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn all_struct_enum_labels() {
    let labels = AllStructEnum::labels();
    assert_eq!(labels.len(), 2);
    assert!(labels.iter().any(|l| l == "Http"));
    assert!(labels.iter().any(|l| l == "Grpc"));
}

// ═════════════════════════════════════════════════════════════════════════════
// Tests: Enums — nested
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn inner_enum_from_label() {
    assert_eq!(Inner::from_label("X"), Some(Inner::X));
    assert_eq!(Inner::from_label("Y"), Some(Inner::Y));
}

#[test]
fn outer_enum_labels() {
    let labels = Outer::labels();
    assert_eq!(labels.len(), 3);
    assert!(labels.iter().any(|l| l == "Leaf"));
    assert!(labels.iter().any(|l| l == "Nested"));
    assert!(labels.iter().any(|l| l == "Config"));
}

#[test]
fn outer_enum_unit_from_label() {
    assert_eq!(Outer::from_label("Leaf"), Some(Outer::Leaf));
    assert_eq!(Outer::from_label("Nested"), None);
    assert_eq!(Outer::from_label("Config"), None);
}

// ═════════════════════════════════════════════════════════════════════════════
// Construction tests (ensures fields are read, preventing dead_code warnings)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn construct_named_structs() {
    let s = PlainStruct { name: "Alice".to_string(), age: 30 };
    assert_eq!(s.name, "Alice");
    assert_eq!(s.age, 30);

    let s = PromptedStruct { host: "localhost".to_string(), port: 8080 };
    assert_eq!(s.host, "localhost");
    assert_eq!(s.port, 8080);

    let s = SkipStruct { visible: "yes".to_string(), internal_id: 42 };
    assert_eq!(s.visible, "yes");
    assert_eq!(s.internal_id, 42);

    let inner = PlainStruct { name: "inner".to_string(), age: 1 };
    let s = NestedNamedStruct { label: "outer".to_string(), inner };
    assert_eq!(s.label, "outer");
    assert_eq!(s.inner.age, 1);

    let s = OptionFieldStruct { required: "yes".to_string(), optional: None };
    assert_eq!(s.required, "yes");
    assert!(s.optional.is_none());
}

#[test]
fn construct_tuple_structs() {
    let n = Newtype("hello".to_string());
    assert_eq!(n.0, "hello");

    let m = MultiField(1.0, 2.0, 3.0);
    assert_eq!(m.0, 1.0);
    assert_eq!(m.1, 2.0);
    assert_eq!(m.2, 3.0);

    let p = PromptedTuple(vec!["a".to_string()]);
    assert_eq!(p.0.len(), 1);
}

// ═════════════════════════════════════════════════════════════════════════════
// Compile-time trait-bound checks (verify all shapes satisfy all required traits)
// ═════════════════════════════════════════════════════════════════════════════

#[test]
fn all_shapes_satisfy_prompt_trait() {
    fn needs_prompt<T: Prompt>() {}
    needs_prompt::<PlainStruct>();
    needs_prompt::<PromptedStruct>();
    needs_prompt::<SkipStruct>();
    needs_prompt::<NestedNamedStruct>();
    needs_prompt::<OptionFieldStruct>();
    needs_prompt::<Newtype>();
    needs_prompt::<MultiField>();
    needs_prompt::<PromptedTuple>();
    needs_prompt::<UnitNoPrompt>();
    needs_prompt::<UnitWithPrompt>();
    needs_prompt::<UnitEnum>();
    needs_prompt::<PromptedEnum>();
    needs_prompt::<MixedEnum>();
    needs_prompt::<AllTupleEnum>();
    needs_prompt::<AllStructEnum>();
    needs_prompt::<Inner>();
    needs_prompt::<Outer>();
}

#[test]
fn all_shapes_satisfy_elicit_introspect() {
    fn needs_introspect<T: ElicitIntrospect>() {}
    needs_introspect::<PlainStruct>();
    needs_introspect::<PromptedStruct>();
    needs_introspect::<SkipStruct>();
    needs_introspect::<NestedNamedStruct>();
    needs_introspect::<OptionFieldStruct>();
    needs_introspect::<Newtype>();
    needs_introspect::<MultiField>();
    needs_introspect::<PromptedTuple>();
    needs_introspect::<UnitNoPrompt>();
    needs_introspect::<UnitWithPrompt>();
    needs_introspect::<UnitEnum>();
    needs_introspect::<PromptedEnum>();
    needs_introspect::<MixedEnum>();
    needs_introspect::<AllTupleEnum>();
    needs_introspect::<AllStructEnum>();
    needs_introspect::<Inner>();
    needs_introspect::<Outer>();
}

#[test]
fn structs_satisfy_survey_trait() {
    fn needs_survey<T: Survey>() {}
    needs_survey::<PlainStruct>();
    needs_survey::<PromptedStruct>();
    needs_survey::<SkipStruct>();
    needs_survey::<NestedNamedStruct>();
    needs_survey::<OptionFieldStruct>();
    needs_survey::<Newtype>();
    needs_survey::<MultiField>();
    needs_survey::<PromptedTuple>();
    needs_survey::<UnitNoPrompt>();
    needs_survey::<UnitWithPrompt>();
}

#[test]
fn enums_satisfy_select_trait() {
    fn needs_select<T: Select>() {}
    needs_select::<UnitEnum>();
    needs_select::<PromptedEnum>();
    needs_select::<MixedEnum>();
    needs_select::<AllTupleEnum>();
    needs_select::<AllStructEnum>();
    needs_select::<Inner>();
    needs_select::<Outer>();
}
