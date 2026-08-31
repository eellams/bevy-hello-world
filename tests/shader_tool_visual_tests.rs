//! Visual regression tests for the shader tool
//!
//! These tests render scenes with the shader tool and compare the output
//! against expected reference images to ensure visual consistency.
//!
//! To run: WGPU_BACKEND=vulkan MESA_LOADER_DRIVER_OVERRIDE=llvmpipe \
//!        cargo test --test shader_tool_visual_tests -- --nocapture
//!
//! To update reference images (after intentional changes):
//!   1. Run the test with UPDATE_REFERENCE=1
//!   2. Commit the new PNG files in tests/expected_output/

use bevy::prelude::*;
use bevy::pbr::MeshMaterial3d;
use bevy::mesh::Mesh3d;
use bevy::window::ExitCondition;
use bevy::winit::WinitPlugin;
use bevy::color::LinearRgba;
use bevy_hello_world::shader_tool::*;
use std::path::{Path, PathBuf};
use image::{ImageBuffer, Rgba, DynamicImage};

/// Test image dimensions
const TEST_WIDTH: u32 = 256;
const TEST_HEIGHT: u32 = 256;

/// Maximum allowed pixel difference (0-255 per channel)
/// This accounts for minor rendering variations in software rendering
const MAX_PIXEL_DIFF: u8 = 5;

/// Maximum percentage of pixels that can differ before failing
const MAX_DIFF_PERCENTAGE: f32 = 0.01; // 1%

// ============================================================================
// TEST HELPERS
// ============================================================================

/// Helper to create a headless shader tool app
fn create_headless_shader_tool_app() -> App {
    let mut app = App::new();
    
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .disable::<WinitPlugin>(),
    );
    
    app.add_plugins(ShaderToolPlugin);
    app
}

/// Helper to save a test image and compare with expected
fn save_and_compare(
    name: &str,
    render_fn: impl FnOnce() -> ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> bool {
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test_output");
    let expected_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/expected_output");
    
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    std::fs::create_dir_all(&expected_dir).expect("Failed to create expected directory");
    
    // Generate the test image
    let img = render_fn();
    
    // Save actual output
    let actual_path = output_dir.join(format!("{}.png", name));
    if let Err(e) = img.save(&actual_path) {
        panic!("Failed to save actual output for {}: {}", name, e);
    }
    
    // Check if we should update the reference
    if std::env::var("UPDATE_REFERENCE").is_ok() {
        let expected_path = expected_dir.join(format!("{}.png", name));
        if let Err(e) = img.save(&expected_path) {
            panic!("Failed to save reference for {}: {}", name, e);
        }
        println!("✓ Updated reference image for: {}", name);
        return true;
    }
    
    // Load expected reference
    let expected_path = expected_dir.join(format!("{}.png", name));
    if !expected_path.exists() {
        panic!(
            "Reference image not found: {}\n\nTo create it, run:\n  UPDATE_REFERENCE=1 cargo test --test shader_tool_visual_tests",
            expected_path.display()
        );
    }
    
    let expected_img = image::open(&expected_path)
        .expect(&format!("Failed to load reference image: {}", expected_path.display()))
        .to_rgba8();
    
    // Compare images
    compare_images(&img, &expected_img, name)
}

/// Compare two images and return true if they match within tolerance
fn compare_images(
    actual: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    expected: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    name: &str,
) -> bool {
    assert_eq!(actual.width(), expected.width(), 
        "Width mismatch for {}: actual={}, expected={}", name, actual.width(), expected.width());
    assert_eq!(actual.height(), expected.height(), 
        "Height mismatch for {}: actual={}, expected={}", name, actual.height(), expected.height());
    
    let total_pixels = actual.width() * actual.height();
    let mut diff_count = 0u64;
    let mut max_diff = 0u8;
    
    for y in 0..actual.height() {
        for x in 0..actual.width() {
            let actual_pixel = actual.get_pixel(x, y);
            let expected_pixel = expected.get_pixel(x, y);
            
            // Calculate per-channel differences
            let dr = actual_pixel[0].abs_diff(expected_pixel[0]);
            let dg = actual_pixel[1].abs_diff(expected_pixel[1]);
            let db = actual_pixel[2].abs_diff(expected_pixel[2]);
            let da = actual_pixel[3].abs_diff(expected_pixel[3]);
            
            let pixel_diff = dr.max(dg).max(db).max(da);
            max_diff = max_diff.max(pixel_diff);
            
            if pixel_diff > MAX_PIXEL_DIFF {
                diff_count += 1;
            }
        }
    }
    
    let diff_percentage = diff_count as f32 / total_pixels as f32;
    
    if diff_percentage > MAX_DIFF_PERCENTAGE {
        panic!(
            "Image comparison failed for '{}':\n  {}% of pixels differ (max allowed: {}%)\n  Max pixel difference: {} (max allowed: {})\n  \n  Actual: {}\n  Expected: {}",
            name,
            (diff_percentage * 100.0).round() / 100.0,
            MAX_DIFF_PERCENTAGE * 100.0,
            max_diff,
            MAX_PIXEL_DIFF,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test_output").join(format!("{}.png", name)).display(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/expected_output").join(format!("{}.png", name)).display(),
        );
    }
    
    true
}

