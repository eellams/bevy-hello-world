//! Shader Testing Tools Entry Point - Test if 2D rendering works at all
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
    
    // Test 1: A colored sprite (definitely should be visible)
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0), // Bright red
            custom_size: Some(Vec2::new(200.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Name::new("Red Rectangle"),
    ));
    
    // Test 2: Text on top of the sprite
    commands.spawn((
        Text::new("HELLO WORLD"),
        TextFont::default(),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
        Name::new("Text"),
    ));
}
