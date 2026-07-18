use glam::Vec2;

use crate::entity::{ComponentStore, DirIntent, Entity, EntityManager, Facing, Position, Speed};

pub fn update_positions(em: &mut EntityManager) {
  for (DirIntent(dir), e) in em.dir_intents.iter() {
    let Some(Speed(speed)) = em.speeds.get(*e) else {
      continue;
    };

    let vel = dir * speed;
    let Some(Position(pos)) = em.positions.get_mut(*e) else {
      continue;
    };
    *pos += vel;
    // update facing, if velocity is (0,0) keep old facing
    if *dir != Vec2::ZERO {
      update_facing(&em.dir_intents, &mut em.facings, *e);
    }
  }
}

fn update_facing(
  dir_intents: &ComponentStore<DirIntent>,
  facings: &mut ComponentStore<Facing>,
  e: Entity,
) {
  use Facing::*;
  let Some(DirIntent(dir)) = dir_intents.get(e) else {
    return;
  };
  let angle = dir.y.atan2(dir.x);
  let octant = (angle / std::f32::consts::FRAC_PI_4).round() as i32;

  let Some(facing) = facings.get_mut(e) else {
    return;
  };
  match octant.rem_euclid(8) {
    0 => *facing = E,
    1 => *facing = NE,
    2 => *facing = N,
    3 => *facing = NW,
    4 => *facing = W,
    5 => *facing = SW,
    6 => *facing = S,
    7 => *facing = SE,
    _ => unreachable!(),
  }
}
