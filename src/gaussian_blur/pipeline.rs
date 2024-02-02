use super::settings::GaussianBlurUniforms;
use super::GAUSSIAN_BLUR_SHADER_HANDLE;
use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::ComponentUniforms,
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            BindGroupEntries, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, CachedRenderPipelineId, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
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
pub struct GaussianBlurNode;
impl GaussianBlurNode {
    pub const NAME: &'static str = "gaussian_blur";
}

// The ViewNode trait is required by the ViewNodeRunner
impl ViewNode for GaussianBlurNode {
    type ViewQuery = (
        &'static ViewTarget,
        // This make sure the node is only run on cameras with an extracted GaussianBlurUniform component
        &'static GaussianBlurUniforms,
    );
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _gaussian_blur_uniforms): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let gaussian_blur_pipeline = world.resource::<GaussianBlurPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();
        let settings_uniforms = world.resource::<ComponentUniforms<GaussianBlurUniforms>>();

        let (Some(settings_binding), Some(horizontal_pipeline), Some(vertical_pipeline)) = (
            settings_uniforms.uniforms().binding(),
            pipeline_cache.get_render_pipeline(gaussian_blur_pipeline.horizontal_pipeline_id),
            pipeline_cache.get_render_pipeline(gaussian_blur_pipeline.vertical_pipeline_id),
        ) else {
            return Ok(());
        };

        render_context
            .command_encoder()
            .push_debug_group("gaussian_blur");

        for pipeline in [horizontal_pipeline, vertical_pipeline] {
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

        render_context.command_encoder().pop_debug_group();

        Ok(())
    }
}

#[derive(Resource)]
pub struct GaussianBlurPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    horizontal_pipeline_id: CachedRenderPipelineId,
    vertical_pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for GaussianBlurPipeline {
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
        // Settings
        let settings = BindGroupLayoutEntry {
            binding: 2,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Buffer {
                ty: bevy::render::render_resource::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(GaussianBlurUniforms::min_size()),
            },
            count: None,
        };

        // Bind group layout
        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("gaussian_blur_bind_group_layout"),
            entries: &[texture, sampler, settings],
        });

        // We can create the sampler here since it won't change at runtime and doesn't depend on the view
        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let horizontal_pipeline_id = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("gaussian_blur_horizontal_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader: GAUSSIAN_BLUR_SHADER_HANDLE,
                    shader_defs: vec![],
                    entry_point: "fragment_horizontal".into(),
                    targets: vec![Some(TextureFormat::bevy_default().into())],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });
        let vertical_pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("gaussian_blur_vertical_pipeline".into()),
                    layout: vec![layout.clone()],
                    // This will setup a fullscreen triangle for the vertex state
                    vertex: fullscreen_shader_vertex_state(),
                    fragment: Some(FragmentState {
                        shader: GAUSSIAN_BLUR_SHADER_HANDLE,
                        shader_defs: vec![],
                        entry_point: "fragment_vertical".into(),
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
            horizontal_pipeline_id,
            vertical_pipeline_id,
        }
    }
}
