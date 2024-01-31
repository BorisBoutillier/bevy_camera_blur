mod pipeline;
mod settings;

pub use settings::KawaseBlurSettings;

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

use self::settings::KawaseBlurUniforms;

const KAWASE_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x25a6854386ee40c28864d2e724268b7a);

/// This plugins adds support for a Kawase blur post-processing effects to 2D or 3D cameras.
///
/// The Kawase blur is a close approximation to a gaussian blur that require less texture sampling per pixels.
///
/// It must be used in conjonction with the  [`KawaseBlurSettings`] component that must be added to any 2D or 3D Camera entity.
///
/// See [`KawaseBlurSettings`] for more details and example.
///
pub struct KawaseBlurPlugin;

impl Plugin for KawaseBlurPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            KAWASE_BLUR_SHADER_HANDLE,
            "kawase_blur.wgsl",
            Shader::from_wgsl
        );
        app.register_type::<KawaseBlurSettings>();

        app.add_plugins((ExtractComponentPlugin::<KawaseBlurSettings>::default(),));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Add kawase blur to the 3d render graph;
            .add_render_graph_node::<ViewNodeRunner<KawaseBlurNode>>(CORE_3D, KawaseBlurNode::NAME)
            .add_render_graph_edges(
                CORE_3D,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    KawaseBlurNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            )
            // Add kawase blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<KawaseBlurNode>>(CORE_2D, KawaseBlurNode::NAME)
            .add_render_graph_edges(
                CORE_2D,
                &[
                    core_2d::graph::node::TONEMAPPING,
                    KawaseBlurNode::NAME,
                    core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<KawaseBlurPipeline>();
    }
}
