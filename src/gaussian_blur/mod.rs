mod pipeline;
mod settings;
mod tweening;

pub use settings::GaussianBlurSettings;
#[cfg(feature = "bevy_tweening")]
pub use tweening::*;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};
use pipeline::*;

use self::settings::GaussianBlurUniforms;

const GAUSSIAN_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x3794890ac6fb4a5f87a69411d39c8fc7);

/// This plugins adds support for a gaussian blur post-processing effects to 2D or 3D cameras.
///
/// It must be used in conjonction with a [`GaussianBlurSettings`] component added to the Camera entity.
///
/// See algorithm details on [Wikipedia](https://en.wikipedia.org/wiki/Gaussian_blur).
/// Additional details on blur filters can be seen in this [Intel article](https://www.intel.com/content/www/us/en/developer/articles/technical/an-investigation-of-fast-real-time-gpu-based-image-blur-algorithms.html)
///
/// This implementation is done with 2 post-processing passes.
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
///
/// See [`GaussianBlurSettings`] for configurability.
pub struct GaussianBlurPlugin;

impl Plugin for GaussianBlurPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            GAUSSIAN_BLUR_SHADER_HANDLE,
            "gaussian_blur.wgsl",
            Shader::from_wgsl
        );
        app.register_type::<GaussianBlurSettings>();

        app.add_plugins((
            ExtractComponentPlugin::<GaussianBlurSettings>::default(),
            UniformComponentPlugin::<GaussianBlurUniforms>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Add gaussian blur to the 3d render graph;
            .add_render_graph_node::<ViewNodeRunner<GaussianBlurNode>>(Core3d, GaussianBlurLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    GaussianBlurLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            )
            // Add gaussian blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<GaussianBlurNode>>(Core2d, GaussianBlurLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    GaussianBlurLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<GaussianBlurPipeline>();
    }
}
