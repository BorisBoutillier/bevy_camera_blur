use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Applies a kawase blur effect to a 2d or 3d camera if the [`KawaseBlurPlugin`] is active.
///
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component, Default)]
pub struct KawaseBlurSettings {
    /// Sampling distances for each consecutive filter pass.
    /// For a value of `d` the pixel value for this pass will be the mean value return for the pixel
    /// sampling done at the four corners `(d+0.5,d+0.5)`, `(d-0.5,d+0,5)`, `(d-0.5,d-0.5)` and `(d+0.5,d-0.5)`
    /// - Each value will be clamped to the range [0..9]
    pub sampling_distances: Vec<u32>,
}
impl Default for KawaseBlurSettings {
    fn default() -> Self {
        Self {
            sampling_distances: vec![0, 1, 2, 2, 3],
        }
    }
}
impl crate::BlurSetting for KawaseBlurSettings {
    const NO_BLUR: KawaseBlurSettings = KawaseBlurSettings {
        sampling_distances: vec![],
    };

    fn sampling_per_pixel(&self) -> f32 {
        (4 * self.sampling_distances.len()) as f32
    }

    fn passes(&self) -> u32 {
        self.sampling_distances.len() as u32
    }
}
impl KawaseBlurSettings {
    /// Computes a new `KawaseBlurSettings` where each attribute is legal as expected by the shader.
    pub fn create_concrete(&self) -> KawaseBlurSettings {
        let sampling_distances = self
            .sampling_distances
            .iter()
            .map(|&v| v.clamp(0, 9))
            .collect::<Vec<_>>();
        KawaseBlurSettings { sampling_distances }
    }
}

impl ExtractComponent for KawaseBlurSettings {
    type QueryData = &'static Self;

    type QueryFilter = ();
    type Out = KawaseBlurSettings;

    fn extract_component(settings: QueryItem<'_, Self::QueryData>) -> Option<Self::Out> {
        if settings.sampling_distances.is_empty() {
            None
        } else {
            Some(settings.create_concrete())
        }
    }
}

/// Data provided as Uniform for the shader.
#[derive(ShaderType, Clone, Default)]
pub struct KawaseBlurUniforms {
    // Sampling distance for the current pass.
    pub sampling_distance: f32,
    pub _webgl2_padding: Vec3,
}
