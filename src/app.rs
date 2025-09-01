use chrono::{DateTime, Local};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::time::{Duration, Instant};

use crate::logger;
use crate::timer::{PomodoroTimer, TimerState};
use crate::ui::screens::{
    clock::ClockScreen, fullscreen::FullscreenScreen, help::HelpScreen, normal::NormalScreen, Screen,
};

pub struct App {
    pub timer: PomodoroTimer,
    pub should_quit: bool,
    pub sessions_completed: u32,
    pub current_session_start: Option<DateTime<Local>>,
    pub current_session_name: String,
    pub naming_mode: bool,
    pub naming_input: String,
    pub current_screen: Box<dyn Screen>,
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
            current_screen: Box::new(NormalScreen),
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
                KeyCode::Char('f') => {
                    self.toggle_screen();
                }
                KeyCode::Char('h') => {
                    self.show_help();
                }
                KeyCode::Char('c') => {
                    self.show_clock();
                }
                KeyCode::Char(' ') | KeyCode::Enter => {
                    if self.timer.state() == TimerState::NotStarted {
                        self.start_timer();
                    } else {
                        self.toggle_pause();
                    }
                }
                KeyCode::Char('r') => {
                    self.reset_timer();
                }
                KeyCode::Char('s') => {
                    self.skip_session();
                }
                KeyCode::Char('n') => {
                    // Allow naming sessions in any state except when already in naming mode
                    if !self.naming_mode {
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
            let session_type = match self.timer.state() {
                TimerState::Work | TimerState::WorkPaused => "work",
                TimerState::ShortBreak | TimerState::ShortBreakPaused => "short break",
                TimerState::LongBreak | TimerState::LongBreakPaused => "long break",
                TimerState::NotStarted => "unknown",
            };
            let session_name = if self.current_session_name.is_empty() {
                None
            } else {
                Some(self.current_session_name.as_str())
            };
            logger::log_session_complete(session_type, session_name);

            if matches!(self.timer.state(), TimerState::Work) {
                self.current_session_start = Some(Local::now());
            }
        }

        self.last_tick = now;
    }

    fn toggle_pause(&mut self) {
        let session_type = match self.timer.state() {
            TimerState::Work | TimerState::WorkPaused => "work",
            TimerState::ShortBreak | TimerState::ShortBreakPaused => "short break",
            TimerState::LongBreak | TimerState::LongBreakPaused => "long break",
            TimerState::NotStarted => "unknown",
        };

        if self.timer.is_paused() {
            self.timer.resume();
            logger::log_session_resume(session_type);
        } else {
            self.timer.pause();
            logger::log_session_pause(session_type);
        }
    }

    fn reset_timer(&mut self) {
        self.timer.reset();
        self.current_session_start = None;
    }

    fn start_timer(&mut self) {
        self.timer.start();
        self.current_session_start = Some(Local::now());
        #[allow(clippy::cast_possible_truncation)]
        logger::log_session_start("work", self.timer.work_duration().as_secs() as u32 / 60);
    }

    fn skip_session(&mut self) {
        let session_type = match self.timer.state() {
            TimerState::Work | TimerState::WorkPaused => "work",
            TimerState::ShortBreak | TimerState::ShortBreakPaused => "short break",
            TimerState::LongBreak | TimerState::LongBreakPaused => "long break",
            TimerState::NotStarted => "unknown",
        };
        logger::log_session_skip(session_type);
        self.timer.skip_to_next();
    }

    pub const fn time_remaining(&self) -> Duration {
        self.timer.time_remaining()
    }

    pub const fn current_state(&self) -> TimerState {
        self.timer.state()
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

    fn toggle_screen(&mut self) {
        if self
            .current_screen
            .as_any()
            .downcast_ref::<NormalScreen>()
            .is_some()
        {
            self.current_screen = Box::new(FullscreenScreen);
        } else if self
            .current_screen
            .as_any()
            .downcast_ref::<FullscreenScreen>()
            .is_some()
        {
            self.current_screen = Box::new(HelpScreen);
        } else if self
            .current_screen
            .as_any()
            .downcast_ref::<HelpScreen>()
            .is_some()
        {
            self.current_screen = Box::new(ClockScreen);
        } else {
            self.current_screen = Box::new(NormalScreen);
        }
    }

    fn show_help(&mut self) {
        self.current_screen = Box::new(HelpScreen);
    }

    fn show_clock(&mut self) {
        self.current_screen = Box::new(ClockScreen);
    }
}
