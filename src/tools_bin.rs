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
use crate::tools::gui::slider::{SliderValueChanged, SliderWithDisplayBundle, update_slider_handle_positions, apply_slider_value_changes, update_slider_display_text};

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
            update_slider_handle_positions,
            apply_slider_value_changes,
            update_slider_display_text,
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
    // Main control panel using bevy_ui Node
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::all(Val::Px(5.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        Name::new("Control Panel"),
    ));

    // Panel title
    commands.spawn((
        Text::new("Shader Testing Framework"),
        Name::new("Title"),
    ));

    // Shader selection buttons using bevy_ui Button
    commands.spawn((
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("Previous Shader Button"),
    ));

    commands.spawn((
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("Next Shader Button"),
    ));

    // Geometry selection buttons
    commands.spawn((
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("Previous Geometry Button"),
    ));

    commands.spawn((
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("Next Geometry Button"),
    ));

    // Shader parameter sliders
    let param_names = vec!["Param1", "Param2", "Param3", "Param4"];
    
    for (i, _name) in param_names.iter().enumerate() {
        // Slider label
        commands.spawn((
            Text::new(format!("Param {}:", i + 1)),
        ));
        
        // Slider with display
        commands.spawn((
            SliderWithDisplayBundle::new(0.0, 1.0, 0.5),
        ));
    }

    // Toggle button example using bevy_ui
    commands.spawn((
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Name::new("Toggle Button"),
    ));

    // Information display
    commands.spawn((
        Text::new("Use arrow keys to switch shaders/geometries\nClick and drag sliders to adjust values"),
    ));
}
