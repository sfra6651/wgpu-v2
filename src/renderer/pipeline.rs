use crate::renderer::vertex::Vertex;

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PipelineType {
  Texture,
  Gridlines,
  SimpleRect,
  Count,
}

impl PipelineType {
  pub const COUNT: usize = PipelineType::Count as usize;
}

pub struct Pipeline {
  pub pipeline: wgpu::RenderPipeline,
}

pub struct CreatePipelineDesc<'a> {
  pub bind_group_layouts: &'a [Option<&'a wgpu::BindGroupLayout>],
  pub targets: &'a [Option<wgpu::ColorTargetState>],
  pub primitive: wgpu::PrimitiveState,
}

impl Pipeline {
  pub fn new(
    device: &wgpu::Device,
    shader_source: wgpu::ShaderSource,
    desc: CreatePipelineDesc,
  ) -> Option<Self> {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: None,
      source: shader_source,
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: None,
      bind_group_layouts: desc.bind_group_layouts,
      immediate_size: 0,
    });

    let buffer_layout = Vertex::desc();

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: None,
      layout: Some(&layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[buffer_layout],
        compilation_options: Default::default(),
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        compilation_options: Default::default(),
        targets: desc.targets,
      }),
      primitive: desc.primitive,
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview_mask: None,
      cache: None,
    });

    Some(Self { pipeline })
  }
}
