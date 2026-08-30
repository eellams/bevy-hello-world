use bevy::prelude::*;

/// Marker component for GUI elements
#[derive(Component, Debug)]
pub struct GuiElement;

/// Marker component for interactive GUI elements
#[derive(Component, Debug)]
pub struct Interactive;

/// State for a GUI element that can be toggled
#[derive(Component, Debug, Clone)]
pub struct ToggleState {
    pub is_active: bool,
    pub active_text: String,
    pub inactive_text: String,
}

impl Default for ToggleState {
    fn default() -> Self {
        Self {
            is_active: false,
            active_text: "ON".to_string(),
            inactive_text: "OFF".to_string(),
        }
    }
}

impl ToggleState {
    pub fn new(active_text: &str, inactive_text: &str) -> Self {
        Self {
            is_active: false,
            active_text: active_text.to_string(),
            inactive_text: inactive_text.to_string(),
        }
    }

    pub fn toggle(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn current_text(&self) -> &str {
        if self.is_active {
            &self.active_text
        } else {
            &self.inactive_text
        }
    }
}

/// Numeric value that can be bound to GUI controls
#[derive(Component, Debug, Clone)]
pub struct NumericValue {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
}

impl Default for NumericValue {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            step: 0.01,
        }
    }
}

impl NumericValue {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            step: (max - min) / 100.0,
        }
    }

    pub fn with_step(value: f32, min: f32, max: f32, step: f32) -> Self {
        Self {
            value: value.clamp(min, max),
            min,
            max,
            step,
        }
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn increment(&mut self) {
        self.value = (self.value + self.step).clamp(self.min, self.max);
    }

    pub fn decrement(&mut self) {
        self.value = (self.value - self.step).clamp(self.min, self.max);
    }

    pub fn normalized(&self) -> f32 {
        if self.max == self.min {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }
}

/// Event for GUI interactions
#[derive(Event, Debug, Clone)]
pub struct GuiEvent {
    pub entity: Entity,
    pub event_type: GuiEventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GuiEventType {
    Click,
    HoverStart,
    HoverEnd,
    ValueChanged(f32),
    ToggleChanged(bool),
    DragStart,
    DragEnd,
    Dragging(f32),
}
