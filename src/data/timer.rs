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
    pub fn format_time(&self) -> String {
        let total_secs = self.remaining_time.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        format!("{minutes:02}:{seconds:02}")
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.total_time.as_secs() == 0 {
            0.0
        } else {
            #[allow(clippy::cast_precision_loss)]
            let remaining = self.remaining_time.as_secs() as f64;
            #[allow(clippy::cast_precision_loss)]
            let total = self.total_time.as_secs() as f64;
            ((total - remaining) / total) * 100.0
        }
    }
}
