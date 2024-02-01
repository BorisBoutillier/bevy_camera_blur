mod pipeline;
mod settings;
mod tweening;

pub use settings::GaussianBlurSettings;
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

use self::settings::GaussianBlurUniforms;

const GAUSSIAN_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x3794890ac6fb4a5f87a69411d39c8fc7);

/// This plugins adds support for a gaussian blur post-processing effects to 2D or 3D cameras.
///
/// It must be used in conjonction with the  [`GaussianBlurSettings`] component that must be added to any 2D or 3D Camera entity.
///
/// See [`GaussianBlurSettings`] for more details and example.
///
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
            .add_render_graph_node::<ViewNodeRunner<GaussianBlurNode>>(
                CORE_3D,
                GaussianBlurNode::NAME,
            )
            .add_render_graph_edges(
                CORE_3D,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    GaussianBlurNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            )
            // Add gaussian blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<GaussianBlurNode>>(
                CORE_2D,
                GaussianBlurNode::NAME,
            )
            .add_render_graph_edges(
                CORE_2D,
                &[
                    core_2d::graph::node::TONEMAPPING,
                    GaussianBlurNode::NAME,
                    core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<GaussianBlurPipeline>();
    }
}
