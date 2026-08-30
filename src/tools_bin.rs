//! Shader Testing Tools Entry Point - Minimal UI test
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
    // Camera2d for UI
    commands.spawn(Camera2d);
    
    // Create a UI root node
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        Name::new("Root"),
    )).with_children(|parent| {
        // Text as child of root
        parent.spawn((
            Text::new("HELLO WORLD"),
            TextFont::default(),
            TextColor(Color::WHITE),
            Node {
                ..default()
            },
        ));
    });
}
