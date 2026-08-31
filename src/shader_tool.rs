//! Material & Shader Testing Tool
//!
//! A comprehensive tool for testing and previewing:
//! - Bevy's built-in StandardMaterial with live parameter editing
//! - Custom shaders with live editing and hot-reloading
//!
//! Features:
//! - Geometry switching (cube, sphere, plane, torus, capsule)
//! - Material parameter controls via egui sliders
//! - Blender-like camera controls (orbit, pan, zoom)
//! - Live shader code editor with file I/O

use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel, MouseButton};
use bevy::pbr::MaterialPipeline;
use bevy::pbr::MaterialPipelineKey;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::color::{LinearRgba, Color};
use bevy_egui::{egui, EguiContexts, EguiPrimaryContextPass};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

/// Tool mode - either testing materials or shaders
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolMode {
    Material,
    Shader,
}

impl ToolMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToolMode::Material => "Material",
            ToolMode::Shader => "Shader",
        }
    }
}

impl Default for ToolMode {
    fn default() -> Self {
        ToolMode::Material
    }
}

/// Custom shader material for the shader tool mode
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ShaderToolMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub intensity: f32,
    #[uniform(2)]
    pub frequency: f32,
    #[uniform(3)]
    pub amplitude: f32,
    #[uniform(4)]
    pub direction: Vec3,
    #[uniform(5)]
    pub offset: Vec3,
    #[uniform(6)]
    pub accent_color: LinearRgba,
    #[uniform(7)]
    pub time_scale: f32,
    #[uniform(8)]
    pub ambient_color: LinearRgba,
    #[uniform(9)]
    pub ambient_intensity: f32,
    #[uniform(10)]
    pub point_light_position: Vec3,
    #[uniform(11)]
    pub point_light_color: LinearRgba,
    #[uniform(12)]
    pub point_light_intensity: f32,
    #[uniform(13)]
    pub point_light_radius: f32,
    #[uniform(14)]
    pub use_point_light: u32,
    #[uniform(15)]
    pub use_ambient_light: u32,
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
            ambient_color: LinearRgba::new(0.1, 0.1, 0.1, 1.0),
            ambient_intensity: 1.0,
            point_light_position: Vec3::new(2.0, 2.0, 2.0),
            point_light_color: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
            point_light_intensity: 1000.0,
            point_light_radius: 10.0,
            use_point_light: 1,
            use_ambient_light: 1,
        }
    }
}

impl Material for ShaderToolMaterial {
    fn fragment_shader() -> ShaderRef {
        "assets/shaders/current.wgsl".into()
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
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;
        
        if let Some(vertex_buffer_layout) = descriptor.vertex.buffers.get_mut(0) {
            vertex_buffer_layout.attributes = vertex_layout.attributes;
        }
        
        Ok(())
    }
}

/// Main plugin for the material and shader testing tool
pub struct ShaderToolPlugin;

impl Plugin for ShaderToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToolState>()
            .init_resource::<MaterialParameters>()
            .init_resource::<ShaderParameters>()
            .init_resource::<ShaderEditorState>()
            .init_resource::<LightingParameters>()
            .add_plugins(MaterialPlugin::<ShaderToolMaterial>::default())
            .add_systems(Startup, setup_tool)
            .add_systems(EguiPrimaryContextPass, ui_system)
            .add_systems(Update, (
                update_camera,
                check_shader_errors,
                handle_camera_controls,
                update_materials_from_params,
                update_point_light,
                update_ambient_light,
                sync_lighting_to_shader_params,
            ))
            ;
    }
}

