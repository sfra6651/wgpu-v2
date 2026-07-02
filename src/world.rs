use glam::Vec2;
use rand::RngExt;
use winit::window::Window;

pub struct Entity {
  pub pos: Vec2,
  pub size: Vec2,
  pub velocity: Vec2,
}

pub struct World {
  pub entities: Vec<Entity>,
}

impl World {
  pub fn new(window: &Window) -> Self {
    let mut entities: Vec<Entity> = Vec::new();
    let width = window.inner_size().width as f32;
    let height = window.inner_size().height as f32;
    let mut rng = rand::rng();

    for _ in 0..500 {
      let size = rng.random_range(40.0..100.0);
      let x_pos = rng.random_range(0.0..width);
      let y_pos = rng.random_range(0.0..height);

      let speed_x = rng.random_range(5.0..20.0) * if rng.random::<bool>() { 1.0 } else { -1.0 };
      let speed_y = rng.random_range(5.0..20.0) * if rng.random::<bool>() { 1.0 } else { -1.0 };

      entities.push(Entity {
        pos: (x_pos, y_pos).into(),
        size: (size, size).into(),
        velocity: (speed_x, speed_y).into(),
      });
    }

    Self { entities }
  }

  pub fn update(&mut self, window: &Window) {
    for entity in self.entities.iter_mut() {
      let mut new_x = entity.pos.x + entity.velocity.x;
      let mut new_y = entity.pos.y + entity.velocity.y;

      if new_x > window.inner_size().width as f32
        || new_x < 0.0
        || new_y > window.inner_size().height as f32
        || new_y < 0.0
      {
        new_x = entity.pos.x - entity.velocity.x;
        new_y = entity.pos.y - entity.velocity.y;
        entity.velocity *= -1.0;
      }

      entity.pos = (new_x, new_y).into();
    }
  }
}
