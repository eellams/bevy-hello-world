use bevy::prelude::*;

/// Resource for shader parameters that can be controlled via GUI
#[derive(Resource, Debug, Clone)]
pub struct ShaderParameters {
    pub params: Vec4,
    pub param_names: Vec<String>,
}

impl Default for ShaderParameters {
    fn default() -> Self {
        Self {
            params: Vec4::ZERO,
            param_names: vec![
                "Param1".to_string(),
                "Param2".to_string(),
                "Param3".to_string(),
                "Param4".to_string(),
            ],
        }
    }
}

impl ShaderParameters {
    pub fn new(param_names: &[&str]) -> Self {
        let mut names = Vec::new();
        for name in param_names {
            names.push(name.to_string());
        }
        // Pad to 4 parameters
        while names.len() < 4 {
            names.push("Unused".to_string());
        }
        Self {
            params: Vec4::ZERO,
            param_names: names,
        }
    }

    pub fn set_param(&mut self, index: usize, value: f32) {
        if index < 4 {
            self.params[index] = value;
        }
    }

    pub fn get_param(&self, index: usize) -> f32 {
        if index < 4 {
            self.params[index]
        } else {
            0.0
        }
    }
}

/// Component to mark entities using custom shaders
#[derive(Component, Debug)]
pub struct CustomShader {
    pub shader_path: String,
}

/// Component to mark shader test entities
#[derive(Component, Debug)]
pub struct ShaderTestEntity;

/// System to update shader material parameters from GUI controls
pub fn update_shader_parameters_system(
    mut shader_params: ResMut<ShaderParameters>,
    mut material_query: Query<&mut Sprite>,
) {
    for mut sprite in &mut material_query {
        // Use params to modify the color
        sprite.color = Color::srgb(
            shader_params.params.x,
            shader_params.params.y,
            shader_params.params.z,
        );
    }
}
