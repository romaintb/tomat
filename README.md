# 🍅 Tomat

A terminal-based Pomodoro technique timer built with Rust and ratatui.

## Features

- ⏱️ Customizable work, short break, and long break durations
- 🎨 Beautiful terminal user interface with progress indicators
- ⌨️ Intuitive keyboard controls
- 📊 Session tracking and statistics
- 🎵 Visual notifications for session transitions
- 📱 Responsive design that works in any terminal size

## Installation

### From Source

```bash
git clone https://github.com/romaintb/tomat.git
cd tomat
cargo install --path .
```

### From Crates.io

```bash
cargo install tomat
```

## Usage

### Basic Usage

```bash
# Start with default settings (25min work, 5min short break, 15min long break)
tomat

# Customize durations (in minutes)
tomat -w 30 -b 10 -l 20
```

### Controls

- **Space/Enter**: Pause/Resume timer
- **R**: Reset current timer
- **S**: Skip to next session
- **Q/Esc**: Quit application

## Command Line Options

```
Usage: tomat [OPTIONS]

Options:
  -w, --work-duration <WORK_DURATION>              Work session duration in minutes [default: 25]
  -b, --break-duration <BREAK_DURATION>            Short break duration in minutes [default: 5]
  -l, --long-break-duration <LONG_BREAK_DURATION>  Long break duration in minutes [default: 15]
  -h, --help                                       Print help
```

## The Pomodoro Technique

The Pomodoro Technique is a time management method that uses a timer to break work into intervals, traditionally 25 minutes in length, separated by short breaks. After every 4 work sessions, take a longer break.

1. 🍅 Work for 25 minutes
2. ☕ Take a 5-minute break
3. 🍅 Repeat 3 more times
4. 🌴 Take a longer 15-minute break
5. Repeat the cycle

## Development

### Prerequisites

- Rust 1.70.0 or higher
- Cargo

### Building

```bash
git clone https://github.com/romaintb/tomat.git
cd tomat
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run all checks
cargo fmt --check && cargo clippy -- -D warnings
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) for the terminal UI
- Inspired by the Pomodoro Technique by Francesco Cirillo