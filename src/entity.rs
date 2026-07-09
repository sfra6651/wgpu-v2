use glam::Vec2;

#[derive(Copy, Clone)]
pub enum Action {
  Idle,
  Run,
}

pub struct Entity {
  pub pos: Vec2,
  pub size: Vec2,
  pub velocity: Vec2,
  pub is_player: bool,
  pub anim_tick: usize,
  pub facing: Facing,
  pub action: Action,
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
  pub fn to_i(&self) -> usize {
    *self as usize
  }
}

impl Entity {
  pub const TICKS_PER_FRAME: usize = 8;
  pub const WALK_FRAMES: usize = 4;

  pub fn intersects(&self, other: &Entity) -> bool {
    self.pos.x < other.pos.x + other.size.x
      && self.pos.x + self.size.x > other.pos.x
      && self.pos.y < other.pos.y + other.size.y
      && self.pos.y + self.size.y > other.pos.y
  }

  pub fn facing(&self) -> Facing {
    use Facing::*;
    if self.velocity == Vec2::ZERO {
      return S;
    }
    let angle = self.velocity.y.atan2(self.velocity.x);
    let octant = (angle / std::f32::consts::FRAC_PI_4).round() as i32;
    match octant.rem_euclid(8) {
      0 => E,
      1 => NE,
      2 => N,
      3 => NW,
      4 => W,
      5 => SW,
      6 => S,
      7 => SE,
      _ => unreachable!(),
    }
  }
}
