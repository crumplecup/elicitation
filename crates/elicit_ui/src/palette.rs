//! Proof-carrying colour palette.
//!
//! A [`Palette`] is a set of colour-role assignments that have been proven
//! WCAG 2.2 Level AA compliant by construction.  It can only be produced by
//! [`PaletteBuilder::build`] after every required contrast pair has passed its
//! threshold.  Holding a `Palette` value *is* the proof of compliance — there
//! is no separate runtime check.
//!
//! ## Contrast thresholds
//!
//! | Role pair | Threshold | Source |
//! |---|---|---|
//! | `Text` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `Text` on `Surface` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `DimText` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `Comment` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `Keyword` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `StringLit` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `Number` on `Background` | 4.5:1 | WCAG 1.4.3 normal text AA |
//! | `Accent` on `Background` | 3:1 | WCAG 1.4.11 non-text component AA |
//! | `Error` on `Background` | 3:1 | WCAG 1.4.11 non-text component AA |
//!
//! All text roles require 4.5:1 — code is text, developers read it, no
//! exemptions are made for syntax colours.
//!
//! ## Heuristic assistance
//!
//! [`PaletteBuilder::suggest`] returns the nearest compliant colour for a
//! failing pair by adjusting lightness in linear-light RGB while preserving
//! hue and saturation as closely as possible.
//!
//! [`PaletteBuilder::build_adjusted`] applies these suggestions automatically,
//! converging to a compliant palette in at most a handful of iterations.

use std::fmt;

use elicitation::Established;
use tracing::instrument;

use crate::{
    ContrastPair, SrgbColor, WcagContrastMinimumNormalText, WcagNonTextContrastMinimum,
    contrast_ratio,
    proof_credentials::{PaletteNonTextVerified, PaletteNormalTextVerified},
};

// ── SemanticRole ──────────────────────────────────────────────────────────────

/// A named colour role in a [`Palette`].
///
/// Each role has a defined WCAG contrast threshold against its expected
/// background that [`PaletteBuilder`] enforces before issuing a [`Palette`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum SemanticRole {
    /// Main page / panel background.
    Background = 0,
    /// Slightly elevated surface (card, sidebar, panel header).
    Surface = 1,
    /// Primary body text — 4.5:1 against `Background` and `Surface`.
    Text = 2,
    /// Secondary / muted annotation text — 4.5:1 against `Background`.
    DimText = 3,
    /// Interactive accent (links, buttons, cursor, selection border) — 3:1.
    Accent = 4,
    /// Error indicators and destructive-action highlights — 3:1.
    Error = 5,
    /// SQL / code keyword — 4.5:1 against `Background`.
    Keyword = 6,
    /// String literal — 4.5:1 against `Background`.
    StringLit = 7,
    /// Comment — 4.5:1 against `Background`.
    Comment = 8,
    /// Numeric literal — 4.5:1 against `Background`.
    Number = 9,
}

impl SemanticRole {
    pub(crate) const COUNT: usize = 10;

    const ALL: [Self; Self::COUNT] = [
        Self::Background,
        Self::Surface,
        Self::Text,
        Self::DimText,
        Self::Accent,
        Self::Error,
        Self::Keyword,
        Self::StringLit,
        Self::Comment,
        Self::Number,
    ];
}

impl fmt::Display for SemanticRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Background => "background",
            Self::Surface => "surface",
            Self::Text => "text",
            Self::DimText => "dim_text",
            Self::Accent => "accent",
            Self::Error => "error",
            Self::Keyword => "keyword",
            Self::StringLit => "string_lit",
            Self::Comment => "comment",
            Self::Number => "number",
        })
    }
}

// ── Verified pairs ─────────────────────────────────────────────────────────

/// A colour pair proved to meet the 4.5:1 WCAG normal-text threshold (SC 1.4.3).
#[derive(Debug, Clone, Copy)]
pub struct NormalTextPair {
    /// The verified foreground/background pair with its measured ratio.
    pub pair: ContrastPair,
    /// Proof token — only obtainable after the ratio check passes.
    pub proof: Established<WcagContrastMinimumNormalText>,
}

