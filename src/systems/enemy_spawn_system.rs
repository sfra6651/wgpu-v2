use glam::vec2;
use rand::RngExt;

use crate::{camera::Camera, entity::EntityManager, entity_templates::goblin, world};

pub fn spawn_enemies(em: &mut EntityManager, camera: &Camera) {
  let mut rng = rand::rng();
  let side = rng.random_range(1..=4);
  let mut x = 0.0;
  let mut y = 0.0;

  let pos = camera.pos;
  let offset = (camera.view_height * camera.zoom) / 2.0;
  //top
  if side == 1 {
    x = rng.random_range(pos.x - offset..pos.x + offset);
    y = 0.0;
  }
  //bottom
  if side == 2 {
    x = rng.random_range(pos.x - offset..pos.x + offset);
    y = world::HEIGHT;
  }

  //left
  if side == 3 {
    x = 0.0;
    y = rng.random_range(pos.y - offset..pos.y + offset);
  }
  //right
  if side == 4 {
    x = world::WIDTH;
    y = rng.random_range(pos.y - offset..pos.y + offset);
  }

  let _ = goblin(em, vec2(x, y));
}
