//! Timer display rendering module for the Pomodoro timer application.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, timer::TimerState};

/// Renders the timer display area including time remaining and session information.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)]) // Both boxes get 3 lines
        .split(area);

    let time_remaining = app.time_remaining();
    let minutes = time_remaining.as_secs() / 60;
    let seconds = time_remaining.as_secs() % 60;

    let time_text = format!("{minutes:02}:{seconds:02}");
    let status_text = if app.timer.is_paused() {
        " (PAUSED)"
    } else {
        ""
    };
    let full_text = format!("{time_text}{status_text}");

    let timer_display = Paragraph::new(full_text)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Time Remaining"),
        );

    frame.render_widget(timer_display, chunks[0]);

    // Display session name if we're in a work session
    if matches!(
        app.current_state(),
        TimerState::Work | TimerState::WorkPaused
    ) {
        let session_name_text = if app.current_session_name.is_empty() {
            "Press [N] to name this session".to_string()
        } else {
            format!("📝 {}", app.current_session_name)
        };

        let session_name = Paragraph::new(session_name_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title("Session"),
            );

        frame.render_widget(session_name, chunks[1]);
    }
}
