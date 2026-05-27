//! [`TuiCommunicator`] — [`ElicitCommunicator`] for crossterm raw-mode terminals.
//!
//! Renders elicitation prompts to the bottom of the terminal and reads player
//! responses character-by-character in the current raw-mode context.
//!
//! # Design intent
//!
//! This module provides a self-contained [`ElicitCommunicator`] for terminal-based
//! interactive applications. It has no application-specific dependencies — only
//! `crossterm`, `elicitation`, `rmcp`, `serde_json`, `tokio`, and `tracing`.
//!
//! # Supported tool calls
//!
//! | Tool | Behaviour |
//! |---|---|
//! | `elicit_select` | Numbered menu; single-keystroke selection for ≤ 9 options |
//! | `elicit_text` | Free-text input identical to [`send_prompt`] |
//! | others | Returns [`ServiceError::Cancelled`] |
//!
//! This means [`ChoiceSet`] works correctly with [`TuiCommunicator`]: filtered
//! runtime option sets are presented as a numbered list and resolved with a
//! single keystroke.
//!
//! # Raw mode contract
//!
//! [`TuiCommunicator`] assumes the terminal is already in raw mode, as
//! established by an enclosing game loop.  It does not call
//! `enable_raw_mode` / `disable_raw_mode` itself.
//!
//! [`send_prompt`]: ElicitCommunicator::send_prompt
//! [`ChoiceSet`]: elicitation::ChoiceSet
//! [`ServiceError::Cancelled`]: rmcp::service::ServiceError::Cancelled

use std::io::{self, Write as _};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal,
};
use elicitation::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, ElicitationContext,
    StyleContext, StyleMarker,
};
use rmcp::model::{CallToolResult, Content};
use tracing::instrument;

/// Default number of rows reserved at the bottom of the screen for the prompt pane.
const DEFAULT_PROMPT_ROWS: u16 = 4;

// ── TuiCommunicator ───────────────────────────────────────────────────────────

/// An [`ElicitCommunicator`] that drives elicitation through a crossterm terminal.
///
/// Prompts are rendered into a reserved pane at the bottom of the screen.
/// For free-text elicitation, the player types and presses Enter.  For
/// `elicit_select` (used by [`ChoiceSet`]), options are presented as a
/// numbered list and the player presses a single digit key.
///
/// Construct with [`TuiCommunicator::new`] for the default 4-row pane, or
/// [`TuiCommunicator::with_prompt_rows`] to match your layout's reserved height.
///
/// [`ChoiceSet`]: elicitation::ChoiceSet
#[derive(Clone)]
pub struct TuiCommunicator {
    style_ctx: StyleContext,
    elicit_ctx: ElicitationContext,
    prompt_rows: u16,
}

impl TuiCommunicator {
    /// Creates a communicator with the default 4-row prompt pane.
    pub fn new() -> Self {
        Self::with_prompt_rows(DEFAULT_PROMPT_ROWS)
    }

    /// Creates a communicator reserving `prompt_rows` rows at the bottom of the
    /// screen for the prompt pane.
    pub fn with_prompt_rows(prompt_rows: u16) -> Self {
        Self {
            style_ctx: StyleContext::default(),
            elicit_ctx: ElicitationContext::default(),
            prompt_rows,
        }
    }
}

impl Default for TuiCommunicator {
    fn default() -> Self {
        Self::new()
    }
}