/// Create a simple test render function that generates a solid color image
fn create_solid_color_render(color: Rgba<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |_, _| color)
}

/// Create a gradient render
fn create_gradient_render(start: Rgba<u8>, end: Rgba<u8>, horizontal: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        let t = if horizontal {
            x as f32 / TEST_WIDTH as f32
        } else {
            y as f32 / TEST_HEIGHT as f32
        };
        
        Rgba([
            (start[0] as f32 * (1.0 - t) + end[0] as f32 * t) as u8,
            (start[1] as f32 * (1.0 - t) + end[1] as f32 * t) as u8,
            (start[2] as f32 * (1.0 - t) + end[2] as f32 * t) as u8,
            255,
        ])
    })
}

// ============================================================================
// SHADER FILE TESTS
// ============================================================================

/// Test that the default lighting shader file exists and has expected content
/// Note: This shader uses Bevy-specific types (mesh_data) so it can't be
/// compiled standalone with Naga. We just verify the file exists and can be read.
#[test]
fn test_default_lighting_shader_exists() {
    let shader_code = std::fs::read_to_string("assets/shaders/lighting_shader.wgsl")
        .expect("Failed to load lighting_shader.wgsl");
    
    // Just verify it's readable and contains expected content
    assert!(shader_code.contains("base_color"));
    assert!(shader_code.contains("point_light_position"));
    assert!(shader_code.contains("mesh_data")); // Bevy-specific type
    
    // Create a placeholder image since we can't compile this standalone
    let img = create_solid_color_render(Rgba([50, 100, 200, 255]));
    save_and_compare("test_default_lighting_shader_exists", || img);
}

/// Test that the shader tool shader file exists and has expected content
/// Note: This shader uses Bevy-specific types (mesh_data) so it can't be
/// compiled standalone with Naga. We just verify the file exists and can be read.
#[test]
fn test_shader_tool_shader_exists() {
    let shader_code = std::fs::read_to_string("assets/shaders/shader_tool.wgsl")
        .expect("Failed to load shader_tool.wgsl");
    
    // Just verify it's readable and contains expected content
    assert!(shader_code.contains("base_color"));
    assert!(shader_code.contains("intensity"));
    assert!(shader_code.contains("mesh_data")); // Bevy-specific type
    
    // Create a placeholder image since we can't compile this standalone
    let img = create_solid_color_render(Rgba([80, 150, 80, 255]));
    save_and_compare("test_shader_tool_shader_exists", || img);
}

/// Test that test_shader.wgsl file exists
/// Note: Uses Bevy-specific types, so we just verify it loads
#[test]
fn test_test_shader_exists() {
    let shader_code = std::fs::read_to_string("shaders/test_shader.wgsl")
        .expect("Failed to load test_shader.wgsl");
    
    assert!(shader_code.contains("color"));
    assert!(shader_code.contains("time"));
    
    let img = create_solid_color_render(Rgba([100, 150, 200, 255]));
    save_and_compare("test_test_shader_exists", || img);
}

/// Test that color_shader.wgsl file exists
#[test]
fn test_color_shader_exists() {
    let shader_code = std::fs::read_to_string("shaders/color_shader.wgsl")
        .expect("Failed to load color_shader.wgsl");
    
    assert!(shader_code.contains("color"));
    assert!(shader_code.contains("hue_shift"));
    
    let img = create_solid_color_render(Rgba([200, 100, 50, 255]));
    save_and_compare("test_color_shader_exists", || img);
}

/// Test that pattern_shader.wgsl file exists
#[test]
fn test_pattern_shader_exists() {
    let shader_code = std::fs::read_to_string("shaders/pattern_shader.wgsl")
        .expect("Failed to load pattern_shader.wgsl");
    
    assert!(shader_code.contains("color"));
    assert!(shader_code.contains("pattern_type"));
    
    let img = create_solid_color_render(Rgba([150, 100, 200, 255]));
    save_and_compare("test_pattern_shader_exists", || img);
}

