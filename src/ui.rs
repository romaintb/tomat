use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::{app::App, timer::TimerState};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(6), // Timer area (increased from 5 to 6 for two 3-line boxes)
            Constraint::Min(0),    // Stats (flexible)
            Constraint::Length(3), // Controls
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_timer(frame, chunks[1], app);
    render_stats(frame, chunks[2], app);
    render_controls(frame, chunks[3], app);

    // Render naming modal on top if in naming mode
    if app.naming_mode {
        render_naming_modal(frame, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
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

fn render_timer(frame: &mut Frame, area: Rect, app: &App) {
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

fn render_stats(frame: &mut Frame, area: Rect, app: &App) {
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

fn render_controls(frame: &mut Frame, area: Rect, app: &App) {
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

fn render_naming_modal(frame: &mut Frame, app: &App) {
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
        .border_type(ratatui::widgets::BorderType::Rounded); // Rounded corners
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
        .alignment(Alignment::Center);

    frame.render_widget(title, chunks[0]);

    // Input field with proper height
    let input_display = if app.naming_input.is_empty() {
        " |".to_string() // Show cursor when empty with padding
    } else {
        format!(" {}|", app.naming_input) // Show content + cursor with padding
    };

    let input = Paragraph::new(input_display)
        .style(Style::default().fg(Color::Black).bg(Color::White))
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title("Session Name"),
        );

    frame.render_widget(input, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Enter to save • Esc to cancel")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);

    frame.render_widget(instructions, chunks[2]);
}
