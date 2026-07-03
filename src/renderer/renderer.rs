use std::sync::Arc;

use wgpu::{ColorTargetState, CurrentSurfaceTexture};
use winit::window::Window;

use crate::{
  renderer::{
    model_transforms::ModelTransforms,
    pipeline::{CreatePipelineDesc, Pipeline, PipelineType, create_pipeline},
    texture::Texture,
    uniform::{UniformManager, UniformVariant},
    vertex::{Vertex, grid_vertices, unit_square},
  },
  world::World,
};

pub struct Renderer {
  instance: wgpu::Instance,
  pub surface: wgpu::Surface<'static>,
  pub adapter: wgpu::Adapter,
  pub device: wgpu::Device,
  queue: wgpu::Queue,
  shape_render_pipeline: Option<wgpu::RenderPipeline>,
  shape_render_pipeline_layout: Option<wgpu::PipelineLayout>,
  texture_render_pipeline: Option<wgpu::RenderPipeline>,
  texture_render_pipeline_layout: Option<wgpu::PipelineLayout>,
  grid_render_pipeline_layout: Option<wgpu::PipelineLayout>,
  grid_render_pipeline: Option<wgpu::RenderPipeline>,
  grid_vertex_count: u32,
  square_buffer: wgpu::Buffer,
  grid_buffer: Option<wgpu::Buffer>,
  pipelines: [Option<Pipeline>; PipelineType::COUNT],
  pub uniform_manager: UniformManager,
  pub model_transforms: ModelTransforms,
  pub texture: Texture,
}

impl Renderer {
  pub async fn new(window: Arc<Window>) -> Self {
    let descriptor = wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY,
      flags: Default::default(),
      memory_budget_thresholds: Default::default(),
      backend_options: Default::default(),
      display: None,
    };

    let instance = wgpu::Instance::new(descriptor);

    let surface = instance.create_surface(window.clone()).unwrap();

