//! Bevy Hello World - Rotating Rectangle Example
//!
//! This library contains the core application logic for a simple Bevy
//! application that displays a rotating rectangle that can be dragged
//! to rotate with the mouse.

use bevy::prelude::*;
use bevy::math::primitives::Rectangle;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

/// Marker component for the rotating rectangle entity
#[derive(Component)]
pub struct RotatingCube;

/// Component to store the rotation state and drag information
#[derive(Component)]
pub struct RotationState {
    /// Whether the rectangle is currently being dragged
    pub is_dragging: bool,
    /// The initial angle when dragging started
    pub initial_angle: f32,
    /// The initial mouse position when dragging started
    pub initial_mouse_pos: Vec2,
}

impl Default for RotationState {
    fn default() -> Self {
        Self {
            is_dragging: false,
            initial_angle: 0.0,
            initial_mouse_pos: Vec2::ZERO,
        }
    }
}

/// System to set up the initial scene with camera and rectangle
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the rotating rectangle with rotation state
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(100.0, 100.0))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.8))),
            transform: Transform::default(),
            ..default()
        },
        RotatingCube,
        RotationState::default(),
    ));
}

/// System to handle mouse input for drag-to-rotate
pub fn handle_drag_rotation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut RotationState, &GlobalTransform), With<RotatingCube>>,
) {
    let window = windows.single();
    
    if let Some(mouse_pos) = window.cursor_position() {
        for (mut transform, mut state, global_transform) in &mut query {
            // Check if mouse is over the rectangle
            let rectangle_size = Vec2::new(100.0, 100.0);
            let rectangle_pos = global_transform.translation().truncate();
            
            // Simple AABB check for mouse over rectangle
            let half_size = rectangle_size / 2.0;
            let is_over = mouse_pos.x >= rectangle_pos.x - half_size.x
                && mouse_pos.x <= rectangle_pos.x + half_size.x
                && mouse_pos.y >= rectangle_pos.y - half_size.y
                && mouse_pos.y <= rectangle_pos.y + half_size.y;
            
            // Start dragging when mouse button is pressed over the rectangle
            if is_over && mouse_button_input.pressed(MouseButton::Left) && !state.is_dragging {
                state.is_dragging = true;
                state.initial_angle = transform.rotation.to_euler(EulerRot::ZXY).0; // Z rotation
                state.initial_mouse_pos = mouse_pos;
            }
            
            // Stop dragging when mouse button is released
            if !mouse_button_input.pressed(MouseButton::Left) && state.is_dragging {
                state.is_dragging = false;
            }
            
            // If dragging, calculate rotation based on mouse movement
            if state.is_dragging {
                let mouse_delta = mouse_pos - state.initial_mouse_pos;
                // Calculate angle from mouse movement relative to center
                let angle = mouse_delta.x * 0.01; // Scale factor for rotation speed
                transform.rotation = Quat::from_rotation_z(state.initial_angle + angle);
            }
        }
    }
}

/// System to rotate the rectangle automatically when not being dragged
pub fn rotate_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RotationState), With<RotatingCube>>,
) {
    for (mut transform, state) in &mut query {
        // Only auto-rotate if not being dragged
        if !state.is_dragging {
            transform.rotation = Quat::from_rotation_z(time.elapsed_seconds());
        }
    }
}

