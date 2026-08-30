//! Shader Testing Tools Entry Point
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    // 2D Camera for UI overlay
    commands.spawn((
        Camera2d,
        Name::new("UI Camera"),
    ));
    
    // A cube with a colored material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Name::new("Cube"),
    ));
    
    // Light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
        GlobalTransform::default(),
        Name::new("Light"),
    ));
    
    // Some text on screen
    commands.spawn((
        Text2d::new("3D Cube with Shader"),
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 200.0, 0.0),
        GlobalTransform::default(),
        Name::new("Title"),
    ));
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
        transform.rotate_x(time.delta_secs() * 0.3);
    }
}
