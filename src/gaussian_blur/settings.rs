use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Applies a gaussian blur effect to a 2d or 3d camera in conjonction with the [`GaussianBlurPlugin`]
///
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component, Default)]
pub struct GaussianBlurSettings {
    /// Kernel size for the computation of the gaussian blur
    /// - It will be clamped to the range [1..401]
    /// - It must be odd, else the first higher odd value will be used.
    /// - A value of 1 correspond to no blur, and will entirely skip the post-processing effect.
    /// - It defaults to 31 (sigma = 5).
    /// The associated `sigma` value for the gaussian function will be computed as `(kernel_size-1)/6`, so that the kernel extends to  a `3*sigma` range.
    ///
    /// The computational cost of the gaussian blur post-processing effect is `2*kernel_size` texture sampling per pixels.
    pub kernel_size: u32,
    /// A factor that is applied whenever the post-processing effect is sampling for a distant pixel.
    /// This can be used to create a bigger blur without impacting the computational cost, but sacrificing quality.
    /// - It will be clamped to the range [1..100]
    /// - Non integer values are supported, this will take advantage of linear filtering.
    /// - Defaults to 1, which is the neutral value, not impacting the algorithm.
    ///
    pub sampling_distance_factor: f32,
}
impl Default for GaussianBlurSettings {
    fn default() -> Self {
        Self {
            kernel_size: 31,
            sampling_distance_factor: 1.,
        }
    }
}
impl crate::BlurSetting for GaussianBlurSettings {
    const NO_BLUR: GaussianBlurSettings = GaussianBlurSettings {
        kernel_size: 1,
        sampling_distance_factor: 1.,
    };

    fn sampling_per_pixel(&self) -> f32 {
        match self.kernel_size {
            1 => 0.,
            k => (2 * k) as f32,
        }
    }

    fn passes(&self) -> u32 {
        match self.kernel_size {
            1 => 0,
            _ => 2,
        }
    }
}
impl GaussianBlurSettings {
    /// Computes a new `GaussianBlurSettings` where each attribute is legal as expected by the shader.
    ///
    /// It also replaces `KernelSize::Auto` by `KernelSize::Val(v)` where v is the first odd value higher than `4.*sigma`.
    pub fn create_concrete(&self) -> GaussianBlurSettings {
        let mut kernel_size = self.kernel_size.clamp(1, 401);
        if kernel_size % 2 == 0 {
            kernel_size += 1;
        }
        GaussianBlurSettings {
            kernel_size,
            sampling_distance_factor: self.sampling_distance_factor.clamp(1.0, 100.0),
        }
    }
}

impl ExtractComponent for GaussianBlurSettings {
    type QueryData = &'static Self;

    type QueryFilter = ();
    type Out = GaussianBlurUniforms;

    fn extract_component(settings: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        let settings = settings.create_concrete();
        if settings.kernel_size == 1 {
            None
        } else {
            let sigma = (settings.kernel_size - 1) as f32 / 6.0;
            Some(GaussianBlurUniforms {
                sigma,
                kernel_size: settings.kernel_size,
                sampling_distance_factor: settings.sampling_distance_factor,
                _webgl2_padding: 0.,
            })
        }
    }
}

/// The uniform struct extracted from [`GaussianBlurSettings`] attached to a Camera.
/// Will be available for use in the gaussian blur shader.
#[derive(Component, ShaderType, Clone)]
pub struct GaussianBlurUniforms {
    // Legalized kernel size.
    pub kernel_size: u32,
    // Computed sigma value based on kernel_size
    pub sigma: f32,
    // Legalized sampling_distance_factor
    pub sampling_distance_factor: f32,
    // webgl2 requires 16B padding
    pub _webgl2_padding: f32,
}
