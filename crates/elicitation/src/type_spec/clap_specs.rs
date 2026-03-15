//! [`ElicitSpec`](crate::ElicitSpec) implementations for clap type elicitation.
//!
//! Available with the `clap-types` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/clap_types/` — those describe *structure* (pattern, variants),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "clap-types")]
mod clap_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_select_spec!
    //
    // Generates ElicitSpec for a clap Select enum. Produces a TypeSpec with:
    //   - "variants" category listing each label and its description
    //   - "source" category noting this is a third-party clap type
    // -------------------------------------------------------------------------

    macro_rules! impl_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("clap v4 — third-party CLI argument parser".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Select — choose one variant from the list".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    // -------------------------------------------------------------------------
    // Macro: impl_builder_spec!
    //
    // Generates ElicitSpec for a clap builder/struct type.
    // -------------------------------------------------------------------------

    macro_rules! impl_builder_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("clap v4 — third-party CLI argument parser".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — structured builder type elicited field by field".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    // =========================================================================
    // ColorChoice — when to emit ANSI color
    // =========================================================================

    impl_select_spec!(
        type    = clap::ColorChoice,
        name    = "clap::ColorChoice",
        summary = "Controls whether clap emits ANSI color sequences in output.",
        variants = [
            ("Auto (detect terminal)", "Enable color if the terminal reports it supports ANSI sequences"),
            ("Always",                 "Always emit ANSI color sequences regardless of terminal"),
            ("Never",                  "Suppress all ANSI color output"),
        ]
    );

    // =========================================================================
    // ArgAction — what to do when an argument is matched
    // =========================================================================

    impl_select_spec!(
        type    = clap::ArgAction,
        name    = "clap::ArgAction",
        summary = "Defines what clap does when a command-line argument is matched.",
        variants = [
            ("Set (store single value)",  "Store the argument value, replacing any previous value"),
            ("Append (accumulate values)","Append the argument value to a list; repeated flags accumulate"),
            ("SetTrue (flag → true)",     "Set a boolean flag to true when the argument appears"),
            ("SetFalse (flag → false)",   "Set a boolean flag to false when the argument appears"),
            ("Count (tally occurrences)", "Increment an integer counter each time the argument appears"),
            ("Help (print help)",         "Print full help text and exit"),
            ("HelpShort (print short help)", "Print condensed help text and exit"),
            ("Version (print version)",   "Print version information and exit"),
        ]
    );

    // =========================================================================
    // ValueSource — where a parsed value came from
    // =========================================================================

    impl_select_spec!(
        type    = clap::parser::ValueSource,
        name    = "clap::parser::ValueSource",
        summary = "Indicates the origin of a parsed argument value.",
        variants = [
            ("DefaultValue", "Value was supplied by the argument's default (not the user)"),
            ("EnvVariable",  "Value came from an environment variable"),
            ("CommandLine",  "Value was provided explicitly on the command line"),
        ]
    );

    // =========================================================================
    // ErrorKind — categories of clap parse errors
    // =========================================================================

    impl_select_spec!(
        type    = clap::error::ErrorKind,
        name    = "clap::error::ErrorKind",
        summary = "Classifies the kind of error encountered during argument parsing.",
        variants = [
            ("InvalidValue",          "An argument was provided but its value failed validation"),
            ("UnknownArgument",       "An unrecognised argument was passed"),
            ("InvalidSubcommand",     "A subcommand name was not recognised"),
            ("NoEquals",              "A flag requiring `=` was given without one"),
            ("ValueValidation",       "A custom value validator rejected the argument"),
            ("TooManyValues",         "More values were supplied than the argument accepts"),
            ("TooFewValues",          "Fewer values were supplied than the argument requires"),
            ("WrongNumberOfValues",   "The number of values does not match the required count"),
            ("ArgumentConflict",      "Two mutually exclusive arguments were both provided"),
            ("MissingRequiredArgument","A required argument was not provided"),
            ("MissingSubcommand",     "A subcommand is required but was not given"),
            ("InvalidUtf8",           "An argument value contained invalid UTF-8 bytes"),
            ("DisplayHelp",           "Help was requested (not a hard error)"),
            ("DisplayHelpOnMissingArgumentOrSubcommand", "Help was auto-displayed due to missing input"),
            ("DisplayVersion",        "Version was requested (not a hard error)"),
            ("Io",                    "An I/O error occurred during output"),
            ("Format",                "A formatting error occurred during output"),
        ]
    );

    // =========================================================================
    // ValueHint — shell completion hints
    // =========================================================================

    impl_select_spec!(
        type    = clap::builder::ValueHint,
        name    = "clap::builder::ValueHint",
        summary = "Hints to shell completion generators what kind of value an argument expects.",
        variants = [
            ("Unknown",                  "No completion hint; shell uses its default behaviour"),
            ("Other",                    "Non-standard hint; completion script may ignore it"),
            ("AnyPath (file or directory)", "Complete with any filesystem path"),
            ("FilePath",                 "Complete with file paths only"),
            ("DirPath",                  "Complete with directory paths only"),
            ("ExecutablePath",           "Complete with executable file paths"),
            ("CommandName",              "Complete with command names (from PATH)"),
            ("CommandString",            "Complete with a full command string"),
            ("CommandWithArguments",     "Complete with a command followed by its arguments"),
            ("Username",                 "Complete with system usernames"),
            ("Hostname",                 "Complete with known hostnames"),
            ("Url",                      "Complete with URLs"),
            ("EmailAddress",             "Complete with email addresses"),
        ]
    );

    // =========================================================================
    // Builder / struct types
    // =========================================================================

    impl_builder_spec!(
        type    = clap::Arg,
        name    = "clap::Arg",
        summary = "Defines a single command-line argument: flags, options, or positionals.",
        fields = [
            ("id",       "Unique string identifier for the argument"),
            ("short",    "Optional single-character short flag (e.g. `-v`)"),
            ("long",     "Optional long flag name (e.g. `--verbose`)"),
            ("help",     "Short description shown in the help message"),
            ("action",   "What clap does when this argument is matched (ArgAction)"),
            ("required", "Whether this argument must be supplied by the user"),
        ]
    );

    impl_builder_spec!(
        type    = clap::ArgGroup,
        name    = "clap::ArgGroup",
        summary = "Groups related arguments to enforce mutual exclusivity or requirements.",
        fields = [
            ("id",       "Unique string identifier for the group"),
            ("args",     "Argument IDs that belong to this group"),
            ("required", "Whether at least one group member must be provided"),
            ("multiple", "Whether multiple group members may be provided simultaneously"),
        ]
    );

    impl_builder_spec!(
        type    = clap::Command,
        name    = "clap::Command",
        summary = "The root command or a subcommand definition with its arguments and metadata.",
        fields = [
            ("name",        "Command name used in help output and subcommand matching"),
            ("version",     "Version string printed by `--version`"),
            ("about",       "One-line description of what the command does"),
            ("args",        "Arguments accepted by this command"),
            ("subcommands", "Nested subcommands"),
        ]
    );

    impl_builder_spec!(
        type    = clap::builder::Str,
        name    = "clap::builder::Str",
        summary = "A cheaply-cloneable string type used internally by clap for argument values.",
        fields = [
            ("value", "The underlying string content"),
        ]
    );

    impl_builder_spec!(
        type    = clap::builder::PossibleValue,
        name    = "clap::builder::PossibleValue",
        summary = "One valid value for an argument, with optional aliases and help text.",
        fields = [
            ("name",    "The canonical string value the user must type"),
            ("aliases", "Alternative strings that are accepted in place of the name"),
            ("help",    "Short description shown next to this value in help output"),
            ("hide",    "Whether to hide this value from completion and help listings"),
        ]
    );

    impl_builder_spec!(
        type    = clap::builder::ValueRange,
        name    = "clap::builder::ValueRange",
        summary = "Constrains how many values an argument may accept (e.g. exactly 2, 1..=3).",
        fields = [
            ("min", "Minimum number of values required"),
            ("max", "Maximum number of values permitted (None = unbounded)"),
        ]
    );
}