/// Resource holding the state of the tool
#[derive(Resource, Debug)]
pub struct ToolState {
    pub mode: ToolMode,
    pub current_geometry: GeometryType,
    pub available_geometries: Vec<GeometryType>,
    pub show_ui: bool,
    pub auto_rotate: bool,
    pub camera_distance: f32,
    pub camera_pitch: f32,
    pub camera_yaw: f32,
    pub camera_target: Vec3,
    pub camera_dragging: bool,
    pub camera_panning: bool,
    pub last_mouse_pos: Option<Vec2>,
    pub current_shader: String,
    pub available_shaders: Vec<String>,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            mode: ToolMode::Material,
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
            current_shader: "".to_string(),
            available_shaders: Vec::new(),
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

/// Resource for StandardMaterial parameters
#[derive(Resource, Debug, Clone)]
pub struct MaterialParameters {
    pub base_color: Color,
    pub emissive: LinearRgba,
    pub emissive_exposure_weight: f32,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub double_sided: bool,
    pub alpha_mode: AlphaMode,
}

impl Default for MaterialParameters {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.8, 0.2, 0.4),
            emissive: LinearRgba::BLACK,
            emissive_exposure_weight: 0.0,
            perceptual_roughness: 0.5,
            metallic: 0.0,
            reflectance: 0.5,
            double_sided: false,
            alpha_mode: AlphaMode::Opaque,
        }
    }
}

impl MaterialParameters {
    pub fn to_standard_material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            emissive: self.emissive,
            emissive_exposure_weight: self.emissive_exposure_weight,
            perceptual_roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            double_sided: self.double_sided,
            alpha_mode: self.alpha_mode.clone(),
            ..default()
        }
    }
}

/// Resource for shader parameters
#[derive(Resource, Debug, Clone)]
pub struct ShaderParameters {
    pub float_uniforms: HashMap<String, f32>,
    pub vector_uniforms: HashMap<String, Vec<f32>>,
    pub color_uniforms: HashMap<String, Color>,
    pub detected_uniforms: Vec<DetectedUniform>,
}

impl Default for ShaderParameters {
    fn default() -> Self {
        let mut float_uniforms = HashMap::new();
        float_uniforms.insert("time_scale".to_string(), 1.0);
        float_uniforms.insert("intensity".to_string(), 1.0);
        float_uniforms.insert("frequency".to_string(), 1.0);
        float_uniforms.insert("amplitude".to_string(), 0.5);
        float_uniforms.insert("ambient_intensity".to_string(), 1.0);
        float_uniforms.insert("point_light_intensity".to_string(), 1000.0);
        float_uniforms.insert("point_light_radius".to_string(), 10.0);
        float_uniforms.insert("use_point_light".to_string(), 1.0);
        float_uniforms.insert("use_ambient_light".to_string(), 1.0);
        
        let mut vector_uniforms = HashMap::new();
        vector_uniforms.insert("direction".to_string(), vec![0.0, 0.0, 1.0]);
        vector_uniforms.insert("offset".to_string(), vec![0.0, 0.0, 0.0]);
        vector_uniforms.insert("point_light_position".to_string(), vec![2.0, 2.0, 2.0]);
        
        let mut color_uniforms = HashMap::new();
        color_uniforms.insert("base_color".to_string(), Color::srgb(0.8, 0.2, 0.4));
        color_uniforms.insert("accent_color".to_string(), Color::srgb(0.2, 0.8, 0.4));
        color_uniforms.insert("ambient_color".to_string(), Color::srgb(0.1, 0.1, 0.1));
        color_uniforms.insert("point_light_color".to_string(), Color::srgb(1.0, 1.0, 1.0));
        
        Self {
            float_uniforms,
            vector_uniforms,
            color_uniforms,
            detected_uniforms: Vec::new(),
        }
    }
}

/// Resource for lighting parameters
#[derive(Resource, Debug, Clone)]
pub struct LightingParameters {
    pub ambient_color: Color,
    pub ambient_intensity: f32,
    pub point_light_position: Vec3,
    pub point_light_color: Color,
    pub point_light_intensity: f32,
    pub point_light_radius: f32,
    pub use_point_light: bool,
    pub use_ambient_light: bool,
}

impl Default for LightingParameters {
    fn default() -> Self {
        Self {
            ambient_color: Color::srgb(0.1, 0.1, 0.1),
            ambient_intensity: 1.0,
            point_light_position: Vec3::new(2.0, 2.0, 2.0),
            point_light_color: Color::srgb(1.0, 1.0, 1.0),
            point_light_intensity: 1000.0,
            point_light_radius: 10.0,
            use_point_light: true,
            use_ambient_light: true,
        }
    }
}

impl ShaderParameters {
    pub fn set_float(&mut self, name: &str, value: f32) {
        self.float_uniforms.insert(name.to_string(), value);
    }
    
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
    
