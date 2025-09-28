# MIDI Mapper Linux

Linux application for mapping MIDI controller inputs to system actions.

## Features

- Auto-detects Korg nanoKONTROL2 MIDI controller
- **Media Controls**: Play, Pause, Play/Pause toggle, Next track, Previous track
- **Audio Controls**: System volume, Mute toggle, Cycle audio outputs
- Persistent configuration with automatic save
- Background mode - keeps mappings active when window is closed
- Autostart on login support
- GTK4 interface with KDE Plasma integration

## Installation

### Prerequisites

```bash
# Fedora/Nobara
sudo dnf install gtk4-devel alsa-lib-devel

# Ubuntu/Debian
sudo apt install libgtk-4-dev libasound2-dev
```

### Build from Source

```bash
git clone https://github.com/wessmeister/midi-mapper-linux.git
cd midi-mapper-linux
cargo build --release
```

### Local Installation

```bash
./install-local.sh  # Installs to ~/.local/bin and creates desktop entry
```

## Usage

1. Connect your MIDI controller
2. Move any control to detect it
3. Select action from dropdown
4. Mappings save automatically

**Background Mode**: Enable checkbox to keep mappings active when window is closed.

## Development

```bash
cargo run              # Development build
cargo build --release  # Release build
./install-local.sh     # Update local installation
```

## Architecture

```
src/
├── main.rs        # Application entry point
├── action/        # Action execution logic
│   ├── media.rs   # Media player controls (MPRIS/qdbus)
│   └── executor.rs # System audio controls (pactl)
├── config/        # Configuration management
├── midi/          # MIDI device handling
│   ├── device.rs  # ALSA device connection
│   └── processor.rs # Event processing thread
└── ui/            # GTK4 user interface
    ├── builder.rs # Main window construction
    ├── components.rs # Reusable UI widgets
    └── state.rs   # Application state management
```

## Configuration

Settings stored in `~/.config/midi-mapper/config.json`

## License

MIT