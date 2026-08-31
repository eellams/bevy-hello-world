// Shader Tool Default Shader
// This shader uses the uniforms defined in ShaderToolMaterial

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
