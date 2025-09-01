use std::time::Duration;

use crate::logger;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Work,
    WorkPaused,
    ShortBreak,
    ShortBreakPaused,
    LongBreak,
    LongBreakPaused,
    NotStarted,
}

pub struct PomodoroTimer {
    work_duration: Duration,
    break_duration: Duration,
    long_break_duration: Duration,
    current_state: TimerState,
    time_remaining: Duration,
    work_sessions_completed: u32,
    total_duration: Duration,
}

impl PomodoroTimer {
    pub fn new(work_minutes: u32, break_minutes: u32, long_break_minutes: u32) -> Self {
        let work_duration = Duration::from_secs(u64::from(work_minutes) * 60);
        let break_duration = Duration::from_secs(u64::from(break_minutes) * 60);
        let long_break_duration = Duration::from_secs(u64::from(long_break_minutes) * 60);

        Self {
            work_duration,
            break_duration,
            long_break_duration,
            current_state: TimerState::NotStarted,
            time_remaining: Duration::ZERO,
            work_sessions_completed: 0,
            total_duration: Duration::ZERO,
        }
    }

    pub fn tick(&mut self, elapsed: Duration) -> bool {
        // Don't tick if paused or not started
        if self.is_paused() || self.current_state == TimerState::NotStarted {
            return false;
        }

        if elapsed >= self.time_remaining {
            self.complete_session();
            true
        } else {
            self.time_remaining -= elapsed;
            false
        }
    }

    fn complete_session(&mut self) {
        match self.current_state {
            TimerState::NotStarted => {
                // Do nothing if timer hasn't started
            }
            TimerState::Work | TimerState::WorkPaused => {
                self.work_sessions_completed += 1;
                if self.work_sessions_completed % 4 == 0 {
                    self.start_long_break();
                } else {
                    self.start_short_break();
                }
            }
            TimerState::ShortBreak
            | TimerState::ShortBreakPaused
            | TimerState::LongBreak
            | TimerState::LongBreakPaused => {
                self.start_work();
            }
        }
    }

    fn start_work(&mut self) {
        self.current_state = TimerState::Work;
        self.time_remaining = self.work_duration;
        self.total_duration = self.work_duration;
        #[allow(clippy::cast_possible_truncation)]
        logger::log_session_start("work", self.work_duration.as_secs() as u32 / 60);
    }

    fn start_short_break(&mut self) {
        self.current_state = TimerState::ShortBreak;
        self.time_remaining = self.break_duration;
        self.total_duration = self.break_duration;
        #[allow(clippy::cast_possible_truncation)]
        logger::log_session_start("short break", self.break_duration.as_secs() as u32 / 60);
    }

    fn start_long_break(&mut self) {
        self.current_state = TimerState::LongBreak;
        self.time_remaining = self.long_break_duration;
        self.total_duration = self.long_break_duration;
        #[allow(clippy::cast_possible_truncation)]
        logger::log_session_start("long break", self.long_break_duration.as_secs() as u32 / 60);
    }

    pub fn start(&mut self) {
        if self.current_state == TimerState::NotStarted {
            self.current_state = TimerState::Work;
            self.time_remaining = self.work_duration;
            self.total_duration = self.work_duration;
        }
    }

    pub fn reset(&mut self) {
        self.current_state = TimerState::NotStarted;
        self.time_remaining = Duration::ZERO;
        self.total_duration = Duration::ZERO;
        self.work_sessions_completed = 0;
    }

    pub fn skip_to_next(&mut self) {
        self.complete_session();
    }

    pub const fn state(&self) -> TimerState {
        self.current_state
    }

    pub const fn time_remaining(&self) -> Duration {
        self.time_remaining
    }

    pub fn pause(&mut self) {
        self.current_state = match self.current_state {
            TimerState::Work => TimerState::WorkPaused,
            TimerState::ShortBreak => TimerState::ShortBreakPaused,
            TimerState::LongBreak => TimerState::LongBreakPaused,
            paused => paused, // Already paused states remain unchanged
        };
    }

    pub fn resume(&mut self) {
        self.current_state = match self.current_state {
            TimerState::WorkPaused => TimerState::Work,
            TimerState::ShortBreakPaused => TimerState::ShortBreak,
            TimerState::LongBreakPaused => TimerState::LongBreak,
            active => active, // Already active states remain unchanged
        };
    }

    pub const fn is_paused(&self) -> bool {
        matches!(
            self.current_state,
            TimerState::WorkPaused | TimerState::ShortBreakPaused | TimerState::LongBreakPaused
        )
    }

    pub const fn work_duration(&self) -> Duration {
        self.work_duration
    }

    pub const fn break_duration(&self) -> Duration {
        self.break_duration
    }

    pub const fn long_break_duration(&self) -> Duration {
        self.long_break_duration
    }
}
