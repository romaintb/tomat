//! Header rendering module for the Pomodoro timer application.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, timer::TimerState};

/// Renders the application header with title and current state information.
pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let title = match app.current_state() {
        TimerState::Work => "🍅 Pomodoro - Work Session",
        TimerState::WorkPaused => "⏸️ Pomodoro - Work Session (Paused)",
        TimerState::ShortBreak => "☕ Pomodoro - Short Break",
        TimerState::ShortBreakPaused => "⏸️ Pomodoro - Short Break (Paused)",
        TimerState::LongBreak => "🌴 Pomodoro - Long Break",
        TimerState::LongBreakPaused => "⏸️ Pomodoro - Long Break (Paused)",
    };

    let color = match app.current_state() {
        TimerState::Work | TimerState::WorkPaused => Color::Red,
        TimerState::ShortBreak | TimerState::ShortBreakPaused => Color::Yellow,
        TimerState::LongBreak | TimerState::LongBreakPaused => Color::Green,
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(color).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );

    frame.render_widget(header, area);
}
