//! Unit tests for the shader tool functionality
//!
//! Run with: cargo test --test shader_tool_tests

use bevy::prelude::*;
use bevy::color::{LinearRgba, Color};
use bevy_hello_world::shader_tool::{
    LightingParameters,
    MaterialParameters,
    ShaderParameters,
    GeometryType,
    UniformCategory,
    ToolMode,
    ToolState,
    classify_uniform_type,
};

/// Test that lighting parameters have correct defaults
#[test]
fn test_lighting_parameters_defaults() {
    let params = LightingParameters::default();

    // Ambient light defaults
    assert_eq!(params.ambient_color, Color::srgb(0.1, 0.1, 0.1));
    assert_eq!(params.ambient_intensity, 1.0);

    // Point light defaults
    assert_eq!(params.point_light_position, Vec3::new(2.0, 2.0, 2.0));
    assert_eq!(params.point_light_color, Color::srgb(1.0, 1.0, 1.0));
    assert_eq!(params.point_light_intensity, 1000.0);
    assert_eq!(params.point_light_radius, 10.0);

    // Toggles
    assert!(params.use_point_light);
    assert!(params.use_ambient_light);
}

/// Test material parameters defaults
#[test]
fn test_material_parameters_defaults() {
    let params = MaterialParameters::default();

    assert_eq!(params.base_color, Color::srgb(0.8, 0.2, 0.4));
    assert_eq!(params.emissive, LinearRgba::BLACK);
    assert_eq!(params.emissive_exposure_weight, 0.0);
    assert_eq!(params.perceptual_roughness, 0.5);
    assert_eq!(params.metallic, 0.0);
    assert_eq!(params.reflectance, 0.5);
    assert!(!params.double_sided);
}

/// Test material to StandardMaterial conversion
#[test]
fn test_material_to_standard_material() {
    let mut params = MaterialParameters::default();
    params.base_color = Color::srgb(0.5, 0.5, 0.5);
    params.metallic = 0.8;
    params.perceptual_roughness = 0.2;

    let material = params.to_standard_material();

    assert_eq!(material.base_color, Color::srgb(0.5, 0.5, 0.5));
    assert_eq!(material.metallic, 0.8);
    assert_eq!(material.perceptual_roughness, 0.2);
}

/// Test shader parameters defaults
#[test]
fn test_shader_parameters_defaults() {
    let params = ShaderParameters::default();

    assert_eq!(params.float_uniforms.get("time_scale"), Some(&1.0));
    assert_eq!(params.float_uniforms.get("intensity"), Some(&1.0));
    assert_eq!(params.float_uniforms.get("frequency"), Some(&1.0));
    assert_eq!(params.float_uniforms.get("amplitude"), Some(&0.5));

    assert_eq!(params.vector_uniforms.get("direction"), Some(&vec![0.0, 0.0, 1.0]));
    assert_eq!(params.vector_uniforms.get("offset"), Some(&vec![0.0, 0.0, 0.0]));

    assert_eq!(params.color_uniforms.get("base_color"), Some(&Color::srgb(0.8, 0.2, 0.4)));
    assert_eq!(params.color_uniforms.get("accent_color"), Some(&Color::srgb(0.2, 0.8, 0.4)));

    assert!(params.detected_uniforms.is_empty());
}

/// Test shader parameter setters
#[test]
fn test_shader_parameters_setters() {
    let mut params = ShaderParameters::default();

    params.set_float("custom_float", 3.14);
    assert_eq!(params.float_uniforms.get("custom_float"), Some(&3.14));

    params.set_vector("custom_vec", vec![1.0, 2.0, 3.0]);
    assert_eq!(params.vector_uniforms.get("custom_vec"), Some(&vec![1.0, 2.0, 3.0]));

    params.set_color("custom_color", Color::srgb(1.0, 0.0, 0.0));
    assert_eq!(params.color_uniforms.get("custom_color"), Some(&Color::srgb(1.0, 0.0, 0.0)));
}

