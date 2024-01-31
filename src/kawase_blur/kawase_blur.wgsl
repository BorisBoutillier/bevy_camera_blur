// This shader computes the one pass of the kawase blur effect

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct KawaseBlurUniforms {
    /// sampling distance for this pass of the Kawase filter
    sampling_distance: f32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>,
}
@group(0) @binding(2) var<uniform> settings: KawaseBlurUniforms;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    let d = settings.sampling_distance + 0.5;
    //let d = 0.5;
    var color = textureSample(screen_texture, texture_sampler, in.uv + vec2(d, d) * texel_size);
    color += textureSample(screen_texture, texture_sampler, in.uv + vec2(-d, d) * texel_size);
    color += textureSample(screen_texture, texture_sampler, in.uv + vec2(-d, -d) * texel_size);
    color += textureSample(screen_texture, texture_sampler, in.uv + vec2(d, -d) * texel_size);
    return color / 4.0;
}