use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};
/// Applies a gaussian blur effect to a 2d or 3d camera.
///
/// See also <https://en.wikipedia.org/wiki/Gaussian_blur>.
#[derive(Component, Reflect, Default, Clone, Copy, ExtractComponent, ShaderType)]
#[reflect(Component, Default)]
pub struct GaussianBlurSettings {
    /// Standard deviation (spread) of the blur
    /// Will be clamped to the range [0.0,100.0]
    /// A value inferior to 0.01 will trigger no blurring,
    /// and skip the whole post-process computation.
    pub sigma: f32,
    /// Kernel size for the computation of the gaussian blur.
    /// When set to 0 (default), the kernel_size will be computed based on the sigma value.
    pub kernel_size: u32,
    /// Defines the sample rate factor to use when reaching for pixels.
    /// Default value is 1.0, higher value will reach for pixels further away.
    /// The value will be clamped to the range [1.0, 100.0]
    /// An higher value allow to create a 'bigger' blur effect without requiring increasing the kernel_size
    pub sample_rate_factor: f32,
    /// WebGL2 structs must be 16 byte aligned.
    pub _webgl2_padding: f32,
}
impl GaussianBlurSettings {
    /// Gaussian blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: GaussianBlurSettings = GaussianBlurSettings {
        sigma: 0.0,
        kernel_size: 1,
        sample_rate_factor: 1.0,
        _webgl2_padding: 0.,
    };
}
