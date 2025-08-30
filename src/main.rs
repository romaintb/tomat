//! A terminal-based Pomodoro technique timer.
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;
use std::io;

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
    short_break: u32,

    #[arg(short, long, default_value_t = 15)]
    long_break_time: u32,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut app = App::new(cli.work, cli.short_break, cli.long_break_time);
    let result = run_app(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key) => {
                    // Only handle key press events, ignore key release
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        if app.handle_key(key) {
                            break;
                        }
                    }
                }
                _ => {} // Ignore other events
            }
        }

        app.tick();
    }

    Ok(())
}
