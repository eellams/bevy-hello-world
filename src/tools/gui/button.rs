use bevy::prelude::*;
use super::state::*;

/// Component for button-specific data
#[derive(Component, Debug)]
pub struct Button;

/// Bundle for creating a button
#[derive(Bundle)]
pub struct ButtonBundle {
    pub node: NodeBundle,
    pub text: TextBundle,
    pub button: Button,
    pub gui_element: GuiElement,
    pub interactive: Interactive,
}

impl Default for ButtonBundle {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Px(120.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
            text: TextBundle::from_section(
                "Button",
                TextStyle {
                    font_size: 16.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            button: Button,
            gui_element: GuiElement,
            interactive: Interactive,
        }
    }
}

impl ButtonBundle {
    pub fn new(text: &str) -> Self {
        let mut bundle = Self::default();
        bundle.text = TextBundle::from_section(
            text,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        );
        bundle
    }

    pub fn with_size(width: f32, height: f32) -> Self {
        let mut bundle = Self::default();
        bundle.node.style.width = Val::Px(width);
        bundle.node.style.height = Val::Px(height);
        bundle
    }

    pub fn with_text(text: &str) -> Self {
        let mut bundle = Self::default();
        bundle.text = TextBundle::from_section(
            text,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        );
        bundle
    }

    pub fn with_toggle_state(active_text: &str, inactive_text: &str) -> Self {
        let mut bundle = Self::default();
        bundle.text = TextBundle::from_section(
            inactive_text,
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            },
        );
        bundle
    }
}

/// System to handle button interactions
pub fn button_interaction_system(
    mut interaction_query: Query<(
        Entity,
        &Interaction,
        &mut BackgroundColor,
        Option<&mut ToggleState>,
        Option<&mut Text>,
    ), (With<Button>, With<Interactive>)>,
    mut gui_events: EventWriter<GuiEvent>,
) {
    for (entity, interaction, mut bg, toggle_state, text) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
                
                // Toggle state if present
                if let Some(mut state) = toggle_state {
                    state.toggle();
                    if let Some(mut text) = text {
                        text.sections = vec![TextSection::new(
                            state.current_text(),
                            TextStyle {
                                font_size: 16.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )];
                    }
                    gui_events.send(GuiEvent {
                        entity,
                        event_type: GuiEventType::ToggleChanged(state.is_active),
                    });
                }
                
                gui_events.send(GuiEvent {
                    entity,
                    event_type: GuiEventType::Click,
                });
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
                gui_events.send(GuiEvent {
                    entity,
                    event_type: GuiEventType::HoverStart,
                });
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
                gui_events.send(GuiEvent {
                    entity,
                    event_type: GuiEventType::HoverEnd,
                });
            }
        }
    }
}

/// Component to mark a button as a toggle button
#[derive(Component, Debug)]
pub struct ToggleButton;

/// Bundle for creating a toggle button
#[derive(Bundle)]
pub struct ToggleButtonBundle {
    pub button: ButtonBundle,
    pub toggle: ToggleButton,
    pub state: ToggleState,
}

impl ToggleButtonBundle {
    pub fn new(active_text: &str, inactive_text: &str) -> Self {
        Self {
            button: ButtonBundle::with_toggle_state(active_text, inactive_text),
            toggle: ToggleButton,
            state: ToggleState::new(active_text, inactive_text),
        }
    }
}
