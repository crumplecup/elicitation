//! Ratatui terminal frontend for the archive CLI.
//!
//! Renders the [`VerifiedTree`] IR to a crossterm terminal using
//! [`RatatuiBackend`] + [`render_node`].

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use elicit_ratatui::{RatatuiBackend, render_node};
use elicit_ui::{UiRenderer, VerifiedTree};
use ratatui::{Terminal, backend::CrosstermBackend};
use tracing::instrument;

use crate::archive::{
    ArchiveResult,
    errors::{ArchiveError, ArchiveErrorKind},
};

/// Render the archive tree to a crossterm terminal, blocking until the user
/// presses `q` or `Esc`.
///
/// # Verification chain
///
/// `VerifiedTree` carries `RenderComplete` proofs from the ArchiveDisplay IR
/// pipeline.  `RatatuiBackend::render(&tree)` asserts `Established<RenderComplete>`
/// at each frame, preserving the formal guarantee end-to-end.
#[instrument(skip(tree))]
pub fn run_tui(tree: VerifiedTree) -> ArchiveResult<()> {
    enable_raw_mode().map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    let backend_term = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend_term).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    let ratatui_backend = RatatuiBackend::new();
    ratatui_backend
        .render(&tree)
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;

    let tui_root = ratatui_backend.last_tui_tree();

    let result = (|| -> ArchiveResult<()> {
        loop {
            terminal
                .draw(|frame: &mut ratatui::Frame<'_>| {
                    if let Some(node) = &tui_root {
                        render_node(frame, frame.area(), node);
                    }
                })
                .map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })?;

            if event::poll(std::time::Duration::from_millis(100)).map_err(|e: std::io::Error| {
                ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
            })? {
                if let Event::Key(key) = event::read().map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })? {
                    if key.kind == KeyEventKind::Press
                        && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                    {
                        break;
                    }
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode().map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    result
}
