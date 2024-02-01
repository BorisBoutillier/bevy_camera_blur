use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Applies a gaussian blur effect to a 2d or 3d camera.
///
/// See algorithm details: <https://en.wikipedia.org/wiki/Gaussian_blur>.
///
/// It must be added as a Component to a 2D or 3D Camera
///
/// ```
///# use bevy::prelude::*;
///# use bevy_camera_blur::*;
///
///pub fn setup(mut commands: Commands) {
///    commands.spawn((
///        Camera2dBundle::default(),
///        GaussianBlurSettings::default(),
///    ));
///}
///```
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component, Default)]
pub struct GaussianBlurSettings {
    /// Kernel size for the computation of the gaussian blur
    /// - It will be clamped to the range [1..401]
    /// - It must be odd, else the first higher odd value will be used.
    /// - A value of 1 will entirely skip the post-processing effect.
    /// - It defaults to 31 (sigma = 5).
    /// The associated `sigma` value for the gaussian function will be computed as `(kernel_size-1)/6`, so that the kernel extends to  a `3*sigma` range.
    ///
    /// The computational cost of the gaussian blur post-processing effect is `2*kernel_size`` texture sampling per pixels.
    pub kernel_size: u32,
}
impl Default for GaussianBlurSettings {
    fn default() -> Self {
        Self { kernel_size: 31 }
    }
}
impl GaussianBlurSettings {
    /// Gaussian blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: GaussianBlurSettings = GaussianBlurSettings { kernel_size: 1 };
    /// Computes a new `GaussianBlurSettings` where each attribute is legal as expected by the shader.
    ///
    /// It also replaces `KernelSize::Auto` by `KernelSize::Val(v)` where v is the first odd value higher than `4.*sigma`.
    pub fn create_concrete(&self) -> GaussianBlurSettings {
        let mut kernel_size = self.kernel_size.clamp(1, 401);
        if kernel_size % 2 == 0 {
            kernel_size += 1;
        }
        GaussianBlurSettings { kernel_size }
    }
}

impl ExtractComponent for GaussianBlurSettings {
    type Query = &'static Self;

    type Filter = ();
    type Out = GaussianBlurUniforms;

    fn extract_component(settings: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        let settings = settings.create_concrete();
        if settings.kernel_size == 1 {
            None
        } else {
            let sigma = (settings.kernel_size - 1) as f32 / 6.0;
            Some(GaussianBlurUniforms {
                sigma,
                kernel_size: settings.kernel_size,
                _webgl2_padding: Vec2::ZERO,
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
    // webgl2 requires 16B padding
    pub _webgl2_padding: Vec2,
}
