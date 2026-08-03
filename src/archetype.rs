use glam::Vec2;

use crate::entity::{Action, Facing, TextureType};

#[derive(Default, Clone, Copy)]
pub struct Physics {
  pos: Vec2,
  dir_intent: Vec2,
  speed: f32,
  speed_overide: f32,
}

#[derive(Default, Clone, Copy)]
pub struct HitBox {
  r: f32,
  rotation: f32,
}

pub struct Renderables {
  facing: Facing,
  action: Action,
  texture_type: TextureType,
}

#[derive(Default, Clone, Copy)]
pub struct Animation {
  tick: usize,
}

pub struct Player {
  physics: Physics,
  hit_box: HitBox,
  renderables: Renderables,
  animation: Animation,
  // other
  last_shot_at: usize,
}
