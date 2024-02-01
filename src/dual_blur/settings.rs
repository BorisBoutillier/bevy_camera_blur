use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;

/// Applies a dual blur effect to a 2d or 3d camera.
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
///        DualBlurSettings::default(),
///    ));
///}
///```
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component, Default)]
pub struct DualBlurSettings {
    /// TODO:
    pub n_downsampling_passes: u32,
}
impl Default for DualBlurSettings {
    fn default() -> Self {
        Self {
            n_downsampling_passes: 4,
        }
    }
}
impl DualBlurSettings {
    /// Dual blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: DualBlurSettings = DualBlurSettings {
        n_downsampling_passes: 0,
    };
    /// Computes a new `DualBlurSettings` where each attribute is legal as expected by the shader.
    pub fn create_concrete(&self) -> DualBlurSettings {
        let n_downsampling_passes = self.n_downsampling_passes.clamp(0, 8);
        DualBlurSettings {
            n_downsampling_passes,
        }
    }
}

impl ExtractComponent for DualBlurSettings {
    type Query = &'static Self;

    type Filter = ();
    type Out = DualBlurSettings;

    fn extract_component(settings: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        if settings.n_downsampling_passes == 0 {
            None
        } else {
            Some(settings.create_concrete())
        }
    }
}