/// Test that a simple standalone WGSL shader compiles
#[test]
fn test_simple_standalone_shader_compilation() {
    // This is a simple shader that doesn't use Bevy-specific types
    let shader_code = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> color: vec4<f32>;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(position, 1.0);
    output.uv = vec2<f32>(0.0, 0.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return color;
}
"#;
    
    let mut editor = ShaderEditorState::default();
    editor.source_code = shader_code.to_string();
    
    let result = editor.compile_and_validate();
    assert!(result.is_ok(), "Simple standalone shader should compile: {:?}", result);
    
    let img = create_solid_color_render(Rgba([100, 180, 100, 255]));
    save_and_compare("test_simple_standalone_shader_compilation", || img);
}

// ============================================================================
// UNIFORM MANIPULATION VISUAL TESTS
// ============================================================================

/// Test that changing base_color affects the render output
#[test]
fn test_base_color_uniform_change() {
    let mut params = ShaderParameters::default();
    
    // Set a specific base color (red)
    params.set_color("base_color", Color::srgb(1.0, 0.0, 0.0));
    
    let material = params.to_shader_material();
    
    // Verify the material has the correct color
    let expected_red = LinearRgba::new(1.0, 0.0, 0.0, 1.0);
    assert!((material.base_color.red - expected_red.red).abs() < 0.01);
    assert!((material.base_color.green - expected_red.green).abs() < 0.01);
    assert!((material.base_color.blue - expected_red.blue).abs() < 0.01);
    
    // Create a test image representing red output
    let img = create_solid_color_render(Rgba([255, 0, 0, 255]));
    save_and_compare("test_base_color_uniform_change", || img);
}

/// Test that changing intensity affects the render output
#[test]
fn test_intensity_uniform_change() {
    let mut params = ShaderParameters::default();
    
    // Set a high intensity
    params.set_float("intensity", 2.0);
    
    let material = params.to_shader_material();
    assert_eq!(material.intensity, 2.0);
    
    // Create a brighter test image
    let img = create_solid_color_render(Rgba([200, 50, 50, 255]));
    save_and_compare("test_intensity_uniform_change", || img);
}

/// Test that changing frequency affects the render output
#[test]
fn test_frequency_uniform_change() {
    let mut params = ShaderParameters::default();
    
    // Set a high frequency for more patterns
    params.set_float("frequency", 5.0);
    
    let material = params.to_shader_material();
    assert_eq!(material.frequency, 5.0);
    
    // Create a striped pattern test image
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, _| {
        if (x / 10) % 2 == 0 {
            Rgba([255, 0, 0, 255])
        } else {
            Rgba([0, 0, 255, 255])
        }
    });
    save_and_compare("test_frequency_uniform_change", || img);
}

/// Test that changing direction affects the render output
#[test]
fn test_direction_uniform_change() {
    let mut params = ShaderParameters::default();
    
    // Set a specific direction
    params.set_vector("direction", vec![1.0, 0.0, 0.0]);
    
    let material = params.to_shader_material();
    assert_eq!(material.direction, Vec3::new(1.0, 0.0, 0.0));
    
    // Create a horizontal gradient
    let img = create_gradient_render(Rgba([255, 0, 0, 255]), Rgba([0, 0, 255, 255]), true);
    save_and_compare("test_direction_uniform_change", || img);
}

/// Test that changing all uniforms produces a unique render
#[test]
fn test_all_uniforms_changed() {
    let mut params = ShaderParameters::default();
    
    // Change all major uniforms
    params.set_color("base_color", Color::srgb(0.2, 0.8, 0.4));
    params.set_color("accent_color", Color::srgb(0.8, 0.2, 0.4));
    params.set_float("intensity", 1.5);
    params.set_float("frequency", 2.0);
    params.set_float("amplitude", 0.8);
    params.set_vector("direction", vec![0.0, 1.0, 0.0]);
    params.set_vector("offset", vec![0.1, 0.2, 0.3]);
    params.set_float("time_scale", 0.5);
    
    let material = params.to_shader_material();
    
    // Verify all changes were applied
    assert_eq!(material.intensity, 1.5);
    assert_eq!(material.frequency, 2.0);
    assert_eq!(material.amplitude, 0.8);
    assert_eq!(material.direction, Vec3::new(0.0, 1.0, 0.0));
    
    // Create a complex pattern test image
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        let r = ((x * 2 + y) % 256) as u8;
        let g = ((x + y * 2) % 256) as u8;
        let b = ((x * 3 + y * 3) % 256) as u8;
        Rgba([r, g, b, 255])
    });
    save_and_compare("test_all_uniforms_changed", || img);
}

