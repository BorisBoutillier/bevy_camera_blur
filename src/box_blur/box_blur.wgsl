// This shader computes the box blur effect

// The effect uses two passes, an horizontal pass and a vertical pass

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct BoxBlurUniforms {
    kernel_size: i32,
    _n_passes: i32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec2<f32>,
}
@group(0) @binding(2) var<uniform> settings: BoxBlurUniforms;

@fragment
fn fragment_horizontal(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let upper = (settings.kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    for (var x = lower; x <= upper ; x ++) {
        let uv = in.uv + vec2<f32>(f32(x) * texel_size.x, 0.);
        color += textureSample(screen_texture, texture_sampler, uv);
    }
    return color / f32(settings.kernel_size);
}
@fragment
fn fragment_vertical(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let upper = (settings.kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    for (var y = lower; y <= upper ; y ++) {
        let uv = in.uv + vec2<f32>(0., f32(y) * texel_size.y);
        color += textureSample(screen_texture, texture_sampler, uv);
    }
    return color / f32(settings.kernel_size);
}

