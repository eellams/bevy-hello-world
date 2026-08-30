//! Shader Testing Tools Entry Point - Minimal text test
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Only a 2D camera
    commands.spawn(Camera2d);
    
    // Just text - nothing else
    commands.spawn((
        Text::new("HELLO WORLD"),
        TextFont::default(),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(100.0),
            ..default()
        },
    ));
}
