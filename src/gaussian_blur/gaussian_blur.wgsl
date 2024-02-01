// This shader computes the gaussian blur effect

// The effect uses two passes, an horizontal pass and a vertical pass

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

const SQ_TWO_PI:f32 = 2.5066282;
const E:f32 = 2.71828;

// Compute the the gaussian weight of the value 'v' given the 'sigma'.
fn gaussian_weight(v: i32, sigma: f32) -> f32 {
    let sigma_square = sigma * sigma;
    return (1.0 / (SQ_TWO_PI * sigma)) * pow(E, -f32(v * v) / (2.0 * sigma_square));
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct GaussianBlurUniforms {
    kernel_size: i32,
    sigma: f32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec2<f32>,
}
@group(0) @binding(2) var<uniform> settings: GaussianBlurUniforms;

@fragment
fn fragment_horizontal(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let upper = (settings.kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    var weight_sum = 0.0;
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    for (var x = lower; x <= upper ; x ++) {
        let uv = in.uv + vec2<f32>(f32(x) * texel_size.x, 0.);
        let weight = gaussian_weight(x, settings.sigma);
        color += weight * textureSample(screen_texture, texture_sampler, uv);
        weight_sum += weight;
    }
    return color / weight_sum;
}
@fragment
fn fragment_vertical(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let upper = (settings.kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    var weight_sum = 0.0;
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = 1.0 / texture_size;
    for (var y = lower; y <= upper ; y ++) {
        let uv = in.uv + vec2<f32>(0.0, f32(y) * texel_size.y);
        let weight = gaussian_weight(y, settings.sigma);
        color += weight * textureSample(screen_texture, texture_sampler, uv);
        weight_sum += weight;
    }
    return color / weight_sum;
}

