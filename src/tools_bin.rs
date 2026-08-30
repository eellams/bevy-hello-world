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
    // Main control panel using bevy_ui Node
    let _panel = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::all(Val::Px(5.0)),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            width: Val::Px(250.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        Name::new("Control Panel"),
    )).id();

    // Panel title
    commands.spawn((
        Text2d::new("Shader Testing Framework"),
        TextColor(Color::WHITE),
        TextFont::default(),
        Node {
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
        Name::new("Title"),
    ));

    // Shader selection buttons row
    let _button_row = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    )).id();

    // Previous Shader Button
    commands.spawn((
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
        Text2d::new("Prev Shader"),
        TextColor(Color::WHITE),
        Name::new("Previous Shader Button"),
    ));

    // Next Shader Button
    commands.spawn((
        Button,
        Node {
            width: Val::Px(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Text2d::new("Next Shader"),
        TextColor(Color::WHITE),
        Name::new("Next Shader Button"),
    ));

    // Geometry selection buttons row
    let _geometry_row = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    )).id();

    // Previous Geometry Button
    commands.spawn((
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
        Text2d::new("Prev Geo"),
        TextColor(Color::WHITE),
        Name::new("Previous Geometry Button"),
    ));

    // Next Geometry Button
    commands.spawn((
        Button,
        Node {
            width: Val::Px(100.0),
            height: Val::Px(30.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        Text2d::new("Next Geo"),
        TextColor(Color::WHITE),
        Name::new("Next Geometry Button"),
    ));

    // Shader parameter sliders - each in its own row
    let param_names = vec!["Red", "Green", "Blue", "Alpha"];
    
    for (i, name) in param_names.iter().enumerate() {
        let _slider_row = commands.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
        )).id();
        
        // Slider label
        commands.spawn((
            Text2d::new(format!("{}:", name)),
            TextColor(Color::WHITE),
            Node {
                width: Val::Px(50.0),
                ..default()
            },
        ));
        
        // Slider with display - spawn as separate entities to avoid bundle conflicts
        spawn_slider(&mut commands, 0.0, 1.0, if i == 0 { 1.0 } else if i == 1 { 0.5 } else if i == 2 { 0.3 } else { 1.0 });
    }

    // Information display
    commands.spawn((
        Text2d::new("Use arrow keys to switch shaders/geometries\nClick and drag sliders to adjust RGB values"),
        TextColor(Color::WHITE),
        TextFont::default().with_font_size(12.0),
        Node {
            margin: UiRect::top(Val::Px(15.0)),
            ..default()
        },
        Name::new("Info Text"),
    ));
}
