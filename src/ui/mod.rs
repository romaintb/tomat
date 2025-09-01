//! UI module for the Pomodoro timer application.
//!
//! This module contains all the UI rendering logic organized into dedicated submodules
//! for better maintainability and separation of concerns.

use ratatui::prelude::*;

use crate::app::App;
use crate::data::TimerData;

pub mod modal;
pub mod screens;

pub use screens::Screen;

/// Main render function that orchestrates the rendering of all UI components.
pub fn render(frame: &mut Frame, app: &App) {
    // Create timer data from app state
    let timer_data = TimerData {
        remaining_time: app.time_remaining(),
        total_time: match app.current_state() {
            crate::timer::TimerState::Work | crate::timer::TimerState::WorkPaused => {
                app.timer.work_duration()
            }
            crate::timer::TimerState::ShortBreak | crate::timer::TimerState::ShortBreakPaused => {
                app.timer.break_duration()
            }
            crate::timer::TimerState::LongBreak | crate::timer::TimerState::LongBreakPaused => {
                app.timer.long_break_duration()
            }
            crate::timer::TimerState::NotStarted => app.timer.work_duration(),
        },
        is_running: app.current_state() != crate::timer::TimerState::NotStarted
            && app.current_state() != crate::timer::TimerState::WorkPaused
            && app.current_state() != crate::timer::TimerState::ShortBreakPaused
            && app.current_state() != crate::timer::TimerState::LongBreakPaused,
        is_paused: matches!(
            app.current_state(),
            crate::timer::TimerState::WorkPaused
                | crate::timer::TimerState::ShortBreakPaused
                | crate::timer::TimerState::LongBreakPaused
        ),
        sessions_completed: app.sessions_completed,
        session_start_time: app
            .current_session_start
            .map(|dt| dt.format("%H:%M:%S").to_string()),
        session_name: app.current_session_name.clone(),
        naming_mode: app.naming_mode,
    };

    // Render the current screen
    app.current_screen.render(frame, &timer_data, frame.area());

    // Render naming modal on top if in naming mode
    if app.naming_mode {
        modal::render(frame, app);
    }
}
