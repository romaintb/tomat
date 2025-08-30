//! Statistics rendering module for the Pomodoro timer application.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::app::App;

/// Renders the statistics area including progress gauge, completed sessions, and current session info.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    let progress = app.progress();
    let progress_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Progress"),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(progress)
        .label(format!("{:.1}%", progress * 100.0));

    frame.render_widget(progress_gauge, chunks[0]);

    let sessions_text = format!("Completed Sessions: {}", app.sessions_completed);
    let sessions_display = Paragraph::new(sessions_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Statistics"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(sessions_display, chunks[1]);

    if let Some(start_time) = app.current_session_start {
        let session_info = format!("Session started: {}", start_time.format("%H:%M:%S"));
        let session_display = Paragraph::new(session_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Current Session"),
            )
            .alignment(Alignment::Center);

        frame.render_widget(session_display, chunks[2]);
    }
}
