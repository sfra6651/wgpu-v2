use glam::Vec2;

use crate::{
  camera::Camera,
  entity::{Action, AiType, Entity, Facing},
};

pub struct World {
  pub entities: Vec<Entity>,
  pub camera: Camera,
  pub size: Vec2,
}

impl World {
  pub const WIDTH: f32 = 30.0;
  pub const HEIGHT: f32 = 30.0;
  pub fn new() -> Self {
    let mut entities: Vec<Entity> = Vec::new();

    //seed player
    entities.push(Entity {
      pos: (Self::WIDTH / 2.0, Self::HEIGHT / 2.0).into(),
      size: (3.0, 3.0).into(),
      velocity: (0.0, 0.0).into(),
      anim_tick: 0,
      is_player: true,
      facing: Facing::S,
      action: Action::Idle,
      ai: None,
    });

    entities.push(Entity {
      pos: (Self::WIDTH / 2.0, Self::HEIGHT / 2.0).into(),
      size: (2.0, 2.0).into(),
      velocity: (0.0, 0.0).into(),
      anim_tick: 0,
      is_player: false,
      facing: Facing::S,
      action: Action::Idle,
      ai: Some(AiType::Goblin),
    });

    entities.push(Entity {
      pos: (10.0, 10.0).into(),
      size: (2.0, 2.0).into(),
      velocity: (0.0, 0.0).into(),
      anim_tick: 0,
      is_player: false,
      facing: Facing::S,
      action: Action::Idle,
      ai: Some(AiType::Goblin),
    });

    entities.push(Entity {
      pos: (20.0, 20.0).into(),
      size: (2.0, 2.0).into(),
      velocity: (0.0, 0.0).into(),
      anim_tick: 0,
      is_player: false,
      facing: Facing::S,
      action: Action::Idle,
      ai: Some(AiType::Goblin),
    });

    Self {
      entities,
      camera: Camera::new((Self::WIDTH, Self::HEIGHT).into()),
      size: (Self::WIDTH, Self::HEIGHT).into(),
    }
  }

  pub fn update(&mut self) {
    self.update_ai();
    self.update_positions();
  }

  fn update_ai(&mut self) {
    use crate::entity::AiType::*;
    let pos = self
      .entities
      .iter()
      .find(|e| e.is_player)
      .unwrap()
      .pos
      .clone();

    for entity in self.entities.iter_mut() {
      let Some(ai) = entity.ai else {
        continue;
      };
      match ai {
        Goblin => {
          if entity.pos.x > pos.x {
            entity.velocity.x = -0.05
          }
          if entity.pos.x < pos.x {
            entity.velocity.x = 0.05
          }
          if entity.pos.y > pos.y {
            entity.velocity.y = -0.05
          }
          if entity.pos.y < pos.y {
            entity.velocity.y = 0.05
          }
        }
      }
    }
  }

  fn update_positions(&mut self) {
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