    let adapter = instance
      .request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        // Request an adapter which can render to our surface
        compatible_surface: Some(&surface),
      })
      .await
      .expect("Renderer creation err: Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
      .request_device(&wgpu::DeviceDescriptor {
        label: None,
        required_features: wgpu::Features::empty(),
        // Make sure we use the texture resolution limits from the adapter,
        // so we can support images the size of the swapchain.
        required_limits: wgpu::Limits::defaults(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::MemoryUsage,
        trace: wgpu::Trace::Off,
      })
      .await
      .expect("Renderer cration err: Failed to create device");

    let config = surface
      .get_default_config(
        &adapter,
        window.inner_size().width,
        window.inner_size().height,
      )
      .unwrap();
    surface.configure(&device, &config);

    let square_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("square buffer"),
      size: size_of::<Vertex>() as u64 * 6,
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let uniform_manager = UniformManager::new(&device);

    let model_transforms = ModelTransforms::new(&device);

    let texture = Texture::new(&device, &queue);

    Self {
      surface,
      instance,
      adapter,
      device,
      queue,
      shape_render_pipeline: None,
      shape_render_pipeline_layout: None,
      texture_render_pipeline: None,
      texture_render_pipeline_layout: None,
      grid_render_pipeline_layout: None,
      grid_render_pipeline: None,
      grid_buffer: None,
      grid_vertex_count: 0,
      square_buffer,
      pipelines: std::array::from_fn(|_| None),
      uniform_manager,
      model_transforms,
      texture,
    }
  }

  pub fn create_pipelines(&mut self) {
    let swapchain_format = self.surface.get_capabilities(&self.adapter).formats[0];

    self.pipelines[PipelineType::Texture as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/texture.wgsl").into()),
      CreatePipelineDesc {
        pipeline_type: PipelineType::Texture,
        bind_group_layouts: &[
          Some(
            &self
              .uniform_manager
              .get(UniformVariant::Projection)
              .bind_group_layout,
          ),
          Some(&self.model_transforms.bind_group_layout),
          Some(&self.texture.bind_group_layout),
        ],
        targets: &[Some(ColorTargetState {
          format: swapchain_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        primitive: wgpu::PrimitiveState::default(),
      },
    );

    self.pipelines[PipelineType::Gridlines as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/grid.wgsl").into()),
      CreatePipelineDesc {},
    );

    self.pipelines[PipelineType::SimpleRect as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/square.wgsl").into()),
      CreatePipelineDesc {},
    );
  }

  pub fn create_grid_render_pipeline(&mut self) -> &mut Self {
    let shader = self
      .device
      .create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("grid shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/grid.wgsl").into()),
      });

    let pipeline_layout = self
      .device
      .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("grid pipeline layout"),
        bind_group_layouts: &[Some(
          &self
            .uniform_manager
            .get(UniformVariant::Projection)
            .bind_group_layout,
        )],
        immediate_size: 0,
      });

    let swapchain_format = self.surface.get_capabilities(&self.adapter).formats[0];

    let render_pipeline = self
      .device
      .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("grid pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
          module: &shader,
          entry_point: Some("vs_main"),
          buffers: &[Vertex::desc()],
          compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
          module: &shader,
          entry_point: Some("fs_main"),
          compilation_options: Default::default(),
          targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::LineList,
          ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
      });

    self.grid_render_pipeline = Some(render_pipeline);
    self.grid_render_pipeline_layout = Some(pipeline_layout);
    self
  }

  pub fn upload_grid(&mut self, size: glam::Vec2, spacing: f32) {
    let verts = grid_vertices(size, spacing);
    let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("grid buffer"),
      size: (size_of::<Vertex>() * verts.len()) as u64,
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });
    self
      .queue
      .write_buffer(&buffer, 0, bytemuck::cast_slice(&verts));
    self.grid_vertex_count = verts.len() as u32;
    self.grid_buffer = Some(buffer);
  }

  pub fn create_texture_render_pipline(&mut self) -> &mut Self {
    let shader = self
      .device
      .create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/texture.wgsl").into()),
      });

    let pipeline_layout = self
      .device
      .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
          Some(
            &self
              .uniform_manager
              .get(UniformVariant::Projection)
              .bind_group_layout,
          ),
          Some(&self.model_transforms.bind_group_layout),
          Some(&self.texture.bind_group_layout),
        ],
        immediate_size: 0,
      });

    let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let buffer_layout = Vertex::desc();

    let render_pipeline = self
      .device
      .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
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
          targets: &[Some(ColorTargetState {
            format: swapchain_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
          })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
      });

    self.texture_render_pipeline = Some(render_pipeline);
    self.texture_render_pipeline_layout = Some(pipeline_layout);

    self
  }

  pub fn create_shape_render_pipeline(&mut self) -> &mut Self {
    let shader = self
      .device
      .create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/square.wgsl").into()),
      });

    let pipeline_layout = self
      .device
      .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
          Some(
            &self
              .uniform_manager
              .get(UniformVariant::Projection)
              .bind_group_layout,
          ),
          Some(&self.model_transforms.bind_group_layout),
        ],
        immediate_size: 0,
      });

    let swapchain_capabilities = self.surface.get_capabilities(&self.adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let buffer_layout = Vertex::desc();

    let render_pipeline = self
      .device
      .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
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
          targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
      });

    self.shape_render_pipeline = Some(render_pipeline);
    self.shape_render_pipeline_layout = Some(pipeline_layout);

    self
  }

  pub fn upload_square(&self) {
    let square = unit_square();
    self
      .queue
      .write_buffer(&self.square_buffer, 0, bytemuck::cast_slice(&square));
  }

  pub fn render(&mut self, window: &Window, world: &World) {
    self
      .model_transforms
      .write_transforms(&self.queue, &world.entities);

    let Some(texture_pipeline) = &self.texture_render_pipeline else {
      return;
    };

    let frame = match self.surface.get_current_texture() {
      CurrentSurfaceTexture::Success(frame) => frame,
      CurrentSurfaceTexture::Occluded => {
        window.request_redraw();
        return;
      }
      other => {
        println!("{:?}", other);
        panic!("failed to get surface texture for frame")
      }
    };

    let width = window.inner_size().width;
    let height = window.inner_size().height;
    let aspect = width as f32 / height.max(1) as f32;
    let view_proj = world.camera.view_projection(aspect);
    self.queue.write_buffer(
      &self.uniform_manager.get(UniformVariant::Projection).buffer,
      0,
      bytemuck::cast_slice(&[view_proj.to_cols_array()]),
    );

    let view = frame
      .texture
      .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self
      .device
      .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
      let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          depth_slice: None,
          resolve_target: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
      });

      if let (Some(grid_pipeline), Some(grid_buffer)) =
        (&self.grid_render_pipeline, &self.grid_buffer)
      {
        rpass.set_pipeline(grid_pipeline);
        rpass.set_bind_group(
          0,
          &self
            .uniform_manager
            .get(UniformVariant::Projection)
            .bind_group,
          &[],
        );
        rpass.set_vertex_buffer(0, grid_buffer.slice(..));
        rpass.draw(0..self.grid_vertex_count, 0..1); // ONE instance
      }

      //rpass.set_pipeline(pipeline);
      rpass.set_pipeline(texture_pipeline);
      rpass.set_vertex_buffer(0, self.square_buffer.slice(..));
      rpass.set_bind_group(
        0,
        &self
          .uniform_manager
          .get(UniformVariant::Projection)
          .bind_group,
        &[],
      );
      rpass.set_bind_group(1, &self.model_transforms.bind_group, &[]);
      rpass.set_bind_group(2, &self.texture.bind_group, &[]);

      rpass.draw(0..6, 0..self.model_transforms.count as u32)
    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }
}
