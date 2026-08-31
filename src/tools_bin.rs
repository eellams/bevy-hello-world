//! Shader Testing Tools Entry Point - Using bevy_egui
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, ui_system))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera for the cube
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    // Light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
        Name::new("Light"),
    ));
    
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
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn ui_system(mut contexts: EguiContexts) {
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Shader Tools")
            .default_pos(egui::pos2(10.0, 10.0))
            .show(ctx, |ui| {
                ui.label("HELLO WORLD - WHITE TEXT");
                ui.label("This is bevy_egui text");
                ui.separator();
                ui.label("Cube should be rotating below");
            });
    }
}
