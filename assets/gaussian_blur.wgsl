// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
const TWO_PI:f32 = 6.28312;
const E:f32 = 2.71828;

fn gaussian_weight(x: i32, y: i32, spread: f32) -> f32 {
    let sigma_square = spread * spread;
    return (1.0 / sqrt(TWO_PI * sigma_square)) * pow(E, -f32(x * x + y * y) / (2.0 * sigma_square));
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    sigma: f32,
    kernel_size: i32,
    sample_rate: f32,
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: f32,
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment_horizontal(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let sigma = clamp(settings.sigma, 0.0, 15.0);
    if sigma < 0.01 {
        return textureSample(screen_texture, texture_sampler, in.uv);
    }
    var kernel_size = clamp(settings.kernel_size, 1, 100);
    if kernel_size % 2 == 0 {
        kernel_size += 1;
    };
    let upper = (kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    var weight_sum = 0.0;
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = clamp(settings.sample_rate, 1.0, 100.) / texture_size;
    let y = 0;
    for (var x = lower; x <= upper ; x ++) {
        let uv = in.uv + vec2<f32>(f32(x) * texel_size.x, f32(y) * texel_size.y);
        let weight = gaussian_weight(x, y, sigma);
        color += weight * textureSample(screen_texture, texture_sampler, uv);
        weight_sum += weight;
    }
    return color / weight_sum;
}
@fragment
fn fragment_vertical(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let sigma = clamp(settings.sigma, 0.0, 15.0);
    if sigma < 0.01 {
        return textureSample(screen_texture, texture_sampler, in.uv);
    }
    var kernel_size = clamp(settings.kernel_size, 1, 100);
    if kernel_size % 2 == 0 {
        kernel_size += 1;
    };
    let upper = (kernel_size - 1) / 2;
    let lower = -upper;
    var color = vec4(0.0);
    var weight_sum = 0.0;
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let texel_size = clamp(settings.sample_rate, 1.0, 100.) / texture_size;
    let x = 0;
    for (var y = lower; y <= upper ; y ++) {
        let uv = in.uv + vec2<f32>(f32(x) * texel_size.x, f32(y) * texel_size.y);
        let weight = gaussian_weight(x, y, sigma);
        color += weight * textureSample(screen_texture, texture_sampler, uv);
        weight_sum += weight;
    }
    return color / weight_sum;
}

