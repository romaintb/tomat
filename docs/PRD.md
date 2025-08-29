# 🍅 Tomat - Product Requirements Document

## Overview

Tomat is a personal Rust project implementing a terminal-based Pomodoro technique timer. It prioritizes simplicity, reliability, and aesthetic appeal in a lightweight TUI application following MVC architectural patterns.

## Project Philosophy

- **Personal Tool First**: Built primarily for personal daily use
- **Simplicity Over Features**: Focused on core Pomodoro functionality without bloat
- **Quality Code**: Well-structured, maintainable Rust following best practices
- **MVC Architecture**: Clear separation between data, presentation, and business logic
- **Terminal Native**: Embraces terminal environment rather than fighting it

## Target Users

**Primary**: The developer (personal use)
**Secondary**: Terminal users who appreciate the Pomodoro technique and want a clean, fast timer

## Core Requirements

### Functional Requirements

#### Timer Functionality
- **Pomodoro Cycles**: Standard 25/5/15 minute work/short break/long break pattern
- **Customizable Durations**: CLI arguments for custom work/break periods
- **Session Management**: Automatic progression through work → break cycles
- **Manual Controls**: Pause/resume, reset, skip session
- **Progress Tracking**: Visual progress indicators for current session

#### User Interface
- **Clean TUI**: Beautiful, responsive terminal interface using ratatui
- **Real-time Updates**: Smooth timer countdown and progress visualization
- **State Indication**: Clear visual distinction between work/break sessions
- **Responsive Design**: Adapts to different terminal sizes
- **Intuitive Controls**: Keyboard shortcuts following common TUI conventions

#### System Integration
- **Fast Startup**: Lightning-fast application launch and response
- **Cross-Platform**: Primary Linux support with macOS/Windows/BSD compatibility
- **Terminal Compatibility**: Works across different terminal emulators
- **Resource Efficient**: Minimal CPU/memory footprint

### Technical Requirements

#### Architecture (MVC Pattern)
```
src/
├── models/          # Data structures and state management
│   ├── timer.rs     # Timer state, durations, progress
│   └── session.rs   # Session tracking, statistics
├── views/           # UI rendering and presentation
│   ├── ui.rs        # Main UI layout and components
│   └── themes.rs    # Color schemes and styling
└── controllers/     # Business logic and event handling
    ├── app.rs       # Application controller, event routing
    └── input.rs     # Keyboard input handling
```

#### Performance Standards
- **Startup Time**: < 100ms on modern hardware
- **Memory Usage**: < 10MB resident memory
- **CPU Usage**: Negligible when idle, minimal during updates
- **Responsiveness**: < 50ms input-to-display latency

#### Platform Support
1. **Linux** (Primary) - Full feature support
2. **macOS** (Secondary) - Full compatibility
3. **Windows** (Nice-to-have) - Basic functionality
4. **BSD** (Personal interest) - OpenBSD, FreeBSD, NetBSD support

## Feature Roadmap

### Phase 1: Core MVP (Current)
- ✅ Basic Pomodoro timer functionality
- ✅ Terminal UI with ratatui
- ✅ CLI argument parsing
- ✅ Manual timer controls
- ✅ Progress visualization

### Phase 2: Enhanced Experience
- **System Notifications**: Desktop notifications when sessions end
- **Session Statistics**: Basic completed session tracking
- **Theme Support**: Multiple color schemes (starting with Catppuccin)
- **Configuration File**: Optional config file for persistent settings

### Phase 3: Polish & Refinement
- **Advanced Statistics**: Detailed session history and patterns
- **Multiple Themes**: Expanded theme collection
- **Sound Notifications**: Optional audio alerts
- **Improved Accessibility**: Better screen reader support

## Non-Goals

- **GUI Versions**: No GTK, Qt, or web interfaces planned
- **Third-party Integrations**: No task managers, time tracking services, etc.
- **Complex Features**: No advanced analytics, team features, or cloud sync
- **Mobile Support**: Terminal-focused, no mobile apps
- **Plugin System**: Keeping architecture simple and focused

## Technical Decisions

### Dependencies
- **ratatui**: Modern terminal UI framework
- **crossterm**: Cross-platform terminal handling
- **tokio**: Async runtime for responsive UI
- **clap**: CLI argument parsing
- **chrono**: Time handling and formatting
- **serde**: Serialization for future config files

### Code Quality Standards
- **Safety**: `#![forbid(unsafe_code)]`
- **Documentation**: Warn on missing docs for public APIs
- **Linting**: Strict clippy rules (pedantic + nursery)
- **Testing**: Unit tests for core timer logic
- **Formatting**: Standard rustfmt configuration

### MVC Implementation
- **Models**: Pure data structures, no UI dependencies
- **Views**: Rendering logic only, no business logic
- **Controllers**: Event handling and state coordination
- **Clear Boundaries**: No circular dependencies between layers

## Success Metrics

### Personal Use Goals
- **Daily Usage**: Tool becomes part of regular workflow
- **Reliability**: Runs without crashes or issues
- **Performance**: Feels instant and responsive
- **Maintainability**: Code remains clean and extensible

### Code Quality Goals
- **Test Coverage**: >80% for core timer logic
- **Documentation**: All public APIs documented
- **Build Time**: <30 seconds clean build
- **Binary Size**: <5MB release binary

## Architecture Notes for Future Developers

### MVC Separation
```rust
// Models - Pure data, no dependencies on UI or controllers
pub struct PomodoroTimer {
    // Timer state and logic
}

// Views - Rendering only, takes model data as input
pub fn render(frame: &mut Frame, model: &AppModel) {
    // UI rendering logic
}

// Controllers - Coordinates between models and views
pub struct AppController {
    // Event handling and state management
}
```

### Extension Points
- **Theme System**: Easy addition of new color schemes
- **Notification Backends**: Pluggable notification implementations
- **Statistics Storage**: Future data persistence layer
- **Configuration**: Expandable settings system

### Constraints
- **Single Binary**: All functionality in one executable
- **Minimal Dependencies**: Prefer standard library when possible
- **Fast Compilation**: Avoid heavy macro or proc-macro dependencies
- **Cross-platform**: Use portable abstractions

This PRD serves as the north star for Tomat development, ensuring the project remains focused on its core mission: a simple, beautiful, reliable Pomodoro timer for terminal users.