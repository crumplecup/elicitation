//! [`RenderContext`] for egui — theme-based colour inspection.
//!
//! egui renders via GPU (not a CPU-accessible buffer), so per-cell colour
//! inspection from a render buffer is not available.  Instead, this context
//! reads the current `ui.visuals()` state at widget-draw time to obtain a
//! best-effort foreground/background pair for WCAG contrast checks.
//!
//! **Limitations:** Widget-specific colour overrides (e.g. `colored_label`,
//! selection highlights, `RichText`) are not captured — the context reflects
//! the theme's default text/background colours only.  When the colour cannot
//! be determined (e.g. alpha < 255 on either channel), `colors_at` returns
//! `None` and the contrast check is silently skipped.

use egui::Rect;
use elicit_ui::{RenderColors, RenderContext};

// ── Helper functions (runtime-proofs) ────────────────────────────────────────

#[cfg(feature = "runtime-proofs")]
fn avg_rgb(samples: &[[u8; 3]]) -> [u8; 3] {
    let n = samples.len() as u32;
    let sum = samples.iter().fold([0u32; 3], |mut acc, &[r, g, b]| {
        acc[0] += r as u32;
        acc[1] += g as u32;
        acc[2] += b as u32;
        acc
    });
    [(sum[0] / n) as u8, (sum[1] / n) as u8, (sum[2] / n) as u8]
}

#[cfg(feature = "runtime-proofs")]
fn rgb_sq_distance(a: [u8; 3], b: [u8; 3]) -> u32 {
    let dr = a[0].abs_diff(b[0]) as u32;
    let dg = a[1].abs_diff(b[1]) as u32;
    let db = a[2].abs_diff(b[2]) as u32;
    dr * dr + dg * dg + db * db
}

// ── EguiRenderContext ─────────────────────────────────────────────────────────

/// A [`RenderContext`] for egui that derives colours from `ui.visuals()`.
///
/// Construct via [`EguiRenderContext::from_ui`] inside a widget closure to
/// capture the theme state at draw time.  All egui colour inspection is
/// best-effort: only fully-opaque theme colours are reported.
pub struct EguiRenderContext {
    fg: Option<[u8; 3]>,
    bg: Option<[u8; 3]>,
}

impl EguiRenderContext {
    /// Derive a render context from the current egui `Ui` theme state.
    ///
    /// Reads `ui.visuals().text_color()` and
    /// `ui.visuals().widgets.noninteractive.bg_fill`.  Returns `None` for any
    /// channel whose alpha is less than 255 (translucent colours are
    /// meaningless for WCAG opaque contrast maths).
    pub fn from_ui(ui: &egui::Ui) -> Self {
        let vis = ui.visuals();
        Self {
            fg: color32_to_rgb(vis.text_color()),
            bg: color32_to_rgb(vis.widgets.noninteractive.bg_fill),
        }
    }
}

/// Extract `[r, g, b]` from a `Color32`, returning `None` when alpha < 255.
fn color32_to_rgb(c: egui::Color32) -> Option<[u8; 3]> {
    if c.a() < 255 {
        None
    } else {
        Some([c.r(), c.g(), c.b()])
    }
}

impl RenderContext for EguiRenderContext {
    /// egui uses floating-point [`egui::Rect`] as its area type.
    type Area = Rect;

    fn symbol_at(&self, _area: &Rect, _col: u16, _row: u16) -> &str {
        ""
    }

    fn area_width(&self, area: &Rect) -> u16 {
        area.width() as u16
    }

    fn area_height(&self, area: &Rect) -> u16 {
        area.height() as u16
    }

    /// Returns the stored theme colours, or `None` if either was translucent.
    ///
    /// `col` and `row` are ignored — egui colour state is uniform across the
    /// widget area; this context stores a single pair derived from visuals.
    fn colors_at(&self, _area: &Rect, _col: u16, _row: u16) -> Option<RenderColors> {
        match (self.fg, self.bg) {
            (Some(fg), Some(bg)) => Some(RenderColors {
                foreground: fg,
                background: bg,
            }),
            _ => None,
        }
    }
}

// ── GpuPixelContext ───────────────────────────────────────────────────────────

/// A [`RenderContext`] backed by raw GPU pixel data read back from a wgpu surface.
///
/// Construct via [`GpuPixelContext::new`] after mapping a staging buffer from
/// `copy_texture_to_buffer` + `map_async` + `device.poll(wait_indefinitely())`.
///
/// ## Coordinate system
///
/// `area` coordinates are in egui logical points; all pixel indexing converts
/// via the `pixels_per_point` (`ppp`) value captured at widget draw time.
///
/// ## Colour accuracy
///
/// This context offers improved best-effort checks over [`EguiRenderContext`]:
/// widget-specific colour overrides (e.g. `colored_label`) are captured here
/// because we read actual rendered pixels rather than theme defaults.
/// Anti-aliasing, compositing, and border pixels can still contaminate
/// corner/divergent sampling — this is improved best-effort, not proof-grade
/// ground truth.
///
/// ## sRGB note
///
/// Pixel values are read verbatim from the surface texture.  For sRGB formats
/// (`Bgra8UnormSrgb`, `Rgba8UnormSrgb`) the stored bytes are gamma-encoded
/// sRGB, which matches what the WCAG contrast maths in `elicit_ui` expects.
/// For linear formats (`Bgra8Unorm`, `Rgba8Unorm`) the bytes are linear —
/// the contrast ratio would be slightly off, but surface textures are almost
/// always sRGB, making this a non-issue in practice.
#[cfg(feature = "runtime-proofs")]
pub struct GpuPixelContext<'a> {
    pixels: &'a [u8],
    width: u32,
    bytes_per_row: u32,
    height: u32,
    format: wgpu::TextureFormat,
    /// Pixels per egui logical point — converts rect coordinates to physical pixels.
    ppp: f32,
}

