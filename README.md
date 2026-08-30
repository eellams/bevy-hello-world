# Bevy Hello World Examples

A collection of Bevy game engine examples demonstrating various features and techniques.

## Examples

### Example 01: Spinning Cube

Located in: `src/example_01_spinning_cube/mod.rs`

A cube that always spins end-over-end around its local Z-axis, while the user can rotate the cube to change the orientation of that spin in world space.

**Features:**
- PBR metallic material with blue color
- End-over-end spin around local Z-axis
- Drag to rotate the cube
- Orbiting point light with shadows
- Static camera

**Controls:**
- **Left Mouse Button + Drag**: Rotate the cube
  - Horizontal movement: Rotate around Y axis (yaw)
  - Vertical movement: Rotate around X axis (pitch)

## Running the Example

To run the spinning cube example:

```bash
cargo run
```

## Project Structure

```
bevy-hello-world/
├── Cargo.toml              # Project configuration
├── README.md               # This file
├── .gitignore              # Git ignore rules
├── src/
│   ├── main.rs             # Entry point (runs example_01)
│   ├── lib.rs              # Library exports
│   ├── example_01_spinning_cube/
│   │   └── mod.rs          # Spinning cube example implementation
│   └── tools_bin.rs        # Shader tools binary
└── shaders/                # Shader files
```

## Adding New Examples

1. Create a new directory under `src/` (e.g., `src/example_02_my_example/`)
2. Add a `mod.rs` file with your example implementation
3. Export the example module in `src/lib.rs`
4. Update `src/main.rs` to run your example (or add a way to select examples)

## License

This project is open source. Feel free to use it as a reference for your own Bevy projects.

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
