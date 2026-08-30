//! Shader Testing Tools Entry Point
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;

// Import our tools modules
mod tools;

use crate::tools::shaders::material::{ShaderParameters, update_shader_parameters_system};
use crate::tools::shaders::testing::{setup_shader_testing_framework, shader_switching_system, update_shader_test_entities, GeometryLibrary, ShaderLibrary, CurrentShader};
use crate::tools::shaders::library::{shader_hot_reload_system, load_shaders_from_directory, ShaderCompilationEvent, ShaderLibraryResource};

// Import our new slider
use crate::tools::gui::{SliderValueChanged, spawn_slider, update_slider_handle_positions, apply_slider_value_changes, slider_interaction_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ShaderLibraryResource>()
        .init_resource::<ShaderLibrary>()
        .init_resource::<GeometryLibrary>()
        .init_resource::<CurrentShader>()
        .init_resource::<ShaderParameters>()
        .add_message::<SliderValueChanged>()
        .add_message::<ShaderCompilationEvent>()
        .add_systems(Startup, (
            setup_gui_camera,
            setup_shader_testing_framework,
            setup_ui,
        ))
        .add_systems(Update, (
            slider_interaction_system,
            update_slider_handle_positions,
            apply_slider_value_changes,
            update_shader_parameters_system,
            shader_switching_system,
            update_shader_test_entities,
            shader_hot_reload_system,
            load_shaders_from_directory,
        ))
        .run();
}

/// Setup the GUI camera
fn setup_gui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Name::new("GUI Camera"),
    ));
}

/// Setup the UI with shader controls
fn setup_ui(
    mut commands: Commands,
) {
    // Spawn the test geometry first - a large colored quad in the center
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.4),
            custom_size: Some(Vec2::new(400.0, 400.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Name::new("Shader Test Quad"),
    ));

    // Main control panel
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(250.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        Name::new("Control Panel"),
    )).with_children(|parent| {
        // Panel title
        parent.spawn((
            Text::new("Shader Testing Framework"),
            TextColor(Color::WHITE),
            TextFont::default(),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            Name::new("Title"),
        ));

        // Shader selection buttons row
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            Name::new("Shader Buttons Row"),
        )).with_children(|row| {
            // Previous Shader Button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                Name::new("Previous Shader Button"),
            ));

            // Next Shader Button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                Name::new("Next Shader Button"),
            ));
        });

        // Geometry selection buttons row
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            Name::new("Geometry Buttons Row"),
        )).with_children(|row| {
            // Previous Geometry Button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                Name::new("Previous Geometry Button"),
            ));

            // Next Geometry Button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                Name::new("Next Geometry Button"),
            ));
        });

        // Shader parameter sliders - spawn labels inside panel, sliders separately
        let param_names = vec!["Red", "Green", "Blue", "Alpha"];
        let param_values = vec![1.0, 0.5, 0.3, 1.0];
        let mut y_offset = 180.0;
        
        for (name, _value) in param_names.iter().zip(param_values.iter()) {
            // Slider label - child of panel
            parent.spawn((
                Text::new(format!("{}:", name)),
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(y_offset - 10.0),
                    width: Val::Px(50.0),
                    ..default()
                },
                Name::new(format!("{} Label", name)),
            ));
            
            // Slider spawned separately at same level as panel (absolute positioning)
            // We store the y_offset for later spawning
            y_offset += 45.0;
        }

        // Information display
        parent.spawn((
            Text::new("Use arrow keys to switch\nClick and drag sliders to adjust RGB"),
            TextColor(Color::WHITE),
            TextFont::default().with_font_size(12.0),
            Node {
                margin: UiRect::top(Val::Px(15.0)),
                ..default()
            },
            Name::new("Info Text"),
        ));
    });
    
    // Now spawn the actual sliders at absolute positions
    let param_values = vec![1.0, 0.5, 0.3, 1.0];
    let mut y_offset = 180.0;
    for value in param_values.iter() {
        spawn_slider(&mut commands, 0.0, 1.0, *value, 70.0, y_offset);
        y_offset += 45.0;
    }
}
