# Bevy Hello World

A simple Bevy application demonstrating a spinning cube with draggable camera view.

## Overview

This is a basic Bevy game engine example that renders a blue rectangle that continuously spins. You can drag with your mouse to rotate your view of the spinning cube. The cube always spins at the same rate from its own perspective, while you can orbit around it to see it from different angles. It serves as a starting point for learning Bevy, a data-driven game engine built in Rust.

## Features

- Bevy 0.14
- 2D rendering with a spinning rectangle
- Simple ECS architecture with components and systems
- Constant spin rate from the cube's perspective
- **Drag to orbit**: Click and drag to rotate your view around the spinning cube
- The cube always spins the same way, but you can change your viewing angle

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Getting Started

### Clone the repository

```bash
git clone https://github.com/eellams/bevy-hello-world.git
cd bevy-hello-world
```

### Run the application

```bash
cargo run
```

This will compile and run the application, opening a window with a spinning blue rectangle. Click and drag anywhere in the window to rotate your view around the cube. The cube always spins at a constant rate from its own perspective.

### Build for release

```bash
cargo build --release
```

The optimized binary will be available in `target/release/bevy-hello-world`.

## Project Structure

```
bevy-hello-world/
├── Cargo.toml    # Project configuration and dependencies
├── README.md     # This file
└── src/
    └── main.rs    # Main application code
```

## Code Explanation

### main.rs

The application uses Bevy's ECS (Entity Component System) architecture:

- **Components**: Data attached to entities. `RotatingCube` is a marker component.
- **Systems**: Logic that runs on entities matching specific queries.
  - `setup`: Called once at startup, creates the camera and rectangle.
  - `rotate_cube`: Called every frame, rotates the rectangle based on elapsed time.

### Key Bevy Concepts Used

- `App`: The main application container
- `DefaultPlugins`: Collection of essential Bevy plugins
- `Commands`: Used to spawn entities
- `ResMut`: Mutable resource access (for assets like meshes and materials)
- `Query`: Used to find and modify entities with specific components
- `Res<Time>`: Access to time information for animations
- `ButtonInput<MouseButton>`: Mouse button input handling
- `Transform`: Position, rotation, and scale of entities
- Multiple entity systems running in sequence with `.chain()`

## Running Tests

```bash
cargo test
```

## License

This project is open source. Feel free to use it as a starting point for your own Bevy projects.

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
