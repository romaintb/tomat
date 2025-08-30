use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

use crate::timer::{PomodoroTimer, TimerState};

pub struct App {
    pub timer: PomodoroTimer,
    pub should_quit: bool,
    pub sessions_completed: u32,
    pub current_session_start: Option<DateTime<Local>>,
    pub current_session_name: String,
    pub naming_mode: bool,
    pub naming_input: String,
    last_tick: Instant,
}

impl App {
    pub fn new(work_duration: u32, break_duration: u32, long_break_duration: u32) -> Self {
        Self {
            timer: PomodoroTimer::new(work_duration, break_duration, long_break_duration),
            should_quit: false,
            sessions_completed: 0,
            current_session_start: None,
            current_session_name: String::new(),
            naming_mode: false,
            naming_input: String::new(),
            last_tick: Instant::now(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.naming_mode {
            self.handle_naming_input(key);
        } else {
            match key.code {
                KeyCode::Char('q') => {
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
                KeyCode::Char('n') => {
                    if self.is_work_session() {
                        self.enter_naming_mode();
                    }
                }
                _ => {}
            }
        }

        self.should_quit
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);

        let session_completed = self.timer.tick(elapsed);
        if session_completed {
            self.sessions_completed += 1;
            if matches!(self.timer.state(), TimerState::Work) {
                self.current_session_start = Some(Local::now());
            }
        }

        self.last_tick = now;
    }

    fn toggle_pause(&mut self) {
        if self.timer.is_paused() {
            self.timer.resume();
        } else {
            self.timer.pause();
        }
    }

    fn reset_timer(&mut self) {
        self.timer.reset();
        self.current_session_start = None;
    }

    fn skip_session(&mut self) {
        self.timer.skip_to_next();
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

    fn handle_naming_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                self.current_session_name = self.naming_input.trim().to_string();
                self.exit_naming_mode();
            }
            KeyCode::Esc => {
                self.exit_naming_mode();
            }
            KeyCode::Backspace => {
                self.naming_input.pop();
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit_naming_mode();
                self.should_quit = true;
            }
            KeyCode::Char(c) => {
                // This handles all other characters (normal text input)
                self.naming_input.push(c);
            }
            _ => {}
        }
    }

    fn enter_naming_mode(&mut self) {
        self.naming_mode = true;
        self.naming_input = self.current_session_name.clone(); // Start with existing name
    }

    fn exit_naming_mode(&mut self) {
        self.naming_mode = false;
        self.naming_input.clear();
    }

    const fn is_work_session(&self) -> bool {
        matches!(
            self.timer.state(),
            TimerState::Work | TimerState::WorkPaused
        )
    }
}
