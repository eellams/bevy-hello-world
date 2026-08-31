//! Shader Testing Tools Entry Point - Debug text rendering
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, debug_text_system))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("DEBUG: Setup called");
    
    // 3D Camera for the cube
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    println!("DEBUG: Spawning 2D camera");
    
    // 2D Camera for UI overlay
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
    
    println!("DEBUG: Spawning UI panel");
    
    // UI Panel with background - make it big and colored so we can see it
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.5)), // Dark blue background
        Name::new("UI Panel"),
    )).with_children(|parent| {
        println!("DEBUG: Spawning text as child");
        // Text as child
        parent.spawn((
            Text::new("HELLO WORLD - WHITE TEXT"),
            TextFont::default(),
            TextColor(Color::WHITE),
            Node {
                ..default()
            },
            Name::new("Text"),
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
    
    if text_count > 0 {
        println!("DEBUG: Found {} Text component(s)", text_count);
    } else {
        println!("DEBUG: No Text components found!");
    }
}