impl ElicitCommunicator for TuiCommunicator {
    /// Render the prompt in the bottom pane and block until the player presses Enter.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let prompt = prompt.to_string();
        let prompt_rows = self.prompt_rows;
        async move { tui_read_text(&prompt, prompt_rows).await }
    }

    /// Handle MCP tool calls for the tools that make sense in a TUI context.
    ///
    /// - `elicit_select` — numbered menu, single-key response
    /// - `elicit_text` — free-text input (same as [`send_prompt`])
    /// - others — [`ServiceError::Cancelled`]
    ///
    /// [`send_prompt`]: ElicitCommunicator::send_prompt
    /// [`ServiceError::Cancelled`]: rmcp::service::ServiceError::Cancelled
    #[instrument(skip(self, params), level = "debug", fields(tool = %params.name))]
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        let name = params.name.to_string();
        let args = params.arguments.clone().unwrap_or_default();
        let prompt_rows = self.prompt_rows;

        async move {
            match name.as_str() {
                "elicit_select" => {
                    let prompt = args
                        .get("prompt")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Choose an option:");
                    let options: Vec<String> = args
                        .get("options")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();

                    if options.is_empty() {
                        return Err(rmcp::service::ServiceError::Cancelled {
                            reason: Some("elicit_select: no options provided".to_string()),
                        });
                    }

                    tui_read_select(prompt, &options, prompt_rows)
                        .await
                        .map(|selected| CallToolResult::success(vec![Content::text(selected)]))
                        .map_err(|e| rmcp::service::ServiceError::Cancelled {
                            reason: Some(e.to_string()),
                        })
                }

                "elicit_text" => {
                    let prompt = args
                        .get("prompt")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Enter text:");

                    tui_read_text(prompt, prompt_rows)
                        .await
                        .map(|text| CallToolResult::success(vec![Content::text(text)]))
                        .map_err(|e| rmcp::service::ServiceError::Cancelled {
                            reason: Some(e.to_string()),
                        })
                }

                other => {
                    tracing::warn!(tool = %other, "TuiCommunicator: unsupported tool");
                    Err(rmcp::service::ServiceError::Cancelled {
                        reason: Some(format!("TUI context does not support tool: {other}")),
                    })
                }
            }
        }
    }

    fn style_context(&self) -> &StyleContext {
        &self.style_ctx
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        &self.elicit_ctx
    }

    fn with_style<
        T: 'static,
        S: StyleMarker + elicitation::style::ElicitationStyle + 'static,
    >(
        &self,
        style: S,
    ) -> Self {
        let mut new = self.clone();
        new.style_ctx.set_style::<T, S>(style).ok();
        new
    }
}

// ── Terminal helpers ──────────────────────────────────────────────────────────

/// Clear the prompt pane and render wrapped text lines in cyan.
///
/// Leaves the cursor on the last row of the pane, ready for input rendering.
fn render_prompt(text: &str, prompt_rows: u16, reserve_input_row: bool) {
    let mut stdout = io::stdout();
    let (cols, rows) = terminal::size().unwrap_or((80, 24));
    let pane_top = rows.saturating_sub(prompt_rows);
    let content_col: u16 = 2;
    let content_width = cols.saturating_sub(4) as usize;
    let max_lines = if reserve_input_row {
        prompt_rows.saturating_sub(1) as usize
    } else {
        prompt_rows as usize
    };

    // Word-wrap each source line to content_width.
    let mut wrapped: Vec<String> = Vec::new();
    for line in text.lines() {
        if line.is_empty() {
            wrapped.push(String::new());
            continue;
        }
        let mut remaining = line;
        while !remaining.is_empty() {
            let split = remaining
                .char_indices()
                .take_while(|(i, _)| *i < content_width)
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(remaining.len());
            wrapped.push(remaining[..split].to_string());
            remaining = &remaining[split..];
        }
    }
    // Keep only the tail that fits.
    if wrapped.len() > max_lines {
        let skip = wrapped.len() - max_lines;
        wrapped = wrapped.split_off(skip);
    }

    // Clear the pane.
    for row in pane_top..rows {
        execute!(
            stdout,
            MoveTo(0, row),
            Print(" ".repeat(cols as usize)),
        )
        .ok();
    }

    // Render lines.
    for (i, line) in wrapped.iter().enumerate() {
        execute!(
            stdout,
            MoveTo(content_col, pane_top + i as u16),
            SetForegroundColor(Color::Cyan),
            Print(line),
            ResetColor,
        )
        .ok();
    }
}

/// Free-text input: render prompt, read characters until Enter.
///
/// Drains stale events first to prevent prior keystrokes from bleeding in.
/// Returns the trimmed input string, or an error on I/O failure or Ctrl-C.
async fn tui_read_text(prompt: &str, prompt_rows: u16) -> ElicitResult<String> {
    let mut stdout = io::stdout();
    let mut input = String::new();

    // Drain stale key events.
    tokio::time::sleep(Duration::from_millis(20)).await;
    while event::poll(Duration::ZERO).unwrap_or(false) {
        let _ = event::read();
    }

    let (_, rows) = terminal::size().unwrap_or((80, 24));
    let input_row = rows.saturating_sub(1);

    render_prompt(prompt, prompt_rows, true);

    // Input cursor on the last pane row.
    execute!(
        stdout,
        MoveTo(2, input_row),
        SetForegroundColor(Color::Green),
        Print("▶ "),
        ResetColor,
    )
    .map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Terminal write error: {e}"
        )))
    })?;
    stdout.flush().map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Terminal flush error: {e}"
        )))
    })?;

    loop {
        let ev = event::read().map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Event read error: {e}"
            )))
        })?;

        let Event::Key(key) = ev else { continue };

        match key.code {
            KeyCode::Enter => break,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                tracing::warn!("TUI elicitation cancelled via Ctrl-C");
                return Err(ElicitError::new(ElicitErrorKind::ParseError(
                    "Cancelled by user".to_string(),
                )));
            }
            KeyCode::Char(c) => {
                input.push(c);
                execute!(stdout, Print(c)).ok();
                stdout.flush().ok();
            }
            KeyCode::Backspace if !input.is_empty() => {
                input.pop();
                execute!(stdout, Print("\x08 \x08")).ok();
                stdout.flush().ok();
            }
            _ => {}
        }
    }

    let result = input.trim().to_string();
    tracing::debug!(response = %result, "TUI text input received");
    Ok(result)
}