/// A colour pair proved to meet the 3:1 WCAG non-text-component threshold (SC 1.4.11).
#[derive(Debug, Clone, Copy)]
pub struct NonTextPair {
    /// The verified pair.
    pub pair: ContrastPair,
    /// Proof token.
    pub proof: Established<WcagNonTextContrastMinimum>,
}

// ── PaletteColors ─────────────────────────────────────────────────────────────

/// Raw sRGB colour assignments for all [`SemanticRole`]s.
///
/// Use [`Palette::colors`] or [`Palette::color`] to access these from a
/// fully-proved palette.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaletteColors {
    /// Main background.
    pub background: SrgbColor,
    /// Elevated surface.
    pub surface: SrgbColor,
    /// Primary text.
    pub text: SrgbColor,
    /// Secondary muted text.
    pub dim_text: SrgbColor,
    /// Interactive accent.
    pub accent: SrgbColor,
    /// Error colour.
    pub error: SrgbColor,
    /// Code keyword.
    pub keyword: SrgbColor,
    /// String literal.
    pub string_lit: SrgbColor,
    /// Comment.
    pub comment: SrgbColor,
    /// Numeric literal.
    pub number: SrgbColor,
}

impl PaletteColors {
    /// Get the colour for a given role.
    pub fn get(&self, role: SemanticRole) -> SrgbColor {
        match role {
            SemanticRole::Background => self.background,
            SemanticRole::Surface => self.surface,
            SemanticRole::Text => self.text,
            SemanticRole::DimText => self.dim_text,
            SemanticRole::Accent => self.accent,
            SemanticRole::Error => self.error,
            SemanticRole::Keyword => self.keyword,
            SemanticRole::StringLit => self.string_lit,
            SemanticRole::Comment => self.comment,
            SemanticRole::Number => self.number,
        }
    }
}

// ── Palette ───────────────────────────────────────────────────────────────────

/// A WCAG 2.2 Level AA–compliant colour palette.
///
/// Constructed exclusively through [`PaletteBuilder::build`] or
/// [`PaletteBuilder::build_adjusted`], both of which validate every required
/// colour pair before issuing the proof tokens stored in the fields below.
///
/// Holding a `Palette` value *is* proof of compliance.
#[derive(Debug, Clone)]
pub struct Palette {
    /// Raw colour values — use for rendering.
    pub colors: PaletteColors,

    // ── 4.5:1 normal-text pairs ──────────────────────────────────────────────
    /// Text on main background — 4.5:1 (WCAG 1.4.3).
    pub text_on_bg: NormalTextPair,
    /// Text on elevated surface — 4.5:1 (WCAG 1.4.3).
    pub text_on_surface: NormalTextPair,
    /// Secondary text on main background — 4.5:1 (WCAG 1.4.3).
    pub dim_text_on_bg: NormalTextPair,
    /// SQL keyword on main background — 4.5:1 (WCAG 1.4.3).
    pub keyword_on_bg: NormalTextPair,
    /// String literal on main background — 4.5:1 (WCAG 1.4.3).
    pub string_on_bg: NormalTextPair,
    /// Numeric literal on main background — 4.5:1 (WCAG 1.4.3).
    pub number_on_bg: NormalTextPair,
    /// Comment on main background — 4.5:1 (WCAG 1.4.3).
    pub comment_on_bg: NormalTextPair,

    // ── 3:1 non-text-component pairs ─────────────────────────────────────────
    /// Accent on main background — 3:1 (WCAG 1.4.11).
    pub accent_on_bg: NonTextPair,
    /// Error indicator on main background — 3:1 (WCAG 1.4.11).
    pub error_on_bg: NonTextPair,
}

