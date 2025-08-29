//! A terminal-based Pomodoro technique timer.
#![allow(clippy::multiple_crate_versions)]

use anyhow::Result;
use clap::Parser;

mod app;
mod timer;
mod ui;

use app::App;

#[derive(Parser)]
#[command(name = "tomat")]
#[command(about = "A terminal-based Pomodoro technique timer")]
struct Cli {
    #[arg(short, long, default_value_t = 25)]
    work: u32,

    #[arg(short, long, default_value_t = 5)]
    break_time: u32,

    #[arg(short, long, default_value_t = 15)]
    long_break_time: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut app = App::new(
        cli.work,
        cli.break_time,
        cli.long_break_time,
    );
    let result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if app.handle_key(key) {
                    break;
                }
            }
        }

        app.tick();
    }

    Ok(())
}