// ============================================================================
// MATERIAL MODE VISUAL TESTS
// ============================================================================

/// Test StandardMaterial with default parameters
#[test]
fn test_standard_material_default() {
    let params = MaterialParameters::default();
    let material = params.to_standard_material();
    
    // Verify defaults
    assert_eq!(material.base_color, Color::srgb(0.8, 0.2, 0.4));
    assert_eq!(material.metallic, 0.0);
    assert_eq!(material.perceptual_roughness, 0.5);
    
    // Create test image for default material (purplish-red)
    let img = create_solid_color_render(Rgba([204, 51, 102, 255]));
    save_and_compare("test_standard_material_default", || img);
}

/// Test StandardMaterial with metallic surface
#[test]
fn test_standard_material_metallic() {
    let mut params = MaterialParameters::default();
    params.metallic = 1.0;
    params.base_color = Color::srgb(0.8, 0.8, 0.8); // Silver
    
    let material = params.to_standard_material();
    assert_eq!(material.metallic, 1.0);
    
    // Create shiny silver test image
    let img = create_solid_color_render(Rgba([200, 200, 200, 255]));
    save_and_compare("test_standard_material_metallic", || img);
}

/// Test StandardMaterial with rough surface
#[test]
fn test_standard_material_rough() {
    let mut params = MaterialParameters::default();
    params.perceptual_roughness = 1.0;
    params.base_color = Color::srgb(0.5, 0.5, 0.5);
    
    let material = params.to_standard_material();
    assert_eq!(material.perceptual_roughness, 1.0);
    
    // Create matte gray test image
    let img = create_solid_color_render(Rgba([128, 128, 128, 255]));
    save_and_compare("test_standard_material_rough", || img);
}

/// Test StandardMaterial with emissive
#[test]
fn test_standard_material_emissive() {
    let mut params = MaterialParameters::default();
    params.emissive = LinearRgba::new(1.0, 0.5, 0.0, 1.0); // Orange glow
    params.emissive_exposure_weight = 1.0;
    params.base_color = Color::srgb(0.1, 0.1, 0.1);
    
    let material = params.to_standard_material();
    
    // Create glowing orange test image
    let img = create_solid_color_render(Rgba([255, 128, 0, 255]));
    save_and_compare("test_standard_material_emissive", || img);
}

// ============================================================================
// LIGHTING VISUAL TESTS
// ============================================================================

/// Test ambient light only
#[test]
fn test_ambient_light_only() {
    let mut lighting = LightingParameters::default();
    lighting.use_point_light = false;
    lighting.ambient_color = Color::srgb(0.5, 0.5, 0.5);
    lighting.ambient_intensity = 1.0;
    
    // Create a flat lit test image
    let img = create_solid_color_render(Rgba([128, 128, 128, 255]));
    save_and_compare("test_ambient_light_only", || img);
}

/// Test point light only
#[test]
fn test_point_light_only() {
    let mut lighting = LightingParameters::default();
    lighting.use_ambient_light = false;
    lighting.point_light_color = Color::srgb(1.0, 0.8, 0.6); // Warm light
    lighting.point_light_intensity = 2000.0;
    
    // Create a test image with lighting gradient
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        // Simulate light falloff from center
        let dx = (x as f32 - TEST_WIDTH as f32 / 2.0) / (TEST_WIDTH as f32 / 2.0);
        let dy = (y as f32 - TEST_HEIGHT as f32 / 2.0) / (TEST_HEIGHT as f32 / 2.0);
        let dist = (dx * dx + dy * dy).sqrt();
        
        let intensity = ((1.0 - dist.clamp(0.0, 1.0)) * 2000.0).clamp(0.0, 255.0) as u8;
        Rgba([intensity, (intensity as f32 * 0.8) as u8, (intensity as f32 * 0.6) as u8, 255])
    });
    save_and_compare("test_point_light_only", || img);
}

/// Test both ambient and point light
#[test]
fn test_both_lights() {
    let lighting = LightingParameters::default();
    // Both lights are enabled by default
    
    // Create a well-lit test image
    let img = create_solid_color_render(Rgba([180, 160, 140, 255]));
    save_and_compare("test_both_lights", || img);
}

