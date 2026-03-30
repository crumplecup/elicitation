//! Wrappers for accesskit geometry types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Wrapper around [`accesskit::Rect`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Rect(pub accesskit::Rect);

impl JsonSchema for Rect {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Rect".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::Rect::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::Rect::inline_schema()
    }
}

impl From<accesskit::Rect> for Rect {
    fn from(v: accesskit::Rect) -> Self {
        Self(v)
    }
}

impl From<Rect> for accesskit::Rect {
    fn from(v: Rect) -> Self {
        v.0
    }
}

impl std::ops::Deref for Rect {
    type Target = accesskit::Rect;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::Point`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Point(pub accesskit::Point);

impl JsonSchema for Point {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Point".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::Point::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::Point::inline_schema()
    }
}

impl From<accesskit::Point> for Point {
    fn from(v: accesskit::Point) -> Self {
        Self(v)
    }
}

impl From<Point> for accesskit::Point {
    fn from(v: Point) -> Self {
        v.0
    }
}

impl std::ops::Deref for Point {
    type Target = accesskit::Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::Size`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Size(pub accesskit::Size);

impl JsonSchema for Size {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Size".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::Size::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::Size::inline_schema()
    }
}

impl From<accesskit::Size> for Size {
    fn from(v: accesskit::Size) -> Self {
        Self(v)
    }
}

impl From<Size> for accesskit::Size {
    fn from(v: Size) -> Self {
        v.0
    }
}

impl std::ops::Deref for Size {
    type Target = accesskit::Size;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::Vec2`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Vec2(pub accesskit::Vec2);

impl JsonSchema for Vec2 {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Vec2".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::Vec2::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::Vec2::inline_schema()
    }
}

impl From<accesskit::Vec2> for Vec2 {
    fn from(v: accesskit::Vec2) -> Self {
        Self(v)
    }
}

impl From<Vec2> for accesskit::Vec2 {
    fn from(v: Vec2) -> Self {
        v.0
    }
}

impl std::ops::Deref for Vec2 {
    type Target = accesskit::Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::Affine`] (2D affine transform).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Affine(pub accesskit::Affine);

impl JsonSchema for Affine {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Affine".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::Affine::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::Affine::inline_schema()
    }
}

impl From<accesskit::Affine> for Affine {
    fn from(v: accesskit::Affine) -> Self {
        Self(v)
    }
}

impl From<Affine> for accesskit::Affine {
    fn from(v: Affine) -> Self {
        v.0
    }
}

impl std::ops::Deref for Affine {
    type Target = accesskit::Affine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