    pub fn set_vector(&mut self, name: &str, value: Vec<f32>) {
        self.vector_uniforms.insert(name.to_string(), value);
    }
    
    pub fn get_color(&self, name: &str) -> Color {
        *self.color_uniforms.get(name).unwrap_or(&Color::WHITE)
    }
    
    pub fn set_color(&mut self, name: &str, value: Color) {
        self.color_uniforms.insert(name.to_string(), value);
    }
    
    pub fn extract_uniforms_from_shader(&mut self, shader_code: &str) {
        self.detected_uniforms.clear();
        
        for line in shader_code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
                continue;
            }
            
            if let Some((name, type_name)) = extract_var_uniform(trimmed) {
                if !self.detected_uniforms.iter().any(|u| u.name == name) {
                    let category = classify_uniform_type(&type_name);
                    let uniform = DetectedUniform {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        category,
                    };
                    self.detected_uniforms.push(uniform);
                }
            }
            else if let Some((name, type_name)) = extract_group_binding_var_uniform(trimmed) {
                if !self.detected_uniforms.iter().any(|u| u.name == name) {
                    let category = classify_uniform_type(&type_name);
                    let uniform = DetectedUniform {
                        name: name.to_string(),
                        type_name: type_name.to_string(),
                        category,
                    };
                    self.detected_uniforms.push(uniform);
                }
            }
        }
        
        self.detected_uniforms.sort_by(|a, b| {
            let a_order = uniform_category_order(&a.category);
            let b_order = uniform_category_order(&b.category);
            a_order.cmp(&b_order).then(a.name.cmp(&b.name))
        });
    }
    
    pub fn to_shader_material(&self) -> ShaderToolMaterial {
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
            ambient_color: to_linear_rgba(self.color_uniforms.get("ambient_color").copied().unwrap_or(Color::srgb(0.1, 0.1, 0.1))),
            ambient_intensity: self.float_uniforms.get("ambient_intensity").copied().unwrap_or(1.0),
            point_light_position: self.vector_uniforms.get("point_light_position")
                .map(|v| Vec3::new(v[0], v[1], v[2])).unwrap_or(Vec3::new(2.0, 2.0, 2.0)),
            point_light_color: to_linear_rgba(self.color_uniforms.get("point_light_color").copied().unwrap_or(Color::srgb(1.0, 1.0, 1.0))),
            point_light_intensity: self.float_uniforms.get("point_light_intensity").copied().unwrap_or(1000.0),
            point_light_radius: self.float_uniforms.get("point_light_radius").copied().unwrap_or(10.0),
            use_point_light: if self.float_uniforms.get("use_point_light").copied().unwrap_or(1.0) > 0.5 { 1 } else { 0 },
            use_ambient_light: if self.float_uniforms.get("use_ambient_light").copied().unwrap_or(1.0) > 0.5 { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DetectedUniform {
    pub name: String,
    pub type_name: String,
    pub category: UniformCategory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UniformCategory {
    Scalar,
    Vector,
    Color,
    Matrix,
    Unknown,
}

pub fn classify_uniform_type(type_name: &str) -> UniformCategory {
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

fn uniform_category_order(category: &UniformCategory) -> u8 {
    match category {
        UniformCategory::Scalar => 0,
        UniformCategory::Vector => 1,
        UniformCategory::Color => 2,
        UniformCategory::Matrix => 3,
        UniformCategory::Unknown => 4,
    }
}

fn extract_var_uniform(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for i in 0..parts.len().saturating_sub(2) {
        if parts[i] == "var<uniform>" {
            if i + 2 < parts.len() {
                let name = parts[i + 1].trim_end_matches(':');
                let type_name = parts[i + 2].trim_end_matches(';');
                if !name.is_empty() && !type_name.is_empty() {
                    return Some((name, type_name));
                }
            }
        }
    }
    None
}

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
            let name = parts[var_idx + 1].trim_end_matches(':');
            let type_name = parts[var_idx + 2].trim_end_matches(';');
            if !name.is_empty() && !type_name.is_empty() {
                return Some((name, type_name));
            }
        }
    }
    None
}

/// State for the shader code editor
#[derive(Resource, Debug, Clone)]
pub struct ShaderEditorState {
    pub source_code: String,
    pub current_file: Option<PathBuf>,
    pub has_errors: bool,
    pub error_messages: Vec<String>,
    pub modified: bool,
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
    pub fn new() -> Self {
        let default_shader = r#"// Default shader
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
    let final_color = base_color * intensity;
    let pattern = sin(in.uv.x * frequency * 10.0) * 
                  cos(in.uv.y * frequency * 10.0) * 
                  amplitude * 0.5 + 0.5;
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

    pub fn load_from_file(&mut self, path: &Path) -> Result<(), String> {
        match fs::read_to_string(path) {
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

    pub fn save_to_file(&self, path: &Path) -> Result<(), String> {
        match fs::write(path, &self.source_code) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write file: {}", e)),
        }
    }

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

    pub fn compile_and_validate(&mut self) -> Result<(), Vec<String>> {
        use naga::front::wgsl;
        match wgsl::parse_str(&self.source_code) {
            Ok(_) => {
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

#[derive(Component)]
pub struct ToolEntity;

#[derive(Component)]
pub struct ToolCamera;

#[derive(Component)]
pub struct PointLightMarker;

#[derive(Component)]
pub struct AmbientLightMarker;

/// Marker for which material type an entity uses
#[derive(Component)]
pub enum EntityMaterialType {
    Standard,
    ShaderTool,
}

fn update_materials_from_params(
    mat_params: Res<MaterialParameters>,
    shader_params: Res<ShaderParameters>,
    lighting_params: Res<LightingParameters>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    mut materials_shader: ResMut<Assets<ShaderToolMaterial>>,
    query: Query<(Entity, &EntityMaterialType)>, 
    mut commands: Commands,
) {
    for (entity, mat_type) in &query {
        match mat_type {
            EntityMaterialType::Standard => {
                let new_material = mat_params.to_standard_material();
                commands.entity(entity).insert(MeshMaterial3d(materials_std.add(new_material)));
            }
            EntityMaterialType::ShaderTool => {
                let mut updated_shader_params = shader_params.clone();
                updated_shader_params.color_uniforms.insert("ambient_color".to_string(), lighting_params.ambient_color);
                updated_shader_params.float_uniforms.insert("ambient_intensity".to_string(), lighting_params.ambient_intensity);
                updated_shader_params.vector_uniforms.insert("point_light_position".to_string(), 
                    vec![lighting_params.point_light_position.x, 
                         lighting_params.point_light_position.y, 
                         lighting_params.point_light_position.z]);
                updated_shader_params.color_uniforms.insert("point_light_color".to_string(), lighting_params.point_light_color);
                updated_shader_params.float_uniforms.insert("point_light_intensity".to_string(), lighting_params.point_light_intensity);
                updated_shader_params.float_uniforms.insert("point_light_radius".to_string(), lighting_params.point_light_radius);
                updated_shader_params.float_uniforms.insert("use_point_light".to_string(), if lighting_params.use_point_light { 1.0 } else { 0.0 });
                updated_shader_params.float_uniforms.insert("use_ambient_light".to_string(), if lighting_params.use_ambient_light { 1.0 } else { 0.0 });
                
                let new_material = updated_shader_params.to_shader_material();
                commands.entity(entity).insert(MeshMaterial3d(materials_shader.add(new_material)));
            }
        }
    }
}

/// Sync lighting parameters to shader parameters so lighting UI changes affect shaders
fn sync_lighting_to_shader_params(
    lighting_params: Res<LightingParameters>,
    mut shader_params: ResMut<ShaderParameters>,
) {
    // Sync ambient light
    shader_params.color_uniforms.insert("ambient_color".to_string(), lighting_params.ambient_color);
    shader_params.float_uniforms.insert("ambient_intensity".to_string(), lighting_params.ambient_intensity);
    
    // Sync point light
    shader_params.vector_uniforms.insert("point_light_position".to_string(), 
        vec![lighting_params.point_light_position.x, 
             lighting_params.point_light_position.y, 
             lighting_params.point_light_position.z]);
    shader_params.color_uniforms.insert("point_light_color".to_string(), lighting_params.point_light_color);
    shader_params.float_uniforms.insert("point_light_intensity".to_string(), lighting_params.point_light_intensity);
    shader_params.float_uniforms.insert("point_light_radius".to_string(), lighting_params.point_light_radius);
    shader_params.float_uniforms.insert("use_point_light".to_string(), if lighting_params.use_point_light { 1.0 } else { 0.0 });
    shader_params.float_uniforms.insert("use_ambient_light".to_string(), if lighting_params.use_ambient_light { 1.0 } else { 0.0 });
}

fn setup_tool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    mut materials_shader: ResMut<Assets<ShaderToolMaterial>>,
    mut state: ResMut<ToolState>,
    mut editor: ResMut<ShaderEditorState>,
    mat_params: Res<MaterialParameters>,
    shader_params: Res<ShaderParameters>,
    lighting_params: Res<LightingParameters>,
) {
    *editor = ShaderEditorState::new();
    
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
    
    // Spawn ambient light for Bevy's PBR lighting
    commands.spawn((
        AmbientLight {
            color: lighting_params.ambient_color,
            brightness: lighting_params.ambient_intensity,
            ..default()
        },
        Name::new("Ambient Light"),
        AmbientLightMarker,
    ));
    
    // Spawn point light for Bevy's PBR lighting
    commands.spawn((
        PointLight {
            color: lighting_params.point_light_color,
            intensity: lighting_params.point_light_intensity,
            range: lighting_params.point_light_radius,
            ..default()
        },
        Transform::from_translation(lighting_params.point_light_position),
        Name::new("Point Light"),
        PointLightMarker,
    ));
    
    spawn_test_geometry(
        &mut commands,
        &mut meshes,
        &mut materials_std,
        &mut materials_shader,
        &state,
        &mat_params,
        &shader_params,
    );
    
    scan_for_shaders(&mut state);
}

fn spawn_test_geometry(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials_std: &mut ResMut<Assets<StandardMaterial>>,
    materials_shader: &mut ResMut<Assets<ShaderToolMaterial>>,
    state: &ToolState,
    mat_params: &MaterialParameters,
    shader_params: &ShaderParameters,
) {
    let mesh = state.current_geometry.create_mesh();
    let mesh_handle = meshes.add(mesh);
    
    if state.mode == ToolMode::Material {
        let material = mat_params.to_standard_material();
        let std_handle = materials_std.add(material);
        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(std_handle),
            EntityMaterialType::Standard,
            ToolEntity,
            Name::new("Test Entity"),
        ));
    } else {
        let material = shader_params.to_shader_material();
        let shader_handle = materials_shader.add(material);
        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(shader_handle),
            EntityMaterialType::ShaderTool,
            ToolEntity,
            Name::new("Test Entity"),
        ));
    }
}

fn scan_for_shaders(state: &mut ResMut<ToolState>) {
    state.available_shaders = vec![
        "assets/shaders/shader_tool.wgsl".to_string(),
        "assets/shaders/current.wgsl".to_string(),
        "shaders/test_shader.wgsl".to_string(),
        "shaders/animate_shader.wgsl".to_string(),
        "shaders/color_shader.wgsl".to_string(),
        "shaders/pattern_shader.wgsl".to_string(),
    ];
}

fn update_camera(
    state: Res<ToolState>,
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

fn update_point_light(
    lighting_params: Res<LightingParameters>,
    mut query: Query<(&mut PointLight, &mut Transform), With<PointLightMarker>>,
) {
    for (mut light, mut transform) in &mut query {
        transform.translation = lighting_params.point_light_position;
        light.color = lighting_params.point_light_color;
        light.intensity = lighting_params.point_light_intensity;
        light.range = lighting_params.point_light_radius;
    }
}

fn update_ambient_light(
    lighting_params: Res<LightingParameters>,
    mut query: Query<&mut AmbientLight, With<AmbientLightMarker>>,
) {
    for mut light in &mut query {
        light.color = lighting_params.ambient_color;
        light.brightness = lighting_params.ambient_intensity;
    }
}

fn handle_camera_controls(
    windows: Query<&Window>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<ToolState>,
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
                let right = Vec3::new(-state.camera_yaw.sin(), 0.0, state.camera_yaw.cos()).normalize();
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

fn check_shader_errors(editor: Res<ShaderEditorState>) {
    if editor.has_errors {
        eprintln!("Shader compilation errors:");
        for error in &editor.error_messages {
            eprintln!("  {}", error);
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<ToolState>,
    mut mat_params: ResMut<MaterialParameters>,
    mut shader_params: ResMut<ShaderParameters>,
    mut lighting_params: ResMut<LightingParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut editor: ResMut<ShaderEditorState>,
    mut materials_std: ResMut<Assets<StandardMaterial>>,
    mut materials_shader: ResMut<Assets<ShaderToolMaterial>>,
    mut geometry_query: Query<&mut Mesh3d, With<ToolEntity>>,
    entity_query: Query<(Entity, &EntityMaterialType)>,
    mut commands: Commands,
) {
    if !state.show_ui {
        return;
    }
    
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Material & Shader Testing Tool")
            .default_pos(egui::pos2(10.0, 10.0))
            .default_size(egui::vec2(400.0, 700.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Mode:");
                    egui::ComboBox::from_id_salt("mode_selector")
                        .selected_text(state.mode.as_str())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.mode, ToolMode::Material, "Material");
                            ui.selectable_value(&mut state.mode, ToolMode::Shader, "Shader");
                        });
                });
                
                ui.separator();
                
                ui.collapsing("Geometry", |ui| {
                    ui.label("Geometry Type:");
                    let current_index = state.available_geometries.iter()
                        .position(|g| g == &state.current_geometry)
                        .unwrap_or(0);
                    
                    let mut selected_geometry = state.current_geometry;
                    egui::ComboBox::from_id_salt("")
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
                    
                    if ui.button("Respawn with current mode").clicked() {
                        for (entity, _) in &entity_query {
                            commands.entity(entity).despawn();
                        }
                        // Despawn old and spawn new with current mode
                        spawn_test_geometry(
                            &mut commands,
                            &mut meshes,
                            &mut materials_std,
                            &mut materials_shader,
                            &state,
                            &mat_params,
                            &shader_params,
                        );
                    }
                });
                
                ui.separator();
                
                match state.mode {
                    ToolMode::Material => material_ui(ui, &mut mat_params),
                    ToolMode::Shader => {
                        let materials_shader_res = materials_shader.into();
                        shader_ui(
                            ui, &mut state, &mut shader_params, &mut editor, &mut meshes,
                            &materials_shader_res, &mut geometry_query, &mut commands,
                        )
                    }
                }
                
                ui.separator();
                ui.collapsing("View", |ui| {
                    ui.checkbox(&mut state.auto_rotate, "Auto Rotate");
                    ui.add(egui::Slider::new(&mut state.camera_distance, 1.0..=20.0).text("Camera Distance"));
                });
                
                ui.separator();
                lighting_ui(ui, &mut lighting_params);
                
                ui.separator();
                ui.collapsing("Camera Controls", |ui| {
                    ui.label("Right-click + drag: Orbit camera");
                    ui.label("Right-click + Shift + drag: Pan camera");
                    ui.label("Scroll: Zoom in/out");
                });
            });
    }
}

