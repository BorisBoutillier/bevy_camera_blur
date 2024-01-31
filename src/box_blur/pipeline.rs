use super::settings::BoxBlurUniforms;
use super::BOX_BLUR_SHADER_HANDLE;
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::ComponentUniforms,
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
            BindGroupLayoutEntry, BindingType, CachedRenderPipelineId, ColorTargetState,
            ColorWrites, FragmentState, MultisampleState, Operations, PipelineCache,
            PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType, TextureViewDimension,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
    },
};
// The post process node used for the render graph
#[derive(Default)]
pub struct BoxBlurNode;
impl BoxBlurNode {
    pub const NAME: &'static str = "box_blur";
}

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for BoxBlurNode {
    type ViewQuery = (
        &'static ViewTarget,
        // This make sure the node is only run on cameras with an extracted BoxBlurUniform component
        &'static BoxBlurUniforms,
    );
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, box_blur_uniforms): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let box_blur_pipeline = world.resource::<BoxBlurPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let settings_uniforms = world.resource::<ComponentUniforms<BoxBlurUniforms>>();

        let (Some(settings_binding), Some(horizontal_pipeline), Some(vertical_pipeline)) = (
            settings_uniforms.uniforms().binding(),
            pipeline_cache.get_render_pipeline(box_blur_pipeline.horizontal_pipeline_id),
            pipeline_cache.get_render_pipeline(box_blur_pipeline.vertical_pipeline_id),
        ) else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("box_blur");

        for _ in 0..box_blur_uniforms.n_passes {
            for pipeline in [horizontal_pipeline, vertical_pipeline] {
                let post_process = view_target.post_process_write();

                let bind_group = render_context.render_device().create_bind_group(
                    "box_blur_bind_group",
                    &box_blur_pipeline.layout,
                    // It's important for this to match the BindGroupLayout defined in the BoxBlurPipeline
                    &BindGroupEntries::sequential((
                        // Make sure to use the source view
                        post_process.source,
                        // Use the sampler created for the pipeline
                        &box_blur_pipeline.sampler,
                        // Set the settings binding
                        settings_binding.clone(),
                    )),
                );

                let mut render_pass =
                    render_context.begin_tracked_render_pass(RenderPassDescriptor {
                        label: Some("box_blur_pass"),
                        color_attachments: &[Some(RenderPassColorAttachment {
                            // We need to specify the post process destination view here
                            // to make sure we write to the appropriate texture.
                            view: post_process.destination,
                            resolve_target: None,
                            ops: Operations::default(),
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                render_pass.set_render_pipeline(pipeline);
                render_pass.set_bind_group(0, &bind_group, &[]);
                render_pass.draw(0..3, 0..1);
            }
        }

        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
pub struct BoxBlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    horizontal_pipeline_id: CachedRenderPipelineId,
    vertical_pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for BoxBlurPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        // Bind group layout
        let layout = render_device.create_bind_group_layout(
            "box_blur_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    // The sampler that will be used to sample the screen texture
                    sampler(SamplerBindingType::Filtering),
                    // The settings uniform that will control the effect
                    uniform_buffer::<BoxBlurUniforms>(false),
                ),
            ),
        );

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let horizontal_pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("box_blur_horizontal_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: BOX_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment_horizontal".into(),
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
        let vertical_pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("box_blur_vertical_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: BOX_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment_vertical".into(),
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
            horizontal_pipeline_id,
            vertical_pipeline_id,
        }
    }
}
