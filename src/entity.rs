use glam::Vec2;

pub struct Entity {
  pub pos: Vec2,
  pub size: Vec2,
  pub velocity: Vec2,
  pub is_player: bool,
}

impl Entity {
  pub fn intersects(&self, other: &Entity) -> bool {
    self.pos.x < other.pos.x + other.size.x
      && self.pos.x + self.size.x > other.pos.x
      && self.pos.y < other.pos.y + other.size.y
      && self.pos.y + self.size.y > other.pos.y
  }
}
