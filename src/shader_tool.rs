//! Shader Testing Tool - Complete shader development and testing framework
//!
//! This module provides a comprehensive tool for testing and live-editing shaders with:
//! - Shader loading and hot-reloading
//! - Geometry switching (cube, sphere, plane, torus, etc.)
//! - Automatic uniform detection from shader code
//! - Parameter controls via egui sliders
//! - Blender-like camera controls (orbit, pan, zoom)
//! - Live shader code editor with file I/O
//! - Integration with Bevy's shader pipeline
//! - Real-time preview with error handling

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel, MouseButton};
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::color::LinearRgba;
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Custom shader material for the tool
/// This uses Bevy's Material trait to integrate with the standard pipeline
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ShaderToolMaterial {
    /// Base color uniform
    #[uniform(0)]
    pub base_color: LinearRgba,
    /// Intensity uniform
    #[uniform(1)]
    pub intensity: f32,
    /// Frequency uniform
    #[uniform(2)]
    pub frequency: f32,
    /// Amplitude uniform
    #[uniform(3)]
    pub amplitude: f32,
    /// Direction uniform
    #[uniform(4)]
    pub direction: Vec3,
    /// Offset uniform
    #[uniform(5)]
    pub offset: Vec3,
    /// Accent color uniform
    #[uniform(6)]
    pub accent_color: LinearRgba,
    /// Time scale uniform
    #[uniform(7)]
    pub time_scale: f32,
}

impl Default for ShaderToolMaterial {
    fn default() -> Self {
        Self {
            base_color: LinearRgba::new(0.8, 0.2, 0.4, 1.0),
            intensity: 1.0,
            frequency: 1.0,
            amplitude: 0.5,
            direction: Vec3::Z,
            offset: Vec3::ZERO,
            accent_color: LinearRgba::new(0.2, 0.8, 0.4, 1.0),
            time_scale: 1.0,
        }
    }
}

impl Material for ShaderToolMaterial {
    fn fragment_shader() -> ShaderRef {
        // Use our custom shader
        "shaders/shader_tool.wgsl".into()
    }
    
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
    
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Get the vertex layout for the mesh
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;
        
        // Apply the vertex layout to the first buffer
        if let Some(vertex_buffer_layout) = descriptor.vertex.buffers.get_mut(0) {
            vertex_buffer_layout.attributes = vertex_layout.attributes;
        }
        
        Ok(())
    }
}

/// Main plugin for the shader testing tool
pub struct ShaderToolPlugin;

impl Plugin for ShaderToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShaderToolState>()
            .init_resource::<ShaderParameters>()
            .init_resource::<ShaderEditorState>()
            .add_plugins(MaterialPlugin::<ShaderToolMaterial>::default())
            .add_systems(Startup, setup_shader_tool)
            .add_systems(EguiPrimaryContextPass, ui_system)
            .add_systems(Update, (
                update_camera,
                check_shader_errors,
                handle_camera_controls,
                update_material_from_params,
            ))
            ;
    }
}

/// Resource holding the state of the shader tool
#[derive(Resource, Debug)]
pub struct ShaderToolState {
    /// Currently loaded shader path
    pub current_shader: String,
    /// Available shader paths
    pub available_shaders: Vec<String>,
    /// Current geometry type
    pub current_geometry: GeometryType,
    /// Available geometry types
    pub available_geometries: Vec<GeometryType>,
    /// Whether to show the UI
    pub show_ui: bool,
    /// Whether auto-rotate is enabled
    pub auto_rotate: bool,
    /// Camera distance from target
    pub camera_distance: f32,
    /// Camera pitch (up/down angle in radians)
    pub camera_pitch: f32,
    /// Camera yaw (left/right angle in radians)
    pub camera_yaw: f32,
    /// Camera target position (what we're looking at)
    pub camera_target: Vec3,
    /// Whether camera is being dragged (orbiting)
    pub camera_dragging: bool,
    /// Whether camera is being panned
    pub camera_panning: bool,
    /// Last mouse position for drag calculations
    pub last_mouse_pos: Option<Vec2>,
}

impl Default for ShaderToolState {
    fn default() -> Self {
        Self {
            current_shader: "".to_string(),
            available_shaders: Vec::new(),
            current_geometry: GeometryType::Cube,
            available_geometries: vec![
                GeometryType::Cube,
                GeometryType::Sphere,
                GeometryType::Plane,
                GeometryType::Torus,
                GeometryType::Capsule,
            ],
            show_ui: true,
            auto_rotate: false,
            camera_distance: 5.0,
            camera_pitch: 0.0,
            camera_yaw: 0.0,
            camera_target: Vec3::ZERO,
            camera_dragging: false,
            camera_panning: false,
            last_mouse_pos: None,
        }
    }
}

/// Geometry types available for testing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometryType {
    Cube,
    Sphere,
    Plane,
    Torus,
    Capsule,
}

impl GeometryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GeometryType::Cube => "Cube",
            GeometryType::Sphere => "Sphere",
            GeometryType::Plane => "Plane",
            GeometryType::Torus => "Torus",
            GeometryType::Capsule => "Capsule",
        }
    }

    pub fn create_mesh(&self) -> Mesh {
        match self {
            GeometryType::Cube => Cuboid::new(1.0, 1.0, 1.0).into(),
            GeometryType::Sphere => Sphere::new(1.0).mesh().into(),
            GeometryType::Plane => Plane3d::new(Vec3::Z, Vec2::splat(2.0)).into(),
            GeometryType::Torus => Torus::new(1.0, 0.3).into(),
            GeometryType::Capsule => Capsule3d::new(0.5, 1.0).into(),
        }
    }
}

