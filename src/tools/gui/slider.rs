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
    /// Step for incremental changes
    pub step: f32,
    /// Track width for position calculations
    pub track_width: f32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 1.0,
            value: 0.5,
            step: 0.01,
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
            step: (max - min) / 100.0,
            track_width: 200.0,
        }
    }

    /// Create a new slider with explicit step
    pub fn with_step(min: f32, max: f32, value: f32, step: f32) -> Self {
        Self {
            min,
            max,
            value: value.clamp(min, max),
            step,
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

/// Bundle for creating a slider handle
#[derive(Bundle)]
pub struct SliderHandleBundle {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub handle: SliderHandle,
}

impl Default for SliderHandleBundle {
    fn default() -> Self {
        Self {
            node: Node {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                left: Val::Px(95.0), // Center position (50% - 10px)
                top: Val::Px(10.0),
                ..default()
            },
            background_color: BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
            handle: SliderHandle,
        }
    }
}

/// Bundle for creating a complete slider
#[derive(Bundle)]
pub struct SliderBundle {
    /// The container node
    pub node: Node,
    /// The track (background bar)
    pub track: Node,
    pub track_background: BackgroundColor,
    /// The handle (draggable part)
    pub handle: SliderHandleBundle,
    /// Slider configuration
    pub slider: Slider,
}

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            node: Node {
                width: Val::Px(200.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            track: Node {
                width: Val::Px(200.0),
                height: Val::Px(8.0),
                ..default()
            },
            track_background: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            handle: SliderHandleBundle::default(),
            slider: Slider::default(),
        }
    }
}

impl SliderBundle {
    /// Create a new slider with the given range and initial value
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        let mut bundle = Self::default();
        bundle.slider = Slider::new(min, max, value);
        let position = bundle.slider.handle_position();
        bundle.handle.node.left = Val::Px(position);
        bundle
    }

    /// Create a new slider with explicit step
    pub fn with_step(min: f32, max: f32, value: f32, step: f32) -> Self {
        let mut bundle = Self::default();
        bundle.slider = Slider::with_step(min, max, value, step);
        let position = bundle.slider.handle_position();
        bundle.handle.node.left = Val::Px(position);
        bundle
    }

    /// Set the width of the slider
    pub fn with_width(mut self, width: f32) -> Self {
        self.node.width = Val::Px(width);
        self.track.width = Val::Px(width);
        self.slider.track_width = width;
        let position = self.slider.handle_position();
        self.handle.node.left = Val::Px(position);
        self
    }
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
                handle_node.left = Val::Px(position);
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
        With<Slider>,
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

/// Bundle for a slider with a text display
#[derive(Bundle)]
pub struct SliderWithDisplayBundle {
    /// The container node
    pub node: Node,
    /// The track (background bar)
    pub track: Node,
    pub track_background: BackgroundColor,
    /// The handle (draggable part)
    pub handle: SliderHandleBundle,
    /// Slider configuration
    pub slider: Slider,
    /// Text display for the value
    pub display: Text,
}

impl SliderWithDisplayBundle {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        let slider = Slider::new(min, max, value);
        let position = slider.handle_position();
        
        Self {
            node: Node {
                width: Val::Px(200.0),
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            track: Node {
                width: Val::Px(200.0),
                height: Val::Px(8.0),
                ..default()
            },
            track_background: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            handle: SliderHandleBundle {
                node: Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(position),
                    top: Val::Px(10.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                handle: SliderHandle,
            },
            slider,
            display: Text::new(format!("{:.2}", value)),
        }
    }
}

/// System to update slider display text when value changes
pub fn update_slider_display_text(
    mut query: Query<(&Slider, &mut Text)>, 
) {
    for (slider, mut text) in &mut query {
        *text = Text::new(format!("{:.2}", slider.value));
    }
}
