use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::{app::App, timer::TimerState};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_timer(frame, chunks[1], app);
    render_stats(frame, chunks[2], app);
    render_controls(frame, chunks[3], app);
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
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}

fn render_timer(frame: &mut Frame, area: Rect, app: &App) {
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
                .title("Time Remaining"),
        );

    frame.render_widget(timer_display, area);
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
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(progress)
        .label(format!("{:.1}%", progress * 100.0));

    frame.render_widget(progress_gauge, chunks[0]);

    let sessions_text = format!("Completed Sessions: {}", app.sessions_completed);
    let sessions_display = Paragraph::new(sessions_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"))
        .alignment(Alignment::Center);

    frame.render_widget(sessions_display, chunks[1]);

    if let Some(start_time) = app.current_session_start {
        let session_info = format!("Session started: {}", start_time.format("%H:%M:%S"));
        let session_display = Paragraph::new(session_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Current Session"),
            )
            .alignment(Alignment::Center);

        frame.render_widget(session_display, chunks[2]);
    }
}

fn render_controls(frame: &mut Frame, area: Rect, _app: &App) {
    let controls_text = "Controls: [Space] Pause/Resume | [R] Reset | [S] Skip | [Q/Esc] Quit";
    let controls = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Controls"));

    frame.render_widget(controls, area);
}
