//! Normal screen implementation for the Pomodoro timer application.
//!
//! This screen displays the full timer interface with header, timer area,
//! statistics, and controls in a structured layout.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
    Frame,
};
use std::any::Any;

use super::Screen;
use crate::data::TimerData;

/// Normal screen that displays the full timer interface.
pub struct NormalScreen;

impl Screen for NormalScreen {
    fn render(&self, frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        // Create the main layout matching the original design
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(6), // Timer area (3 + 3 for timer and session)
                Constraint::Min(0),    // Stats (flexible)
                Constraint::Length(3), // Controls
            ])
            .split(area);

        Self::render_header(frame, timer_data, chunks[0]);
        Self::render_timer_area(frame, timer_data, chunks[1]);
        Self::render_stats_area(frame, timer_data, chunks[2]);
        Self::render_controls(frame, timer_data, chunks[3]);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl NormalScreen {
    /// Render the header section with title and status.
    fn render_header(frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        let title = if timer_data.is_paused {
            "⏸️ Pomodoro - Paused"
        } else if timer_data.is_running {
            "🍅 Pomodoro - Active"
        } else {
            "🍅 Pomodoro - Ready to Start"
        };

        let color = if timer_data.is_paused {
            Color::Yellow
        } else if timer_data.is_running {
            Color::Red
        } else {
            Color::Blue
        };

        let header = Paragraph::new(title)
            .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        frame.render_widget(header, area);
    }

    /// Render the timer area with time remaining and session info.
    fn render_timer_area(frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        let timer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)])
            .split(area);

        // Time remaining display
        let time_text = if timer_data.remaining_time.as_secs() == 0 {
            String::from("--:--")
        } else {
            timer_data.format_time()
        };

        let status_text = if timer_data.is_paused {
            " (PAUSED)"
        } else if !timer_data.is_running {
            " (PRESS SPACE/ENTER TO START)"
        } else {
            ""
        };

        let full_text = format!("{time_text}{status_text}");
        let timer_display = Paragraph::new(full_text)
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Time Remaining"),
            );
        frame.render_widget(timer_display, timer_chunks[0]);

        // Session info
        let session_text: String = if timer_data.naming_mode {
            "📝 NAMING MODE - Type session name...".to_string()
        } else if timer_data.session_name.is_empty() {
            "Press [N] to name this session".to_string()
        } else {
            format!("📝 {}", timer_data.session_name)
        };
        let session_display = Paragraph::new(session_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Session"),
            );
        frame.render_widget(session_display, timer_chunks[1]);
    }

    /// Render the statistics area with progress gauge and session info.
    fn render_stats_area(frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        let stats_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Progress gauge
                Constraint::Length(3), // Statistics
                Constraint::Min(0),    // Current session info
            ])
            .split(area);

        // Progress gauge
        let progress = timer_data.progress_percentage() / 100.0;
        let progress_gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Progress"),
            )
            .gauge_style(Style::default().fg(Color::Cyan))
            .ratio(progress)
            .label(format!("{:.1}%", timer_data.progress_percentage()));
        frame.render_widget(progress_gauge, stats_chunks[0]);

        // Statistics
        let stats_text = format!("Completed Sessions: {}", timer_data.sessions_completed);
        let stats_display = Paragraph::new(stats_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Statistics"),
            )
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(stats_display, stats_chunks[1]);

        // Current session info
        let session_info = timer_data.session_start_time.as_ref().map_or_else(
            || "Session started: --:--:--".to_string(),
            |start_time| format!("Session started: {start_time}"),
        );
        let session_display = Paragraph::new(session_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Current Session"),
            )
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(session_display, stats_chunks[2]);
    }

    /// Render the controls section with available keyboard shortcuts.
    fn render_controls(frame: &mut Frame, timer_data: &TimerData, area: Rect) {
        let controls_text = if timer_data.is_running {
            "Controls: [Space] Pause/Resume | [R] Reset | [S] Skip | [N] Name Session | [F] Fullscreen | [H] Help | [C] Clock | [Q] Quit"
        } else if timer_data.session_start_time.is_some() {
            // Timer has started but is currently paused
            "Controls: [Space/Enter] Resume | [R] Reset | [N] Name Session | [F] Fullscreen | [H] Help | [C] Clock | [Q] Quit"
        } else {
            // Timer has never started
            "Controls: [Space/Enter] Start Timer | [R] Reset | [N] Name Session | [F] Fullscreen | [H] Help | [C] Clock | [Q] Quit"
        };

        let controls = Paragraph::new(controls_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Controls"),
            );
        frame.render_widget(controls, area);
    }
}
