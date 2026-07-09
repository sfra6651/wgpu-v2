use glam::Vec2;

use crate::{
  camera::Camera,
  entity::{Action, Entity, Facing},
};

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
      size: (2.0, 2.0).into(),
      velocity: (0.0, 0.0).into(),
      anim_tick: 0,
      is_player: true,
      facing: Facing::S,
      action: Action::Idle,
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
      if entity.velocity != Vec2::ZERO {
        entity.anim_tick += 1;
      } else {
        entity.anim_tick = 0;
      }
      entity.action = if entity.velocity != Vec2::ZERO {
        Action::Run
      } else {
        Action::Idle
      };
      if entity.velocity != Vec2::ZERO {
        entity.facing = entity.facing();
      }
    }
    self.camera.pos += self.camera.velocity;
  }
}
