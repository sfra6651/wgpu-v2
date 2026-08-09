use std::sync::Arc;

use glam::{Vec2, vec2};
use wgpu::{ColorTargetState, CurrentSurfaceTexture, RenderPass};
use winit::window::Window;

use crate::{
  entity::{Entity, Layer, Position, RenderSize, Rotation, TextureType},
  renderer::{
    character_sprite_set::CharacterSpriteSet,
    model_transforms::ModelTransforms,
    pipeline::{CreatePipelineDesc, Pipeline, PipelineType},
    texture::TextureLoader,
    ui_transforms::UiTransfroms,
    uniform::{UniformManager, UniformVariant},
    vertex::{Vertex, grid_vertices, unit_square},
  },
  ui::{AnchorPoint, RenderPos, UiSize},
  utils::ui_projection_matrix,
  world::World,
};

// transforms
pub mod model_transforms;
pub mod ui_transforms;
//
pub mod character_sprite_set;
pub mod pipeline;
pub mod texture;
pub mod uniform;
pub mod vertex;

pub struct DrawEntity {
  pub e: Entity,
  pub pos: Position,
  pub rotation: f32,
  pub render_size: RenderSize,
  pub sort_y: f32,
  pub layer: Layer,
  pub texture_type: TextureType,
}

pub struct UiElement {
  pub pos: Vec2,
  pub size: UiSize,
}

pub struct Renderer {
  surface: wgpu::Surface<'static>,
  adapter: wgpu::Adapter,
  device: wgpu::Device,
  queue: wgpu::Queue,
  pipelines: [Option<Pipeline>; PipelineType::COUNT],
  grid_vertex_count: u32,
  square_buffer: wgpu::Buffer,
  grid_buffer: Option<wgpu::Buffer>,
  uniform_manager: UniformManager,
  model_transforms: ModelTransforms,
  ui_transforms: UiTransfroms,
  texture_loader: TextureLoader,
  player_sprite_set: CharacterSpriteSet,
  goblin_sprite_set: CharacterSpriteSet,
  arrow_texture: wgpu::BindGroup,
  draw_entities: Vec<DrawEntity>,
  ui_elemets: Vec<UiElement>,
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

    let ui_transforms = UiTransfroms::new(&device);

    let texture_loader = TextureLoader::new(&device);

    let sprite_set = CharacterSpriteSet::new(&texture_loader, &device, &queue, "conduit");
    let goblin_sprite_set = CharacterSpriteSet::new(&texture_loader, &device, &queue, "goblin");
    let arrow_texture = texture_loader.load(
      &device,
      &queue,
      &format!("src/assets/objects/arrow.png"),
      "arrow",
    );

