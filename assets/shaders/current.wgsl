// Current active shader - this file is edited by the shader tool
// It will be hot-reloaded by Bevy when saved

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_position: vec3<f32>,
};

// Uniforms matching ShaderToolMaterial bindings
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

@group(0) @binding(8)
var<uniform> ambient_color: vec4<f32>;

@group(0) @binding(9)
var<uniform> ambient_intensity: f32;

@group(0) @binding(10)
var<uniform> point_light_position: vec3<f32>;

@group(0) @binding(11)
var<uniform> point_light_color: vec4<f32>;

@group(0) @binding(12)
var<uniform> point_light_intensity: f32;

@group(0) @binding(13)
var<uniform> point_light_radius: f32;

@group(0) @binding(14)
var<uniform> use_point_light: u32;

@group(0) @binding(15)
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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting with color
    let base = base_color * intensity;
    
    // Ambient light
    var final_color = vec4<f32>(0.0, 0.0, 0.0, base.a);
    if use_ambient_light != 0u {
        final_color.rgb += ambient_color.rgb * ambient_intensity;
    }
    
    // Point light
    if use_point_light != 0u {
        let light_dir = point_light_position - in.world_position;
        let distance = length(light_dir);
        
        if distance <= point_light_radius {
            let attenuation = max(1.0 - (distance / point_light_radius) * (distance / point_light_radius), 0.0);
            let light_dir_normalized = light_dir / distance;
            let diffuse = max(dot(normalize(in.normal), normalize(light_dir_normalized)), 0.0);
            final_color.rgb += point_light_color.rgb * point_light_intensity * diffuse * attenuation;
        }
    }
    
    // Mix with accent color based on UV
    let uv_pattern = sin(in.uv.x * frequency * 10.0) * cos(in.uv.y * frequency * 10.0);
    let mixed = mix(base, accent_color, uv_pattern * amplitude * 0.5 + 0.5);
    
    return mixed + final_color;
}