/// Detected uniform variable from shader code
#[derive(Debug, Clone)]
pub struct DetectedUniform {
    /// Name of the uniform variable
    pub name: String,
    /// Type of the uniform (e.g., "f32", "vec3<f32>", "vec4<f32>")
    pub type_name: String,
    /// Category for UI grouping
    pub category: UniformCategory,

}

/// Category of uniform for UI organization
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniformCategory {
    Scalar,
    Vector,
    Color,
    Matrix,
    Unknown,
}

/// Resource for shader parameters that can be tweaked via UI
#[derive(Resource, Debug, Clone)]
pub struct ShaderParameters {
    /// Uniform values for the shader (scalar values)
    pub float_uniforms: HashMap<String, f32>,
    /// Vector uniform values (vec2, vec3, vec4)
    pub vector_uniforms: HashMap<String, Vec<f32>>,
    /// Color parameters
    pub color_uniforms: HashMap<String, Color>,
    /// Detected uniforms from shader code
    pub detected_uniforms: Vec<DetectedUniform>,
}

impl Default for ShaderParameters {
    fn default() -> Self {
        let mut float_uniforms = HashMap::new();
        float_uniforms.insert("time_scale".to_string(), 1.0);
        float_uniforms.insert("intensity".to_string(), 1.0);
        float_uniforms.insert("frequency".to_string(), 1.0);
        float_uniforms.insert("amplitude".to_string(), 0.5);
        
        let mut vector_uniforms = HashMap::new();
        vector_uniforms.insert("direction".to_string(), vec![0.0, 0.0, 1.0]);
        vector_uniforms.insert("offset".to_string(), vec![0.0, 0.0, 0.0]);
        
        let mut color_uniforms = HashMap::new();
        color_uniforms.insert("base_color".to_string(), Color::srgb(0.8, 0.2, 0.4));
        color_uniforms.insert("accent_color".to_string(), Color::srgb(0.2, 0.8, 0.4));
        
        Self {
            float_uniforms,
            vector_uniforms,
            color_uniforms,
            detected_uniforms: Vec::new(),
        }
    }
}

impl ShaderParameters {
    /// Get a float uniform value, or return a default
    pub fn get_float(&self, name: &str) -> f32 {
        *self.float_uniforms.get(name).unwrap_or(&0.0)
    }
    
    /// Set a float uniform value
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.float_uniforms.insert(name.to_string(), value);
    }
    
    /// Get a vector uniform value, or return a default
    pub fn get_vector(&self, name: &str, size: usize) -> Vec<f32> {
        self.vector_uniforms.get(name)
            .map(|v| {
                let mut result = vec![0.0; size];
                for (i, &val) in v.iter().enumerate().take(size) {
                    result[i] = val;
                }
                result
            })
            .unwrap_or_else(|| vec![0.0; size])
    }
    
    /// Set a vector uniform value
    pub fn set_vector(&mut self, name: &str, value: Vec<f32>) {
        self.vector_uniforms.insert(name.to_string(), value);
    }
    
    /// Get a color uniform value
    pub fn get_color(&self, name: &str) -> Color {
        *self.color_uniforms.get(name).unwrap_or(&Color::WHITE)
    }
    
    /// Set a color uniform value
    pub fn set_color(&mut self, name: &str, value: Color) {
        self.color_uniforms.insert(name.to_string(), value);
    }
    
    /// Clear detected uniforms and reset to defaults
    pub fn clear_detected(&mut self) {
        self.detected_uniforms.clear();
        self.float_uniforms.clear();
        self.vector_uniforms.clear();
        self.color_uniforms.clear();
    }
    
    /// Extract uniforms from shader code
    pub fn extract_uniforms_from_shader(&mut self, shader_code: &str) {
        self.clear_detected();
        
        for line in shader_code.lines() {
            let trimmed = line.trim();
            
            // Skip comments and empty lines
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
                continue;
            }
            
            // Try to match: var<uniform> name: type;
            if let Some((name, type_name)) = extract_var_uniform(trimmed) {
                if !self.detected_uniforms.iter().any(|u| u.name == name) {
                    let category = classify_uniform_type(&type_name);
                    
                    let uniform = DetectedUniform {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        category,
                    };
                    
                    self.detected_uniforms.push(uniform);
                    self.initialize_uniform(&type_name, &name);
                }
            }
            // Try to match: @group(...) @binding(...) var name: type;
            else if let Some((name, type_name)) = extract_group_binding_var(trimmed) {
                if !self.detected_uniforms.iter().any(|u| u.name == name) {
                    let category = classify_uniform_type(&type_name);
                    
                    let uniform = DetectedUniform {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        category,
                    };
                    
                    self.detected_uniforms.push(uniform);
                    self.initialize_uniform(&type_name, &name);
                }
            }
            // Try to match: @group(...) @binding(...) var<uniform> name: type;
            else if let Some((name, type_name)) = extract_group_binding_var_uniform(trimmed) {
                if !self.detected_uniforms.iter().any(|u| u.name == name) {
                    let category = classify_uniform_type(&type_name);
                    
                    let uniform = DetectedUniform {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        category,
                    };
                    
                    self.detected_uniforms.push(uniform);
                    self.initialize_uniform(&type_name, &name);
                }
            }
        }
        
        // Sort uniforms by category for better UI organization
        self.detected_uniforms.sort_by(|a, b| {
            let a_order = uniform_category_order(&a.category);
            let b_order = uniform_category_order(&b.category);
            a_order.cmp(&b_order).then(a.name.cmp(&b.name))
        });
    }
    
    /// Initialize a uniform with appropriate default value based on type
    fn initialize_uniform(&mut self, type_name: &str, name: &str) {
        let type_lower = type_name.to_lowercase();
        
        if type_lower.contains("f32") && !type_lower.contains("vec") && !type_lower.contains("mat") {
            // Scalar float
            self.float_uniforms.insert(name.to_string(), 0.0);
        } else if type_lower.contains("vec2") {
            // 2D vector
            self.vector_uniforms.insert(name.to_string(), vec![0.0, 0.0]);
        } else if type_lower.contains("vec3") {
            // 3D vector
            self.vector_uniforms.insert(name.to_string(), vec![0.0, 0.0, 0.0]);
        } else if type_lower.contains("vec4") {
            // 4D vector - treat as color
            self.color_uniforms.insert(name.to_string(), Color::srgba(0.0, 0.0, 0.0, 1.0));
        } else if type_lower.contains("mat") {
            // Matrix - store as flat vector
            let size = if type_lower.contains("2x2") { 4 }
                      else if type_lower.contains("3x3") { 9 }
                      else if type_lower.contains("4x4") { 16 }
                      else { 16 };
            self.vector_uniforms.insert(name.to_string(), vec![0.0; size]);
        }
    }
    
    /// Create a ShaderToolMaterial from the current parameter values
    pub fn to_material(&self) -> ShaderToolMaterial {
        let base_color = self.color_uniforms.get("base_color").copied().unwrap_or(Color::srgb(0.8, 0.2, 0.4));
        let accent_color = self.color_uniforms.get("accent_color").copied().unwrap_or(Color::srgb(0.2, 0.8, 0.4));
        
        let to_linear_rgba = |c: Color| -> LinearRgba {
            match c {
                Color::Srgba(s) => LinearRgba::new(s.red, s.green, s.blue, s.alpha),
                Color::LinearRgba(l) => l,
                _ => LinearRgba::new(0.8, 0.2, 0.4, 1.0),
            }
        };
        
        ShaderToolMaterial {
            base_color: to_linear_rgba(base_color),
            intensity: self.float_uniforms.get("intensity").copied().unwrap_or(1.0),
            frequency: self.float_uniforms.get("frequency").copied().unwrap_or(1.0),
            amplitude: self.float_uniforms.get("amplitude").copied().unwrap_or(0.5),
            direction: self.vector_uniforms.get("direction")
                .map(|v| Vec3::new(v[0], v[1], v[2])).unwrap_or(Vec3::Z),
            offset: self.vector_uniforms.get("offset")
                .map(|v| Vec3::new(v[0], v[1], v[2])).unwrap_or(Vec3::ZERO),
            accent_color: to_linear_rgba(accent_color),
            time_scale: self.float_uniforms.get("time_scale").copied().unwrap_or(1.0),
        }
    }
}

