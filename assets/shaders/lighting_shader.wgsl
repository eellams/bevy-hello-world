// Lighting Shader with Point Light and Ambient Light
// This shader demonstrates Phong lighting with configurable point light and ambient light

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> base_color: vec4<f32>;

@group(0) @binding(1)
var<uniform> intensity: f32;

@group(0) @binding(2)
var<uniform> ambient_color: vec4<f32>;

@group(0) @binding(3)
var<uniform> ambient_intensity: f32;

@group(0) @binding(4)
var<uniform> point_light_position: vec3<f32>;

@group(0) @binding(5)
var<uniform> point_light_color: vec4<f32>;

@group(0) @binding(6)
var<uniform> point_light_intensity: f32;

@group(0) @binding(7)
var<uniform> point_light_radius: f32;

@group(0) @binding(8)
var<uniform> use_point_light: u32;

@group(0) @binding(9)
var<uniform> use_ambient_light: u32;

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
    output.world_position = (model * vec4<f32>(mesh.position, 1.0)).xyz;
    return output;
}

// Helper function to calculate diffuse lighting
fn calculate_diffuse(normal: vec3<f32>, light_dir: vec3<f32>) -> f32 {
    let normalized_normal = normalize(normal);
    let normalized_light_dir = normalize(light_dir);
    return max(dot(normalized_normal, normalized_light_dir), 0.0);
}

// Helper function to calculate specular lighting (Blinn-Phong)
fn calculate_specular(normal: vec3<f32>, light_dir: vec3<f32>, view_dir: vec3<f32>, shininess: f32) -> f32 {
    let normalized_normal = normalize(normal);
    let normalized_light_dir = normalize(light_dir);
    let normalized_view_dir = normalize(view_dir);
    
    let half_vec = normalize(normalized_light_dir + normalized_view_dir);
    let spec = pow(max(dot(normalized_normal, half_vec), 0.0), shininess);
    return spec;
}

// Calculate attenuation based on distance
fn calculate_attenuation(distance: f32, radius: f32) -> f32 {
    // Smooth attenuation within the light radius
    let normalized_distance = distance / radius;
    return max(1.0 - normalized_distance * normalized_distance, 0.0);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let base = base_color * intensity;
    var final_color = vec4<f32>(0.0, 0.0, 0.0, base.a);
    
    // Ambient light
    if use_ambient_light != 0u {
        final_color.rgb += ambient_color.rgb * ambient_intensity;
    }
    
    // Point light
    if use_point_light != 0u {
        let light_dir = point_light_position - in.world_position;
        let distance = length(light_dir);
        
        // Only apply if within radius
        if distance <= point_light_radius {
            let attenuation = calculate_attenuation(distance, point_light_radius);
            let light_dir_normalized = light_dir / distance;
            
            // View direction (from fragment to camera)
            let view_dir = -normalize(in.world_position);
            
            // Diffuse component
            let diffuse = calculate_diffuse(in.normal, light_dir);
            
            // Specular component (Blinn-Phong)
            let specular = calculate_specular(in.normal, light_dir, view_dir, 32.0);
            
            // Combine lighting components
            let light_contribution = (diffuse + specular * 0.5) * attenuation;
            final_color.rgb += point_light_color.rgb * point_light_intensity * light_contribution * 0.01;
        }
    }
    
    // Multiply by base color
    final_color.rgb *= base.rgb;
    
    return final_color;
}
