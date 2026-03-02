//! [`ElicitSpec`](crate::ElicitSpec) implementations for std extras:
//! `DurationPositive`, `PathBufExists`, `PathBufReadable`, `PathBufIsDir`, `PathBufIsFile`.

use crate::verification::types::{
    DurationPositive, PathBufExists, PathBufIsDir, PathBufIsFile, PathBufReadable,
};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── Duration ──────────────────────────────────────────────────────────────────

impl ElicitSpec for DurationPositive {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("positive".to_string())
                    .description(
                        "Duration must be greater than zero (at least 1 nanosecond).".to_string(),
                    )
                    .expression(Some("duration.as_nanos() > 0".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("DurationPositive".to_string())
            .summary("A std::time::Duration guaranteed to be greater than zero.".to_string())
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "DurationPositive",
    DurationPositive::type_spec,
    std::any::TypeId::of::<DurationPositive>
));

// ── PathBuf contract types ────────────────────────────────────────────────────

macro_rules! impl_pathbuf_spec {
    (
        type    = $ty:ty,
        name    = $name:literal,
        summary = $summary:literal,
        requires = [($req_label:literal, $req_desc:literal, $req_expr:literal)] $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label($req_label.to_string())
                            .description($req_desc.to_string())
                            .expression(Some($req_expr.to_string()))
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                let notes = SpecCategoryBuilder::default()
                    .name("notes".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("runtime_check".to_string())
                            .description(
                                "Validation is a runtime filesystem check, not compile-time."
                                    .to_string(),
                            )
                            .expression(None)
                            .build()
                            .expect("valid SpecEntry"),
                    ])
                    .build()
                    .expect("valid SpecCategory");
                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires, notes])
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

impl_pathbuf_spec!(
    type    = PathBufExists,
    name    = "PathBufExists",
    summary = "A PathBuf guaranteed to exist on the filesystem at construction time.",
    requires = [("exists", "Path must exist on the filesystem.", "path.exists()")],
);

impl_pathbuf_spec!(
    type    = PathBufReadable,
    name    = "PathBufReadable",
    summary = "A PathBuf guaranteed to be readable (metadata accessible) at construction time.",
    requires = [("readable", "Path metadata must be accessible.", "path.metadata().is_ok()")],
);

impl_pathbuf_spec!(
    type    = PathBufIsDir,
    name    = "PathBufIsDir",
    summary = "A PathBuf guaranteed to point to a directory at construction time.",
    requires = [("is_dir", "Path must be an existing directory.", "path.is_dir()")],
);

impl_pathbuf_spec!(
    type    = PathBufIsFile,
    name    = "PathBufIsFile",
    summary = "A PathBuf guaranteed to point to a regular file at construction time.",
    requires = [("is_file", "Path must be an existing regular file.", "path.is_file()")],
);
