//! Bevy UI Slider Component
//!
//! A slider control that follows bevy_ui conventions.
//! This provides a draggable slider with configurable min/max values.

use bevy::prelude::*;

/// Component for slider configuration and state
#[derive(Component, Debug, Clone)]
pub struct Slider {
    /// Minimum value
    pub min: f32,
    /// Maximum value
    pub max: f32,
    /// Current value
    pub value: f32,
    /// Track width for position calculations
    pub track_width: f32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            value: 0.5,
            track_width: 200.0,
        }
    }
}

impl Slider {
    /// Create a new slider with the given range and initial value
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        Self {
            min,
            max,
            value: value.clamp(min, max),
            track_width: 200.0,
        }
    }

    /// Get the normalized value (0.0 to 1.0)
    pub fn normalized(&self) -> f32 {
        if self.max == self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Set the value, clamping to min/max
    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Get the handle position based on current value
    pub fn handle_position(&self) -> f32 {
        self.normalized() * self.track_width - 10.0 // Handle width / 2
    }
}

/// Message emitted when slider value changes
#[derive(Message, Debug, Clone)]
pub struct SliderValueChanged {
    /// Entity of the slider container
    pub entity: Entity,
    /// New value
    pub value: f32,
}

/// Marker component for the slider handle (draggable part)
#[derive(Component, Debug)]
pub struct SliderHandle;

/// Spawn a complete slider with proper entity hierarchy.
/// The slider is spawned as a container with track and handle as children.
pub fn spawn_slider(
    commands: &mut Commands,
    min: f32,
    max: f32,
    value: f32,
    x: f32,
    y: f32,
) -> Entity {
    let slider = Slider::new(min, max, value);
    let position = slider.handle_position();
    let track_width = slider.track_width;

    // Spawn the container with the Slider component at specified position
    let container = commands.spawn((
        Node {
            width: Val::Px(track_width),
            height: Val::Px(40.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            left: Val::Px(x),
            top: Val::Px(y),
            ..default()
        },
        slider,
    )).id();

    // Spawn the track as a child
    commands.spawn((
        Node {
            width: Val::Px(track_width),
            height: Val::Px(8.0),
            position_type: PositionType::Absolute,
            top: Val::Px(15.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
    ));

    // Spawn the handle as a child with interaction components
    commands.spawn((
        Node {
            width: Val::Px(20.0),
            height: Val::Px(20.0),
            position_type: PositionType::Absolute,
            left: Val::Px(position + 10.0),
            top: Val::Px(5.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
        SliderHandle,
        Interaction::None,
    ));

    container
}

/// System to update slider handle position when value changes
pub fn update_slider_handle_positions(
    mut query: Query<(&Slider, &Children), With<Slider>>,
    mut handle_query: Query<&mut Node, With<SliderHandle>>,
) {
    for (slider, children) in &mut query {
        let position = slider.handle_position();
        
        // Find and update the handle child
        for child in children.iter() {
            if let Ok(mut handle_node) = handle_query.get_mut(child) {
                handle_node.left = Val::Px(position + 10.0);
            }
        }
    }
}

/// System to handle slider interactions
pub fn slider_interaction_system(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut interaction_query: Query<(
        Entity,
        &Interaction,
        &GlobalTransform,
        &Slider,
        &Children,
        &mut BackgroundColor,
    ), (
        With<SliderHandle>,
        Changed<Interaction>,
    )>,
    handle_query: Query<&SliderHandle>,
    mut value_changed_writer: MessageWriter<SliderValueChanged>,
) {
    let window = if let Ok(w) = windows.single() { w } else { return };
    let (camera, camera_transform) = if let Ok(c) = cameras.single() { c } else { return };
    
    for (entity, interaction, global_transform, slider, children, mut bg) in &mut interaction_query {
        // Find the handle entity
        let mut is_handle_interaction = false;
        for child in children.iter() {
            if handle_query.get(child).is_ok() {
                is_handle_interaction = true;
                break;
            }
        }
        
        if !is_handle_interaction {
            continue;
        }
        
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                
                // Update value based on cursor position
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        let track_left = global_transform.translation().x - slider.track_width / 2.0;
                        let normalized = ((world_pos.x - track_left) / slider.track_width).clamp(0.0, 1.0);
                        let new_value = slider.min + normalized * (slider.max - slider.min);
                        
                        if (new_value - slider.value).abs() > 0.001 {
                            value_changed_writer.write(SliderValueChanged {
                                entity,
                                value: new_value,
                            });
                        }
                    }
                }
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}

/// System to apply value changes from messages to slider components
pub fn apply_slider_value_changes(
    mut slider_query: Query<&mut Slider>,
    mut value_changed_reader: MessageReader<SliderValueChanged>,
) {
    for event in value_changed_reader.read() {
        if let Ok(mut slider) = slider_query.get_mut(event.entity) {
            slider.set(event.value);
        }
    }
}
