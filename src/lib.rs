//! Bevy Hello World - Rotating Rectangle Example
//!
//! This library contains the core application logic for a simple Bevy
//! application that displays a rotating rectangle.

use bevy::prelude::*;
use bevy::math::primitives::Rectangle;
use bevy::sprite::MaterialMesh2dBundle;

/// Marker component for the rotating rectangle entity
#[derive(Component)]
pub struct RotatingCube;

/// System to set up the initial scene with camera and rectangle
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the rotating rectangle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(Rectangle::new(100.0, 100.0))).into(),
            material: materials.add(ColorMaterial::from(Color::srgb(0.2, 0.4, 0.8))),
            transform: Transform::default(),
            ..default()
        },
        RotatingCube,
    ));
}

/// System to rotate the rectangle based on elapsed time
pub fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(time.elapsed_seconds());
    }
}

/// Run the application
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
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
            .add_systems(Update, rotate_cube);
        
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
        let _: fn(Res<Time>, Query<&mut Transform, With<RotatingCube>>) = rotate_cube;
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
}
