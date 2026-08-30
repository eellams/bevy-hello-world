// Test shader for Bevy shader testing framework
// This is a basic shader that demonstrates various features

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> color: vec4<f32>;

@group(0) @binding(1)
var<uniform> time: f32;

@group(0) @binding(2)
var<uniform> resolution: vec2<f32>;

@group(0) @binding(3)
var<uniform> mouse_position: vec2<f32>;

@group(0) @binding(4)
var<uniform> custom_params: vec4<f32>;

@vertex
fn vs_main(
    model: mat4<f32>,
    view: mat4<f32>,
    projection: mat4<f32>,
    vertex: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = projection * view * model * vec4<f32>(vertex.position, 1.0);
    output.uv = vertex.uv;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Use custom parameters to create interesting patterns
    let uv = input.uv * 2.0 - 1.0;
    
    // Create a pulsing effect based on time and custom parameter
    let pulse = sin(time * custom_params.x) * 0.5 + 0.5;
    
    // Create a gradient based on UV
    let gradient = vec3<f32>(uv.x * 0.5 + 0.5, uv.y * 0.5 + 0.5, 0.5);
    
    // Mix with custom color
    let mixed_color = mix(color.rgb, gradient, custom_params.y);
    
    // Add some pattern based on custom parameters
    let pattern = sin(uv.x * 10.0 * custom_params.z) * cos(uv.y * 10.0 * custom_params.w);
    
    // Combine everything
    let final_color = mixed_color * (pulse + pattern * 0.2);
    
    return vec4<f32>(final_color, 1.0);
}
