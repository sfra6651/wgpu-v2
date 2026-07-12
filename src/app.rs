use std::{sync::Arc, time::Instant};

use bevy_ecs::prelude::*;
use bevy_ecs::world::World;
use glam::Vec2;
use winit::window::Window;

use crate::{frame_counter::FrameCounter, renderer::renderer::Renderer};

struct WorldSize {
  size: Vec2,
}

#[derive(Default)]
struct App {
  window: Option<Arc<Window>>,
  frame_counter: FrameCounter,
  renderer: Option<Renderer>,
  world: Option<World>,
  last_update: Option<Instant>,
  accumulator: f32,
}

impl App {
  pub fn init_bevy_world(&mut self) {}

  fn init(&mut self, window: Arc<Window>) {
    self.world = Some(World::default());
    self.renderer = Some(pollster::block_on(Renderer::new(window)));
    self.renderer.as_mut().unwrap().create_pipelines();

    //let size = self.world.as_ref().unwrap().size;
    //self.renderer.as_mut().unwrap().upload_grid(size, 1.0); // 1.0-unit spacing
    self.renderer.as_mut().unwrap().upload_square();
  }
}
