//! Help screen implementation for the Pomodoro timer application.
//!
//! This screen displays helpful information about using the Pomodoro timer,
//! including keyboard shortcuts, session types, and general instructions.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::any::Any;

use super::Screen;
use crate::data::TimerData;

/// Help screen that displays usage instructions and keyboard shortcuts.
pub struct HelpScreen;

impl Screen for HelpScreen {
    fn render(&self, frame: &mut Frame, _timer_data: &TimerData, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        Self::render_header(frame, chunks[0]);
        Self::render_content(frame, chunks[1]);
        Self::render_footer(frame, chunks[2]);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HelpScreen {
    /// Render the header section with title.
    fn render_header(frame: &mut Frame, area: Rect) {
        let header = Paragraph::new("🍅 Tomat - Help & Instructions")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        frame.render_widget(header, area);
    }

    /// Render the main content with instructions and shortcuts.
    fn render_content(frame: &mut Frame, area: Rect) {
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // What is Pomodoro
                Constraint::Length(6), // Session types
                Constraint::Length(8), // Keyboard shortcuts
                Constraint::Min(0),    // Additional info
            ])
            .split(area);

        Self::render_pomodoro_info(frame, content_chunks[0]);
        Self::render_session_types(frame, content_chunks[1]);
        Self::render_shortcuts(frame, content_chunks[2]);
        Self::render_additional_info(frame, content_chunks[3]);
    }

    /// Render information about the Pomodoro technique.
    fn render_pomodoro_info(frame: &mut Frame, area: Rect) {
        let text = "The Pomodoro Technique is a time management method that uses a timer to break work into intervals.\nTraditionally 25 minutes of focused work followed by a 5-minute break.\nAfter 4 work sessions, take a longer 15-minute break to recharge.";
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("What is Pomodoro?"),
            )
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(paragraph, area);
    }

    /// Render information about different session types.
    fn render_session_types(frame: &mut Frame, area: Rect) {
        let text = "• Work Session: 25 minutes of focused work\n• Short Break: 5 minutes of rest\n• Long Break: 15 minutes after 4 work sessions\n\nCustomize durations with CLI arguments: --work, --short-break, --long-break";
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Session Types"),
            )
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(paragraph, area);
    }

    /// Render keyboard shortcuts.
    fn render_shortcuts(frame: &mut Frame, area: Rect) {
        let text = "• [Space/Enter] Start/Pause timer\n• [R] Reset timer to beginning\n• [S] Skip current session\n• [N] Name current session\n• [F] Toggle fullscreen mode\n• [H] Show this help screen\n• [Q] Quit application\n• [Ctrl+C] Force quit";
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Keyboard Shortcuts"),
            )
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(paragraph, area);
    }

    /// Render additional information.
    fn render_additional_info(frame: &mut Frame, area: Rect) {
        let text = "• Sessions are automatically logged to tomat.log\n• Progress is shown as a percentage complete\n• Use [N] to name sessions for better tracking\n• The app follows the MVC architecture pattern\n• Built with Rust and ratatui for terminal UI";
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Additional Information"),
            )
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(paragraph, area);
    }

    /// Render the footer with navigation hint.
    fn render_footer(frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new("Press [H] to return to timer | [Q] to quit")
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        frame.render_widget(footer, area);
    }
}
