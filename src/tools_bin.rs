//! Shader Testing Tools Entry Point
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use shader_tool::ShaderToolPlugin;

mod shader_tool;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(ShaderToolPlugin)
        .run();
}