fn lighting_ui(ui: &mut egui::Ui, lighting_params: &mut ResMut<LightingParameters>) {
    ui.collapsing("Lighting Controls", |ui| {
        ui.checkbox(&mut lighting_params.use_ambient_light, "Use Ambient Light");
        if lighting_params.use_ambient_light {
            ui.horizontal(|ui| {
                ui.label("Ambient Color:");
                let ambient_srgba = lighting_params.ambient_color.to_srgba();
                let mut rgb = [ambient_srgba.red, ambient_srgba.green, ambient_srgba.blue];
                if ui.color_edit_button_rgb(&mut rgb).changed() {
                    lighting_params.ambient_color = Color::srgba(rgb[0], rgb[1], rgb[2], 1.0);
                }
            });
            ui.add(egui::Slider::new(&mut lighting_params.ambient_intensity, 0.0..=2.0).text("Ambient Intensity"));
        }
        
        ui.separator();
        
        ui.checkbox(&mut lighting_params.use_point_light, "Use Point Light");
        if lighting_params.use_point_light {
            ui.horizontal(|ui| {
                ui.label("Point Light Color:");
                let point_srgba = lighting_params.point_light_color.to_srgba();
                let mut rgb = [point_srgba.red, point_srgba.green, point_srgba.blue];
                if ui.color_edit_button_rgb(&mut rgb).changed() {
                    lighting_params.point_light_color = Color::srgba(rgb[0], rgb[1], rgb[2], 1.0);
                }
            });
            ui.add(egui::Slider::new(&mut lighting_params.point_light_intensity, 0.0..=2000.0).text("Point Light Intensity"));
            ui.add(egui::Slider::new(&mut lighting_params.point_light_radius, 0.1..=50.0).text("Point Light Radius"));
            
            ui.separator();
            ui.label("Point Light Position:");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::Slider::new(&mut lighting_params.point_light_position.x, -10.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Y:");
                ui.add(egui::Slider::new(&mut lighting_params.point_light_position.y, -10.0..=10.0));
            });
            ui.horizontal(|ui| {
                ui.label("Z:");
                ui.add(egui::Slider::new(&mut lighting_params.point_light_position.z, -10.0..=10.0));
            });
        }
    });
}

