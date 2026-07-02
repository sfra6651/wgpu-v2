use std::sync::Arc;

use wgpu::CurrentSurfaceTexture;
use winit::window::Window;

use crate::{
  model_transforms::ModelTransforms,
  uniforms::{UniformManager, UniformVariant},
  utils::{Vertex, model_matrix, pos_to_trangle, projection_matrix, unit_square},
  world::World,
};

pub struct Renderer {
  instance: wgpu::Instance,
  surface: wgpu::Surface<'static>,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  shape_render_pipeline: Option<wgpu::RenderPipeline>,
  shape_render_pipeline_layout: Option<wgpu::PipelineLayout>,
  square_buffer: wgpu::Buffer,
  uniform_manager: UniformManager,
  model_transforms: ModelTransforms,
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

    Self {
      surface,
      instance,
      adapter,
      device,
      queue,
      shape_render_pipeline: None,
      shape_render_pipeline_layout: None,
      square_buffer,
      uniform_manager,
      model_transforms,
    }
  }

  pub fn create_shape_render_pipeline(&mut self) {
    let shader = self
      .device
      .create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
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

    let Some(pipeline) = &self.shape_render_pipeline else {
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
    let proj_mat = projection_matrix(width as f32, height as f32);
    self.queue.write_buffer(
      &self.uniform_manager.get(UniformVariant::Projection).buffer,
      0,
      bytemuck::cast_slice(&[proj_mat.to_cols_array()]),
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
            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
            store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
      });

      rpass.set_pipeline(pipeline);
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

      rpass.draw(0..6, 0..self.model_transforms.count as u32)
    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }
}
