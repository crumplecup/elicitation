//! Elicitation for [`accesskit::geometry`] types:
//! [`Point`](accesskit::Point), [`Vec2`](accesskit::Vec2),
//! [`Size`](accesskit::Size), [`Rect`](accesskit::Rect),
//! and [`Affine`](accesskit::Affine).

use accesskit::{Affine, Point, Rect, Size, Vec2};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

// ── Point ─────────────────────────────────────────────────────────────────────

impl Prompt for Point {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2-D point (x, y):")
    }
}

crate::default_style!(Point => PointStyle);

impl Elicitation for Point {
    type Style = PointStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Point");
        Ok(Self {
            x: f64::elicit(communicator).await?,
            y: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Point {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Point",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "x", type_name: "f64", prompt: Some("X coordinate:") },
                    FieldInfo { name: "y", type_name: "f64", prompt: Some("Y coordinate:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Point {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Point".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Point {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! { accesskit::Point { x: #x, y: #y } }
    }
}

// ── Vec2 ──────────────────────────────────────────────────────────────────────

impl Prompt for Vec2 {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2-D vector (x, y):")
    }
}

crate::default_style!(Vec2 => Vec2Style);

impl Elicitation for Vec2 {
    type Style = Vec2Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Vec2");
        Ok(Self {
            x: f64::elicit(communicator).await?,
            y: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Vec2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Vec2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "x", type_name: "f64", prompt: Some("X component:") },
                    FieldInfo { name: "y", type_name: "f64", prompt: Some("Y component:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Vec2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Vec2".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Vec2 {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! { accesskit::Vec2 { x: #x, y: #y } }
    }
}

// ── Size ──────────────────────────────────────────────────────────────────────

impl Prompt for Size {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2-D size (width × height):")
    }
}

crate::default_style!(Size => SizeStyle);

impl Elicitation for Size {
    type Style = SizeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Size");
        Ok(Self {
            width: f64::elicit(communicator).await?,
            height: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Size {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Size",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "width", type_name: "f64", prompt: Some("Width:") },
                    FieldInfo { name: "height", type_name: "f64", prompt: Some("Height:") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Size {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Size".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(f64::prompt_tree())),
                ("height".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Size {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let w = self.width;
        let h = self.height;
        quote::quote! { accesskit::Size { width: #w, height: #h } }
    }
}

// ── Rect ──────────────────────────────────────────────────────────────────────

impl Prompt for Rect {
    fn prompt() -> Option<&'static str> {
        Some("Specify a rectangle (x0, y0, x1, y1 — top-left to bottom-right):")
    }
}

crate::default_style!(Rect => RectStyle);

impl Elicitation for Rect {
    type Style = RectStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Rect");
        Ok(Self {
            x0: f64::elicit(communicator).await?,
            y0: f64::elicit(communicator).await?,
            x1: f64::elicit(communicator).await?,
            y1: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Rect {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Rect",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "x0", type_name: "f64", prompt: Some("Left edge (x0):") },
                    FieldInfo { name: "y0", type_name: "f64", prompt: Some("Top edge (y0):") },
                    FieldInfo { name: "x1", type_name: "f64", prompt: Some("Right edge (x1):") },
                    FieldInfo { name: "y1", type_name: "f64", prompt: Some("Bottom edge (y1):") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Rect {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Rect".to_string(),
            fields: vec![
                ("x0".to_string(), Box::new(f64::prompt_tree())),
                ("y0".to_string(), Box::new(f64::prompt_tree())),
                ("x1".to_string(), Box::new(f64::prompt_tree())),
                ("y1".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Rect {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x0 = self.x0;
        let y0 = self.y0;
        let x1 = self.x1;
        let y1 = self.y1;
        quote::quote! { accesskit::Rect { x0: #x0, y0: #y0, x1: #x1, y1: #y1 } }
    }
}

// ── Affine ────────────────────────────────────────────────────────────────────

impl Prompt for Affine {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2-D affine transform (6 coefficients [a, b, c, d, e, f]):")
    }
}

crate::default_style!(Affine => AffineStyle);

impl Elicitation for Affine {
    type Style = AffineStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Affine");
        let a = f64::elicit(communicator).await?;
        let b = f64::elicit(communicator).await?;
        let c = f64::elicit(communicator).await?;
        let d = f64::elicit(communicator).await?;
        let e = f64::elicit(communicator).await?;
        let f = f64::elicit(communicator).await?;
        Ok(Affine::new([a, b, c, d, e, f]))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        for _ in 0..5 {
            ts.extend(<f64 as Elicitation>::kani_proof());
        }
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        for _ in 0..5 {
            ts.extend(<f64 as Elicitation>::verus_proof());
        }
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        for _ in 0..5 {
            ts.extend(<f64 as Elicitation>::creusot_proof());
        }
        ts
    }
}

impl ElicitIntrospect for Affine {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Affine",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "a", type_name: "f64", prompt: Some("Coefficient a (scale x):") },
                    FieldInfo { name: "b", type_name: "f64", prompt: Some("Coefficient b (shear y):") },
                    FieldInfo { name: "c", type_name: "f64", prompt: Some("Coefficient c (shear x):") },
                    FieldInfo { name: "d", type_name: "f64", prompt: Some("Coefficient d (scale y):") },
                    FieldInfo { name: "e", type_name: "f64", prompt: Some("Coefficient e (translate x):") },
                    FieldInfo { name: "f", type_name: "f64", prompt: Some("Coefficient f (translate y):") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Affine {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Affine".to_string(),
            fields: vec![
                ("a".to_string(), Box::new(f64::prompt_tree())),
                ("b".to_string(), Box::new(f64::prompt_tree())),
                ("c".to_string(), Box::new(f64::prompt_tree())),
                ("d".to_string(), Box::new(f64::prompt_tree())),
                ("e".to_string(), Box::new(f64::prompt_tree())),
                ("f".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Affine {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let [a, b, c, d, e, f] = self.as_coeffs();
        quote::quote! { accesskit::Affine::new([#a, #b, #c, #d, #e, #f]) }
    }
}
