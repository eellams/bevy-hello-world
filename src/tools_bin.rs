//! Shader Testing Tools Entry Point
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use tools::gui::slider::{spawn_slider, update_slider_handle_positions, slider_interaction_system, apply_slider_value_changes};
use tools::shaders::testing::{ShaderLibrary, GeometryLibrary, setup_shader_testing_framework, shader_switching_system, update_shader_test_entities};

mod tools {
    pub mod gui {
        pub mod slider;
    }
    pub mod shaders {
        pub mod testing;
        pub mod library;
        pub mod material;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .init_resource::<ShaderLibrary>()
        .init_resource::<GeometryLibrary>()
        .add_systems(Startup, (setup, setup_shader_testing_framework))
        .add_systems(EguiPrimaryContextPass, ui_system)
        .add_systems(Update, (rotate_cube, shader_switching_system, update_shader_test_entities))
        .add_systems(Update, (update_slider_handle_positions, slider_interaction_system, apply_slider_value_changes).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D Camera - renders first
    commands.spawn((
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.1)),
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    
    // 2D Camera for UI - renders second, on top
    commands.spawn((
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Camera2d,
    ));
    
    // Light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
    ));
    
    // A cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Spawn a slider for testing
    spawn_slider(&mut commands, 0.0, 1.0, 0.5, 20.0, 200.0);
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
                ui.label("Shader Testing Tools");
                ui.label("Use arrow keys to switch shaders/geometries");
                ui.separator();
                ui.label("Cube is rotating below");
            });
    }
}
