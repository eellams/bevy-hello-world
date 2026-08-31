use bevy::prelude::*;

/// Resource for shader parameters that can be controlled via GUI
#[derive(Resource, Debug, Clone)]
pub struct ShaderParameters {
    pub params: Vec4,
}

impl Default for ShaderParameters {
    fn default() -> Self {
        Self {
            params: Vec4::ZERO,
        }
    }
}

/// Component to mark shader test entities
#[derive(Component, Debug)]
pub struct ShaderTestEntity;

/// System to update shader material parameters from GUI controls
pub fn update_shader_parameters_system(
    shader_params: Res<ShaderParameters>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_parameters_default() {
        let params = ShaderParameters::default();
        assert_eq!(params.params, Vec4::ZERO);
    }

    #[test]
    fn test_shader_parameters_clone() {
        let params = ShaderParameters {
            params: Vec4::new(1.0, 2.0, 3.0, 4.0),
        };
        let clone = params.clone();
        assert_eq!(params.params, clone.params);
    }
}
