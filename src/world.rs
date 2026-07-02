use glam::Vec2;
use rand::RngExt;
use winit::window::Window;

pub struct Entity {
  pub pos: Vec2,
  pub size: Vec2,
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

    for _ in 0..1000 {
      let size = rng.random_range(40.0..100.0);
      let x_pos = rng.random_range(0.0..width);
      let y_pos = rng.random_range(0.0..height);
      entities.push(Entity {
        pos: (x_pos, y_pos).into(),
        size: (size, size).into(),
      });
    }

    Self { entities }
  }
}
