//! Clock screen implementation for the Pomodoro timer application.
//!
//! This screen displays a simple digital clock showing the current time.

use chrono::Local;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::any::Any;

use super::Screen;
use crate::data::TimerData;

/// Clock screen that displays the current time.
pub struct ClockScreen;

impl Screen for ClockScreen {
    fn render(&self, frame: &mut Frame, _timer_data: &TimerData, area: Rect) {
        let now = Local::now();
        let time_str = now.format("%H:%M").to_string();

        let clock_display = Paragraph::new(time_str)
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Current Time"),
            );

        frame.render_widget(clock_display, area);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