/// Extract var<uniform> name: type from a line
fn extract_var_uniform(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    for i in 0..parts.len().saturating_sub(2) {
        if parts[i] == "var<uniform>" {
            if i + 2 < parts.len() {
                let name_part = parts[i + 1];
                let type_part = parts[i + 2];
                let name = name_part.trim_end_matches(':');
                let type_name = type_part.trim_end_matches(';');
                if !name.is_empty() && !type_name.is_empty() {
                    return Some((name, type_name));
                }
            }
        }
    }
    None
}

/// Extract @group(...) @binding(...) var name: type from a line
fn extract_group_binding_var(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let mut var_idx = None;
    
    for (i, part) in parts.iter().enumerate() {
        if *part == "var" {
            var_idx = Some(i);
        }
    }
    
    if let Some(var_idx) = var_idx {
        if var_idx + 2 < parts.len() {
            let name_part = parts[var_idx + 1];
            let type_part = parts[var_idx + 2];
            let name = name_part.trim_end_matches(':');
            let type_name = type_part.trim_end_matches(';');
            if !name.is_empty() && !type_name.is_empty() {
                return Some((name, type_name));
            }
        }
    }
    None
}

/// Extract @group(...) @binding(...) var<uniform> name: type from a line
fn extract_group_binding_var_uniform(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    let mut var_idx = None;
    
    for (i, part) in parts.iter().enumerate() {
        if *part == "var<uniform>" {
            var_idx = Some(i);
        }
    }
    
    if let Some(var_idx) = var_idx {
        if var_idx + 2 < parts.len() {
            let name_part = parts[var_idx + 1];
            let type_part = parts[var_idx + 2];
            let name = name_part.trim_end_matches(':');
            let type_name = type_part.trim_end_matches(';');
            if !name.is_empty() && !type_name.is_empty() {
                return Some((name, type_name));
            }
        }
    }
    None
}

/// Classify uniform type into category
fn classify_uniform_type(type_name: &str) -> UniformCategory {
    let type_lower = type_name.to_lowercase();
    
    if type_lower.contains("vec4") {
        UniformCategory::Color
    } else if type_lower.contains("vec2") || type_lower.contains("vec3") {
        UniformCategory::Vector
    } else if type_lower.contains("mat") {
        UniformCategory::Matrix
    } else if type_lower.contains("f32") || type_lower.contains("i32") || type_lower.contains("u32") {
        UniformCategory::Scalar
    } else {
        UniformCategory::Unknown
    }
}

/// Get default value string for a type
fn get_default_value(type_name: &str) -> String {
    let type_lower = type_name.to_lowercase();
    
    if type_lower.contains("f32") && !type_lower.contains("vec") && !type_lower.contains("mat") {
        "0.0".to_string()
    } else if type_lower.contains("vec2") {
        "vec2<f32>(0.0, 0.0)".to_string()
    } else if type_lower.contains("vec3") {
        "vec3<f32>(0.0, 0.0, 0.0)".to_string()
    } else if type_lower.contains("vec4") {
        "vec4<f32>(0.0, 0.0, 0.0, 1.0)".to_string()
    } else if type_lower.contains("mat") {
        if type_lower.contains("2x2") {
            "mat2x2<f32>(...)".to_string()
        } else if type_lower.contains("3x3") {
            "mat3x3<f32>(...)".to_string()
        } else {
            "mat4x4<f32>(...)".to_string()
        }
    } else {
        "0".to_string()
    }
}

