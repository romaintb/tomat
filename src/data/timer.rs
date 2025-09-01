use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TimerData {
    pub remaining_time: Duration,
    pub total_time: Duration,
    pub is_running: bool,
    pub is_paused: bool,
    pub sessions_completed: u32,
    pub session_start_time: Option<String>,
    pub session_name: String,
    pub naming_mode: bool,
}

impl TimerData {
    pub fn new(total_time: Duration) -> Self {
        Self {
            remaining_time: total_time,
            total_time,
            is_running: false,
            is_paused: false,
            sessions_completed: 0,
            session_start_time: None,
            session_name: String::new(),
            naming_mode: false,
        }
    }

    pub fn format_time(&self) -> String {
        let total_secs = self.remaining_time.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.total_time.as_secs() == 0 {
            0.0
        } else {
            let remaining = self.remaining_time.as_secs() as f64;
            let total = self.total_time.as_secs() as f64;
            ((total - remaining) / total) * 100.0
        }
    }
}
