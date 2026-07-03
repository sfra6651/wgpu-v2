use glam::Vec2;
use rand::RngExt;
use winit::window::Window;

use crate::entity::Entity;

pub struct World {
  pub entities: Vec<Entity>,
}

impl World {
  pub fn new(window: &Window) -> Self {
    let mut entities: Vec<Entity> = Vec::new();
    let width = window.inner_size().width as f32;
    let height = window.inner_size().height as f32;
    let mut rng = rand::rng();

    //seed player
    entities.push(Entity {
      pos: (width / 2.0, height / 2.0).into(),
      size: (150.0, 150.0).into(),
      velocity: (0.0, 0.0).into(),
      is_player: true,
    });

    for _ in 0..1 {
      continue;
      let size = rng.random_range(90.0..100.0);
      let x_pos = rng.random_range(0.0..width);
      let y_pos = rng.random_range(0.0..height);

      let speed_x = rng.random_range(5.0..20.0) * if rng.random::<bool>() { 1.0 } else { -1.0 };
      let speed_y = rng.random_range(5.0..20.0) * if rng.random::<bool>() { 1.0 } else { -1.0 };

      entities.push(Entity {
        pos: (x_pos, y_pos).into(),
        size: (size, size).into(),
        velocity: (0.0, 0.0).into(),
        is_player: false,
      });
    }

    Self { entities }
  }

  pub fn update(&mut self) {
    for entity in self.entities.iter_mut() {
      let new_x = entity.pos.x + entity.velocity.x;
      let new_y = entity.pos.y + entity.velocity.y;

      entity.pos = (new_x, new_y).into();
    }
  }

  pub fn handle_colisions(&mut self) {}
}
