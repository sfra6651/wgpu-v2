use glam::vec2;
use rand::RngExt;

use crate::{entity::EntityManager, entity_templates::goblin, world};

pub fn spawn_enmies(em: &mut EntityManager) {
  let mut rng = rand::rng();
  let side = rng.random_range(1..=4);
  let mut x = 0.0;
  let mut y = 0.0;

  //top
  if side == 1 {
    x = rng.random_range(0.0..world::WIDTH);
    y = 0.0;
  }
  //bottom
  if side == 2 {
    x = rng.random_range(0.0..world::WIDTH);
    y = world::HEIGHT;
  }

  //left
  if side == 3 {
    x = 0.0;
    y = rng.random_range(0.0..world::HEIGHT);
  }
  //right
  if side == 4 {
    x = world::WIDTH;
    y = rng.random_range(0.0..world::HEIGHT);
  }

  let _ = goblin(em, vec2(x, y));
}
