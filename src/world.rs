use glam::Vec2;
use rand::RngExt;
use winit::window::Window;

pub struct Entity {
  pub pos: Vec2,
  pub size: Vec2,
  pub velocity: Vec2,
}

impl Entity {
  /// Returns true if this entity's box overlaps `other`'s box.
  pub fn intersects(&self, other: &Entity) -> bool {
    self.pos.x < other.pos.x + other.size.x
      && self.pos.x + self.size.x > other.pos.x
      && self.pos.y < other.pos.y + other.size.y
      && self.pos.y + self.size.y > other.pos.y
  }
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

    for _ in 0..200 {
      let size = rng.random_range(50.0..100.0);
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

      if new_x > window.inner_size().width as f32 || new_x < 0.0 {
        new_x = entity.pos.x - entity.velocity.x;
        entity.velocity.x *= -1.0;
      }
      if new_y > window.inner_size().height as f32 || new_y < 0.0 {
        new_y = entity.pos.y - entity.velocity.y;
        entity.velocity.y *= -1.0;
      }

      entity.pos = (new_x, new_y).into();
    }
    self.handle_colisions();
  }

  pub fn handle_colisions(&mut self) {
    let mut collisions: Vec<(usize, usize)> = Vec::new();
    for i in 0..self.entities.len() {
      for j in (i + 1)..self.entities.len() {
        if self.entities[i].intersects(&self.entities[j]) {
          collisions.push((i, j));
        }
      }
    }

    // Resolve each pair: push them apart along the axis of least overlap,
    // then flip velocity on that axis so they bounce.
    for (i, j) in collisions {
      let a = &self.entities[i];
      let b = &self.entities[j];

      // Vector between the two box centers.
      let center_a = a.pos + a.size * 0.5;
      let center_b = b.pos + b.size * 0.5;
      let delta = center_b - center_a;

      // How much they overlap on each axis (positive means overlapping).
      let overlap_x = (a.size.x + b.size.x) * 0.5 - delta.x.abs();
      let overlap_y = (a.size.y + b.size.y) * 0.5 - delta.y.abs();

      // Guard against exact-center overlap (delta == 0) with a default push.
      let dir_x = if delta.x < 0.0 { -1.0 } else { 1.0 };
      let dir_y = if delta.y < 0.0 { -1.0 } else { 1.0 };

      if overlap_x < overlap_y {
        // Least penetration on X: separate horizontally, each by half.
        let push = overlap_x * 0.5;
        self.entities[i].pos.x -= dir_x * push;
        self.entities[j].pos.x += dir_x * push;
        self.entities[i].velocity.x *= -1.0;
        self.entities[j].velocity.x *= -1.0;
      } else {
        // Least penetration on Y: separate vertically.
        let push = overlap_y * 0.5;
        self.entities[i].pos.y -= dir_y * push;
        self.entities[j].pos.y += dir_y * push;
        self.entities[i].velocity.y *= -1.0;
        self.entities[j].velocity.y *= -1.0;
      }
    }
  }
}
