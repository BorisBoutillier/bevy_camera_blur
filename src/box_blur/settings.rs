use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Applies a box blur effect to a 2d or 3d camera.
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
///        BoxBlurSettings::default(),
///    ));
///}
///```
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component, Default)]
pub struct BoxBlurSettings {
    /// Kernel size for the computation of the box blur
    /// - This value  will be clamped to the range `[1..401]` and must be odd.
    ///   if not, it will be replaced by the first higher odd value.
    /// - A value of 1 disable the blurring effect.
    /// - Defaults to 21.
    ///
    /// The computational cost of the box blur post-processing effect is `2*kernel_size*passes` texture sampling per pixels.
    pub kernel_size: u32,
    /// Defines the number of time the box convolution is apply successively.
    /// Multiple passes increases the quality of the blur and reduce the 'box' artefacts.
    /// - This value will be clamped to the range [1..5]
    /// - Defaults to 2
    ///
    /// The computational cost of the box blur post-processing effect is `2*kernel_size*passes` texture sampling per pixels.
    pub passes: u32,
}
impl Default for BoxBlurSettings {
    fn default() -> Self {
        Self {
            kernel_size: 21,
            passes: 2,
        }
    }
}
impl crate::BlurSetting for BoxBlurSettings {
    const NO_BLUR: BoxBlurSettings = BoxBlurSettings {
        kernel_size: 1,
        passes: 1,
    };

    fn sampling_per_pixel(&self) -> f32 {
        match self.kernel_size {
            1 => 0.0,
            k => (2 * k * self.passes) as f32,
        }
    }

    fn passes(&self) -> u32 {
        match self.kernel_size {
            1 => 0,
            _ => self.passes,
        }
    }
}
impl BoxBlurSettings {
    /// Computes a new `BoxBlurSettings` where each attribute is legal as expected by the shader.
    pub fn create_concrete(&self) -> BoxBlurSettings {
        let mut kernel_size = self.kernel_size.clamp(1, 401);
        if kernel_size % 2 == 0 {
            kernel_size += 1;
        }
        let passes = self.passes.clamp(1, 5);
        BoxBlurSettings {
            kernel_size,
            passes,
        }
    }
}

impl ExtractComponent for BoxBlurSettings {
    type Query = &'static Self;

    type Filter = ();
    type Out = BoxBlurUniforms;

    fn extract_component(settings: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        let settings = settings.create_concrete();
        if settings.kernel_size == 1 {
            None
        } else {
            Some(BoxBlurUniforms {
                kernel_size: settings.kernel_size,
                passes: settings.passes,
                _webgl2_padding: Vec2::ZERO,
            })
        }
    }
}

/// The uniform struct extracted from [`BoxBlurSettings`] attached to a Camera.
/// Will be available for use in the box blur shader.
#[derive(Component, ShaderType, Clone)]
pub struct BoxBlurUniforms {
    pub kernel_size: u32,
    pub passes: u32,
    pub _webgl2_padding: Vec2,
}
