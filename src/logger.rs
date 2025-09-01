use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tomat.log")?;

    let config = ConfigBuilder::new()
        .set_time_format("%Y-%m-%dT%H:%M:%S%.3fZ".to_string())
        .build();

    WriteLogger::init(LevelFilter::Info, config, log_file)?;

    Ok(())
}

pub fn log_session_start(session_type: &str, duration_minutes: u32) {
    info!("Started {session_type} session ({duration_minutes} minutes)");
}

pub fn log_session_complete(session_type: &str, session_name: Option<&str>) {
    session_name.map_or_else(
        || {
            info!("Completed {session_type} session");
        },
        |name| {
            info!("Completed {session_type} session: {name}");
        },
    );
}

pub fn log_session_pause(session_type: &str) {
    info!("Paused {session_type} session");
}

pub fn log_session_resume(session_type: &str) {
    info!("Resumed {session_type} session");
}

pub fn log_session_skip(session_type: &str) {
    info!("Skipped {session_type} session");
}

pub fn log_app_start(work_duration: u32, break_duration: u32, long_break_duration: u32) {
    info!(
        "App started with work: {work_duration}min, short break: {break_duration}min, long break: {long_break_duration}min"
    );
}

pub fn log_app_quit() {
    info!("App quit");
}
