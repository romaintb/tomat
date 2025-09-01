# Screen System

The tomat application now supports multiple screen modes for different viewing experiences.

## Available Screens

### Normal Screen
The default screen that shows:
- Header with application name
- Timer display with borders
- Controls information
- Help text

### Fullscreen Screen
A minimal screen that shows:
- Centered timer display (large, easy to read)
- Help text at the bottom

## Screen Switching

Press the **F** key to toggle between normal and fullscreen modes.

## Architecture

The screen system is built on a trait-based architecture:

- **`Screen` trait**: Defines the interface for all screens
- **`TimerData`**: Shared data structure containing timer state
- **Screen implementations**: Render-only objects that consume data

### Key Benefits

1. **Separation of concerns**: Screens only handle rendering, not data management
2. **Extensibility**: Easy to add new screen types in the future
3. **Shared state**: Timer state is consistent across all screens
4. **Clean switching**: Instant switching between screens with no transitions

## Future Screens

Planned screens for future releases:
- Settings/Configuration screen
- Statistics/History screen  
- Help/Documentation screen

## Implementation Details

- Screens are stored as `Box<dyn Screen>` in the app
- Screen switching uses downcasting to determine current screen type
- All screens receive the same `TimerData` structure
- Screen state is not persisted between switches
