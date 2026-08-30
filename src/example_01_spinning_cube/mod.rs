//! Example 01: Spinning Cube
//!
//! A cube that always spins end-over-end around its local Z-axis,
//! while the user can rotate the cube to change the orientation of
//! that spin in world space. Features PBR materials and dynamic lighting.

use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
use bevy::mesh::Mesh3d;

/// Marker component for the spinning cube entity
#[derive(Component)]
pub struct SpinningCube;

/// Marker component for the point light
#[derive(Component)]
pub struct SceneLight;

/// Component to track the user's manual rotation of the cube
#[derive(Component)]
pub struct CubeRotation {
    /// The user's accumulated rotation (without the automatic spin)
    pub user_rotation: Quat,
    /// Whether the cube is currently being dragged
    pub is_dragging: bool,
    /// The user rotation when dragging started
    pub initial_user_rotation: Quat,
    /// The initial mouse position when dragging started
    pub initial_mouse_pos: Vec2,
}

impl Default for CubeRotation {
    fn default() -> Self {
        Self {
            user_rotation: Quat::IDENTITY,
            is_dragging: false,
            initial_user_rotation: Quat::IDENTITY,
            initial_mouse_pos: Vec2::ZERO,
        }
    }
}

/// System to set up the initial scene with static camera, cube, and lighting
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the cube at the origin with a metallic PBR material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            reflectance: 0.5,
            ..default()
        })),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        SpinningCube,
        CubeRotation::default(),
    ));

    // Spawn a point light that orbits the cube
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            radius: 10.0,
            color: Color::srgb(1.0, 0.9, 0.8),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3::new(3.0, 1.0, 0.0)),
        SceneLight,
    ));

    // Spawn a static camera looking at the cube
    commands.spawn((
        Camera3d {
            ..default()
        },
        Camera {
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// System to orbit the point light around the cube
pub fn orbit_light(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<SceneLight>>,
) {
    for mut transform in &mut query {
        let t = time.elapsed_secs() * 0.5;
        // Orbit the light in a circle around the cube
        transform.translation = Vec3::new(
            3.0 * t.cos(),
            1.0,
            3.0 * t.sin(),
        );
    }
}

/// System to spin the cube end-over-end around its local Z-axis
/// while preserving the user's manual rotation
pub fn spin_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CubeRotation), With<SpinningCube>>,
) {
    for (mut transform, rotation) in &mut query {
        // Constant end-over-end spin around local Z-axis
        let spin_rotation = Quat::from_rotation_z(time.elapsed_secs() * 2.0);
        
        // Combine: user rotation first, then spin in local space
        // This means the cube always spins end-over-end from its own perspective,
        // but the user can rotate the cube to change which way the spin points
        transform.rotation = rotation.user_rotation * spin_rotation;
    }
}

/// System to handle mouse drag to rotate the cube's user rotation
pub fn handle_cube_rotation(
    windows: Query<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut CubeRotation, With<SpinningCube>>,
) {
    if let Ok(window) = windows.single() {
        if let Some(mouse_pos) = window.cursor_position() {
            for mut cube_rotation in &mut query {
                // Start dragging when mouse button is pressed
                if mouse_button_input.pressed(MouseButton::Left) && !cube_rotation.is_dragging {
                    cube_rotation.is_dragging = true;
                    cube_rotation.initial_user_rotation = cube_rotation.user_rotation;
                    cube_rotation.initial_mouse_pos = mouse_pos;
                }
                
                // Stop dragging when mouse button is released
                if !mouse_button_input.pressed(MouseButton::Left) && cube_rotation.is_dragging {
                    cube_rotation.is_dragging = false;
                }
                
                // If dragging, update the user's rotation based on mouse movement
                if cube_rotation.is_dragging {
                    let mouse_delta = mouse_pos - cube_rotation.initial_mouse_pos;
                    
                    // Horizontal movement rotates around world Y axis
                    let yaw_rotation = Quat::from_rotation_y(mouse_delta.x * 0.01);
                    
                    // Vertical movement rotates around world X axis
                    let pitch_rotation = Quat::from_rotation_x(mouse_delta.y * 0.01);
                    
                    // Apply rotation relative to initial user rotation
                    // Order: apply pitch first, then yaw (so it feels natural)
                    cube_rotation.user_rotation = 
                        cube_rotation.initial_user_rotation * 
                        pitch_rotation * 
                        yaw_rotation;
                }
            }
        }
    }
}

/// Run the example
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_light, handle_cube_rotation, spin_cube).chain())
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
            .add_systems(Update, (orbit_light, handle_cube_rotation, spin_cube).chain());
        
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
        let _: fn(Res<Time>, Query<(&mut Transform, &CubeRotation), With<SpinningCube>>) = spin_cube;
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

    /// Test that SceneLight component can be created
    #[test]
    fn test_scene_light_component() {
        let component = SceneLight;
        // Just verify it can be created
        let _ = component;
    }

    /// Test that CubeRotation component can be created with defaults
    #[test]
    fn test_cube_rotation_default() {
        let rotation = CubeRotation::default();
        assert!(!rotation.is_dragging);
        assert_eq!(rotation.user_rotation, Quat::IDENTITY);
        assert_eq!(rotation.initial_user_rotation, Quat::IDENTITY);
        assert_eq!(rotation.initial_mouse_pos, Vec2::ZERO);
    }

    /// Test that CubeRotation can be created with custom values
    #[test]
    fn test_cube_rotation_custom() {
        let rotation = CubeRotation {
            user_rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
            is_dragging: true,
            initial_user_rotation: Quat::from_rotation_z(std::f32::consts::PI / 2.0),
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
            Query<&Window>,
            Res<ButtonInput<MouseButton>>,
            Query<&mut CubeRotation, With<SpinningCube>>
        ) = handle_cube_rotation;
    }

    /// Test that orbit_light system has the correct signature
    #[test]
    fn test_orbit_light_signature() {
        // Compile-time test for system signature
        let _: fn(Res<Time>, Query<&mut Transform, With<SceneLight>>) = orbit_light;
    }

    /// Test end-over-end spin rotation
    #[test]
    fn test_end_over_end_spin() {
        // The cube spins around its local Z-axis
        let spin_rotation = Quat::from_rotation_z(std::f32::consts::PI); // 180 degrees
        
        // Verify it's a Z-axis rotation
        let euler = spin_rotation.to_euler(EulerRot::ZXY);
        assert!(euler.0.abs() > 0.0); // Z component should be non-zero
    }

    /// Test combined user and spin rotation
    #[test]
    fn test_combined_rotation() {
        // User rotation around Y axis
        let user_rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        
        // Spin rotation around Z axis
        let spin_rotation = Quat::from_rotation_z(std::f32::consts::PI / 4.0);
        
        // Combined: user_rotation * spin_rotation
        let combined = user_rotation * spin_rotation;
        
        // Result should not be identity
        assert_ne!(combined, Quat::IDENTITY);
        
        // Result should be different from just user rotation
        assert_ne!(combined, user_rotation);
    }

    /// Test light orbit calculation
    #[test]
    fn test_light_orbit_calculation() {
        let t: f32 = 0.0;
        let x = 3.0 * t.cos();
        let z = 3.0 * t.sin();
        
        // At t=0, light should be at (3, 1, 0)
        assert_eq!(x, 3.0);
        assert_eq!(z, 0.0);
    }

    /// Test quaternion multiplication order
    #[test]
    fn test_quaternion_multiplication_order() {
        let q1 = Quat::from_rotation_x(std::f32::consts::PI / 2.0);
        let q2 = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        
        let order1 = q1 * q2;
        let order2 = q2 * q1;
        
        // Quaternion multiplication is not commutative
        assert_ne!(order1, order2);
    }
}
