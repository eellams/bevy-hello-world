// Color manipulation shader
// Demonstrates color transformations and adjustments

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
    // Color transformation based on custom parameters
    let base_color = color.rgb;
    
    // Hue rotation
    let hue_shift = custom_params.x * 3.14159 * 2.0;
    let hue_rotation = mat3<f32>(
        0.5 + 0.5 * cos(hue_shift), -0.5 * sin(hue_shift), 0.5 - 0.5 * cos(hue_shift),
        0.5 * sin(hue_shift), 0.5 + 0.5 * cos(hue_shift), -0.5 * sin(hue_shift),
        0.5 - 0.5 * cos(hue_shift), 0.5 * sin(hue_shift), 0.5 + 0.5 * cos(hue_shift)
    );
    let rotated_color = hue_rotation * base_color;
    
    // Saturation adjustment
    let saturation = custom_params.y * 2.0;
    let gray = dot(rotated_color, vec3<f32>(0.3, 0.59, 0.11));
    let saturated_color = mix(vec3<f32>(gray), rotated_color, saturation);
    
    // Brightness adjustment
    let brightness = custom_params.z * 2.0;
    let bright_color = saturated_color * brightness;
    
    // Add time-based pulse
    let pulse = sin(time * 2.0) * 0.1 + 0.9;
    let final_color = bright_color * pulse;
    
    return vec4<f32>(final_color, color.a);
}
