//! Fullscreen screen implementation for the Pomodoro timer application.
//!
//! This screen displays a minimal, focused timer view that maximizes
//! the timer display for distraction-free work sessions.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::any::Any;

use super::Screen;
use crate::data::TimerData;

/// Fullscreen screen that displays a minimal, focused timer view.
pub struct FullscreenScreen;

impl Screen for FullscreenScreen {
    fn render(&self, frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        // Create the main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Timer area (centered)
                Constraint::Length(3), // Help at bottom
            ])
            .split(area);

        Self::render_timer(frame, timer_data, chunks[0]);
        Self::render_help(frame, chunks[1]);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FullscreenScreen {
    /// Render the centered timer display.
    fn render_timer(frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        let timer_text = vec![Line::from(vec![Span::styled(
            timer_data.format_time(),
            Style::default().fg(ratatui::style::Color::Yellow),
        )])];

        // Center the timer both horizontally and vertically
        let timer_x = area.x
            + (area
                .width
                .saturating_sub(u16::try_from(timer_data.format_time().len()).unwrap_or(0)))
                / 2;
        let timer_y = area.y + (area.height.saturating_sub(1)) / 2;

        let timer = Paragraph::new(timer_text).block(Block::default().borders(Borders::NONE));
        frame.render_widget(
            timer,
            Rect::new(
                timer_x,
                timer_y,
                u16::try_from(timer_data.format_time().len()).unwrap_or(0),
                1,
            ),
        );
    }

    /// Render the help text at the bottom.
    fn render_help(frame: &mut Frame, area: Rect) {
        let help_text = vec![Line::from(vec![Span::raw(
            "Press F to return to normal view",
        )])];
        let help = Paragraph::new(help_text).block(Block::default().borders(Borders::TOP));
        frame.render_widget(help, area);
    }
}
