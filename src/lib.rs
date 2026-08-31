//! Bevy Hello World - Main library that runs all examples

pub mod example_01_spinning_cube;
pub mod shader_tool;

/// Run the default example (spinning cube)
pub fn run_app() {
    example_01_spinning_cube::run_app();
}