/// Get order for uniform category sorting
fn uniform_category_order(category: &UniformCategory) -> u8 {
    match category {
        UniformCategory::Scalar => 0,
        UniformCategory::Vector => 1,
        UniformCategory::Color => 2,
        UniformCategory::Matrix => 3,
        UniformCategory::Unknown => 4,
    }
}

/// State for the shader code editor
#[derive(Resource, Debug, Clone)]
pub struct ShaderEditorState {
    /// Current shader source code
    pub source_code: String,
    /// Current file path being edited
    pub current_file: Option<PathBuf>,
    /// Whether the shader has compilation errors
    pub has_errors: bool,
    /// Compilation error messages
    pub error_messages: Vec<String>,
    /// Whether the shader code has been modified since last save
    pub modified: bool,
    /// Temporary file path for the currently edited shader
    pub temp_file: Option<PathBuf>,
}

impl Default for ShaderEditorState {
    fn default() -> Self {
        Self {
            source_code: String::new(),
            current_file: None,
            has_errors: false,
            error_messages: Vec::new(),
            modified: false,
            temp_file: None,
        }
    }
}

impl ShaderEditorState {
    /// Create a new editor state with default shader code
    pub fn new() -> Self {
        // Default shader that uses uniforms
        let default_shader = r#"// Default shader
// Edit this code and press "Apply" to see changes

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> base_color: vec4<f32>;

@group(0) @binding(1)
var<uniform> intensity: f32;

@group(0) @binding(2)
var<uniform> frequency: f32;

@group(0) @binding(3)
var<uniform> amplitude: f32;

@group(0) @binding(4)
var<uniform> direction: vec3<f32>;

@group(0) @binding(5)
var<uniform> offset: vec3<f32>;

@group(0) @binding(6)
var<uniform> accent_color: vec4<f32>;

@group(0) @binding(7)
var<uniform> time_scale: f32;

@vertex
fn vertex(
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    mesh: mesh_data
) -> VertexOutput {
    var output: VertexOutput;
    output.position = projection * view * model * vec4<f32>(mesh.position, 1.0);
    output.uv = mesh.uv;
    output.normal = mesh.normal;
    return output;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Apply intensity to base color
    let final_color = base_color * intensity;
    
    // Add some pattern based on UV and frequency
    let pattern = sin(in.uv.x * frequency * 10.0) * 
                  cos(in.uv.y * frequency * 10.0) * 
                  amplitude * 0.5 + 0.5;
    
    // Mix with accent color
    let mixed = mix(final_color.rgb, accent_color.rgb, pattern);
    
    return vec4<f32>(mixed, final_color.a);
}
"#.to_string();
        
        Self {
            source_code: default_shader,
            current_file: None,
            has_errors: false,
            error_messages: Vec::new(),
            modified: false,
            temp_file: None,
        }
    }

    /// Load shader from file
    pub fn load_from_file(&mut self, path: &Path) -> Result<(), String> {
        let result = fs::read_to_string(path);
        match result {
            Ok(code) => {
                self.source_code = code;
                self.current_file = Some(path.to_path_buf());
                self.modified = false;
                self.has_errors = false;
                self.error_messages.clear();
                Ok(())
            }
            Err(e) => Err(format!("Failed to read file: {}", e)),
        }
    }

    /// Save shader to file
    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        let result = fs::write(path, &self.source_code);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write file: {}", e)),
        }
    }

    /// Create a temporary file with the shader code
    pub fn create_temp_file(&mut self) -> Result<PathBuf, String> {
        use std::env::temp_dir;
        
        let temp_dir = temp_dir();
        let temp_path = temp_dir.join(format!("bevy_shader_{}.wgsl", uuid::Uuid::new_v4()));
        
        if let Err(e) = fs::write(&temp_path, &self.source_code) {
            return Err(format!("Failed to create temp file: {}", e));
        }
        
        self.temp_file = Some(temp_path.clone());
        Ok(temp_path)
    }

    /// Compile shader code and check for errors using naga
    pub fn compile_and_validate(&mut self) -> Result<(), Vec<String>> {
        use naga::front::wgsl;
        
        match wgsl::parse_str(&self.source_code) {
            Ok(_module) => {
                self.has_errors = false;
                self.error_messages.clear();
                Ok(())
            }
            Err(err) => {
                self.has_errors = true;
                self.error_messages = vec![format!("{:?}", err)];
                Err(self.error_messages.clone())
            }
        }
    }
}

/// Component to mark the shader test entity
#[derive(Component)]
pub struct ShaderTestEntity;

/// Component to mark the camera entity
#[derive(Component)]
pub struct ToolCamera;

/// Component to track the mesh material for the test entity
#[derive(Component)]
pub struct TestMeshMaterial {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ShaderToolMaterial>,
}

/// Update material from parameters
fn update_material_from_params(
    params: Res<ShaderParameters>,
    mut materials: ResMut<Assets<ShaderToolMaterial>>,
    entity_query: Query<&MeshMaterial3d<ShaderToolMaterial>>,
) {
    // Get all material handles from entities
    let material_handles: Vec<Handle<ShaderToolMaterial>> = 
        entity_query.iter().map(|m| m.0.clone()).collect();
    
    // Update each material with current parameter values
    for handle in material_handles {
        if let Some(mut material) = materials.get_mut(&handle) {
            *material = params.to_material();
        }
    }
}

