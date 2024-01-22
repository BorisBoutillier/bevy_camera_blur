use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Used to define the size of the kernel for an effect
#[derive(Clone, Copy, PartialEq, Eq, Debug, Reflect, Default)]
pub enum KernelSize {
    /// The size of the kernel is computed based on the value of other
    /// settings of the effect
    #[default]
    Auto,
    /// Explicitely define the size of the kernel to use for the effect
    Val(u32),
}

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
///        GaussianBlurSettings {
///            sigma: 4.,
///            ..default()
///        },
///    ));
///}
///```
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component, Default)]
pub struct GaussianBlurSettings {
    /// Standard deviation (spread) of the blur.
    /// - This value will be clamp to the range `[0.0,100.0]`
    ///  - A value inferior to 0.1 will create no blurring
    ///  - Defaults to 10.0
    pub sigma: f32,
    /// Kernel size for the computation of the gaussian blur
    /// - `KernelSize::Auto`: [default]. the kernel_size will be computed as the first odd value higher than `4*sigma`
    /// - `KernelSize::Val(v)`: The explicit size of the kernel
    ///     `v` will be clamped to the range `[1..401]` and must be odd. if not, it will be replaced by the first higher odd value
    ///
    /// The computational cost of the gaussian blur post-processing effect is 2*kernel_size texture sampling per pixels.
    pub kernel_size: KernelSize,
}
impl Default for GaussianBlurSettings {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            kernel_size: KernelSize::Auto,
        }
    }
}
impl GaussianBlurSettings {
    /// Gaussian blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: GaussianBlurSettings = GaussianBlurSettings {
        sigma: 0.0,
        kernel_size: KernelSize::Auto,
    };
    /// Computes a new `GaussianBlurSettings` where each attribute is legal as expected by the shader.
    ///
    /// It also replaces `KernelSize::Auto` by `KernelSize::Val(v)` where v is the first odd value higher than `4.*sigma`.
    pub fn create_concrete(&self) -> GaussianBlurSettings {
        let sigma = self.sigma.clamp(0.0, 100.0);
        let kernel_size = match self.kernel_size {
            KernelSize::Auto => KernelSize::Val(GaussianBlurSettings::default_kernel_size(sigma)),
            KernelSize::Val(v) => {
                let v = v.clamp(1, 401);
                KernelSize::Val(if v % 2 == 1 { v } else { v + 1 })
            }
        };
        GaussianBlurSettings {
            sigma: self.sigma,
            kernel_size,
        }
    }
    fn default_kernel_size(sigma: f32) -> u32 {
        let v = (4. * sigma).ceil() as u32;
        if v % 2 == 0 {
            v + 1
        } else {
            v
        }
    }
}

impl ExtractComponent for GaussianBlurSettings {
    type Query = &'static Self;

    type Filter = ();
    type Out = GaussianBlurUniforms;

    fn extract_component(settings: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        let settings = settings.create_concrete();
        if settings.sigma <= 0.1 {
            None
        } else {
            let kernel_size = match settings.kernel_size {
                KernelSize::Auto => panic!(),
                KernelSize::Val(v) => v,
            };
            Some(GaussianBlurUniforms {
                sigma: settings.sigma,
                kernel_size,
                _webgl2_padding: Vec2::ZERO,
            })
        }
    }
}

/// The uniform struct extracted from [`GaussianBlurSettings`] attached to a Camera.
/// Will be available for use in the gaussian blur shader.
#[derive(Component, ShaderType, Clone)]
pub struct GaussianBlurUniforms {
    pub sigma: f32,
    pub kernel_size: u32,
    pub _webgl2_padding: Vec2,
}
