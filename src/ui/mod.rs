//! UI module for the Pomodoro timer application.
//!
//! This module contains all the UI rendering logic organized into dedicated submodules
//! for better maintainability and separation of concerns.

use ratatui::prelude::*;

use crate::app::App;

pub mod controls;
pub mod header;
pub mod modal;
pub mod stats;
pub mod timer;

/// Main render function that orchestrates the rendering of all UI components.
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(6), // Timer area (increased from 5 to 6 for two 3-line boxes)
            Constraint::Min(0),    // Stats (flexible)
            Constraint::Length(3), // Controls
        ])
        .split(frame.area());

    header::render(frame, chunks[0], app);
    timer::render(frame, chunks[1], app);
    stats::render(frame, chunks[2], app);
    controls::render(frame, chunks[3], app);

    // Render naming modal on top if in naming mode
    if app.naming_mode {
        modal::render(frame, app);
    }
}
