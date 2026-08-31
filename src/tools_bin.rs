//! Shader Testing Tools Entry Point - Try with bright colors and center positions
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy::text::TextFont;
use bevy::text::FontSize;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, debug_text_system, debug_node_system, debug_text2d_system))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("DEBUG: Setup called");
    
    // 3D Camera for the cube - make background a different color to confirm it's rendering
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    println!("DEBUG: Spawning 2D camera");
    
    // 2D Camera for UI overlay - render AFTER 3d
    commands.spawn((
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Camera2d,
        Name::new("UI Camera"),
    ));
    
    println!("DEBUG: Spawning light");
    
    // Light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
        Name::new("Light"),
    ));
    
    println!("DEBUG: Spawning cube");
    
    // A cube with bright color
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Cube"),
    ));
    
    println!("DEBUG: Spawning UI");
    
    // Try Text2d at center of screen with BRIGHT GREEN, very large
    commands.spawn((
        Text2d::new("TEXT2D - BRIGHT GREEN"),
        TextFont {
            font_size: FontSize::Px(128.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 10.0), // Z=10 to ensure it's on top
        Name::new("Text2d"),
    ));
    
    // Also try UI text at center with BRIGHT YELLOW
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.5, 0.0, 0.0)), // Red background for visibility
        Name::new("Fullscreen UI Panel"),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("UI TEXT - BRIGHT YELLOW"),
            TextFont {
                font_size: FontSize::Px(128.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Name::new("UI Text"),
        ));
    });
    
    println!("DEBUG: Setup complete");
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

/// Debug system to print text entity info
fn debug_text_system(
    text_query: Query<&Text>,
) {
    let text_count = text_query.iter().count();
    println!("DEBUG: Found {} Text component(s)", text_count);
}

/// Debug system to print text2d entity info
fn debug_text2d_system(
    text2d_query: Query<&Text2d>,
) {
    let text2d_count = text2d_query.iter().count();
    println!("DEBUG: Found {} Text2d component(s)", text2d_count);
}

/// Debug system to print node info
fn debug_node_system(
    node_query: Query<&Node>,
) {
    let node_count = node_query.iter().count();
    println!("DEBUG: Found {} Node component(s)", node_count);
}
