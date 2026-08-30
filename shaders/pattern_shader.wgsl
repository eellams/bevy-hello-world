// Pattern generation shader
// Creates various geometric patterns based on parameters

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

// Hash function for noise
fn hash(vec: vec2<f32>) -> f32 {
    let vec = vec * mat2<f32>(12.9898, 78.233, -12.9898, 78.233);
    return fract(sin(dot(vec, vec)) * 43758.5453);
}

// Smooth noise function
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    let a = hash(i);
    let b = hash(i + vec2<f32>(1.0, 0.0));
    let c = hash(i + vec2<f32>(0.0, 1.0));
    let d = hash(i + vec2<f32>(1.0, 1.0));
    
    let u = f * f * (3.0 - 2.0 * f);
    
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = input.uv * 2.0 - 1.0;
    let aspect = resolution.x / resolution.y;
    let uv_corrected = vec2<f32>(uv.x * aspect, uv.y);
    
    // Select pattern type based on custom_params.x
    let pattern_type = floor(custom_params.x * 4.0 + 0.5);
    
    let pattern: f32;
    
    // Different pattern types
    if (pattern_type == 0.0) {
        // Stripes
        let scale = 10.0 * custom_params.y;
        pattern = sin(uv_corrected.x * scale * 3.14159) * 0.5 + 0.5;
    } else if (pattern_type == 1.0) {
        // Checkerboard
        let scale = 5.0 * custom_params.y;
        let x = floor(uv_corrected.x * scale);
        let y = floor(uv_corrected.y * scale);
        pattern = mod(x + y, 2.0);
    } else if (pattern_type == 2.0) {
        // Circles
        let scale = 10.0 * custom_params.y;
        let v = uv_corrected * scale;
        pattern = sin(length(v) * 3.14159) * 0.5 + 0.5;
    } else {
        // Noise
        let scale = 5.0 * custom_params.y;
        pattern = noise(uv_corrected * scale + time * 0.1);
    }
    
    // Animate pattern based on time and custom_params.z
    let animated_pattern = pattern * (sin(time * custom_params.z) * 0.5 + 0.5);
    
    // Create color based on pattern and base color
    let final_color = color.rgb * animated_pattern;
    
    return vec4<f32>(final_color, 1.0);
}
