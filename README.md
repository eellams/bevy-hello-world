# Bevy Hello World

A simple Bevy application demonstrating a spinning cube with 3D draggable camera view.

## Overview

This is a basic Bevy game engine example that renders a blue rectangle that continuously spins on all three axes (X, Y, Z). You can drag with your mouse to rotate your view of the spinning cube in 3D space. The cube always spins at the same rate from its own perspective, while you can orbit around it to see it from any angle. It serves as a starting point for learning Bevy, a data-driven game engine built in Rust.

## Features

- Bevy 0.14 with PBR rendering
- 3D rendering with a proper cube mesh
- Simple ECS architecture with components and systems
- Cube spins on all three axes (X, Y, Z) at different rates
- **Drag to orbit**: Click and drag to rotate your 3D view around the cube
- Full 2-axis camera rotation: horizontal (yaw) and vertical (pitch)
- The cube always spins the same way from its own perspective
- Proper lighting and materials for 3D visualization

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

This will compile and run the application, opening a window with a spinning blue rectangle. Click and drag anywhere in the window to rotate your 3D view around the cube on all axes. The cube continuously spins on X, Y, and Z axes from its own perspective.

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
    ├── main.rs    # Application entry point
    └── lib.rs     # Core application logic and tests
```

## Code Explanation

### main.rs & lib.rs

The application uses Bevy's ECS (Entity Component System) architecture:

- **Components**: Data attached to entities.
  - `SpinningCube`: Marker component for the spinning rectangle
  - `OrbitCamera`: Contains camera state for orbiting (distance, pitch, yaw, drag state)

- **Systems**: Logic that runs on entities matching specific queries.
  - `setup`: Called once at startup, creates the cube and camera.
  - `spin_cube`: Called every frame, rotates the cube on all three axes.
  - `handle_camera_orbit`: Called every frame, handles mouse drag to rotate the camera view.

### Key Bevy Concepts Used

- `App`: The main application container
- `DefaultPlugins`: Collection of essential Bevy plugins
- `Commands`: Used to spawn entities
- `ResMut`: Mutable resource access (for assets like meshes and materials)
- `Query`: Used to find and modify entities with specific components
- `Res<Time>`: Access to time information for animations
- `ButtonInput<MouseButton>`: Mouse button input handling
- `Transform`: Position, rotation, and scale of entities
- `Camera3dBundle`: 3D camera with perspective projection
- Multiple entity systems running in sequence with `.chain()`

## Running Tests

```bash
cargo test
```

## Controls

- **Left Mouse Button + Drag**: Rotate your view around the cube
  - Horizontal movement (left/right): Yaw rotation (around Y axis)
  - Vertical movement (up/down): Pitch rotation (around X axis)
- The cube will continuously spin on all three axes regardless of camera view

## License

This project is open source. Feel free to use it as a starting point for your own Bevy projects.

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
