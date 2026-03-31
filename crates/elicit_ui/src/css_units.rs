//! CSS unit parsing and responsive breakpoint definitions.
//!
//! Bridges Mozilla's `cssparser` to the elicit_ui constraint system,
//! providing:
//! - CSS length units (px, em, rem, vw, vh, %)
//! - Zoom-invariant size verification
//! - Responsive breakpoint sets for reflow testing

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// CSS length unit with numeric value.
///
/// Represents the subset of CSS `<length>` and `<percentage>` values
/// relevant to WCAG spatial constraints.
#[derive(Debug, Clone, Copy, PartialEq, Display, Serialize, Deserialize)]
pub enum CssLength {
    /// Absolute pixels.
    #[display("{}px", _0)]
    Px(f64),
    /// Relative to font size.
    #[display("{}em", _0)]
    Em(f64),
    /// Relative to root font size.
    #[display("{}rem", _0)]
    Rem(f64),
    /// Percentage of viewport width.
    #[display("{}vw", _0)]
    Vw(f64),
    /// Percentage of viewport height.
    #[display("{}vh", _0)]
    Vh(f64),
    /// Percentage of containing block.
    #[display("{}%", _0)]
    Percent(f64),
}

impl CssLength {
    /// Resolve to absolute pixels given a context.
    ///
    /// - `font_size_px`: current element's computed font size
    /// - `root_font_size_px`: root element's font size (default 16)
    /// - `viewport_width`: viewport width in px
    /// - `viewport_height`: viewport height in px
    /// - `containing_block`: containing block dimension in px (for %)
    #[tracing::instrument(level = "trace")]
    pub fn to_px(
        &self,
        font_size_px: f64,
        root_font_size_px: f64,
        viewport_width: f64,
        viewport_height: f64,
        containing_block: f64,
    ) -> f64 {
        match self {
            Self::Px(v) => *v,
            Self::Em(v) => v * font_size_px,
            Self::Rem(v) => v * root_font_size_px,
            Self::Vw(v) => v * viewport_width / 100.0,
            Self::Vh(v) => v * viewport_height / 100.0,
            Self::Percent(v) => v * containing_block / 100.0,
        }
    }

    /// Parse a CSS length string.
    ///
    /// Supports: `"16px"`, `"1.5em"`, `"2rem"`, `"50vw"`, `"100vh"`, `"80%"`.
    #[cfg(feature = "css")]
    #[tracing::instrument(level = "debug")]
    pub fn parse(input: &str) -> Result<Self, CssParseError> {
        use cssparser::{Parser, ParserInput};

        let mut parser_input = ParserInput::new(input);
        let mut parser = Parser::new(&mut parser_input);

        parser
            .try_parse(|p| {
                let token = p.next()?;
                match token {
                    cssparser::Token::Dimension { value, unit, .. } => {
                        match &**unit {
                            "px" => Ok(CssLength::Px(f64::from(*value))),
                            "em" => Ok(CssLength::Em(f64::from(*value))),
                            "rem" => Ok(CssLength::Rem(f64::from(*value))),
                            "vw" => Ok(CssLength::Vw(f64::from(*value))),
                            "vh" => Ok(CssLength::Vh(f64::from(*value))),
                            _ => Err(p.new_custom_error(())),
                        }
                    }
                    cssparser::Token::Percentage { unit_value, .. } => {
                        Ok(CssLength::Percent(f64::from(*unit_value) * 100.0))
                    }
                    _ => Err(p.new_custom_error(())),
                }
            })
            .map_err(|_: cssparser::ParseError<'_, ()>| {
                CssParseError::new(format!("invalid CSS length: {input}"))
            })
    }
}

/// Error from CSS parsing.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("CSS parse error: {} at {}:{}", message, file, line)]
pub struct CssParseError {
    /// Error description.
    pub message: String,
    /// Source file.
    pub file: &'static str,
    /// Source line.
    pub line: u32,
}

impl CssParseError {
    /// Create a new CSS parse error with caller location.
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: message.into(),
            file: loc.file(),
            line: loc.line(),
        }
    }
}

/// A named responsive breakpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Human-readable name (e.g., "mobile", "tablet").
    pub name: String,
    /// Minimum viewport width in px.
    pub min_width: u32,
    /// Maximum viewport width in px (inclusive).
    pub max_width: u32,
}

impl Breakpoint {
    /// Create a new breakpoint.
    pub fn new(name: impl Into<String>, min_width: u32, max_width: u32) -> Self {
        Self {
            name: name.into(),
            min_width,
            max_width,
        }
    }
}

/// A set of responsive breakpoints for reflow testing.
///
/// WCAG 1.4.10 requires content to reflow at 320px width.
#[derive(Debug, Clone)]
pub struct BreakpointSet {
    breakpoints: Vec<Breakpoint>,
}

impl BreakpointSet {
    /// Standard WCAG breakpoint set.
    ///
    /// Includes the mandatory 320px reflow width plus common device sizes.
    pub fn wcag() -> Self {
        Self {
            breakpoints: vec![
                Breakpoint::new("reflow-320", 320, 320),
                Breakpoint::new("mobile", 320, 479),
                Breakpoint::new("tablet", 480, 1023),
                Breakpoint::new("desktop", 1024, 1920),
            ],
        }
    }

    /// Get all breakpoints.
    pub fn breakpoints(&self) -> &[Breakpoint] {
        &self.breakpoints
    }

    /// Add a custom breakpoint.
    pub fn with_breakpoint(mut self, bp: Breakpoint) -> Self {
        self.breakpoints.push(bp);
        self
    }
}

/// Check if a dimension is zoom-invariant.
///
/// A dimension is zoom-invariant if it's expressed in relative units
/// (em, rem, vw, vh, %) rather than absolute pixels.
pub fn is_zoom_invariant(length: &CssLength) -> bool {
    !matches!(length, CssLength::Px(_))
}
