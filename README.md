# Night Sky TUI

A beautiful terminal-based night sky visualization built with Ratatui.

## Features

- **Twinkling Stars**: Stars with different brightness levels that twinkle at various speeds
- **Shooting Stars**: Random shooting stars with glowing trails that streak across the sky
- **Satellites**: Blinking satellites that orbit across the screen
- **Smooth Animations**: 60 FPS rendering for fluid motion
- **Simple Controls**: Easy keyboard controls

## Installation

Install the `nk` command globally:

```bash
cargo install --path .
```

After installation, you can run it from anywhere:

```bash
nk
```

## Development

Run directly without installing:

```bash
cargo run
```

## Controls

- `q` or `Esc` - Quit the application

## Visual Elements

- Stars: Various sizes (·, •, ✦) with twinkling effects
- Shooting Stars: ☄ with trailing particles
- Satellites: ◆ with blinking lights

## Requirements

- Rust 1.70 or higher
- A terminal with Unicode support for best visuals
- 256-color terminal support recommended

## Dependencies

- ratatui - Terminal UI framework
- crossterm - Cross-platform terminal manipulation
- rand - Random number generation for celestial object placement

