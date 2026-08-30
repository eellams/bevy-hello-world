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

    pub fn with_position(x: f32, y: f32, z: f32) -> Self {
        let mut bundle = Self::default();
        bundle.node.transform.translation.x = x;
        bundle.node.transform.translation.y = y;
        bundle.node.transform.translation.z = z;
        bundle
    }

    pub fn with_size(width: Val, height: Val) -> Self {
        let mut bundle = Self::default();
        bundle.node.style.width = width;
        bundle.node.style.height = height;
        bundle
    }
}
