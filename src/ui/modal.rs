//! Modal rendering module for the Pomodoro timer application.

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

/// Renders the naming modal overlay for session naming functionality.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create a larger centered modal area
    let modal_width = (area.width * 7 / 10).min(70);
    let modal_height = 9; // Increased height
    let modal_x = (area.width - modal_width) / 2;
    let modal_y = (area.height - modal_height) / 2;

    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    // Clear the background
    let clear_block = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .border_type(BorderType::Rounded); // Rounded corners
    frame.render_widget(clear_block, modal_area);

    // Create the modal layout with proper spacing
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1) // Add margin inside the modal
        .constraints([
            Constraint::Length(2), // Title - more space
            Constraint::Length(3), // Input - enough space for content + borders
            Constraint::Length(1), // Instructions
        ])
        .split(modal_area);

    // Title
    let title = Paragraph::new("📝 Name this work session:")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(title, chunks[0]);

    // Input field with proper height
    let input_display = if app.naming_input.is_empty() {
        " |".to_string() // Show cursor when empty with padding
    } else {
        format!(" {}|", app.naming_input) // Show content + cursor with padding
    };

    let input = Paragraph::new(input_display)
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .alignment(ratatui::layout::Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(BorderType::Rounded)
                .title("Session Name"),
        );

    frame.render_widget(input, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Enter to save • Esc to cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(instructions, chunks[2]);
}
