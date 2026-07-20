use glam::{Vec2, vec2};

use crate::{
  entity::{Damage, Entity, EntityManager, Health, HitBox, Kind, Position},
  world::World,
};

pub enum CollisionResponse {
  Block,
  Push,
  Trigger,
  Ignore,
}

pub struct Correction {
  e: Entity,
  correction: Vec2,
}

impl Correction {
  pub fn new(e: Entity, correction: Vec2) -> Self {
    Correction { e, correction }
  }
}

fn collision_response(a: &Kind, b: &Kind) -> CollisionResponse {
  use Kind::*;
  match (a, b) {
    (Player, Enemy) => CollisionResponse::Push,
    (Enemy, Player) => CollisionResponse::Push,
    (Enemy, Enemy) => CollisionResponse::Push,
    (Projectile, Enemy) => CollisionResponse::Trigger,
    (Enemy, Projectile) => CollisionResponse::Ignore,
    _ => CollisionResponse::Ignore,
  }
}

pub fn handle_collisions(w: &mut World) {
  // clear corrections
  w.position_corrections.clear();
  // step 1: build collections
  for (&Position(pos), &e) in w.em.positions.iter() {
    let near = w.spatial_grid.near_entities(pos);
    for &n in near.iter() {
      if n == e {
        continue;
      }

      let Some(mtv) = penetration(&w.em, e, n) else {
        continue;
      };

      let Some(e_kind) = w.em.kinds.get(e) else {
        continue;
      };
      let Some(n_kind) = w.em.kinds.get(n) else {
        continue;
      };
      match collision_response(e_kind, n_kind) {
        // full push-out; `other` is treated as immovable
        CollisionResponse::Block => w.position_corrections.push(Correction::new(e, mtv)),
        // each pair is visited from both sides, so each entity
        // takes half the separation
        CollisionResponse::Push => w.position_corrections.push(Correction::new(e, mtv / 2.0)),
        CollisionResponse::Ignore => {}
        CollisionResponse::Trigger => {
          let Some(Damage(dmg)) = w.em.damages.get(e) else {
            continue;
          };
          let Some(Health(hp)) = w.em.healths.get_mut(n) else {
            continue;
          };
          *hp -= dmg;
          w.removals.push(e);
          if *hp <= 0.0 {
            w.removals.push(n);
          }
        }
      }
    }
  }

  //step 2: resolve corrections
  for c in w.position_corrections.iter() {
    let Some(Position(pos)) = w.em.positions.get_mut(c.e) else {
      continue;
    };
    *pos += c.correction;
  }
}

/// Minimum translation vector that pushes `a` out of `b`, or `None` if the
/// boxes don't overlap. Positions are centers; hitboxes are full width/height.
fn penetration(em: &EntityManager, e_a: Entity, e_b: Entity) -> Option<Vec2> {
  let &Position(pos_a) = em.positions.get(e_a)?;
  let &Position(pos_b) = em.positions.get(e_b)?;
  let &HitBox(hitbox_a) = em.hit_boxes.get(e_a)?;
  let &HitBox(hitbox_b) = em.hit_boxes.get(e_b)?;

  let delta = pos_a - pos_b;
  let overlap = (hitbox_a + hitbox_b) / 2.0 - delta.abs();
  if overlap.x <= 0.0 || overlap.y <= 0.0 {
    return None;
  }
  // push out along the shallower axis
  if overlap.x < overlap.y {
    Some(vec2(overlap.x.copysign(delta.x), 0.0))
  } else {
    Some(vec2(0.0, overlap.y.copysign(delta.y)))
  }
}
