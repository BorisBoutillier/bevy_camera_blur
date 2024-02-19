use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

/// Applies a dual blur effect to a 2d or 3d camera.
///
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component, Default)]
pub struct DualBlurSettings {
    /// Defines the number of downsampling passes to do. There will be an equivalent number of upsampling passes.
    /// Each of these passes will use the kernel described by Marius Bjorge in his presentation.
    /// - It will be clamped to the range [0..8]
    /// - Defaults to 4
    pub downsampling_passes: u32,
}
impl Default for DualBlurSettings {
    fn default() -> Self {
        Self {
            downsampling_passes: 4,
        }
    }
}
impl crate::BlurSetting for DualBlurSettings {
    const NO_BLUR: DualBlurSettings = DualBlurSettings {
        downsampling_passes: 0,
    };

    fn sampling_per_pixel(&self) -> f32 {
        // For each pass there is 5 for downsampling but at image size/4 + 8 for upsampling.
        (0..self.downsampling_passes).fold(0.0, |samplings, pass| {
            samplings + (5.0 / 4.0 + 8.0) / (4.0_f32.powi(pass as i32))
        })
    }

    fn passes(&self) -> u32 {
        self.downsampling_passes * 2
    }
}
impl DualBlurSettings {
    /// Computes a new `DualBlurSettings` where each attribute is legal as expected by the shader.
    pub fn create_concrete(&self) -> DualBlurSettings {
        let downsampling_passes = self.downsampling_passes.clamp(0, 8);
        DualBlurSettings {
            downsampling_passes,
        }
    }
}

impl ExtractComponent for DualBlurSettings {
    type QueryData = &'static Self;

    type QueryFilter = ();
    type Out = DualBlurSettings;

    fn extract_component(settings: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        if settings.downsampling_passes == 0 {
            None
        } else {
            Some(settings.create_concrete())
        }
    }
}