impl Palette {
    /// Get the colour for a given role.
    pub fn color(&self, role: SemanticRole) -> SrgbColor {
        self.colors.get(role)
    }
}

// ── PaletteBuildError ─────────────────────────────────────────────────────────

/// Error returned by [`PaletteBuilder::build`] when the palette is not
/// WCAG-compliant.
#[derive(Debug)]
pub enum PaletteBuildError {
    /// One or more colour roles were not set before calling `build`.
    Missing(Vec<SemanticRole>),
    /// One or more colour pairs failed their contrast threshold.
    ///
    /// Each report includes a heuristic [`ContrastSuggestion`].
    Contrast(Vec<ContrastReport>),
}

impl fmt::Display for PaletteBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(roles) => {
                write!(f, "missing colour roles: ")?;
                for (i, r) in roles.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{r}")?;
                }
                Ok(())
            }
            Self::Contrast(reports) => {
                writeln!(f, "contrast failures:")?;
                for r in reports {
                    writeln!(f, "  {r}")?;
                }
                Ok(())
            }
        }
    }
}

// ── ContrastSuggestion ────────────────────────────────────────────────────────

/// Heuristic suggestion for a non-compliant colour.
///
/// Produced by [`PaletteBuilder::suggest`] and embedded in [`ContrastReport`].
/// The suggested colour is the nearest compliant colour found by adjusting
/// lightness in linear-light RGB while preserving the original hue and
/// saturation as closely as possible.
#[derive(Debug, Clone, Copy)]
pub struct ContrastSuggestion {
    /// Suggested replacement colour.
    pub color: SrgbColor,
    /// Contrast ratio achieved by the suggested colour against its background.
    pub achieved_ratio: f32,
}

// ── ContrastReport ────────────────────────────────────────────────────────────

/// Diagnostic emitted when a colour pair fails a contrast check.
#[derive(Debug, Clone)]
pub struct ContrastReport {
    /// The foreground role that failed.
    pub foreground_role: SemanticRole,
    /// The background role it was checked against.
    pub background_role: SemanticRole,
    /// Measured contrast ratio.
    pub ratio: f32,
    /// Required minimum ratio for this pair.
    pub required: f32,
    /// Nearest compliant colour (if the heuristic found one).
    pub suggestion: Option<ContrastSuggestion>,
}

impl fmt::Display for ContrastReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} on {}: {:.2}:1 < {:.1}:1",
            self.foreground_role, self.background_role, self.ratio, self.required
        )?;
        if let Some(s) = self.suggestion {
            write!(
                f,
                " → suggest {} (achieves {:.2}:1)",
                s.color.to_hex(),
                s.achieved_ratio
            )?;
        }
        Ok(())
    }
}

// ── PaletteBuilder ────────────────────────────────────────────────────────────

/// Builder for a WCAG 2.2 Level AA–compliant [`Palette`].
///
/// Set every [`SemanticRole`] with [`set`][Self::set], then call
/// [`build`][Self::build].  Every required contrast pair is checked; failures
/// are returned as [`ContrastReport`]s with heuristic suggestions.
///
/// For automatic correction, call [`build_adjusted`][Self::build_adjusted]
/// instead — it applies the nearest-compliant suggestion for each failing pair
/// and iterates until the palette is fully compliant.
#[derive(Debug, Clone)]
pub struct PaletteBuilder {
    colors: [Option<SrgbColor>; SemanticRole::COUNT],
}

