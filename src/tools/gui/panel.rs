use bevy::prelude::*;
use super::state::*;

/// Component for panel-specific data
#[derive(Component, Debug)]
pub struct Panel;

/// Bundle for creating a panel (container for GUI elements)
#[derive(Bundle)]
pub struct PanelBundle {
    pub node: NodeBundle,
    pub panel: Panel,
    pub gui_element: GuiElement,
}

impl Default for PanelBundle {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ..default()
            },
            panel: Panel,
            gui_element: GuiElement,
        }
    }
}

impl PanelBundle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(width: Val, height: Val) -> Self {
        let mut bundle = Self::default();
        bundle.node.style.width = width;
        bundle.node.style.height = height;
        bundle
    }
}

/// Bundle for a panel with a title
#[derive(Bundle)]
pub struct PanelWithTitleBundle {
    pub panel: PanelBundle,
    pub title: TextBundle,
}

impl PanelWithTitleBundle {
    pub fn new(title: &str) -> Self {
        Self {
            panel: PanelBundle::default(),
            title: TextBundle::from_section(
                title,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        }
    }
}
