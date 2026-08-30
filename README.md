# Bevy Hello World

A simple Bevy application demonstrating a cube that always spins end-over-end with PBR materials and dynamic lighting.

## Overview

This is a basic Bevy game engine example that renders a metallic blue cube that continuously spins end-over-end around its local Z-axis. You can click and drag on the cube to rotate it, which changes the orientation of the spin in world space. The cube always spins the same way (end-over-end) from its own perspective, but you can rotate the cube so that this spin moves around in different directions. A point light orbits the cube to show off the PBR material's lighting effects. It serves as a starting point for learning Bevy, a data-driven game engine built in Rust.

## Features

- Bevy 0.14 with PBR rendering
- 3D rendering with a proper cube mesh
- Simple ECS architecture with components and systems
- Cube always spins end-over-end around its local Z-axis
- **Drag to rotate**: Click and drag on the cube to change its orientation
- Static camera with the cube at the center
- User rotation + automatic spin are combined
- The spin direction moves around as you rotate the cube
- **PBR metallic material** with blue color, high metallic, low roughness
- **Orbiting point light** with warm color that circles the cube
- Real-time shadows and lighting effects

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

This will compile and run the application, opening a window with a spinning blue cube. The cube always spins end-over-end (around its local Z-axis). Click and drag on the cube to rotate it - this changes which direction the spin points in world space. The cube always spins the same way from its own perspective.

### Build for release

```bash
cargo build --release
```

The optimized binary will be available in `target/release/bevy-hello-world`.

## Project Structure

```
bevy-hello-world/
├── Cargo.toml          # Project configuration and dependencies
├── README.md           # This file
└── src/
    ├── main.rs          # Application entry point
    └── lib.rs           # Core application logic and tests
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
- `Camera3dBundle`: Static 3D camera with perspective projection
- `MaterialMeshBundle`: 3D mesh rendering with materials
- Quaternion math for 3D rotations
- Multiple entity systems running in sequence with `.chain()`

## Running Tests

```bash
cargo test
```

## Controls

- **Left Mouse Button + Drag**: Rotate the cube
  - Horizontal movement (left/right): Rotate around Y axis (yaw)
  - Vertical movement (up/down): Rotate around X axis (pitch)
- The cube always spins end-over-end around its local Z-axis
- Your manual rotations change the cube's orientation, which changes how the spin appears in world space

## License

This project is open source. Feel free to use it as a starting point for your own Bevy projects.

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
