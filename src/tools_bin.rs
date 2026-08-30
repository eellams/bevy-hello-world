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
    // 3D Camera - renders the cube (order -1 = render first)
    commands.spawn((
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    // 2D Camera for UI overlay - order 0 = renders after 3D
    // Don't clear, render on top of 3D
    commands.spawn((
        Camera {
            order: 0,
            clear_color: ClearColorConfig::None,
            ..default()
        },
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
    
    // UI Panel - using bevy_ui Node
    // Positioned in screen space (2D coordinates)
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(200.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        Name::new("UI Panel"),
    )).with_children(|parent| {
        // Title text
        parent.spawn((
            Text::new("3D Cube with Shader"),
            TextColor(Color::WHITE),
            TextFont::default(),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
        
        // Info text
        parent.spawn((
            Text::new("The cube is rotating"),
            TextColor(Color::srgb(0.8, 0.8, 0.8)),
            TextFont::default().with_font_size(12.0),
        ));
    });
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