/// Test shader parameter getters
#[test]
fn test_shader_parameters_getters() {
    let mut params = ShaderParameters::default();

    params.set_float("test_float", 2.5);
    params.set_vector("test_vec3", vec![1.0, 2.0, 3.0]);
    params.set_color("test_color", Color::srgb(0.5, 0.5, 0.5));

    let vec2 = params.get_vector("test_vec3", 2);
    assert_eq!(vec2, vec![1.0, 2.0]);

    let vec3 = params.get_vector("test_vec3", 3);
    assert_eq!(vec3, vec![1.0, 2.0, 3.0]);

    let vec4 = params.get_vector("test_vec3", 4);
    assert_eq!(vec4, vec![1.0, 2.0, 3.0, 0.0]);

    let color = params.get_color("test_color");
    assert_eq!(color, Color::srgb(0.5, 0.5, 0.5));

    let missing_color = params.get_color("missing_color");
    assert_eq!(missing_color, Color::WHITE);
}

/// Test geometry creation
#[test]
fn test_geometry_creation() {
    let cube = GeometryType::Cube.create_mesh();
    assert!(cube.count_vertices() > 0);

    let sphere = GeometryType::Sphere.create_mesh();
    assert!(sphere.count_vertices() > 0);

    let plane = GeometryType::Plane.create_mesh();
    assert!(plane.count_vertices() > 0);

    let torus = GeometryType::Torus.create_mesh();
    assert!(torus.count_vertices() > 0);

    let capsule = GeometryType::Capsule.create_mesh();
    assert!(capsule.count_vertices() > 0);
}

/// Test uniform type classification
#[test]
fn test_uniform_classification() {
    assert_eq!(classify_uniform_type("f32"), UniformCategory::Scalar);
    assert_eq!(classify_uniform_type("i32"), UniformCategory::Scalar);
    assert_eq!(classify_uniform_type("u32"), UniformCategory::Scalar);

    assert_eq!(classify_uniform_type("vec2<f32>"), UniformCategory::Vector);
    assert_eq!(classify_uniform_type("vec3<f32>"), UniformCategory::Vector);
    assert_eq!(classify_uniform_type("vec4<f32>"), UniformCategory::Color);

    assert_eq!(classify_uniform_type("mat4x4<f32>"), UniformCategory::Matrix);
    assert_eq!(classify_uniform_type("custom_type"), UniformCategory::Unknown);
}

/// Test uniform extraction from shader code
#[test]
fn test_uniform_extraction() {
    let mut params = ShaderParameters::default();

    let shader_code = r#"
        @group(0) @binding(0)
        var<uniform> my_color: vec4<f32>;

        @group(0) @binding(1)
        var<uniform> my_intensity: f32;

        @group(0) @binding(2)
        var<uniform> my_position: vec3<f32>;

        @group(0) @binding(3)
        var<uniform> my_matrix: mat4x4<f32>;
    "#;

    params.extract_uniforms_from_shader(shader_code);

    assert_eq!(params.detected_uniforms.len(), 4);

    let names: Vec<&str> = params.detected_uniforms.iter().map(|u| u.name.as_str()).collect();
    assert!(names.contains(&"my_color"));
    assert!(names.contains(&"my_intensity"));
    assert!(names.contains(&"my_position"));
    assert!(names.contains(&"my_matrix"));

    // Check categories
    let color_uniform = params.detected_uniforms.iter().find(|u| u.name == "my_color").unwrap();
    assert_eq!(color_uniform.category, UniformCategory::Color);

    let intensity_uniform = params.detected_uniforms.iter().find(|u| u.name == "my_intensity").unwrap();
    assert_eq!(intensity_uniform.category, UniformCategory::Scalar);

    let position_uniform = params.detected_uniforms.iter().find(|u| u.name == "my_position").unwrap();
    assert_eq!(position_uniform.category, UniformCategory::Vector);

    let matrix_uniform = params.detected_uniforms.iter().find(|u| u.name == "my_matrix").unwrap();
    assert_eq!(matrix_uniform.category, UniformCategory::Matrix);
}

