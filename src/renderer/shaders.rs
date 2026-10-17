/// WGSL shader drawing the Game Boy screen as a single textured quad.
///
/// The texture holds the pixels in gamma space and the swapchain is not sRGB,
/// so the sampled colours are written out unchanged.
pub const SCREEN: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(position, 0.0, 1.0);
    output.tex_coords = tex_coords;
    return output;
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var screen_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(screen_texture, screen_sampler, input.tex_coords);
}
"#;