/// Numbered-select input: render prompt + numbered options, await a digit key.
///
/// For ≤ 9 options the player presses a single digit key — no Enter required.
/// For > 9 options the player types a number and presses Enter.
/// Returns the label of the selected option.
async fn tui_read_select(
    prompt: &str,
    options: &[String],
    prompt_rows: u16,
) -> ElicitResult<String> {
    let mut stdout = io::stdout();

    // Drain stale events.
    tokio::time::sleep(Duration::from_millis(20)).await;
    while event::poll(Duration::ZERO).unwrap_or(false) {
        let _ = event::read();
    }

    // Build a single text block: prompt + numbered options.
    let mut text = prompt.to_string();
    text.push('\n');
    for (i, opt) in options.iter().enumerate() {
        text.push_str(&format!("  {}. {opt}\n", i + 1));
    }

    let single_key = options.len() <= 9;

    render_prompt(&text, prompt_rows, true);

    let (_, rows) = terminal::size().unwrap_or((80, 24));
    let input_row = rows.saturating_sub(1);

    let hint = if single_key {
        format!("▶ [1-{}]", options.len())
    } else {
        "▶ number + Enter: ".to_string()
    };

    execute!(
        stdout,
        MoveTo(2, input_row),
        SetForegroundColor(Color::Green),
        Print(&hint),
        ResetColor,
    )
    .map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Terminal write error: {e}"
        )))
    })?;
    stdout.flush().map_err(|e| {
        ElicitError::new(ElicitErrorKind::ParseError(format!(
            "Terminal flush error: {e}"
        )))
    })?;

    if single_key {
        // Accept a single digit keypress — no Enter required.
        loop {
            let ev = event::read().map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Event read error: {e}"
                )))
            })?;

            let Event::Key(key) = ev else { continue };

            if let KeyCode::Char('c') = key.code
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                tracing::warn!("TUI elicitation cancelled via Ctrl-C");
                return Err(ElicitError::new(ElicitErrorKind::ParseError(
                    "Cancelled by user".to_string(),
                )));
            }

            if let KeyCode::Char(c) = key.code
                && let Some(n) = c.to_digit(10)
            {
                let idx = (n as usize).saturating_sub(1);
                if idx < options.len() {
                    let selected = options[idx].clone();
                    tracing::debug!(selected = %selected, "TUI select: single-key choice");
                    return Ok(selected);
                }
            }
        }
    } else {
        // Read a number followed by Enter.
        let mut input = String::new();
        loop {
            let ev = event::read().map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Event read error: {e}"
                )))
            })?;

            let Event::Key(key) = ev else { continue };

            match key.code {
                KeyCode::Enter => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        let idx = n.saturating_sub(1);
                        if idx < options.len() {
                            let selected = options[idx].clone();
                            tracing::debug!(selected = %selected, "TUI select: typed choice");
                            return Ok(selected);
                        }
                    }
                    // Invalid — clear and retry.
                    input.clear();
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    tracing::warn!("TUI elicitation cancelled via Ctrl-C");
                    return Err(ElicitError::new(ElicitErrorKind::ParseError(
                        "Cancelled by user".to_string(),
                    )));
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    input.push(c);
                    execute!(stdout, Print(c)).ok();
                    stdout.flush().ok();
                }
                KeyCode::Backspace if !input.is_empty() => {
                    input.pop();
                    execute!(stdout, Print("\x08 \x08")).ok();
                    stdout.flush().ok();
                }
                _ => {}
            }
        }
    }
}