#[cfg(feature = "runtime-proofs")]
impl<'a> GpuPixelContext<'a> {
    /// Construct from a mapped staging buffer slice.
    ///
    /// * `pixels` — byte slice from [`wgpu::BufferView`] (via `get_mapped_range`)
    /// * `width` — surface width in physical pixels
    /// * `bytes_per_row` — padded row stride in bytes (must be a multiple of 256)
    /// * `height` — surface height in physical pixels
    /// * `format` — surface texture format (determines byte order)
    /// * `ppp` — pixels per egui logical point from `egui::Context::pixels_per_point()`
    pub fn new(
        pixels: &'a [u8],
        width: u32,
        bytes_per_row: u32,
        height: u32,
        format: wgpu::TextureFormat,
        ppp: f32,
    ) -> Self {
        Self { pixels, width, bytes_per_row, height, format, ppp }
    }

    /// Sample one pixel at physical coordinates `(x_px, y_px)`.
    ///
    /// Returns `None` if coordinates are out of bounds or the buffer is too small.
    fn sample_pixel(&self, x_px: u32, y_px: u32) -> Option<[u8; 3]> {
        if x_px >= self.width || y_px >= self.height {
            return None;
        }
        // All common surface formats are 4 bytes per pixel (RGBA / BGRA / etc.).
        let bpp: u32 = 4;
        let offset = (y_px * self.bytes_per_row + x_px * bpp) as usize;
        if offset + 4 > self.pixels.len() {
            return None;
        }
        let b0 = self.pixels[offset];
        let b1 = self.pixels[offset + 1];
        let b2 = self.pixels[offset + 2];
        // Bgra8* formats store bytes as [B, G, R, A]; Rgba8* as [R, G, B, A].
        let rgb = match self.format {
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb => [b2, b1, b0],
            _ => [b0, b1, b2],
        };
        Some(rgb)
    }
}

#[cfg(feature = "runtime-proofs")]
impl<'a> RenderContext for GpuPixelContext<'a> {
    /// egui uses floating-point [`egui::Rect`] as its area type.
    type Area = egui::Rect;

    fn symbol_at(&self, _area: &egui::Rect, _col: u16, _row: u16) -> &str {
        ""
    }

    fn area_width(&self, area: &egui::Rect) -> u16 {
        area.width() as u16
    }

    fn area_height(&self, area: &egui::Rect) -> u16 {
        area.height() as u16
    }

    /// Sample the pixel at the given logical `(col, row)` offset within `area`.
    ///
    /// Returns the same colour for both foreground and background — single-pixel
    /// queries cannot distinguish text from background.  For proper fg/bg
    /// detection use [`sample_colors`](Self::sample_colors).
    fn colors_at(&self, area: &egui::Rect, col: u16, row: u16) -> Option<RenderColors> {
        let x = (area.left() * self.ppp) as u32 + col as u32;
        let y = (area.top() * self.ppp) as u32 + row as u32;
        let rgb = self.sample_pixel(x, y)?;
        Some(RenderColors { foreground: rgb, background: rgb })
    }

    /// Estimate foreground and background colours using corner sampling + grid search.
    ///
    /// **Background**: average of the four corner pixels.  Corners are dominated
    /// by the widget background (padding area), making them a reliable estimate.
    ///
    /// **Foreground**: the pixel in a 5×5 grid whose squared-RGB distance from
    /// the background estimate is greatest.  This catches text and icon pixels
    /// that diverge visually from the background.
    ///
    /// Returns `None` if the rect is too small, all pixels are identical, or
    /// the rect falls entirely outside the surface bounds.
    fn sample_colors(&self, area: &egui::Rect) -> Option<RenderColors> {
        let left_px = (area.left() * self.ppp) as u32;
        let top_px = (area.top() * self.ppp) as u32;
        let right_px = ((area.right() * self.ppp) as u32)
            .saturating_sub(1)
            .min(self.width.saturating_sub(1));
        let bottom_px = ((area.bottom() * self.ppp) as u32)
            .saturating_sub(1)
            .min(self.height.saturating_sub(1));

        if right_px <= left_px || bottom_px <= top_px {
            return None;
        }

        // Background: average of four corners.
        let corners = [
            (left_px, top_px),
            (right_px, top_px),
            (left_px, bottom_px),
            (right_px, bottom_px),
        ];
        let bg_samples: Vec<[u8; 3]> = corners
            .iter()
            .filter_map(|&(x, y)| self.sample_pixel(x, y))
            .collect();
        if bg_samples.is_empty() {
            return None;
        }
        let bg = avg_rgb(&bg_samples);

        // Foreground: most-divergent pixel in a 5×5 grid.
        let step_x = ((right_px - left_px) / 4).max(1);
        let step_y = ((bottom_px - top_px) / 4).max(1);
        let mut best_rgb = bg;
        let mut best_dist = 0u32;
        for row in 0..5u32 {
            for col in 0..5u32 {
                let x = (left_px + col * step_x).min(right_px);
                let y = (top_px + row * step_y).min(bottom_px);
                if let Some(rgb) = self.sample_pixel(x, y) {
                    let dist = rgb_sq_distance(rgb, bg);
                    if dist > best_dist {
                        best_dist = dist;
                        best_rgb = rgb;
                    }
                }
            }
        }

        // If all sampled pixels are the same colour, fg = bg is meaningless.
        if best_dist == 0 {
            return None;
        }

        Some(RenderColors { foreground: best_rgb, background: bg })
    }
}