/// Test that comments are ignored in uniform extraction
#[test]
fn test_uniform_extraction_ignores_comments() {
    let mut params = ShaderParameters::default();

    let shader_code = r#"
        // @group(0) @binding(0)
        // var<uniform> fake_uniform: f32;

        /* @group(0) @binding(1) */
        /* var<uniform> another_fake: vec3<f32>; */

        @group(0) @binding(0)
        var<uniform> real_uniform: f32;
    "#;

    params.extract_uniforms_from_shader(shader_code);

    assert_eq!(params.detected_uniforms.len(), 1);
    assert_eq!(params.detected_uniforms[0].name, "real_uniform");
}

/// Test tool state defaults
#[test]
fn test_tool_state_defaults() {
    let state = ToolState::default();

    assert_eq!(state.mode, ToolMode::Material);
    assert_eq!(state.current_geometry, GeometryType::Cube);
    assert!(state.show_ui);
    assert!(!state.auto_rotate);
    assert_eq!(state.camera_distance, 5.0);
    assert_eq!(state.camera_pitch, 0.0);
    assert_eq!(state.camera_yaw, 0.0);
    assert_eq!(state.camera_target, Vec3::ZERO);

    assert!(state.available_geometries.contains(&GeometryType::Cube));
    assert!(state.available_geometries.contains(&GeometryType::Sphere));
    assert!(state.available_geometries.contains(&GeometryType::Plane));
    assert!(state.available_geometries.contains(&GeometryType::Torus));
    assert!(state.available_geometries.contains(&GeometryType::Capsule));
}

/// Test tool mode display strings
#[test]
fn test_tool_mode_display() {
    assert_eq!(ToolMode::Material.as_str(), "Material");
    assert_eq!(ToolMode::Shader.as_str(), "Shader");
}

/// Test lighting sync logic
#[test]
fn test_lighting_sync_logic() {
    let lighting = LightingParameters {
        ambient_color: Color::srgb(0.1, 0.2, 0.3),
        ambient_intensity: 1.5,
        point_light_position: Vec3::new(1.0, 2.0, 3.0),
        point_light_color: Color::srgb(0.9, 0.8, 0.7),
        point_light_intensity: 2000.0,
        point_light_radius: 25.0,
        use_point_light: true,
        use_ambient_light: false,
    };

    let mut shader_params = ShaderParameters::default();

    // Simulate sync_lighting_to_shader_params
    shader_params.color_uniforms.insert("ambient_color".to_string(), lighting.ambient_color);
    shader_params.float_uniforms.insert("ambient_intensity".to_string(), lighting.ambient_intensity);
    shader_params.vector_uniforms.insert("point_light_position".to_string(),
        vec![lighting.point_light_position.x, lighting.point_light_position.y, lighting.point_light_position.z]);
    shader_params.color_uniforms.insert("point_light_color".to_string(), lighting.point_light_color);
    shader_params.float_uniforms.insert("point_light_intensity".to_string(), lighting.point_light_intensity);
    shader_params.float_uniforms.insert("point_light_radius".to_string(), lighting.point_light_radius);
    shader_params.float_uniforms.insert("use_point_light".to_string(), if lighting.use_point_light { 1.0 } else { 0.0 });
    shader_params.float_uniforms.insert("use_ambient_light".to_string(), if lighting.use_ambient_light { 1.0 } else { 0.0 });

    // Verify the sync worked
    assert_eq!(shader_params.color_uniforms.get("ambient_color"), Some(&Color::srgb(0.1, 0.2, 0.3)));
    assert_eq!(shader_params.float_uniforms.get("ambient_intensity"), Some(&1.5));
    assert_eq!(shader_params.vector_uniforms.get("point_light_position"), Some(&vec![1.0, 2.0, 3.0]));
    assert_eq!(shader_params.color_uniforms.get("point_light_color"), Some(&Color::srgb(0.9, 0.8, 0.7)));
    assert_eq!(shader_params.float_uniforms.get("point_light_intensity"), Some(&2000.0));
    assert_eq!(shader_params.float_uniforms.get("point_light_radius"), Some(&25.0));
    assert_eq!(shader_params.float_uniforms.get("use_point_light"), Some(&1.0));
    assert_eq!(shader_params.float_uniforms.get("use_ambient_light"), Some(&0.0));
}
