mod pipeline;
mod settings;
mod tweening;

pub use settings::BoxBlurSettings;
#[cfg(feature = "bevy_tweening")]
pub use tweening::*;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d::{self, CORE_2D},
    core_pipeline::core_3d::{self, CORE_3D},
    prelude::*,
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};
use pipeline::*;

use self::settings::BoxBlurUniforms;

const BOX_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xb95e014dc9aa489d8896aa486b01c666);

/// This plugins adds support for a box blur post-processing effects to 2D or 3D cameras.
///
/// It must be used in conjonction with a [`BoxBlurSettings`] component added to the Camera entity.
///
/// See algorithm details on [Wikipedia](https://en.wikipedia.org/wiki/Box_blur).
/// Additional details on blur filters can be found in this [Intel article](https://www.intel.com/content/www/us/en/developer/articles/technical/an-investigation-of-fast-real-time-gpu-based-image-blur-algorithms.html)
///
/// This implementation is done with 2 post-processing passes.
///
/// ```
///# use bevy::prelude::*;
///# use bevy_camera_blur::*;
///
///fn setup(mut commands: Commands) {
///    commands.spawn((
///        Camera2dBundle::default(),
///        BoxBlurSettings::default(),
///    ));
///}
///```
///
/// See [`BoxBlurSettings`] for configurability.
///
pub struct BoxBlurPlugin;

impl Plugin for BoxBlurPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            BOX_BLUR_SHADER_HANDLE,
            "box_blur.wgsl",
            Shader::from_wgsl
        );
        app.register_type::<BoxBlurSettings>();

        app.add_plugins((
            ExtractComponentPlugin::<BoxBlurSettings>::default(),
            UniformComponentPlugin::<BoxBlurUniforms>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Add box blur to the 3d render graph;
            .add_render_graph_node::<ViewNodeRunner<BoxBlurNode>>(CORE_3D, BoxBlurNode::NAME)
            .add_render_graph_edges(
                CORE_3D,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    BoxBlurNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            )
            // Add box blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<BoxBlurNode>>(CORE_2D, BoxBlurNode::NAME)
            .add_render_graph_edges(
                CORE_2D,
                &[
                    core_2d::graph::node::TONEMAPPING,
                    BoxBlurNode::NAME,
                    core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<BoxBlurPipeline>();
    }
}