impl Default for PaletteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PaletteBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self {
            colors: [None; SemanticRole::COUNT],
        }
    }

    /// Set a colour role.  Returns `self` for chaining.
    pub fn set(mut self, role: SemanticRole, color: SrgbColor) -> Self {
        self.colors[role as usize] = Some(color);
        self
    }

    /// Current contrast ratio for a foreground/background role pair.
    ///
    /// Returns `None` if either role has not been set yet.
    pub fn contrast_for(&self, fg: SemanticRole, bg: SemanticRole) -> Option<f32> {
        let fg_c = self.colors[fg as usize]?;
        let bg_c = self.colors[bg as usize]?;
        Some(contrast_ratio(&fg_c, &bg_c))
    }

    /// Heuristic suggestion for a failing role pair at `min_ratio`.
    ///
    /// Returns `None` if either role is unset or the pair already passes.
    pub fn suggest(
        &self,
        fg: SemanticRole,
        bg: SemanticRole,
        min_ratio: f32,
    ) -> Option<ContrastSuggestion> {
        let fg_c = self.colors[fg as usize]?;
        let bg_c = self.colors[bg as usize]?;
        suggest_compliant(fg_c, bg_c, min_ratio)
    }

    /// Validate all required pairs and produce a [`Palette`].
    ///
    /// Returns [`PaletteBuildError::Missing`] if any role has not been set.
    /// Returns [`PaletteBuildError::Contrast`] if any pair fails its threshold,
    /// with heuristic suggestions included in each report.
    #[instrument(skip(self))]
    pub fn build(self) -> Result<Palette, PaletteBuildError> {
        self.build_inner(true)
    }

    fn build_inner(self, warn_on_failure: bool) -> Result<Palette, PaletteBuildError> {
        // ── Check for unset roles ─────────────────────────────────────────────
        let missing: Vec<SemanticRole> = SemanticRole::ALL
            .iter()
            .copied()
            .filter(|&r| self.colors[r as usize].is_none())
            .collect();
        if !missing.is_empty() {
            return Err(PaletteBuildError::Missing(missing));
        }

        let c = |r: SemanticRole| self.colors[r as usize].unwrap();
        let bg = c(SemanticRole::Background);
        let surface = c(SemanticRole::Surface);
        let text = c(SemanticRole::Text);
        let dim_text = c(SemanticRole::DimText);
        let accent = c(SemanticRole::Accent);
        let error = c(SemanticRole::Error);
        let keyword = c(SemanticRole::Keyword);
        let string_lit = c(SemanticRole::StringLit);
        let comment = c(SemanticRole::Comment);
        let number = c(SemanticRole::Number);

        // ── Check 4.5:1 normal-text pairs ────────────────────────────────────
        let normal_checks: &[(SrgbColor, SemanticRole, SrgbColor, SemanticRole)] = &[
            (text, SemanticRole::Text, bg, SemanticRole::Background),
            (text, SemanticRole::Text, surface, SemanticRole::Surface),
            (
                dim_text,
                SemanticRole::DimText,
                bg,
                SemanticRole::Background,
            ),
            (keyword, SemanticRole::Keyword, bg, SemanticRole::Background),
            (
                string_lit,
                SemanticRole::StringLit,
                bg,
                SemanticRole::Background,
            ),
            (comment, SemanticRole::Comment, bg, SemanticRole::Background),
            (number, SemanticRole::Number, bg, SemanticRole::Background),
        ];

        // ── Check 3:1 non-text pairs ──────────────────────────────────────────
        let non_text_checks: &[(SrgbColor, SemanticRole, SrgbColor, SemanticRole)] = &[
            (accent, SemanticRole::Accent, bg, SemanticRole::Background),
            (error, SemanticRole::Error, bg, SemanticRole::Background),
        ];

        let mut failures: Vec<ContrastReport> = Vec::new();

        for &(fg_c, fg_role, bg_c, bg_role) in normal_checks {
            let ratio = contrast_ratio(&fg_c, &bg_c);
            if ratio < 4.5 {
                failures.push(ContrastReport {
                    foreground_role: fg_role,
                    background_role: bg_role,
                    ratio,
                    required: 4.5,
                    suggestion: suggest_compliant(fg_c, bg_c, 4.5),
                });
            }
        }

        for &(fg_c, fg_role, bg_c, bg_role) in non_text_checks {
            let ratio = contrast_ratio(&fg_c, &bg_c);
            if ratio < 3.0 {
                failures.push(ContrastReport {
                    foreground_role: fg_role,
                    background_role: bg_role,
                    ratio,
                    required: 3.0,
                    suggestion: suggest_compliant(fg_c, bg_c, 3.0),
                });
            }
        }

        if !failures.is_empty() {
            // Only warn when called directly by user code.  build_adjusted calls
            // build_inner() which suppresses this warning — its intermediate
            // failures are expected and already logged at debug level.
            if warn_on_failure {
                tracing::warn!(count = failures.len(), "palette contrast failures");
            } else {
                tracing::debug!(
                    count = failures.len(),
                    "palette contrast failures (adjusting)"
                );
            }
            return Err(PaletteBuildError::Contrast(failures));
        }

        // ── Mint proof tokens ─────────────────────────────────────────────────
        let normal_pair = |fg: SrgbColor, bg_c: SrgbColor| NormalTextPair {
            pair: ContrastPair {
                foreground: fg,
                background: bg_c,
                ratio: contrast_ratio(&fg, &bg_c).into(),
            },
            proof: Established::prove(&PaletteNormalTextVerified),
        };

        let non_text_pair = |fg: SrgbColor, bg_c: SrgbColor| NonTextPair {
            pair: ContrastPair {
                foreground: fg,
                background: bg_c,
                ratio: contrast_ratio(&fg, &bg_c).into(),
            },
            proof: Established::prove(&PaletteNonTextVerified),
        };

        Ok(Palette {
            colors: PaletteColors {
                background: bg,
                surface,
                text,
                dim_text,
                accent,
                error,
                keyword,
                string_lit,
                comment,
                number,
            },
            text_on_bg: normal_pair(text, bg),
            text_on_surface: normal_pair(text, surface),
            dim_text_on_bg: normal_pair(dim_text, bg),
            keyword_on_bg: normal_pair(keyword, bg),
            string_on_bg: normal_pair(string_lit, bg),
            number_on_bg: normal_pair(number, bg),
            comment_on_bg: normal_pair(comment, bg),
            accent_on_bg: non_text_pair(accent, bg),
            error_on_bg: non_text_pair(error, bg),
        })
    }

    /// Build a palette, automatically applying the nearest-compliant heuristic
    /// correction for every failing pair.
    ///
    /// Iterates until all pairs pass (converges in practice within 2–3 rounds).
    /// Panics if any role was not set, or if the heuristic cannot find a
    /// compliant colour (e.g. the background is mid-gray, making 4.5:1
    /// impossible for any foreground).
    #[instrument(skip(self))]
    pub fn build_adjusted(self) -> Palette {
        let mut colors = self.colors;
        let mut round = 0u32;
        loop {
            round += 1;
            assert!(
                round <= 16,
                "PaletteBuilder::build_adjusted did not converge"
            );

            let builder = PaletteBuilder { colors };
            match builder.build_inner(false) {
                Ok(palette) => {
                    tracing::debug!(rounds = round, "palette converged");
                    return palette;
                }
                Err(PaletteBuildError::Missing(roles)) => {
                    panic!(
                        "PaletteBuilder::build_adjusted: unset roles: {}",
                        roles
                            .iter()
                            .map(|r| r.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                Err(PaletteBuildError::Contrast(reports)) => {
                    let mut changed = false;
                    for report in &reports {
                        if let Some(s) = report.suggestion {
                            tracing::debug!(
                                role = %report.foreground_role,
                                from = %colors[report.foreground_role as usize]
                                    .map(|c| c.to_hex())
                                    .unwrap_or_default(),
                                to = %s.color.to_hex(),
                                was = report.ratio,
                                now = s.achieved_ratio,
                                "adjusting colour for compliance"
                            );
                            colors[report.foreground_role as usize] = Some(s.color);
                            changed = true;
                        } else {
                            panic!(
                                "PaletteBuilder::build_adjusted: no suggestion for {} \
                                 (ratio {:.2}, required {:.1}) — background may be mid-gray",
                                report.foreground_role, report.ratio, report.required
                            );
                        }
                    }
                    if !changed {
                        panic!("PaletteBuilder::build_adjusted: no progress made");
                    }
                }
            }
        }
    }
}

// ── Heuristics ────────────────────────────────────────────────────────────────

/// Linearise a gamma-encoded sRGB channel value (0.0–1.0) to linear light.
fn linearise(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Apply sRGB gamma encoding to a linear-light channel value (0.0–1.0).
fn gamma_encode(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// WCAG 2.1 relative luminance of an sRGB colour.
fn relative_luminance(c: SrgbColor) -> f32 {
    0.2126 * linearise(c.r) + 0.7152 * linearise(c.g) + 0.0722 * linearise(c.b)
}

/// Scale the linear-light channels of `color` so that the resulting colour has
/// relative luminance `target_lum`.
///
/// Preserves the proportional relationship between channels (i.e. preserves hue
/// and saturation in linear-light RGB).  Returns `None` if the current colour
/// has zero luminance and cannot be scaled upward.
fn scale_to_luminance(color: SrgbColor, target_lum: f32) -> Option<SrgbColor> {
    let lr = linearise(color.r);
    let lg = linearise(color.g);
    let lb = linearise(color.b);
    let current_lum = 0.2126 * lr + 0.7152 * lg + 0.0722 * lb;

    if current_lum < 1e-7 {
        // All-black: cannot scale proportionally.  Emit a neutral grey at the
        // target luminance instead.
        let l = gamma_encode(target_lum.cbrt());
        return Some(SrgbColor::new(l, l, l));
    }

    let scale = target_lum / current_lum;
    Some(SrgbColor::new(
        gamma_encode((lr * scale).clamp(0.0, 1.0)),
        gamma_encode((lg * scale).clamp(0.0, 1.0)),
        gamma_encode((lb * scale).clamp(0.0, 1.0)),
    ))
}

/// Euclidean distance in sRGB space (used to pick the closer correction).
fn srgb_distance(a: SrgbColor, b: SrgbColor) -> f32 {
    let dr = a.r - b.r;
    let dg = a.g - b.g;
    let db = a.b - b.b;
    (dr * dr + dg * dg + db * db).sqrt()
}

/// Return the nearest WCAG-compliant colour for `candidate` against
/// `background` at threshold `min_ratio`.
///
/// Returns `None` if `candidate` already passes.  Otherwise, tries both
/// lightening and darkening and returns whichever requires the smaller sRGB
/// channel shift.
pub fn suggest_compliant(
    candidate: SrgbColor,
    background: SrgbColor,
    min_ratio: f32,
) -> Option<ContrastSuggestion> {
    if contrast_ratio(&candidate, &background) >= min_ratio {
        return None;
    }

    let bg_lum = relative_luminance(background);

    // Required luminance if foreground is lighter than background.
    let req_lum_lighter = (bg_lum + 0.05) * min_ratio - 0.05;
    // Required luminance if foreground is darker than background.
    let req_lum_darker = (bg_lum + 0.05) / min_ratio - 0.05;

    let lighter = if req_lum_lighter <= 1.0 {
        scale_to_luminance(candidate, req_lum_lighter.max(0.0))
    } else {
        None
    };

    let darker = if req_lum_darker >= 0.0 {
        scale_to_luminance(candidate, req_lum_darker)
    } else {
        None
    };

    // Pick the correction closest to the original.
    let chosen = match (lighter, darker) {
        (Some(l), Some(d)) => {
            if srgb_distance(l, candidate) <= srgb_distance(d, candidate) {
                l
            } else {
                d
            }
        }
        (Some(l), None) => l,
        (None, Some(d)) => d,
        (None, None) => return None,
    };

    Some(ContrastSuggestion {
        color: chosen,
        achieved_ratio: contrast_ratio(&chosen, &background),
    })
}
