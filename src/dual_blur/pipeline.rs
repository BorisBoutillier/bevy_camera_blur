use super::{DualBlurTexture, DUAL_BLUR_SHADER_HANDLE};
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState,
            MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType,
            TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
    },
};
// The post process node used for the render graph
#[derive(Default)]
pub struct DualBlurNode;
impl DualBlurNode {
    pub const NAME: &'static str = "dual_blur";
}

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for DualBlurNode {
    type ViewQuery = (&'static ViewTarget, &'static DualBlurTexture);
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, dual_blur_texture): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let dual_blur_pipeline = world.resource::<DualBlurPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let (Some(downsample_pipeline), Some(upsample_pipeline)) = (
            pipeline_cache.get_render_pipeline(dual_blur_pipeline.downsample_pipeline_id),
            pipeline_cache.get_render_pipeline(dual_blur_pipeline.upsample_pipeline_id),
        ) else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("dual_blur");

        {
            let post_process = view_target.post_process_write();
            let texture_views = (0..dual_blur_texture.len())
                .map(|i| dual_blur_texture.view(i))
                .collect::<Vec<_>>();

            for i in 0..dual_blur_texture.len() {
                let source_view = if i == 0 {
                    post_process.source
                } else {
                    &texture_views[i - 1]
                };
                let destination_view = &texture_views[i];
                let bind_group = render_context.render_device().create_bind_group(
                    "dual_blur_bind_group",
                    &dual_blur_pipeline.layout,
                    // It's important for this to match the BindGroupLayout defined in the DualBlurPipeline
                    &BindGroupEntries::sequential((
                        // Make sure to use the source view
                        source_view,
                        // Use the sampler created for the pipeline
                        &dual_blur_pipeline.sampler,
                    )),
                );

                let mut render_pass =
                    render_context.begin_tracked_render_pass(RenderPassDescriptor {
                        label: Some("dual_blur_pass"),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            // We need to specify the post process destination view here
                            // to make sure we write to the appropriate texture.
                            view: destination_view,
                            resolve_target: None,
                            ops: Operations::default(),
                        })],
                        depth_stencil_attachment: None,
                    });

                render_pass.set_render_pipeline(downsample_pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..3, 0..1);
            }
            for i in (0..dual_blur_texture.len()).rev() {
                let source_view = &texture_views[i];
                let destination_view = if i == 0 {
                    post_process.destination
                } else {
                    &texture_views[i - 1]
                };
                let bind_group = render_context.render_device().create_bind_group(
                    "dual_blur_bind_group",
                    &dual_blur_pipeline.layout,
                    // It's important for this to match the BindGroupLayout defined in the DualBlurPipeline
                    &BindGroupEntries::sequential((
                        // Make sure to use the source view
                        source_view,
                        // Use the sampler created for the pipeline
                        &dual_blur_pipeline.sampler,
                    )),
                );

                let mut render_pass =
                    render_context.begin_tracked_render_pass(RenderPassDescriptor {
                        label: Some("dual_blur_pass"),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            // We need to specify the post process destination view here
                            // to make sure we write to the appropriate texture.
                            view: destination_view,
                            resolve_target: None,
                            ops: Operations::default(),
                        })],
                        depth_stencil_attachment: None,
                    });

                render_pass.set_render_pipeline(upsample_pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..3, 0..1);
            }
        }
        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
pub struct DualBlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    downsample_pipeline_id: CachedRenderPipelineId,
    upsample_pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for DualBlurPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // Input texture binding
        let texture = BindGroupLayoutEntry {
            binding: 0,
            ty: BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            visibility: ShaderStages::FRAGMENT,
            count: None,
        };

        // Sampler binding
        let sampler = BindGroupLayoutEntry {
            binding: 1,
            ty: BindingType::Sampler(SamplerBindingType::Filtering),
            visibility: ShaderStages::FRAGMENT,
            count: None,
        };

        // Bind group layout
        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("dual_blur_bind_group_layout"),
            entries: &[texture, sampler],
        });

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let downsample_pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("dual_blur_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: DUAL_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    entry_point: "fragment_downsample".into(),
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

        let upsample_pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("dual_blur_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: DUAL_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    entry_point: "fragment_upsample".into(),
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
            downsample_pipeline_id,
            upsample_pipeline_id,
        }
    }
}
