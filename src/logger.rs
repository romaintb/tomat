use simplelog::*;
use std::fs::OpenOptions;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tomat.log")?;

    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .build();

    WriteLogger::init(LevelFilter::Info, config, log_file)?;

    Ok(())
}

pub fn log_session_start(session_type: &str, duration_minutes: u32) {
    info!("Started {} session ({} minutes)", session_type, duration_minutes);
}

pub fn log_session_complete(session_type: &str, session_name: Option<&str>) {
    if let Some(name) = session_name {
        info!("Completed {} session: {}", session_type, name);
    } else {
        info!("Completed {} session", session_type);
    }
}

pub fn log_session_pause(session_type: &str) {
    info!("Paused {} session", session_type);
}

pub fn log_session_resume(session_type: &str) {
    info!("Resumed {} session", session_type);
}

pub fn log_session_skip(session_type: &str) {
    info!("Skipped {} session", session_type);
}

pub fn log_app_start(work_duration: u32, break_duration: u32, long_break_duration: u32) {
    info!("App started with work: {}min, short break: {}min, long break: {}min",
          work_duration, break_duration, long_break_duration);
}

pub fn log_app_quit() {
    info!("App quit");
}