    Self {
      surface,
      adapter,
      device,
      queue,
      grid_buffer: None,
      grid_vertex_count: 0,
      square_buffer,
      pipelines: std::array::from_fn(|_| None),
      uniform_manager,
      model_transforms,
      ui_transforms,
      texture_loader,
      player_sprite_set: sprite_set,
      goblin_sprite_set,
      arrow_texture,
      draw_entities: Vec::new(),
      ui_elemets: Vec::new(),
    }
  }

  pub fn create_pipelines(&mut self) {
    let swapchain_format = self.surface.get_capabilities(&self.adapter).formats[0];

    self.pipelines[PipelineType::Texture as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/texture.wgsl").into()),
      CreatePipelineDesc {
        bind_group_layouts: &[
          Some(&self.uniform_manager.bind_group_layout),
          Some(&self.model_transforms.bind_group_layout),
          Some(&self.texture_loader.bind_group_layout),
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
      CreatePipelineDesc {
        bind_group_layouts: &[Some(&self.uniform_manager.bind_group_layout)],
        targets: &[Some(swapchain_format.into())],
        primitive: wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::LineList,
          ..Default::default()
        },
      },
    );

    self.pipelines[PipelineType::Ui as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/ui_shader.wgsl").into()),
      CreatePipelineDesc {
        bind_group_layouts: &[
          Some(&self.uniform_manager.bind_group_layout),
          Some(&self.ui_transforms.bind_group_layout),
        ],
        targets: &[Some(ColorTargetState {
          format: swapchain_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
        primitive: wgpu::PrimitiveState::default(),
      },
    );

    self.pipelines[PipelineType::SimpleRect as usize] = Pipeline::new(
      &self.device,
      wgpu::ShaderSource::Wgsl(include_str!("../shaders/square.wgsl").into()),
      CreatePipelineDesc {
        bind_group_layouts: &[
          Some(&self.uniform_manager.bind_group_layout),
          Some(&self.model_transforms.bind_group_layout),
        ],
        targets: &[Some(swapchain_format.into())],
        primitive: wgpu::PrimitiveState::default(),
      },
    );
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

  pub fn upload_square(&self) {
    let square = unit_square();
    self
      .queue
      .write_buffer(&self.square_buffer, 0, bytemuck::cast_slice(&square));
  }

  fn prepare_draw_entities(&mut self, w: &World) {
    self.draw_entities.clear();
    for (&render_size, &e) in w.em.render_sizes.iter() {
      let Some(&pos) = w.em.positions.get(e) else {
        eprintln!("entity is missing pos, skipping adding to draw entities");
        continue;
      };
      let Some(&layer) = w.em.layers.get(e) else {
        eprintln!("entity is missing layer, skipping adding to draw_entities");
        continue;
      };

      let Some(&texture_type) = w.em.texture_types.get(e) else {
        eprintln!("entity is missing texture_type, skipping adding to draw_entities");
        continue;
      };

      let mut rotation = 0.0;
      if let Some(Rotation(rot)) = w.em.rotations.get(e) {
        rotation = *rot;
      };
      self.draw_entities.push(DrawEntity {
        e,
        pos,
        rotation,
        render_size,
        layer,
        sort_y: pos.0.y - render_size.0.y / 2.0,
        texture_type,
      });
    }
    // sort the draw intities
    self.draw_entities.sort_unstable_by(|a, b| {
      a.layer
        .cmp(&b.layer) // layers first
        .then(b.sort_y.total_cmp(&a.sort_y)) // then higher y drawn first
        .then(a.e.cmp(&b.e)) // stable tiebreak — prevents flicker
    });
    self
      .model_transforms
      .write_transforms(&self.draw_entities, &self.queue);
  }

  fn prepare_ui_renderables(&mut self, world: &World) {
    self.ui_elemets.clear();

    for (&ap, &e) in world.em.anchor_points.iter() {
      if let Some(&size) = world.em.ui_size.get(e) {
        self.ui_elemets.push(UiElement { pos: ap.pos, size });
      };
    }
    self
      .ui_transforms
      .write_transforms(&self.ui_elemets, &self.queue);
  }

  fn write_uniforms(&mut self, window: &Window, world: &World) {
    let width = window.inner_size().width;
    let height = window.inner_size().height;
    let aspect = width as f32 / height.max(1) as f32;
    let view_proj = world.camera.view_projection(aspect);
    self.queue.write_buffer(
      &self
        .uniform_manager
        .get(UniformVariant::WorldSpaceProjection)
        .buffer,
      0,
      bytemuck::cast_slice(&[view_proj.to_cols_array()]),
    );

    let ui_proj = ui_projection_matrix(vec2(1920.0, 1080.0));
    self.queue.write_buffer(
      &self
        .uniform_manager
        .get(UniformVariant::ScreenSpaceProjection)
        .buffer,
      0,
      bytemuck::cast_slice(&[ui_proj.to_cols_array()]),
    );
  }

  pub fn render(&mut self, window: &Window, world: &World) {
    // pre-render steps
    self.prepare_draw_entities(world);
    self.write_uniforms(window, world);
    self.prepare_ui_renderables(world);

    let frame = match self.surface.get_current_texture() {
      CurrentSurfaceTexture::Success(frame) => frame,
      CurrentSurfaceTexture::Occluded => {
        window.request_redraw();
        return;
      }
      other => {
        eprintln!("{:?}", other);
        panic!("failed to get surface texture for frame")
      }
    };

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

      // do actual rendering work
      self.render_grid_lines(&mut rpass);
      self.render_entities(&mut rpass, world);
      self.render_ui(&mut rpass);
    }

    self.queue.submit(Some(encoder.finish()));
    frame.present();
  }

  fn render_ui(&self, rpass: &mut RenderPass) {
    let Some(pipeline) = &self.pipelines[PipelineType::Ui as usize] else {
      return;
    };
    rpass.set_pipeline(&pipeline.pipeline);
    rpass.set_vertex_buffer(0, self.square_buffer.slice(..));
    rpass.set_bind_group(
      0,
      &self
        .uniform_manager
        .get(UniformVariant::ScreenSpaceProjection)
        .bind_group,
      &[],
    );
    rpass.set_bind_group(1, &self.ui_transforms.bind_group, &[]);

    let mut i = 0;
    self.ui_elemets.iter().for_each(|element| {
      rpass.draw(0..6, i as u32..i as u32 + 1);
      i += 1;
    });
  }

  fn render_grid_lines(&self, rpass: &mut RenderPass) {
    let (Some(grid_pipeline), Some(grid_buffer)) = (
      &self.pipelines[PipelineType::Gridlines as usize],
      &self.grid_buffer,
    ) else {
      return;
    };
    rpass.set_pipeline(&grid_pipeline.pipeline);
    rpass.set_bind_group(
      0,
      &self
        .uniform_manager
        .get(UniformVariant::WorldSpaceProjection)
        .bind_group,
      &[],
    );
    rpass.set_vertex_buffer(0, grid_buffer.slice(..));
    rpass.draw(0..self.grid_vertex_count, 0..1); // ONE instance
  }

  fn render_entities(&self, rpass: &mut RenderPass, world: &World) {
    let Some(texture_pipeline) = &self.pipelines[PipelineType::Texture as usize] else {
      return;
    };
    rpass.set_pipeline(&texture_pipeline.pipeline);
    rpass.set_vertex_buffer(0, self.square_buffer.slice(..));
    rpass.set_bind_group(
      0,
      &self
        .uniform_manager
        .get(UniformVariant::WorldSpaceProjection)
        .bind_group,
      &[],
    );
    rpass.set_bind_group(1, &self.model_transforms.bind_group, &[]);

    let mut i = 0;
    // draw entiteis

    self.draw_entities.iter().for_each(|draw_e| {
      let e = draw_e.e;
      if let Some(bind_group) = match draw_e.texture_type {
        TextureType::Player => self.player_sprite_set.resolve(&world.em, e),
        TextureType::Goblin => self.goblin_sprite_set.resolve(&world.em, e),
        TextureType::Arrow => Some(&self.arrow_texture),
      } {
        rpass.set_bind_group(2, bind_group, &[]);
      }

      rpass.draw(0..6, i as u32..i as u32 + 1);

      i += 1;
    });
  }
}
