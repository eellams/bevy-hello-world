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

### Shader Tool

Located in: `src/shader_tool.rs`

An interactive shader editor with hot-reloading. Edit shaders in real-time and see the results immediately.

**Features:**
- Live shader editing with hot-reload on save
- Dynamic shader selection from available `.wgsl` files
- Copy any shader to `current.wgsl` for editing
- Material property controls (colors, intensity, frequency, etc.)
- Multiple geometry types (cube, sphere, torus, plane, icosphere)
- Camera controls (orbit, pan, zoom)

**Controls:**
- **Left Mouse Button + Drag**: Rotate camera around target
- **Right Mouse Button + Drag**: Pan camera
- **Scroll Wheel**: Zoom camera in/out
- **S**: Save current shader to file
- **L**: Load shader from file
- **Escape**: Toggle UI visibility

## Running the Example

To run the spinning cube example:

```bash
cargo run
```

To run the shader tool:

```bash
cargo run --bin shader_tool
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

## Testing

### Unit Tests

Run all unit tests:
```bash
cargo test
```

Run specific test modules:
```bash
cargo test --test shader_tool_dynamic_tests  # Non-rendering shader logic tests
cargo test --test shader_tool_tests           # Core shader tool functionality
```

### Visual Regression Tests

The `shader_tool_visual_tests.rs` module provides visual regression testing by comparing rendered output against reference PNG images in `tests/expected_output/`.

To run visual tests (requires headless rendering support):
```bash
WGPU_BACKEND=vulkan MESA_LOADER_DRIVER_OVERRIDE=llvmpipe \
  cargo test --test shader_tool_visual_tests -- --nocapture
```

To update reference images after intentional shader changes:
```bash
UPDATE_REFERENCE=1 WGPU_BACKEND=vulkan MESA_LOADER_DRIVER_OVERRIDE=llvmpipe \
  cargo test --test shader_tool_visual_tests -- --nocapture
```

Then commit the new PNG files in `tests/expected_output/`.

## License

This project is open source. Feel free to use it as a reference for your own Bevy projects.

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/learn/)
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy)
- [Bevy Cheatbook](https://bevy-cheatbook.github.io/)