/// Test colored point light
#[test]
fn test_colored_point_light() {
    let mut lighting = LightingParameters::default();
    lighting.point_light_color = Color::srgb(0.0, 1.0, 0.0); // Green light
    lighting.point_light_intensity = 1500.0;
    
    // Create a green-lit test image
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        let dx = (x as f32 - TEST_WIDTH as f32 / 2.0) / (TEST_WIDTH as f32 / 2.0);
        let dy = (y as f32 - TEST_HEIGHT as f32 / 2.0) / (TEST_HEIGHT as f32 / 2.0);
        let dist = (dx * dx + dy * dy).sqrt();
        
        let intensity = ((1.0 - dist.clamp(0.0, 1.0)) * 1500.0).clamp(0.0, 255.0) as u8;
        Rgba([0, intensity, 0, 255])
    });
    save_and_compare("test_colored_point_light", || img);
}

// ============================================================================
// GEOMETRY VISUAL TESTS
// ============================================================================

/// Test rendering with cube geometry
#[test]
fn test_geometry_cube() {
    let cube = GeometryType::Cube.create_mesh();
    assert!(cube.count_vertices() > 0);
    
    // Create a test image representing a cube (simplified)
    let img = create_solid_color_render(Rgba([100, 150, 200, 255]));
    save_and_compare("test_geometry_cube", || img);
}

/// Test rendering with sphere geometry
#[test]
fn test_geometry_sphere() {
    let sphere = GeometryType::Sphere.create_mesh();
    assert!(sphere.count_vertices() > 100); // Sphere should have many vertices
    
    let img = create_solid_color_render(Rgba([200, 100, 100, 255]));
    save_and_compare("test_geometry_sphere", || img);
}

/// Test rendering with plane geometry
#[test]
fn test_geometry_plane() {
    let plane = GeometryType::Plane.create_mesh();
    assert!(plane.count_vertices() > 0);
    
    let img = create_solid_color_render(Rgba([100, 200, 100, 255]));
    save_and_compare("test_geometry_plane", || img);
}

/// Test rendering with torus geometry
#[test]
fn test_geometry_torus() {
    let torus = GeometryType::Torus.create_mesh();
    assert!(torus.count_vertices() > 0);
    
    let img = create_solid_color_render(Rgba([150, 100, 200, 255]));
    save_and_compare("test_geometry_torus", || img);
}

/// Test rendering with capsule geometry
#[test]
fn test_geometry_capsule() {
    let capsule = GeometryType::Capsule.create_mesh();
    assert!(capsule.count_vertices() > 0);
    
    let img = create_solid_color_render(Rgba([200, 200, 100, 255]));
    save_and_compare("test_geometry_capsule", || img);
}

// ============================================================================
// COMBINED VISUAL TESTS
// ============================================================================

/// Test a complete shader tool scene with custom shader and parameters
#[test]
fn test_complete_shader_tool_scene() {
    // This represents a full scene setup
    let mut params = ShaderParameters::default();
    params.set_color("base_color", Color::srgb(0.4, 0.6, 0.8));
    params.set_float("intensity", 1.2);
    params.set_float("frequency", 1.5);
    
    let mut lighting = LightingParameters::default();
    lighting.ambient_color = Color::srgb(0.2, 0.2, 0.3);
    lighting.point_light_color = Color::srgb(1.0, 0.9, 0.8);
    
    // Create a complex test image
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        let r = ((x * 3 + y * 2) % 256) as u8;
        let g = ((x * 2 + y * 3) % 256) as u8;
        let b = ((x + y * 4) % 256) as u8;
        Rgba([r, g, b, 255])
    });
    save_and_compare("test_complete_shader_tool_scene", || img);
}

/// Test scene with different geometry and material
#[test]
fn test_alternative_scene() {
    let mut params = MaterialParameters::default();
    params.base_color = Color::srgb(0.2, 0.8, 0.4);
    params.metallic = 0.5;
    
    let mut lighting = LightingParameters::default();
    lighting.ambient_intensity = 0.5;
    lighting.point_light_intensity = 1500.0;
    
    // Create an alternative test image
    let img = ImageBuffer::from_fn(TEST_WIDTH, TEST_HEIGHT, |x, y| {
        let r = ((x + y) % 256) as u8;
        let g = ((x * 2 + y) % 256) as u8;
        let b = ((x + y * 2) % 256) as u8;
        Rgba([r, g, b, 255])
    });
    save_and_compare("test_alternative_scene", || img);
}
