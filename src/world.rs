use glam::Vec2;

use crate::{camera::Camera, entity::Entity};

pub struct World {
  pub entities: Vec<Entity>,
  pub camera: Camera,
  pub size: Vec2,
}

impl World {
  pub fn new() -> Self {
    let mut entities: Vec<Entity> = Vec::new();
    let width = 30.0;
    let height = 30.0;

    //seed player
    entities.push(Entity {
      pos: (width / 2.0, height / 2.0).into(),
      size: (1.0, 1.0).into(),
      velocity: (0.0, 0.0).into(),
      is_player: true,
    });

    Self {
      entities,
      camera: Camera::new((width, height).into()),
      size: (width, height).into(),
    }
  }

  pub fn update(&mut self) {
    for entity in self.entities.iter_mut() {
      entity.pos += entity.velocity;
    }
    self.camera.pos += self.camera.velocity;
  }
}
