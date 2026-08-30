//! Shader Testing Tools Entry Point
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;

// Import our tools modules
mod tools;

use crate::tools::gui::state::*;
use crate::tools::gui::button::{ButtonBundle as GuiButtonBundle, ToggleButtonBundle, button_interaction_system};
use crate::tools::gui::slider::{SliderWithDisplayBundle, slider_interaction_system, update_slider_handle_position, update_slider_value_display};
use crate::tools::gui::panel::PanelBundle;
use crate::tools::shaders::material::{ShaderParameters, update_shader_parameters_system};
use crate::tools::shaders::testing::{setup_shader_testing_framework, shader_switching_system, update_shader_test_entities, GeometryLibrary, ShaderLibrary, CurrentShader};
use crate::tools::shaders::library::{shader_hot_reload_system, load_shaders_from_directory, ShaderCompilationEvent, ShaderLibraryResource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<ShaderLibraryResource>()
        .init_resource::<ShaderLibrary>()
        .init_resource::<GeometryLibrary>()
        .init_resource::<CurrentShader>()
        .init_resource::<ShaderParameters>()
        .add_event::<GuiEvent>()
        .add_event::<ShaderCompilationEvent>()
        .add_systems(Startup, (
            setup_gui_camera,
            setup_shader_testing_framework,
            setup_ui,
        ))
        .add_systems(Update, (
            button_interaction_system,
            slider_interaction_system,
            update_slider_handle_position,
            update_slider_value_display,
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
        Camera2dBundle::default(),
        Name::new("GUI Camera"),
    ));
}

/// Setup the UI with shader controls
fn setup_ui(
    mut commands: Commands,
) {
    // Main control panel (PanelBundle already includes Transform via NodeBundle)
    let mut panel = PanelBundle::new();
    panel.node.transform = Transform::from_xyz(-300.0, 200.0, 0.0);
    commands.spawn(panel);
    
    // Panel title
    commands.spawn((
        TextBundle::from_section(
            "Shader Testing Framework",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        Transform::from_xyz(-300.0, 250.0, 0.0),
    ));

    // Shader selection buttons
    let mut prev_shader_btn = GuiButtonBundle::with_text("Previous Shader");
    prev_shader_btn.node.transform = Transform::from_xyz(-150.0, 150.0, 0.0);
    commands.spawn((prev_shader_btn, Name::new("Previous Shader Button")));

    let mut next_shader_btn = GuiButtonBundle::with_text("Next Shader");
    next_shader_btn.node.transform = Transform::from_xyz(0.0, 150.0, 0.0);
    commands.spawn((next_shader_btn, Name::new("Next Shader Button")));

    // Geometry selection buttons
    let mut prev_geo_btn = GuiButtonBundle::with_text("Previous Geometry");
    prev_geo_btn.node.transform = Transform::from_xyz(-150.0, 100.0, 0.0);
    commands.spawn((prev_geo_btn, Name::new("Previous Geometry Button")));

    let mut next_geo_btn = GuiButtonBundle::with_text("Next Geometry");
    next_geo_btn.node.transform = Transform::from_xyz(0.0, 100.0, 0.0);
    commands.spawn((next_geo_btn, Name::new("Next Geometry Button")));

    // Shader parameter sliders
    let param_names = vec!["Param1", "Param2", "Param3", "Param4"];
    
    for (i, name) in param_names.iter().enumerate() {
        let y_pos = 50.0 - i as f32 * 50.0;
        
        // Slider label
        commands.spawn((
            TextBundle::from_section(
                format!("{}:", name),
                TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            Transform::from_xyz(-150.0, y_pos, 0.0),
        ));
        
        // Slider
        let mut slider = SliderWithDisplayBundle::new(0.0, 1.0, 0.5, name);
        slider.slider.container.transform = Transform::from_xyz(-150.0, y_pos - 20.0, 0.0);
        commands.spawn(slider);
    }

    // Toggle button example
    let mut toggle = ToggleButtonBundle::new("ON", "OFF");
    toggle.button.node.transform = Transform::from_xyz(-150.0, -200.0, 0.0);
    commands.spawn(toggle);

    // Information display
    commands.spawn((
        TextBundle::from_section(
            "Use arrow keys to switch shaders/geometries\nClick and drag sliders to adjust values",
            TextStyle {
                font_size: 12.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ),
        Transform::from_xyz(-300.0, -250.0, 0.0),
    ));
}
