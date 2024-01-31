use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::{extract_component::ExtractComponent, render_resource::ShaderType};

/// Applies a kawase blur effect to a 2d or 3d camera.
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
///        KawaseBlurSettings::default(),
///    ));
///}
///```
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component, Default)]
pub struct KawaseBlurSettings {
    /// TODO:
    pub kernels: Vec<u32>,
}
impl Default for KawaseBlurSettings {
    fn default() -> Self {
        Self {
            //kernels: vec![0, 1, 2, 2, 3],
            kernels: vec![0, 1, 2, 3, 4, 4, 5, 6, 7],
        }
    }
}
impl KawaseBlurSettings {
    /// Kawase blur setting that will not trigger any blur post-processing
    pub const NO_BLUR: KawaseBlurSettings = KawaseBlurSettings { kernels: vec![] };
    /// Computes a new `KawaseBlurSettings` where each attribute is legal as expected by the shader.
    pub fn create_concrete(&self) -> KawaseBlurSettings {
        let kernels = self
            .kernels
            .iter()
            .map(|&v| v.clamp(0, 9))
            .collect::<Vec<_>>();
        KawaseBlurSettings { kernels }
    }
}

impl ExtractComponent for KawaseBlurSettings {
    type Query = &'static Self;

    type Filter = ();
    type Out = KawaseBlurSettings;

    fn extract_component(settings: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        if settings.kernels.is_empty() {
            None
        } else {
            Some(settings.create_concrete())
        }
    }
}

/// TODO:
#[derive(ShaderType, Clone, Default)]
pub struct KawaseBlurUniforms {
    pub kernel: f32,
    pub _webgl2_padding: Vec3,
}