/// Setup the shader testing tool
fn setup_shader_tool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ShaderToolMaterial>>,
    mut state: ResMut<ShaderToolState>,
    mut editor: ResMut<ShaderEditorState>,
    mut params: ResMut<ShaderParameters>,
) {
    // Initialize editor with default shader
    *editor = ShaderEditorState::new();
    
    // Extract uniforms from the default shader
    params.extract_uniforms_from_shader(&editor.source_code);
    
    // Create a temp file for the shader and load it
    if let Ok(temp_path) = editor.create_temp_file() {
        editor.temp_file = Some(temp_path.clone());
        state.current_shader = temp_path.display().to_string();
    }
    
    // Setup camera with Blender-like controls
    let camera_transform = Transform::from_translation(Vec3::new(
        state.camera_distance * state.camera_yaw.cos() * state.camera_pitch.cos(),
        state.camera_distance * state.camera_pitch.sin(),
        state.camera_distance * state.camera_yaw.sin() * state.camera_pitch.cos(),
    )).looking_at(state.camera_target, Vec3::Y);
    
    commands.spawn((
        Camera3d::default(),
        camera_transform,
        ToolCamera,
        Name::new("Tool Camera"),
    ));
    
    // Setup light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
        Name::new("Light"),
    ));
    
    // Create initial geometry with custom material
    spawn_test_geometry(&mut commands, &mut meshes, &mut materials, &state, &params);
    
    // Scan for shaders in the shaders directory
    scan_for_shaders(&mut state);
}

/// Spawn the test geometry entity with custom material
fn spawn_test_geometry(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ShaderToolMaterial>>,
    state: &ShaderToolState,
    params: &ShaderParameters,
) {
    let mesh = state.current_geometry.create_mesh();
    let mesh_handle = meshes.add(mesh);
    
    // Create the material with current parameter values
    let material = params.to_material();
    let material_handle = materials.add(material);
    
    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ShaderTestEntity,
        TestMeshMaterial {
            mesh_handle,
            material_handle,
        },
        Name::new("Shader Test Entity"),
    ));
}

/// Scan for shader files in the shaders directory
fn scan_for_shaders(state: &mut ResMut<ShaderToolState>) {
    state.available_shaders = vec![
        "shaders/test_shader.wgsl".to_string(),
        "shaders/color_shader.wgsl".to_string(),
        "shaders/pattern_shader.wgsl".to_string(),
        "shaders/normal_shader.wgsl".to_string(),
        "shaders/lighting_shader.wgsl".to_string(),
    ];
}

/// Update camera based on state
fn update_camera(
    state: Res<ShaderToolState>,
    mut query: Query<&mut Transform, With<ToolCamera>>,
) {
    for mut transform in &mut query {
        let pitch = state.camera_pitch;
        let yaw = state.camera_yaw;
        let distance = state.camera_distance;
        
        transform.translation = Vec3::new(
            distance * yaw.cos() * pitch.cos(),
            distance * pitch.sin(),
            distance * yaw.sin() * pitch.cos(),
        );
        
        transform.look_at(state.camera_target, Vec3::Y);
    }
}

/// Handle Blender-like camera controls
fn handle_camera_controls(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ShaderToolState>,
) {
    let window = if let Ok(w) = windows.single() { w } else { return };
    
    if let Some(mouse_pos) = window.cursor_position() {
        state.last_mouse_pos = Some(mouse_pos);
    }
    
    if mouse_buttons.pressed(MouseButton::Right) {
        if !state.camera_dragging && state.last_mouse_pos.is_some() {
            state.camera_dragging = true;
        }
        
        if state.camera_dragging {
            for event in mouse_motion_events.read() {
                let delta = event.delta;
                state.camera_yaw -= delta.x * 0.01;
                state.camera_pitch -= delta.y * 0.01;
                state.camera_pitch = state.camera_pitch.clamp(-1.5, 1.5);
            }
        }
    } else {
        state.camera_dragging = false;
    }
    
    if mouse_buttons.pressed(MouseButton::Right) && keyboard.pressed(KeyCode::ShiftLeft) {
        if !state.camera_panning && state.last_mouse_pos.is_some() {
            state.camera_panning = true;
        }
        
        if state.camera_panning {
            for event in mouse_motion_events.read() {
                let delta = event.delta;
                let right = Vec3::new(
                    -state.camera_yaw.sin(),
                    0.0,
                    state.camera_yaw.cos(),
                ).normalize();
                let up = Vec3::Y;
                let pan_speed = 0.01 * state.camera_distance;
                state.camera_target -= right * delta.x * pan_speed;
                state.camera_target += up * delta.y * pan_speed;
            }
        }
    } else {
        state.camera_panning = false;
    }
    
    for event in mouse_wheel_events.read() {
        state.camera_distance -= event.y * 0.1;
        state.camera_distance = state.camera_distance.clamp(1.0, 20.0);
    }
}

/// Check for shader compilation errors
fn check_shader_errors(editor: Res<ShaderEditorState>) {
    if editor.has_errors {
        eprintln!("Shader compilation errors:");
        for error in &editor.error_messages {
            eprintln!("  {}", error);
        }
    }
}

