use bevy::prelude::*;
use bevy::math::primitives::Rectangle;
use super::material::*;

/// Resource to hold available shaders
#[derive(Resource, Debug, Default)]
pub struct ShaderLibrary {
    pub shaders: Vec<ShaderInfo>,
    pub current_index: usize,
}

#[derive(Debug, Clone)]
pub struct ShaderInfo {
    pub name: String,
    pub path: String,
    pub description: String,
}

impl ShaderLibrary {
    pub fn add_shader(&mut self, name: &str, path: &str, description: &str) {
        self.shaders.push(ShaderInfo {
            name: name.to_string(),
            path: path.to_string(),
            description: description.to_string(),
        });
    }

    pub fn next(&mut self) {
        if !self.shaders.is_empty() {
            self.current_index = (self.current_index + 1) % self.shaders.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.shaders.is_empty() {
            self.current_index = (self.current_index + self.shaders.len() - 1) % self.shaders.len();
        }
    }

    pub fn current(&self) -> Option<&ShaderInfo> {
        if self.shaders.is_empty() {
            None
        } else {
            Some(&self.shaders[self.current_index])
        }
    }
}

/// Resource to hold available geometries
#[derive(Resource, Debug, Default)]
pub struct GeometryLibrary {
    pub geometries: Vec<GeometryInfo>,
    pub current_index: usize,
}

#[derive(Debug, Clone)]
pub struct GeometryInfo {
    pub name: String,
    pub size: Vec2,
}

impl GeometryLibrary {
    pub fn add_geometry(&mut self, name: &str, size: Vec2) {
        self.geometries.push(GeometryInfo {
            name: name.to_string(),
            size,
        });
    }

    pub fn next(&mut self) {
        if !self.geometries.is_empty() {
            self.current_index = (self.current_index + 1) % self.geometries.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.geometries.is_empty() {
            self.current_index = (self.current_index + self.geometries.len() - 1) % self.geometries.len();
        }
    }

    pub fn current(&self) -> Option<&GeometryInfo> {
        if self.geometries.is_empty() {
            None
        } else {
            Some(&self.geometries[self.current_index])
        }
    }
}

/// Resource to hold the current shader being tested
#[derive(Resource, Debug, Clone)]
pub struct CurrentShader {
    pub shader_path: String,
    pub name: String,
}

impl Default for CurrentShader {
    fn default() -> Self {
        Self {
            shader_path: "shaders/test_shader.wgsl".to_string(),
            name: "Test Shader".to_string(),
        }
    }
}

/// System to setup shader testing framework
pub fn setup_shader_testing_framework(
    mut commands: Commands,
    mut shader_library: ResMut<ShaderLibrary>,
    mut geometry_library: ResMut<GeometryLibrary>,
) {
    // Add some example shaders
    shader_library.add_shader(
        "Test Shader",
        "shaders/test_shader.wgsl",
        "A basic test shader"
    );
    shader_library.add_shader(
        "Color Shader",
        "shaders/color_shader.wgsl",
        "Color manipulation shader"
    );
    shader_library.add_shader(
        "Pattern Shader",
        "shaders/pattern_shader.wgsl",
        "Pattern generation shader"
    );

    // Add some geometries
    geometry_library.add_geometry("Rectangle", Vec2::new(200.0, 200.0));
    geometry_library.add_geometry("Circle", Vec2::new(200.0, 200.0));
    geometry_library.add_geometry("Triangle", Vec2::new(200.0, 200.0));

    // Spawn shader test entity
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.8),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        ShaderTestEntity,
        CustomShader {
            shader_path: "shaders/test_shader.wgsl".to_string(),
        },
    ));

    // Add framework marker
    commands.insert_resource(CurrentShader::default());
}

/// System to handle shader switching
pub fn shader_switching_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shader_library: ResMut<ShaderLibrary>,
    mut geometry_library: ResMut<GeometryLibrary>,
    mut current_shader: ResMut<CurrentShader>,
    mut shader_params: ResMut<ShaderParameters>,
    mut test_query: Query<&mut Sprite>, 
) {
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        geometry_library.next();
        if let Some(geo) = geometry_library.current() {
            for mut sprite in &mut test_query {
                sprite.custom_size = Some(geo.size);
            }
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        geometry_library.previous();
        if let Some(geo) = geometry_library.current() {
            for mut sprite in &mut test_query {
                sprite.custom_size = Some(geo.size);
            }
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        shader_library.next();
        if let Some(shader) = shader_library.current() {
            current_shader.shader_path = shader.path.clone();
            current_shader.name = shader.name.clone();
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        shader_library.previous();
        if let Some(shader) = shader_library.current() {
            current_shader.shader_path = shader.path.clone();
            current_shader.name = shader.name.clone();
        }
    }
}

/// System to update shader test entities
pub fn update_shader_test_entities(
    mut test_query: Query<&mut Transform, With<ShaderTestEntity>>,
) {
    for mut transform in &mut test_query {
        // Center the test entity
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.z = 0.0;
    }
}
