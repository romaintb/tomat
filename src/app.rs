
use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

use crate::timer::{PomodoroTimer, TimerState};

pub struct App {
    pub timer: PomodoroTimer,
    pub should_quit: bool,
    pub paused: bool,
    pub sessions_completed: u32,
    pub current_session_start: Option<DateTime<Local>>,
    last_tick: Instant,
}

impl App {
    pub fn new(work_duration: u32, break_duration: u32, long_break_duration: u32) -> Self {
        Self {
            timer: PomodoroTimer::new(work_duration, break_duration, long_break_duration),
            should_quit: false,
            paused: false,
            sessions_completed: 0,
            current_session_start: None,
            last_tick: Instant::now(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                self.toggle_pause();
            }
            KeyCode::Char('r') => {
                self.reset_timer();
            }
            KeyCode::Char('s') => {
                self.skip_session();
            }
            _ => {}
        }

        self.should_quit
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);

        if !self.paused {
            let session_completed = self.timer.tick(elapsed);
            if session_completed {
                self.sessions_completed += 1;
                if matches!(self.timer.state(), TimerState::Work) {
                    self.current_session_start = Some(Local::now());
                }
            }
        }

        self.last_tick = now;
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn reset_timer(&mut self) {
        self.timer.reset();
        self.paused = false;
        self.current_session_start = None;
    }

    fn skip_session(&mut self) {
        self.timer.skip_to_next();
        self.paused = false;
    }

    pub const fn time_remaining(&self) -> Duration {
        self.timer.time_remaining()
    }

    pub const fn current_state(&self) -> TimerState {
        self.timer.state()
    }

    pub fn progress(&self) -> f64 {
        self.timer.progress()
    }
}
