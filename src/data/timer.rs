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
        // Early return 0.0 for NotStarted/paused states
        if !self.is_running || self.total_time.as_secs() == 0 {
            return 0.0;
        }

        // Use as_secs_f64 to avoid clippy warnings
        let remaining = self.remaining_time.as_secs_f64();
        let total = self.total_time.as_secs_f64();

        // Compute percentage as ((total - remaining) / total) * 100.0
        let percentage = ((total - remaining) / total) * 100.0;

        // Clamp the result between 0.0 and 100.0
        percentage.clamp(0.0, 100.0)
    }
}