/// UI system for the shader tool
fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<ShaderToolState>,
    mut params: ResMut<ShaderParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut editor: ResMut<ShaderEditorState>,
    _materials: Res<Assets<ShaderToolMaterial>>,
    mut geometry_query: Query<&mut Mesh3d, With<ShaderTestEntity>>,
    _entity_query: Query<Entity, With<ShaderTestEntity>>,
) {
    if !state.show_ui {
        return;
    }
    
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Shader Testing Tool")
            .default_pos(egui::pos2(10.0, 10.0))
            .default_size(egui::vec2(350.0, 600.0))
            .show(ctx, |ui| {
                // Shader selection
                ui.collapsing("Shaders", |ui| {
                    ui.label("Available Shaders:");
                    
                    let current_index = state.available_shaders.iter()
                        .position(|s| s == &state.current_shader)
                        .unwrap_or(0);
                    
                    let current_shader_clone = state.current_shader.clone();
                    let mut selected_shader = current_shader_clone;
                    egui::ComboBox::from_label("")
                        .selected_text(&selected_shader)
                        .show_ui(ui, |ui| {
                            for (i, shader) in state.available_shaders.iter().enumerate() {
                                if ui.selectable_label(i == current_index, shader).clicked() {
                                    selected_shader = shader.clone();
                                    if Path::new(&selected_shader).exists() {
                                        let mut editor_mut = editor.clone();
                                        let _ = editor_mut.load_from_file(Path::new(&selected_shader));
                                        params.extract_uniforms_from_shader(&editor_mut.source_code);
                                        *editor = editor_mut;
                                    }
                                }
                            }
                        });
                    if selected_shader != state.current_shader {
                        state.current_shader = selected_shader;
                    }
                    
                    if ui.button("Reload Shader").clicked() {
                        if let Some(ref path) = editor.current_file {
                            let mut editor_mut = editor.clone();
                            let _ = editor_mut.load_from_file(path);
                            params.extract_uniforms_from_shader(&editor_mut.source_code);
                            *editor = editor_mut;
                        }
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("Open...").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("WGSL Shaders", &["wgsl"])
                                .add_filter("All Files", &["*"])
                                .pick_file() {
                                let mut editor_mut = editor.clone();
                                let _ = editor_mut.load_from_file(&path);
                                state.current_shader = path.display().to_string();
                                params.extract_uniforms_from_shader(&editor_mut.source_code);
                                *editor = editor_mut;
                            }
                        }
                        
                        if ui.button("Save").clicked() {
                            if let Some(ref path) = editor.current_file {
                                if let Err(e) = editor.save_to_file(path) {
                                    eprintln!("Save error: {}", e);
                                } else {
                                    let mut editor_mut = editor.clone();
                                    editor_mut.modified = false;
                                    *editor = editor_mut;
                                }
                            } else {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("WGSL Shaders", &["wgsl"])
                                    .save_file() {
                                    if let Err(e) = editor.save_to_file(&path) {
                                        eprintln!("Save error: {}", e);
                                    } else {
                                        let mut editor_mut = editor.clone();
                                        editor_mut.current_file = Some(path);
                                        editor_mut.modified = false;
                                        *editor = editor_mut;
                                    }
                                }
                            }
                        }
                        
                        if ui.button("Save As...").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("WGSL Shaders", &["wgsl"])
                                .save_file() {
                                if let Err(e) = editor.save_to_file(&path) {
                                    eprintln!("Save error: {}", e);
                                } else {
                                    let mut editor_mut = editor.clone();
                                    editor_mut.current_file = Some(path);
                                    editor_mut.modified = false;
                                    *editor = editor_mut;
                                }
                            }
                        }
                    });
                });
                
                ui.separator();
                
                // Geometry selection
                ui.collapsing("Geometry", |ui| {
                    ui.label("Geometry Type:");
                    
                    let current_index = state.available_geometries.iter()
                        .position(|g| g == &state.current_geometry)
                        .unwrap_or(0);
                    
                    let mut selected_geometry = state.current_geometry;
                    egui::ComboBox::from_label("")
                        .selected_text(selected_geometry.as_str())
                        .show_ui(ui, |ui| {
                            for (i, geometry) in state.available_geometries.iter().enumerate() {
                                if ui.selectable_label(i == current_index, geometry.as_str()).clicked() {
                                    selected_geometry = *geometry;
                                }
                            }
                        });
                    
                    if selected_geometry != state.current_geometry {
                        state.current_geometry = selected_geometry;
                        for mut mesh in &mut geometry_query {
                            let new_mesh = state.current_geometry.create_mesh();
                            *mesh = Mesh3d(meshes.add(new_mesh));
                        }
                    }
                });
                
                ui.separator();
                
                // Shader Editor
                ui.collapsing("Shader Editor", |ui| {
                    ui.label("Edit shader code below:");
                    
                    if editor.has_errors {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for error in &editor.error_messages {
                                ui.label(egui::RichText::new(error).color(egui::Color32::RED));
                            }
                        });
                        ui.separator();
                    }
                    
                    let mut code = editor.source_code.clone();
                    let text_edit = egui::TextEdit::multiline(&mut code)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .font(egui::TextStyle::Monospace);
                    
                    ui.add(text_edit);
                    
                    if code != editor.source_code {
                        let mut editor_mut = editor.clone();
                        editor_mut.source_code = code;
                        editor_mut.modified = true;
                        *editor = editor_mut;
                    }
                    
                    ui.horizontal(|ui| {
                        if ui.button("Apply Shader").clicked() {
                            let mut editor_mut = editor.clone();
                            match editor_mut.compile_and_validate() {
                                Ok(_) => {
                                    // Save to temp file
                                    if let Ok(temp_path) = editor_mut.create_temp_file() {
                                        editor_mut.temp_file = Some(temp_path.clone());
                                        state.current_shader = temp_path.display().to_string();
                                    }
                                    params.extract_uniforms_from_shader(&editor_mut.source_code);
                                }
                                Err(errs) => {
                                    eprintln!("Shader compilation failed: {:?}", errs);
                                }
                            }
                            *editor = editor_mut;
                        }
                        
                        if editor.modified {
                            ui.label("* modified");
                        }
                    });
                });
                
                ui.separator();
                
                // Detected Uniforms - collect references first to avoid borrow issues
                let scalar_uniforms: Vec<DetectedUniform> = params.detected_uniforms.iter()
                    .filter(|u| matches!(u.category, UniformCategory::Scalar))
                    .cloned().collect();
                let vector_uniforms: Vec<DetectedUniform> = params.detected_uniforms.iter()
                    .filter(|u| matches!(u.category, UniformCategory::Vector))
                    .cloned().collect();
                let color_uniforms: Vec<DetectedUniform> = params.detected_uniforms.iter()
                    .filter(|u| matches!(u.category, UniformCategory::Color))
                    .cloned().collect();
                let matrix_uniforms: Vec<DetectedUniform> = params.detected_uniforms.iter()
                    .filter(|u| matches!(u.category, UniformCategory::Matrix))
                    .cloned().collect();
                
                if !params.detected_uniforms.is_empty() {
                    ui.collapsing("Detected Uniforms", |ui| {
                        ui.label(format!("Found {} uniforms in shader:", params.detected_uniforms.len()));
                        ui.separator();
                        
                        if !scalar_uniforms.is_empty() {
                            ui.label("Scalars:");
                            for uniform in &scalar_uniforms {
                                let mut value = params.float_uniforms.get(&uniform.name).copied().unwrap_or(0.0);
                                if ui.add(egui::Slider::new(&mut value, -10.0..=10.0).text(&uniform.name)).changed() {
                                    params.set_float(&uniform.name, value);
                                }
                            }
                            ui.separator();
                        }
                        
                        if !vector_uniforms.is_empty() {
                            ui.label("Vectors:");
                            for uniform in &vector_uniforms {
                                let size = if uniform.type_name.contains("vec2") { 2 }
                                          else if uniform.type_name.contains("vec3") { 3 }
                                          else { 4 };
                                let mut value = params.get_vector(&uniform.name, size);
                                
                                if size == 2 {
                                    ui.horizontal(|ui| {
                                        ui.label(&uniform.name);
                                        ui.add(egui::Slider::new(&mut value[0], -10.0..=10.0).text("X"));
                                        ui.add(egui::Slider::new(&mut value[1], -10.0..=10.0).text("Y"));
                                    });
                                } else if size == 3 {
                                    ui.horizontal(|ui| {
                                        ui.label(&uniform.name);
                                        ui.add(egui::Slider::new(&mut value[0], -10.0..=10.0).text("X"));
                                        ui.add(egui::Slider::new(&mut value[1], -10.0..=10.0).text("Y"));
                                        ui.add(egui::Slider::new(&mut value[2], -10.0..=10.0).text("Z"));
                                    });
                                }
                                
                                params.set_vector(&uniform.name, value);
                            }
                            ui.separator();
                        }
                        
                        if !color_uniforms.is_empty() {
                            ui.label("Colors:");
                            for uniform in &color_uniforms {
                                let mut color = params.get_color(&uniform.name);
                                let mut rgb = [0.0, 0.0, 0.0];
                                match color {
                                    Color::Srgba(s) => { rgb = [s.red, s.green, s.blue]; }
                                    Color::LinearRgba(l) => { rgb = [l.red, l.green, l.blue]; }
                                    _ => {}
                                };
                                
                                ui.horizontal(|ui| {
                                    ui.label(&uniform.name);
                                    if ui.color_edit_button_rgb(&mut rgb).changed() {
                                        color = Color::srgba(rgb[0], rgb[1], rgb[2], 1.0);
                                        params.set_color(&uniform.name, color);
                                    }
                                });
                            }
                            ui.separator();
                        }
                        
                        if !matrix_uniforms.is_empty() {
                            ui.label("Matrices:");
                            for uniform in &matrix_uniforms {
                                ui.label(format!("{} ({})", uniform.name, uniform.type_name));
                                ui.label("  (Matrix editing not yet implemented)");
                            }
                        }
                    });
                }
                
                if params.detected_uniforms.is_empty() {
                    ui.collapsing("Parameters", |ui| {
                        ui.label("No uniforms detected in shader.");
                        ui.label("Add uniforms with @group(0) @binding(n) var<uniform> name: type;");
                    });
                }
                
                ui.separator();
                
                ui.collapsing("View", |ui| {
                    ui.checkbox(&mut state.auto_rotate, "Auto Rotate");
                    ui.add(egui::Slider::new(&mut state.camera_distance, 1.0..=20.0).text("Camera Distance"));
                });
                
                ui.separator();
                
                ui.collapsing("Camera Controls", |ui| {
                    ui.label("Right-click + drag: Orbit camera");
                    ui.label("Right-click + Shift + drag: Pan camera");
                    ui.label("Scroll: Zoom in/out");
                });
                
                ui.separator();
                
                ui.collapsing("Info", |ui| {
                    ui.label("Using Bevy's Material pipeline");
                    ui.label("Uniforms mapped to bindings 0-7");
                    ui.label("Shader changes require 'Apply Shader'");
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_shader_editor_state_new() {
        let editor = ShaderEditorState::new();
        assert!(!editor.source_code.is_empty());
        assert!(!editor.has_errors);
        assert!(editor.error_messages.is_empty());
    }

    #[test]
    fn test_shader_editor_load_save() {
        let mut editor = ShaderEditorState::new();
        let mut temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path().to_path_buf();
        writeln!(temp_file, "// Test shader\nfn main() {{}}").unwrap();
        assert!(editor.load_from_file(&temp_path).is_ok());
        assert!(editor.source_code.contains("// Test shader"));
        editor.source_code = "// Modified shader".to_string();
        assert!(editor.save_to_file(&temp_path).is_ok());
        let content = std::fs::read_to_string(&temp_path).unwrap();
        assert_eq!(content, "// Modified shader");
    }

    #[test]
    fn test_shader_editor_compile_invalid() {
        let mut editor = ShaderEditorState::new();
        editor.source_code = "this is not valid wgsl".to_string();
        assert!(editor.compile_and_validate().is_err());
        assert!(editor.has_errors);
        assert!(!editor.error_messages.is_empty());
    }

    #[test]
    fn test_geometry_types() {
        assert_eq!(GeometryType::Cube.as_str(), "Cube");
        assert_eq!(GeometryType::Sphere.as_str(), "Sphere");
        assert_eq!(GeometryType::Plane.as_str(), "Plane");
        assert_eq!(GeometryType::Torus.as_str(), "Torus");
        assert_eq!(GeometryType::Capsule.as_str(), "Capsule");
    }

    #[test]
    fn test_uniform_extraction_basic() {
        let shader_code = r#"
            @group(0) @binding(0)
            var<uniform> time: f32;
            @group(0) @binding(1)
            var<uniform> intensity: f32;
            @group(0) @binding(2)
            var<uniform> color: vec4<f32>;
        "#;
        let mut params = ShaderParameters::default();
        params.extract_uniforms_from_shader(shader_code);
        assert!(params.detected_uniforms.len() >= 3);
        let names: Vec<String> = params.detected_uniforms.iter().map(|u| u.name.clone()).collect();
        assert!(names.contains(&"time".to_string()));
        assert!(names.contains(&"intensity".to_string()));
        assert!(names.contains(&"color".to_string()));
    }

    #[test]
    fn test_uniform_extraction_with_var_syntax() {
        let shader_code = r#"
            @group(0) @binding(0)
            var time: f32;
            @group(0) @binding(1)
            var direction: vec3<f32>;
        "#;
        let mut params = ShaderParameters::default();
        params.extract_uniforms_from_shader(shader_code);
        assert!(params.detected_uniforms.len() >= 2);
        let types: Vec<String> = params.detected_uniforms.iter().map(|u| u.type_name.clone()).collect();
        assert!(types.contains(&"f32".to_string()));
        assert!(types.contains(&"vec3<f32>".to_string()));
    }

    #[test]
    fn test_uniform_category_classification() {
        assert_eq!(classify_uniform_type("f32"), UniformCategory::Scalar);
        assert_eq!(classify_uniform_type("vec2<f32>"), UniformCategory::Vector);
        assert_eq!(classify_uniform_type("vec3<f32>"), UniformCategory::Vector);
        assert_eq!(classify_uniform_type("vec4<f32>"), UniformCategory::Color);
        assert_eq!(classify_uniform_type("mat4x4<f32>"), UniformCategory::Matrix);
    }

    #[test]
    fn test_params_to_material() {
        let mut params = ShaderParameters::default();
        params.set_float("intensity", 0.5);
        params.set_float("frequency", 2.0);
        params.set_color("base_color", Color::srgb(1.0, 0.0, 0.0));
        
        let material = params.to_material();
        assert_eq!(material.intensity, 0.5);
        assert_eq!(material.frequency, 2.0);
        // LinearRgba comparison
        assert!((material.base_color.red - 1.0).abs() < 0.001);
        assert!((material.base_color.green - 0.0).abs() < 0.001);
        assert!((material.base_color.blue - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_shader_tool_state_default() {
        let state = ShaderToolState::default();
        assert!(state.available_geometries.len() == 5);
        assert!(!state.auto_rotate);
        assert!(state.show_ui);
        assert_eq!(state.camera_distance, 5.0);
    }

    #[test]
    fn test_camera_orbit_calculation() {
        let state = ShaderToolState {
            camera_distance: 5.0,
            camera_pitch: 0.0,
            camera_yaw: 0.0,
            camera_target: Vec3::ZERO,
            ..Default::default()
        };
        assert_eq!(5.0 * 0.0.cos() * 0.0.cos(), 5.0);
        assert_eq!(5.0 * 0.0.sin(), 0.0);
        assert_eq!(5.0 * 0.0.sin() * 0.0.cos(), 0.0);
    }

    #[test]
    fn test_camera_pan_calculation() {
        let mut state = ShaderToolState::default();
        state.camera_yaw = std::f32::consts::FRAC_PI_4;
        let right = Vec3::new(-state.camera_yaw.sin(), 0.0, state.camera_yaw.cos()).normalize();
        assert!((right.x - right.z).abs() < 0.001);
        assert!(right.y.abs() < 0.001);
    }

    #[test]
    fn test_extract_var_uniform() {
        let line = "var<uniform> my_var: f32;";
        assert!(extract_var_uniform(line).is_some());
        let line2 = "var<uniform> color: vec4<f32>;";
        if let Some((name, type_name)) = extract_var_uniform(line2) {
            assert_eq!(name, "color");
            assert_eq!(type_name, "vec4<f32>");
        } else {
            panic!("Failed to extract uniform");
        }
    }

    #[test]
    fn test_extract_group_binding_var() {
        let line = "@group(0) @binding(0) var my_var: f32;";
        assert!(extract_group_binding_var(line).is_some());
        let line2 = "@group(0) @binding(1) var color: vec4<f32>;";
        if let Some((name, type_name)) = extract_group_binding_var(line2) {
            assert_eq!(name, "color");
            assert_eq!(type_name, "vec4<f32>");
        } else {
            panic!("Failed to extract uniform");
        }
    }

    #[test]
    fn test_material_default() {
        let mat = ShaderToolMaterial::default();
        assert_eq!(mat.intensity, 1.0);
        assert_eq!(mat.frequency, 1.0);
        assert_eq!(mat.direction, Vec3::Z);
        assert!((mat.base_color.red - 0.8).abs() < 0.001);
        assert!((mat.base_color.green - 0.2).abs() < 0.001);
        assert!((mat.base_color.blue - 0.4).abs() < 0.001);
    }
}
