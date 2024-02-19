mod pipeline;
mod settings;

pub use settings::DualBlurSettings;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
    },
    prelude::*,
    render::{
        camera::ExtractedCamera,
        extract_component::ExtractComponentPlugin,
        render_graph::{RenderGraphApp, ViewNodeRunner},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
            TextureView, TextureViewDescriptor,
        },
        renderer::RenderDevice,
        texture::{BevyDefault, CachedTexture, TextureCache},
        Render, RenderApp, RenderSet,
    },
};
use pipeline::*;

const DUAL_BLUR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x44c57a955745419aadd439a609c5c191);

/// This plugins adds support for a Dual blur post-processing effects to 2D or 3D cameras.
///
/// It must be used in conjonction with a [`DualBlurSettings`] component added to the Camera entity.
///
/// It implements the "Dual Kawase Blur" algorithm from Marius Bjorge as described in his [SIGGRAPH 2015 presentation](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjj2e69hauEAxXaTqQEHWaoB9AQFnoECBIQAQ&url=https%3A%2F%2Fcommunity.arm.com%2Fcfs-file%2F__key%2Fcommunityserver-blogs-components-weblogfiles%2F00-00-00-20-66%2Fsiggraph2015_2D00_mmg_2D00_marius_2D00_slides.pdf&usg=AOvVaw1Ks3_FP92nnFhANxeIlBAS&opi=89978449)
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
///
/// See [`DualBlurSettings`] for configurability.
///
pub struct DualBlurPlugin;

impl Plugin for DualBlurPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            DUAL_BLUR_SHADER_HANDLE,
            "dual_blur.wgsl",
            Shader::from_wgsl
        );
        app.register_type::<DualBlurSettings>();

        app.add_plugins((ExtractComponentPlugin::<DualBlurSettings>::default(),));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_systems(
                Render,
                (prepare_dual_blur_textures.in_set(RenderSet::PrepareResources),),
            )
            // Add dual blur to the 3d render graph;
            .add_render_graph_node::<ViewNodeRunner<DualBlurNode>>(Core3d, DualBlurLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    DualBlurLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            )
            // Add dual blur to the 2d render graph
            .add_render_graph_node::<ViewNodeRunner<DualBlurNode>>(Core2d, DualBlurLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    DualBlurLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<DualBlurPipeline>();
    }
}

fn prepare_dual_blur_textures(
    mut commands: Commands,
    mut texture_cache: ResMut<TextureCache>,
    render_device: Res<RenderDevice>,
    views: Query<(Entity, &ExtractedCamera, &DualBlurSettings)>,
) {
    for (entity, camera, settings) in &views {
        if let Some(UVec2 {
            x: width,
            y: height,
        }) = camera.physical_viewport_size
        {
            let mut textures = vec![];
            for i in 0..settings.downsampling_passes {
                let texture_descriptor = TextureDescriptor {
                    label: Some("dual_blur_texture"),
                    size: Extent3d {
                        width: (width >> i).max(1),
                        height: (height >> i).max(1),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::bevy_default(),
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                };

                textures.push(texture_cache.get(&render_device, texture_descriptor));
            }

            commands.entity(entity).insert(DualBlurTexture { textures });
        }
    }
}
#[derive(Component)]
pub(crate) struct DualBlurTexture {
    textures: Vec<CachedTexture>,
}

impl DualBlurTexture {
    fn view(&self, index: usize) -> TextureView {
        self.textures[index]
            .texture
            .create_view(&TextureViewDescriptor {
                base_mip_level: 0,
                mip_level_count: Some(1u32),
                ..Default::default()
            })
    }
    fn len(&self) -> usize {
        self.textures.len()
    }
}