/// Run the application
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_drag_rotation, rotate_cube).chain())
        .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    /// Test that the app can be created without panicking
    #[test]
    fn test_app_creation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_drag_rotation, rotate_cube).chain());
        
        // The app should be created successfully - just verify it doesn't panic
        let _ = app;
    }

    /// Test that Rectangle can be created with correct dimensions
    #[test]
    fn test_rectangle_creation() {
        let rectangle = Rectangle::new(100.0, 100.0);
        assert_eq!(rectangle.half_size, Vec2::new(50.0, 50.0));
    }

    /// Test that Rectangle with different dimensions works correctly
    #[test]
    fn test_rectangle_various_sizes() {
        let rect1 = Rectangle::new(200.0, 100.0);
        assert_eq!(rect1.half_size, Vec2::new(100.0, 50.0));
        
        let rect2 = Rectangle::new(50.0, 25.0);
        assert_eq!(rect2.half_size, Vec2::new(25.0, 12.5));
    }

    /// Test that color is created correctly
    #[test]
    fn test_color_creation() {
        let color = Color::srgb(0.2, 0.4, 0.8);
        // Just verify it can be created without panicking
        let linear = color.to_linear();
        // Check that the color has non-zero components
        assert!(linear.red > 0.0 || linear.green > 0.0 || linear.blue > 0.0);
    }

    /// Test that color values are in expected range
    #[test]
    fn test_color_values() {
        let color = Color::srgb(0.2, 0.4, 0.8);
        let linear = color.to_linear();
        
        // SRGB to linear conversion should produce positive values
        assert!(linear.red >= 0.0);
        assert!(linear.green >= 0.0);
        assert!(linear.blue >= 0.0);
    }

    /// Test that the rotate_cube system has the correct signature
    #[test]
    fn test_rotate_cube_system_signature() {
        // This is a compile-time test that the rotate_cube function
        // has the correct signature for a Bevy system
        // The test passes if it compiles
        let _: fn(Res<Time>, Query<(&mut Transform, &RotationState), With<RotatingCube>>) = rotate_cube;
    }

    /// Test that Transform rotation works as expected
    #[test]
    fn test_transform_rotation() {
        let mut transform = Transform::default();
        let angle = std::f32::consts::PI / 2.0; // 90 degrees
        transform.rotation = Quat::from_rotation_z(angle);
        
        // A 90 degree rotation should have a non-identity quaternion
        assert_ne!(transform.rotation, Quat::IDENTITY);
    }

    /// Test that mesh can be created from rectangle
    #[test]
    fn test_mesh_from_rectangle() {
        let rectangle = Rectangle::new(100.0, 100.0);
        let mesh = Mesh::from(rectangle);
        
        // Mesh should have vertices
        assert!(mesh.count_vertices() > 0);
    }

    /// Test that RotatingCube component can be created
    #[test]
    fn test_rotating_cube_component() {
        let component = RotatingCube;
        // Just verify it can be created
        let _ = component;
    }

    /// Test that RotationState component can be created with defaults
    #[test]
    fn test_rotation_state_default() {
        let state = RotationState::default();
        assert!(!state.is_dragging);
        assert_eq!(state.initial_angle, 0.0);
        assert_eq!(state.initial_mouse_pos, Vec2::ZERO);
    }

    /// Test that RotationState can be created with custom values
    #[test]
    fn test_rotation_state_custom() {
        let state = RotationState {
            is_dragging: true,
            initial_angle: std::f32::consts::PI,
            initial_mouse_pos: Vec2::new(100.0, 200.0),
        };
        assert!(state.is_dragging);
        assert_eq!(state.initial_angle, std::f32::consts::PI);
        assert_eq!(state.initial_mouse_pos, Vec2::new(100.0, 200.0));
    }

    /// Test mouse over detection logic
    #[test]
    fn test_mouse_over_detection() {
        let rectangle_pos = Vec2::new(0.0, 0.0);
        let rectangle_size = Vec2::new(100.0, 100.0);
        let half_size = rectangle_size / 2.0;
        
        // Test mouse at center - should be over
        let mouse_at_center = Vec2::new(0.0, 0.0);
        let is_over_center = mouse_at_center.x >= rectangle_pos.x - half_size.x
            && mouse_at_center.x <= rectangle_pos.x + half_size.x
            && mouse_at_center.y >= rectangle_pos.y - half_size.y
            && mouse_at_center.y <= rectangle_pos.y + half_size.y;
        assert!(is_over_center);
        
        // Test mouse far away - should not be over
        let mouse_far_away = Vec2::new(1000.0, 1000.0);
        let is_over_far = mouse_far_away.x >= rectangle_pos.x - half_size.x
            && mouse_far_away.x <= rectangle_pos.x + half_size.x
            && mouse_far_away.y >= rectangle_pos.y - half_size.y
            && mouse_far_away.y <= rectangle_pos.y + half_size.y;
        assert!(!is_over_far);
    }

    /// Test that handle_drag_rotation system has the correct signature
    #[test]
    fn test_handle_drag_rotation_signature() {
        // Compile-time test for system signature
        let _: fn(
            Query<&Window, With<PrimaryWindow>>,
            Res<ButtonInput<MouseButton>>,
            Query<(&mut Transform, &mut RotationState, &GlobalTransform), With<RotatingCube>>
        ) = handle_drag_rotation;
    }
}
