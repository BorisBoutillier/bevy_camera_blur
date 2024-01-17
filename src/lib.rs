#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

use bevy::{
    core_pipeline::{core_3d, fullscreen_vertex_shader::fullscreen_shader_vertex_state},
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
            MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderStages, ShaderType, TextureFormat, TextureSampleType,
            TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
        RenderApp,
    },
};

/// Add support for Gaussian Blur post-processing effects
pub struct GaussianBlurPlugin;

impl Plugin for GaussianBlurPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<GaussianBlurSettings>::default(),
            UniformComponentPlugin::<GaussianBlurSettings>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<GaussianBlurNode>>(
                core_3d::graph::NAME,
                GaussianBlurNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    GaussianBlurNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
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

// The post process node used for the render graph
#[derive(Default)]
struct GaussianBlurNode;
impl GaussianBlurNode {
    pub const NAME: &'static str = "gaussian_blur";
}

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for GaussianBlurNode {
    type ViewQuery = &'static ViewTarget;
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let gaussian_blur_pipeline = world.resource::<GaussianBlurPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline_x) =
            pipeline_cache.get_render_pipeline(gaussian_blur_pipeline.pipeline_x_id)
        else {
            return Ok(());
        };
        let Some(pipeline_y) =
            pipeline_cache.get_render_pipeline(gaussian_blur_pipeline.pipeline_y_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<GaussianBlurSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        for pipeline in [pipeline_x, pipeline_y] {
            let post_process = view_target.post_process_write();

            let bind_group = render_context.render_device().create_bind_group(
                "gaussian_blur_bind_group",
                &gaussian_blur_pipeline.layout,
                // It's important for this to match the BindGroupLayout defined in the GaussianBlurPipeline
                &BindGroupEntries::sequential((
                    // Make sure to use the source view
                    post_process.source,
                    // Use the sampler created for the pipeline
                    &gaussian_blur_pipeline.sampler,
                    // Set the settings binding
                    settings_binding.clone(),
                )),
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("gaussian_blur_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    // We need to specify the post process destination view here
                    // to make sure we write to the appropriate texture.
                    view: post_process.destination,
                    resolve_target: None,
                    ops: Operations::default(),
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_render_pipeline(pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        Ok(())
    }
}

#[derive(Resource)]
struct GaussianBlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_x_id: CachedRenderPipelineId,
    pipeline_y_id: CachedRenderPipelineId,
}

impl FromWorld for GaussianBlurPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // We need to define the bind group layout used for our pipeline
        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("gaussian_blur_bind_group_layout"),
            entries: &[
                // The screen texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // The sampler that will be used to sample the screen texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // The settings uniform that will control the effect
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(GaussianBlurSettings::min_size()),
                    },
                    count: None,
                },
            ],
        });

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        // Get the shader handle
        let shader = world.resource::<AssetServer>().load("gaussian_blur.wgsl");

        let pipeline_x_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("gaussian_blur_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment_x".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });
        let pipeline_y_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("gaussian_blur_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment_y".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_x_id,
            pipeline_y_id,
        }
    }
}

/// Component that, when added to a camera will trigger a gaussian blur post-processing effect.
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct GaussianBlurSettings {
    /// Standard deviation (spread) of the blur
    /// Will be clamped to the range [0.0,15.0]
    /// A value inferior to 0.01 will have not blur and skip any computation.
    pub sigma: f32,
    /// Kernel size for the computation of the gaussian blur.
    /// When set to 0 (default), the kernel_size will be computed based on the sigma value.
    pub kernel_size: u32,
    /// Defines the sample rate factor to use when reaching for pixels.
    /// Default value is 1.0, higher value will reach for pixels further away.
    /// The value will be clamped to the range [1.0, 100.0]
    /// An higher value allow to create a 'bigger' blur effect without requiring increasing the kernel_size
    pub sample_rate_factor: f32,
    /// WebGL2 structs must be 16 byte aligned.
    pub _webgl2_padding: f32,
}

//use bevy_tweening::Lens;
//pub struct GaussianBlurLens {
//    pub start: GaussianBlurSettings,
//    pub end: GaussianBlurSettings,
//}
//impl Lens<GaussianBlurSettings> for GaussianBlurLens {
//    fn lerp(&mut self, target: &mut GaussianBlurSettings, ratio: f32) {
//        target.sigma = self.start.sigma + (self.end.sigma - self.start.sigma) * ratio;
//        target.kernel_size = (self.start.kernel_size as f32
//            + (self.end.kernel_size as f32 - self.start.kernel_size as f32) * ratio)
//            as u32;
//        target.sample_rate_factor = self.start.sample_rate_factor
//            + (self.end.sample_rate_factor - self.start.sample_rate_factor) * ratio;
//    }
//}
