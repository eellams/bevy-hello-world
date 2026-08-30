use bevy::prelude::*;
use super::state::*;

/// Component for slider-specific data
#[derive(Component, Debug)]
pub struct Slider;

/// Bundle for creating a slider
#[derive(Bundle)]
pub struct SliderBundle {
    pub container: NodeBundle,
    pub track: NodeBundle,
    pub handle: NodeBundle,
    pub value: NumericValue,
    pub gui_element: GuiElement,
    pub interactive: Interactive,
    pub slider: Slider,
}

impl Default for SliderBundle {
    fn default() -> Self {
        Self {
            container: NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            track: NodeBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(8.0),
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(10.0), Val::Px(0.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                ..default()
            },
            handle: NodeBundle {
                style: Style {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.6, 0.6, 0.6)),
                ..default()
            },
            value: NumericValue::default(),
            gui_element: GuiElement,
            interactive: Interactive,
            slider: Slider,
        }
    }
}

impl SliderBundle {
    pub fn new(min: f32, max: f32, value: f32) -> Self {
        let mut bundle = Self::default();
        bundle.value = NumericValue::new(value, min, max);
        bundle
    }

    pub fn with_step(min: f32, max: f32, value: f32, step: f32) -> Self {
        let mut bundle = Self::default();
        bundle.value = NumericValue::with_step(value, min, max, step);
        bundle
    }
}

/// System to update slider handle position based on value
pub fn update_slider_handle_position(
    mut query: Query<(&NumericValue, &mut Style), With<Slider>>,
) {
    for (value, mut handle_style) in &mut query {
        // Calculate handle position based on normalized value
        let normalized = value.normalized();
        let offset = (normalized * 200.0) - 10.0; // Handle width / 2
        handle_style.left = Val::Px(offset);
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
        &mut NumericValue,
        &mut BackgroundColor,
    ), With<Slider>>,
    mut gui_events: EventWriter<GuiEvent>,
) {
    let window = windows.single();
    let (camera, camera_transform) = cameras.single();
    
    for (entity, interaction, global_transform, mut value, mut bg) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                gui_events.send(GuiEvent {
                    entity,
                    event_type: GuiEventType::DragStart,
                });
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                
                // Handle mouse wheel
                if let Some(cursor_pos) = window.cursor_position() {
                    if let Some(pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                        // Calculate value based on cursor position
                        let track_width = 200.0;
                        let track_left = global_transform.translation().x - track_width / 2.0;
                        let normalized = ((pos.x - track_left) / track_width).clamp(0.0, 1.0);
                        let new_value = value.min + normalized * (value.max - value.min);
                        value.set(new_value);
                        
                        gui_events.send(GuiEvent {
                            entity,
                            event_type: GuiEventType::ValueChanged(value.value),
                        });
                    }
                }
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                gui_events.send(GuiEvent {
                    entity,
                    event_type: GuiEventType::DragEnd,
                });
            }
        }
    }
}

/// Component for displaying the slider value as text
#[derive(Component, Debug)]
pub struct SliderValueDisplay;

/// System to update slider value display
pub fn update_slider_value_display(
    mut query: Query<(&NumericValue, &mut Text), (With<SliderValueDisplay>, Without<Slider>)>,
) {
    for (value, mut text) in &mut query {
        text.sections = vec![TextSection::new(
            format!("{:.2}", value.value),
            TextStyle {
                font_size: 14.0,
                color: Color::WHITE,
                ..default()
            },
        )];
    }
}

/// Bundle for a complete slider with value display
#[derive(Bundle)]
pub struct SliderWithDisplayBundle {
    pub slider: SliderBundle,
    pub display: TextBundle,
    pub value_display: SliderValueDisplay,
}

impl SliderWithDisplayBundle {
    pub fn new(min: f32, max: f32, value: f32, _label: &str) -> Self {
        Self {
            slider: SliderBundle::new(min, max, value),
            display: TextBundle::from_section(
                format!("{:.2}", value),
                TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            value_display: SliderValueDisplay,
        }
    }
}
