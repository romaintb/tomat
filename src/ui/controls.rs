//! Controls rendering module for the Pomodoro timer application.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, timer::TimerState};

/// Renders the controls area showing available keyboard shortcuts.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let controls_text = if matches!(
        app.current_state(),
        TimerState::Work | TimerState::WorkPaused
    ) {
        "Controls: [Space] Pause/Resume | [R] Reset | [S] Skip | [N] Name Session | [Q] Quit"
    } else {
        "Controls: [Space] Pause/Resume | [R] Reset | [S] Skip | [Q] Quit"
    };

    let controls = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Controls"),
        );

    frame.render_widget(controls, area);
}
