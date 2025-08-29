use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Work,
    ShortBreak,
    LongBreak,
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
            current_state: TimerState::Work,
            time_remaining: work_duration,
            work_sessions_completed: 0,
            total_duration: work_duration,
        }
    }

    pub fn tick(&mut self, elapsed: Duration) -> bool {
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
            TimerState::Work => {
                self.work_sessions_completed += 1;
                if self.work_sessions_completed % 4 == 0 {
                    self.start_long_break();
                } else {
                    self.start_short_break();
                }
            }
            TimerState::ShortBreak | TimerState::LongBreak => {
                self.start_work();
            }
        }
    }

    fn start_work(&mut self) {
        self.current_state = TimerState::Work;
        self.time_remaining = self.work_duration;
        self.total_duration = self.work_duration;
    }

    fn start_short_break(&mut self) {
        self.current_state = TimerState::ShortBreak;
        self.time_remaining = self.break_duration;
        self.total_duration = self.break_duration;
    }

    fn start_long_break(&mut self) {
        self.current_state = TimerState::LongBreak;
        self.time_remaining = self.long_break_duration;
        self.total_duration = self.long_break_duration;
    }

    pub fn reset(&mut self) {
        self.current_state = TimerState::Work;
        self.time_remaining = self.work_duration;
        self.total_duration = self.work_duration;
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

    pub fn progress(&self) -> f64 {
        let elapsed = self.total_duration - self.time_remaining;
        elapsed.as_secs_f64() / self.total_duration.as_secs_f64()
    }


}
