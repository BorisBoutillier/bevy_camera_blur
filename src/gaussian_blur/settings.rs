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
/// See also <https://en.wikipedia.org/wiki/Gaussian_blur>.
#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component, Default)]
pub struct GaussianBlurSettings {
    /// Standard deviation (spread) of the blur
    /// It must be a positive number.
    /// A value inferior to 0.1 will have no blurring, and skip the post-process computation,
    pub sigma: f32,
    /// Kernel size for the computation of the gaussian blur.
    /// When set to `KernelSize::Auto` (default), the kernel_size will be computed as the first odd value higher than `4*sigma`.
    /// When set to `KernelSize::Val(v)`, `v` must be an odd value.
    pub kernel_size: KernelSize,
}
impl GaussianBlurSettings {
    /// Gaussian blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: GaussianBlurSettings = GaussianBlurSettings {
        sigma: 0.0,
        kernel_size: KernelSize::Auto,
    };
    /// Compute a new `GaussianBlurSettings` where the `kernel_size` is not `KernelSize::Auto`
    ///
    /// When `Auto` the `kernel_size` is computed as the first odd value higher than 4*sigma.
    pub fn make_concrete(&self) -> GaussianBlurSettings {
        assert!(
            self.sigma.is_sign_positive() && self.sigma.is_finite(),
            "Invalid 'sigma' value [{}], it must be a positive and finite float value.",
            self.sigma
        );
        assert!(match self.kernel_size {
    KernelSize::Auto => true,
    KernelSize::Val(v) => v%2==1,
},"Invalid 'kernel_size' value [{:?}]. When set to KernelSize::Val(v), 'v' must be an odd number.",self.kernel_size);

        let kernel_size = match self.kernel_size {
            KernelSize::Auto => {
                KernelSize::Val(GaussianBlurSettings::default_kernel_size(self.sigma))
            }
            KernelSize::Val(_) => self.kernel_size,
        };
        GaussianBlurSettings {
            sigma: self.sigma,
            kernel_size,
        }
    }
    fn default_kernel_size(sigma: f32) -> u32 {
        assert!(
            sigma.is_sign_positive() && sigma.is_finite(),
            "Invalid 'sigma' value [{}], it must be a positive and finite float value.",
            sigma
        );
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
        // FIXME:
        // Currently cannot filter out low sigma, as this trigger a fatal in bevy render/winit
        // When used with animation, when frames go from Some() to None
        if settings.sigma <= 0.1 {
            dbg!("None");
            None
        } else {
            //{
            let settings = settings.make_concrete();
            let kernel_size = match settings.kernel_size {
                KernelSize::Auto => panic!(),
                KernelSize::Val(v) => v,
            };
            dbg!(settings.sigma, kernel_size);
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
