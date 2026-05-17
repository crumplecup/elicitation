//! Elicitation bridge primitives for SurrealDB types.
//!
//! Re-exports bridge types (implementing [`elicitation::Elicit`]) that allow
//! SurrealDB fields to appear in `#[derive(Elicit)]` structs.

pub mod surreal_types;

pub use surreal_types::{
    Datetime as SurrealDatetime, DatetimeStyle as SurrealDatetimeStyle,
    Duration as SurrealDuration, DurationStyle as SurrealDurationStyle,
    Geometry as SurrealGeometry, GeometryKind, GeometryKindStyle,
    GeometryStyle as SurrealGeometryStyle, Kind as SurrealKind, KindStyle as SurrealKindStyle,
    Number as SurrealNumber, NumberStyle as SurrealNumberStyle, PatchOp as SurrealPatchOp,
    PatchOpStyle as SurrealPatchOpStyle, RecordId as SurrealRecordId,
    RecordIdStyle as SurrealRecordIdStyle, Table as SurrealTable, TableStyle as SurrealTableStyle,
    Value as SurrealValue, ValueStyle as SurrealValueStyle,
};