fn material_ui(ui: &mut egui::Ui, mat_params: &mut ResMut<MaterialParameters>) {
    ui.collapsing("StandardMaterial Properties", |ui| {
        ui.label("Base Color");
        let base_srgba = mat_params.base_color.to_srgba();
        let mut rgb = [base_srgba.red, base_srgba.green, base_srgba.blue];
        if ui.color_edit_button_rgb(&mut rgb).changed() {
            mat_params.base_color = Color::srgba(rgb[0], rgb[1], rgb[2], 1.0);
        }
        
        ui.separator();
        
        ui.label("PBR Properties");
        ui.add(egui::Slider::new(&mut mat_params.metallic, 0.0..=1.0).text("Metallic"));
        ui.add(egui::Slider::new(&mut mat_params.perceptual_roughness, 0.0..=1.0).text("Perceptual Roughness"));
        ui.add(egui::Slider::new(&mut mat_params.reflectance, 0.0..=1.0).text("Reflectance"));
        
        ui.separator();
        
        ui.label("Emissive");
        let emissive = mat_params.emissive;
        let mut emissive_rgb = [emissive.red, emissive.green, emissive.blue];
        if ui.color_edit_button_rgb(&mut emissive_rgb).changed() {
            mat_params.emissive = LinearRgba::new(emissive_rgb[0], emissive_rgb[1], emissive_rgb[2], 1.0);
        }
        ui.add(egui::Slider::new(&mut mat_params.emissive_exposure_weight, 0.0..=1.0).text("Emissive Exposure Weight"));
        
        ui.separator();
        ui.checkbox(&mut mat_params.double_sided, "Double Sided");
    });
}

