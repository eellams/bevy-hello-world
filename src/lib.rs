//! Bevy Hello World - Rotating Cube Example
//!
//! This library contains the core application logic for a simple Bevy
//! application that displays a cube that spins automatically and can be
//! rotated by dragging with the mouse.

use bevy::prelude::*;
use bevy::pbr::MaterialMeshBundle;
use bevy::window::PrimaryWindow;

/// Marker component for the spinning cube entity
#[derive(Component)]
pub struct SpinningCube;

/// Component to track drag state for cube rotation
#[derive(Component)]
pub struct CubeRotation {
    /// Whether the cube is currently being dragged
    pub is_dragging: bool,
    /// The initial cube rotation when dragging started
    pub initial_rotation: Quat,
    /// The initial mouse position when dragging started
    pub initial_mouse_pos: Vec2,
}

impl Default for CubeRotation {
    fn default() -> Self {
        Self {
            is_dragging: false,
            initial_rotation: Quat::IDENTITY,
            initial_mouse_pos: Vec2::ZERO,
        }
    }
}

/// System to set up the initial scene with static camera and cube
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the cube at the origin
    commands.spawn((
        MaterialMeshBundle::<StandardMaterial> {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.4, 0.8),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        SpinningCube,
        CubeRotation::default(),
    ));

    // Spawn a static camera looking at the cube
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// System to spin the cube on all three axes at a constant rate
pub fn spin_cube(time: Res<Time>, mut query: Query<&mut Transform, With<SpinningCube>>) {
    for mut transform in &mut query {
        // Spin on all three axes at different rates for interesting motion
        let t = time.elapsed_seconds();
        let rotation_x = Quat::from_rotation_x(t * 0.5);
        let rotation_y = Quat::from_rotation_y(t * 0.7);
        let rotation_z = Quat::from_rotation_z(t * 1.0);
        
        // Combine rotations: Z * Y * X (order matters)
        transform.rotation = rotation_z * rotation_y * rotation_x;
    }
}

/// System to handle mouse drag to rotate the cube
pub fn handle_cube_rotation(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &mut CubeRotation), With<SpinningCube>>,
) {
    let window = windows.single();
    
    if let Some(mouse_pos) = window.cursor_position() {
        for (mut transform, mut rotation) in &mut query {
            // Start dragging when mouse button is pressed
            if mouse_button_input.pressed(MouseButton::Left) && !rotation.is_dragging {
                rotation.is_dragging = true;
                rotation.initial_rotation = transform.rotation;
                rotation.initial_mouse_pos = mouse_pos;
            }
            
            // Stop dragging when mouse button is released
            if !mouse_button_input.pressed(MouseButton::Left) && rotation.is_dragging {
                rotation.is_dragging = false;
            }
            
            // If dragging, rotate the cube based on mouse movement
            if rotation.is_dragging {
                let mouse_delta = mouse_pos - rotation.initial_mouse_pos;
                
                // Horizontal movement rotates around Y axis
                let yaw_rotation = Quat::from_rotation_y(mouse_delta.x * 0.01);
                
                // Vertical movement rotates around X axis
                let pitch_rotation = Quat::from_rotation_x(mouse_delta.y * 0.01);
                
                // Apply rotation relative to initial rotation
                transform.rotation = rotation.initial_rotation * yaw_rotation * pitch_rotation;
            }
        }
    }
}

/// Run the application
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_cube_rotation, spin_cube).chain())
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
            .add_systems(Update, (handle_cube_rotation, spin_cube).chain());
        
        // The app should be created successfully - just verify it doesn't panic
        let _ = app;
    }

    /// Test that Cuboid can be created with correct dimensions
    #[test]
    fn test_cuboid_creation() {
        let cuboid = Cuboid::new(1.0, 1.0, 1.0);
        assert_eq!(cuboid.half_size, Vec3::new(0.5, 0.5, 0.5));
    }

    /// Test that Cuboid with different dimensions works correctly
    #[test]
    fn test_cuboid_various_sizes() {
        let cuboid1 = Cuboid::new(2.0, 1.0, 0.5);
        assert_eq!(cuboid1.half_size, Vec3::new(1.0, 0.5, 0.25));
        
        let cuboid2 = Cuboid::new(0.5, 0.25, 0.125);
        assert_eq!(cuboid2.half_size, Vec3::new(0.25, 0.125, 0.0625));
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

    /// Test that the spin_cube system has the correct signature
    #[test]
    fn test_spin_cube_system_signature() {
        // This is a compile-time test that the spin_cube function
        // has the correct signature for a Bevy system
        let _: fn(Res<Time>, Query<&mut Transform, With<SpinningCube>>) = spin_cube;
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

    /// Test that mesh can be created from cuboid
    #[test]
    fn test_mesh_from_cuboid() {
        let cuboid = Cuboid::new(1.0, 1.0, 1.0);
        let mesh = Mesh::from(cuboid);
        
        // Mesh should have vertices
        assert!(mesh.count_vertices() > 0);
    }

    /// Test that SpinningCube component can be created
    #[test]
    fn test_spinning_cube_component() {
        let component = SpinningCube;
        // Just verify it can be created
        let _ = component;
    }

    /// Test that CubeRotation component can be created with defaults
    #[test]
    fn test_cube_rotation_default() {
        let rotation = CubeRotation::default();
        assert!(!rotation.is_dragging);
        assert_eq!(rotation.initial_rotation, Quat::IDENTITY);
        assert_eq!(rotation.initial_mouse_pos, Vec2::ZERO);
    }

    /// Test that CubeRotation can be created with custom values
    #[test]
    fn test_cube_rotation_custom() {
        let rotation = CubeRotation {
            is_dragging: true,
            initial_rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            initial_mouse_pos: Vec2::new(100.0, 200.0),
        };
        assert!(rotation.is_dragging);
        assert_eq!(rotation.initial_mouse_pos, Vec2::new(100.0, 200.0));
    }

    /// Test that handle_cube_rotation system has the correct signature
    #[test]
    fn test_handle_cube_rotation_signature() {
        // Compile-time test for system signature
        let _: fn(
            Query<&Window, With<PrimaryWindow>>,
            Res<ButtonInput<MouseButton>>,
            Query<(&mut Transform, &mut CubeRotation), With<SpinningCube>>
        ) = handle_cube_rotation;
    }

    /// Test 3D rotation on all axes
    #[test]
    fn test_3d_rotation_all_axes() {
        let mut transform = Transform::default();
        let t = 1.0;
        
        // Create rotations on all three axes
        let rotation_x = Quat::from_rotation_x(t * 0.5);
        let rotation_y = Quat::from_rotation_y(t * 0.7);
        let rotation_z = Quat::from_rotation_z(t * 1.0);
        
        // Combine rotations
        transform.rotation = rotation_z * rotation_y * rotation_x;
        
        // Result should not be identity
        assert_ne!(transform.rotation, Quat::IDENTITY);
    }

    /// Test quaternion multiplication order
    #[test]
    fn test_quaternion_multiplication() {
        let q1 = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
        let q2 = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        
        let combined = q1 * q2;
        
        // Combined rotation should not be identity
        assert_ne!(combined, Quat::IDENTITY);
    }
}
