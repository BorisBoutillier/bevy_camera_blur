mod pipeline;
mod settings;

pub use settings::KawaseBlurSettings;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};
use pipeline::*;

const KAWASE_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x25a6854386ee40c28864d2e724268b7a);

/// This plugins adds support for a Kawase blur post-processing effects to 2D or 3D cameras.
/// It must be used in conjonction with a [`KawaseBlurSettings`] component added to the Camera entity.
///
/// The Kawase blur is a close approximation to a gaussian blur that require less texture sampling per pixels but multiple passes.
/// It has been introduced by Masaki Kawase in his GDC2003 presentation “Frame Buffer Postprocessing Effects in DOUBLE-S.T.E.A.L (Wreckless)” [PPT](http://www.daionet.gr.jp/~masa/archives/GDC2003_DSTEAL.ppt).
/// Additional details on blur filters can be seen in this [Intel article](https://www.intel.com/content/www/us/en/developer/articles/technical/an-investigation-of-fast-real-time-gpu-based-image-blur-algorithms.html)
///
/// This implementation is done at full resolution, with one pass per sampling distance value.
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
///
/// See [`KawaseBlurSettings`] for configurability.
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
            .add_render_graph_node::<ViewNodeRunner<KawaseBlurNode>>(Core3d, KawaseBlurLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    KawaseBlurLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            )
            // Add kawase blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<KawaseBlurNode>>(Core2d, KawaseBlurLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    KawaseBlurLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<KawaseBlurPipeline>();
    }
}