fn shader_ui(
    ui: &mut egui::Ui,
    state: &mut ResMut<ToolState>,
    shader_params: &mut ResMut<ShaderParameters>,
    editor: &mut ResMut<ShaderEditorState>,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials_shader: &Res<Assets<ShaderToolMaterial>>,
    _geometry_query: &mut Query<&mut Mesh3d, With<ToolEntity>>,
    _commands: &mut Commands,
) {
    ui.collapsing("Shaders", |ui| {
        ui.label("Available Shaders:");
        let current_index = state.available_shaders.iter()
            .position(|s| s == &state.current_shader)
            .unwrap_or(0);
        
        let current_shader_clone = state.current_shader.clone();
        let mut selected_shader = current_shader_clone;
        egui::ComboBox::from_id_salt("")
            .selected_text(&selected_shader)
            .show_ui(ui, |ui| {
                for (i, shader) in state.available_shaders.iter().enumerate() {
                    if ui.selectable_label(i == current_index, shader).clicked() {
                        selected_shader = shader.clone();
                        if Path::new(&selected_shader).exists() {
                            editor.load_from_file(Path::new(&selected_shader)).ok();
                            shader_params.extract_uniforms_from_shader(&editor.source_code);
                            // Copy the selected shader to current.wgsl for hot-reloading
                            if let Ok(shader_code) = std::fs::read_to_string(&selected_shader) {
                                std::fs::write("assets/shaders/current.wgsl", &shader_code).ok();
                            }
                        }
                    }
                }
            });
        if selected_shader != state.current_shader {
            state.current_shader = selected_shader;
        }
        
        if ui.button("Reload Shader").clicked() {
            if let Some(ref path) = editor.current_file {
                let path_clone = path.clone();
                editor.load_from_file(&path_clone).ok();
                shader_params.extract_uniforms_from_shader(&editor.source_code);
            }
        }
        
        ui.horizontal(|ui| {
            if ui.button("Open...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("WGSL Shaders", &["wgsl"])
                    .add_filter("All Files", &["*"])
                    .pick_file() {
                    editor.load_from_file(&path).ok();
                    state.current_shader = path.display().to_string();
                    shader_params.extract_uniforms_from_shader(&editor.source_code);
                    // Copy to current.wgsl for live editing
                    if let Ok(code) = std::fs::read_to_string(&path) {
                        std::fs::write("assets/shaders/current.wgsl", &code).ok();
                    }
                }
            }
            
            if ui.button("Save").clicked() {
                // Always save to current.wgsl for live editing
                if editor.save_to_file(Path::new("assets/shaders/current.wgsl")).is_err() {
                    eprintln!("Save error");
                } else {
                    editor.modified = false;
                }
                
                // Also save to original file if one was opened
                if let Some(ref path) = editor.current_file {
                    std::fs::write(path, &editor.source_code).ok();
                }
            }
        });
    });
    
    ui.separator();
    
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
            editor.source_code = code;
            editor.modified = true;
        }
        
        ui.horizontal(|ui| {
            if ui.button("Apply Shader").clicked() {
                match editor.compile_and_validate() {
                    Ok(_) => {
                        if let Ok(temp_path) = editor.create_temp_file() {
                            editor.temp_file = Some(temp_path.clone());
                            state.current_shader = temp_path.display().to_string();
                        }
                        shader_params.extract_uniforms_from_shader(&editor.source_code);
                    }
                    Err(errs) => {
                        eprintln!("Shader compilation failed: {:?}", errs);
                    }
                }
            }
            if editor.modified {
                ui.label("* modified");
            }
        });
    });
    
    ui.separator();
    
    let scalar_uniforms: Vec<DetectedUniform> = shader_params.detected_uniforms.iter()
        .filter(|u| matches!(u.category, UniformCategory::Scalar))
        .cloned().collect();
    let vector_uniforms: Vec<DetectedUniform> = shader_params.detected_uniforms.iter()
        .filter(|u| matches!(u.category, UniformCategory::Vector))
        .cloned().collect();
    let color_uniforms: Vec<DetectedUniform> = shader_params.detected_uniforms.iter()
        .filter(|u| matches!(u.category, UniformCategory::Color))
        .cloned().collect();
    
    if !shader_params.detected_uniforms.is_empty() {
        ui.collapsing("Detected Uniforms", |ui| {
            ui.label(format!("Found {} uniforms:", shader_params.detected_uniforms.len()));
            ui.separator();
            
            if !scalar_uniforms.is_empty() {
                ui.label("Scalars:");
                for uniform in &scalar_uniforms {
                    let mut value = shader_params.float_uniforms.get(&uniform.name).copied().unwrap_or(0.0);
                    if ui.add(egui::Slider::new(&mut value, -10.0..=10.0).text(&uniform.name)).changed() {
                        shader_params.set_float(&uniform.name, value);
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
                    let mut value = shader_params.get_vector(&uniform.name, size);
                    
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
                    shader_params.set_vector(&uniform.name, value);
                }
                ui.separator();
            }
            
            if !color_uniforms.is_empty() {
                ui.label("Colors:");
                for uniform in &color_uniforms {
                    let mut color = shader_params.get_color(&uniform.name);
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
                            shader_params.set_color(&uniform.name, color);
                        }
                    });
                }
            }
        });
    }
    
    if shader_params.detected_uniforms.is_empty() {
        ui.collapsing("Parameters", |ui| {
            ui.label("No uniforms detected in shader.");
            ui.label("Add uniforms with @group(0) @binding(n) var<uniform> name: type;");
        });
    }
}
