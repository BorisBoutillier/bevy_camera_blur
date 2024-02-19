use super::settings::{KawaseBlurSettings, KawaseBlurUniforms};
use super::KAWASE_BLUR_SHADER_HANDLE;
use bevy::render::render_graph::RenderLabel;
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::render_resource::{BindGroupLayoutEntries, FilterMode, UniformBuffer};
use bevy::render::renderer::RenderQueue;
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            BindGroupEntries, BindGroupLayout, CachedRenderPipelineId, FragmentState,
            MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment,
            RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType,
            SamplerDescriptor, ShaderStages, TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::ViewTarget,
    },
};
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub(crate) struct KawaseBlurLabel;

// The post process node used for the render graph
#[derive(Default)]
pub(crate) struct KawaseBlurNode;

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for KawaseBlurNode {
    type ViewQuery = (&'static ViewTarget, &'static KawaseBlurSettings);
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, kawase_blur_settings): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let kawase_blur_pipeline = world.resource::<KawaseBlurPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let queue = world.resource::<RenderQueue>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(kawase_blur_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("kawase_blur");

        let mut uniform = UniformBuffer::<KawaseBlurUniforms>::default();
        for &sampling_distance in kawase_blur_settings.sampling_distances.iter() {
            uniform.set(KawaseBlurUniforms {
                sampling_distance: sampling_distance as f32,
                ..default()
            });
            uniform.write_buffer(render_context.render_device(), queue);
            let post_process = view_target.post_process_write();

            let bind_group = render_context.render_device().create_bind_group(
                "kawase_blur_bind_group",
                &kawase_blur_pipeline.layout,
                // It's important for this to match the BindGroupLayout defined in the KawaseBlurPipeline
                &BindGroupEntries::sequential((
                    // Make sure to use the source view
                    post_process.source,
                    // Use the sampler created for the pipeline
                    &kawase_blur_pipeline.sampler,
                    // Set the settings binding
                    uniform.binding().unwrap().clone(),
                )),
            );

            let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
                label: Some("kawase_blur_pass"),
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

        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
pub struct KawaseBlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for KawaseBlurPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "kawase_blur_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<KawaseBlurUniforms>(false),
                ),
            ),
        );

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor {
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..default()
        });

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("kawase_blur_pipeline".into()),
                layout: vec![layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: KAWASE_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(TextureFormat::bevy_default().into())],
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
            pipeline_id,
        }
    }
}
