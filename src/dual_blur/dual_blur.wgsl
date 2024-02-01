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

fn downsample(uv: vec2<f32>, halfpixel: vec2<f32>) -> vec4<f32> {
    var sum = textureSample(screen_texture, texture_sampler, uv) * 4.0;
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x + halfpixel.x, uv.y + halfpixel.y));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x + halfpixel.x, uv.y - halfpixel.y));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x - halfpixel.x, uv.y - halfpixel.y));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x - halfpixel.x, uv.y + halfpixel.y));
    return sum / 8.0;
}

@fragment
fn fragment_downsample(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    let halfpixel = texel_size * 0.5;
    return downsample(in.uv, halfpixel);
}

fn upsample(uv: vec2<f32>, halfpixel: vec2<f32>) -> vec4<f32> {
    var sum = textureSample(screen_texture, texture_sampler, vec2(uv.x - halfpixel.x * 2.0, uv.y));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x + halfpixel.x * 2.0, uv.y));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x, uv.y + halfpixel.y * 2.0));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x, uv.y - halfpixel.y * 2.0));
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x - halfpixel.x, uv.y + halfpixel.y)) * 2.0;
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x + halfpixel.x, uv.y + halfpixel.y)) * 2.0;
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x + halfpixel.x, uv.y - halfpixel.y)) * 2.0;
    sum += textureSample(screen_texture, texture_sampler, vec2(uv.x - halfpixel.x, uv.y - halfpixel.y)) * 2.0;
    return sum / 12.0;
}

@fragment
fn fragment_upsample(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    let halfpixel = texel_size * 0.5;
    return upsample(in.uv, halfpixel);
}