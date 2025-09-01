//! Screen implementations for the Pomodoro timer application.
//!
//! This module contains all the different screen types that can be rendered
//! in the application, each implementing the Screen trait.

use ratatui::{layout::Rect, Frame};
use std::any::Any;

use crate::data::TimerData;

pub mod fullscreen;
pub mod normal;

/// Trait that all screens must implement for rendering.
pub trait Screen {
    /// Render the screen content to the given frame.
    fn render(&self, frame: &mut Frame, timer_data: &TimerData, area: Rect);

    /// Return a reference to the screen as Any for type checking.
    fn as_any(&self) -> &dyn Any;
}
