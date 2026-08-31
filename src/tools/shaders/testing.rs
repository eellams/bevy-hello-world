use bevy::prelude::*;
use super::material::*;

/// Resource to hold available shaders
#[derive(Resource, Debug, Default)]
pub struct ShaderLibrary {
    pub shaders: Vec<ShaderInfo>,
    pub current_index: usize,
}

#[derive(Debug, Clone)]
pub struct ShaderInfo {
    pub path: String,
}

impl ShaderLibrary {
    pub fn add_shader(&mut self, path: &str) {
        self.shaders.push(ShaderInfo {
            path: path.to_string(),
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
    pub size: Vec2,
}

impl GeometryLibrary {
    pub fn add_geometry(&mut self, size: Vec2) {
        self.geometries.push(GeometryInfo {
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
}

impl Default for CurrentShader {
    fn default() -> Self {
        Self {
            shader_path: "shaders/test_shader.wgsl".to_string(),
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
    shader_library.add_shader("shaders/test_shader.wgsl");
    shader_library.add_shader("shaders/color_shader.wgsl");
    shader_library.add_shader("shaders/pattern_shader.wgsl");

    // Add some geometries
    geometry_library.add_geometry(Vec2::new(200.0, 200.0));
    geometry_library.add_geometry(Vec2::new(200.0, 200.0));
    geometry_library.add_geometry(Vec2::new(200.0, 200.0));

    // Spawn shader test entity - a large colored quad in the center
    commands.spawn((
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.4),
            custom_size: Some(Vec2::new(400.0, 400.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        ShaderTestEntity,
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
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        shader_library.previous();
        if let Some(shader) = shader_library.current() {
            current_shader.shader_path = shader.path.clone();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_library_default() {
        let library = ShaderLibrary::default();
        assert!(library.shaders.is_empty());
        assert_eq!(library.current_index, 0);
    }

    #[test]
    fn test_shader_library_add() {
        let mut library = ShaderLibrary::default();
        library.add_shader("test.wgsl");
        assert_eq!(library.shaders.len(), 1);
        assert_eq!(library.shaders[0].path, "test.wgsl");
    }

    #[test]
    fn test_shader_library_next() {
        let mut library = ShaderLibrary::default();
        library.add_shader("a.wgsl");
        library.add_shader("b.wgsl");
        library.add_shader("c.wgsl");
        
        assert_eq!(library.current_index, 0);
        library.next();
        assert_eq!(library.current_index, 1);
        library.next();
        assert_eq!(library.current_index, 2);
        library.next(); // Wraps around
        assert_eq!(library.current_index, 0);
    }

    #[test]
    fn test_shader_library_previous() {
        let mut library = ShaderLibrary::default();
        library.add_shader("a.wgsl");
        library.add_shader("b.wgsl");
        library.add_shader("c.wgsl");
        
        library.current_index = 0;
        library.previous(); // Wraps around
        assert_eq!(library.current_index, 2);
    }

    #[test]
    fn test_shader_library_current() {
        let mut library = ShaderLibrary::default();
        library.add_shader("a.wgsl");
        library.add_shader("b.wgsl");
        
        assert!(library.current().is_some());
        assert_eq!(library.current().unwrap().path, "a.wgsl");
    }

    #[test]
    fn test_shader_library_current_empty() {
        let library = ShaderLibrary::default();
        assert!(library.current().is_none());
    }

    #[test]
    fn test_geometry_library_default() {
        let library = GeometryLibrary::default();
        assert!(library.geometries.is_empty());
        assert_eq!(library.current_index, 0);
    }

    #[test]
    fn test_geometry_library_add() {
        let mut library = GeometryLibrary::default();
        library.add_geometry(Vec2::new(100.0, 100.0));
        assert_eq!(library.geometries.len(), 1);
        assert_eq!(library.geometries[0].size, Vec2::new(100.0, 100.0));
    }

    #[test]
    fn test_geometry_library_next() {
        let mut library = GeometryLibrary::default();
        library.add_geometry(Vec2::new(100.0, 100.0));
        library.add_geometry(Vec2::new(200.0, 200.0));
        
        assert_eq!(library.current_index, 0);
        library.next();
        assert_eq!(library.current_index, 1);
        library.next(); // Wraps around
        assert_eq!(library.current_index, 0);
    }

    #[test]
    fn test_current_shader_default() {
        let current = CurrentShader::default();
        assert_eq!(current.shader_path, "shaders/test_shader.wgsl");
    }
}
