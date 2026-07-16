use glam::Vec2;

use crate::world;

#[derive(Copy, Clone)]
pub enum Action {
  Idle,
  Run,
}

#[derive(Copy, Clone)]
pub enum Facing {
  S,
  SE,
  E,
  NE,
  N,
  NW,
  W,
  SW,
}

impl Facing {
  pub fn usize(&self) -> usize {
    *self as usize
  }
}

#[derive(Copy, Clone)]
pub enum AiType {
  None,
  Goblin,
}

//unique player state only relavent to the player
//The player also needs an Entity for normal entity data
pub struct PlayerState {
  pub last_shot_at: usize,
}

#[derive(Copy, Clone)]
pub enum Kind {
  Player,
  Enemy,
  Projectile,
  Dummy,
}

#[derive(Copy, Clone)]
pub struct Entity {
  pub pos: Vec2,
  pub render_size: Vec2,
  pub hb: Vec2,
  pub dir_intent: Vec2,
  pub speed: f32,
  pub anim_tick: usize,
  pub facing: Facing,
  pub action: Action,
  pub kind: Kind,
  pub ai: AiType,
}

impl Entity {
  pub const TICKS_PER_FRAME: usize = 8;
  pub const WALK_FRAMES: usize = 4;

  pub fn default() -> Self {
    Entity {
      pos: (world::WIDTH / 2.0, world::HEIGHT / 2.0).into(),
      render_size: (1.0, 1.0).into(),
      hb: (1.0, 1.0).into(),
      dir_intent: (0.0, 0.0).into(),
      speed: 0.1,
      anim_tick: 0,
      facing: Facing::S,
      action: Action::Idle,
      kind: Kind::Dummy,
      ai: AiType::None,
    }
  }

  pub fn player() -> Self {
    Entity {
      pos: (world::WIDTH / 2.0, world::HEIGHT / 2.0).into(),
      render_size: (3.0, 3.0).into(),
      hb: (1.0, 1.5).into(),
      kind: Kind::Player,
      ..Entity::default()
    }
  }

  pub fn goblin(pos: Vec2) -> Self {
    Self {
      pos,
      render_size: (2.0, 2.0).into(),
      hb: (0.5, 0.5).into(),
      speed: 0.05,
      kind: Kind::Enemy,
      ai: AiType::Goblin,
      ..Entity::default()
    }
  }

  pub fn arrow(pos: Vec2) -> Self {
    Self {
      pos,
      render_size: (0.5, 0.5).into(),
      hb: (0.5, 0.2).into(),
      dir_intent: (1.0, 0.0).into(),
      speed: 0.1,
      kind: Kind::Projectile,
      ..Entity::default()
    }
  }

  /// Minimum translation vector: the smallest push that moves `self` out of
  /// `other`, or None if the boxes don't overlap. Always along a single axis.
  pub fn penetration(&self, other: &Entity) -> Option<Vec2> {
    let delta = self.pos - other.pos;
    let overlap = (self.hb + other.hb) / 2.0 - delta.abs();
    if overlap.x <= 0.0 || overlap.y <= 0.0 {
      return None;
    }
    if overlap.x < overlap.y {
      Some(Vec2::new(overlap.x.copysign(delta.x), 0.0))
    } else {
      Some(Vec2::new(0.0, overlap.y.copysign(delta.y)))
    }
  }
}
