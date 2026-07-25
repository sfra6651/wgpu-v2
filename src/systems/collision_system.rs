use std::time::Instant;

use glam::{Vec2, vec2};

use crate::{
  entity::{Damage, Entity, EntityManager, HitBox, Kind, Position},
  world::{PosUpdate, World},
};

pub enum CollisionResponse {
  Block,
  Push,
  Trigger,
  Ignore,
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
  // step 1: build corrections
  for (&Position(pos), &e) in w.em.positions.iter() {
    for n in w.spatial_grid.near_entities(pos) {
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
        CollisionResponse::Block => w.position_updates.push(PosUpdate { e, offset: mtv }),
        // each pair is visited from both sides, so each entity
        // takes half the separation
        CollisionResponse::Push => w.position_updates.push(PosUpdate {
          e,
          offset: mtv / 2.0,
        }),
        CollisionResponse::Ignore => {}
        CollisionResponse::Trigger => {
          let Some(Damage(dmg)) = w.em.damages.get(e) else {
            continue;
          };
          //TODO: damage resolve
          w.removals.push(e);
        }
      }
    }
  }
}

/// Minimum translation vector that pushes `a` out of `b`, or `None` if the
/// circles don't overlap. Positions are centers; hitboxes are radius's
fn penetration(em: &EntityManager, e_a: Entity, e_b: Entity) -> Option<Vec2> {
  let &Position(pos_a) = em.positions.get(e_a)?;
  let &Position(pos_b) = em.positions.get(e_b)?;
  let &HitBox(r_a) = em.hit_boxes.get(e_a)?;
  let &HitBox(r_b) = em.hit_boxes.get(e_b)?;

  let delta = pos_a - pos_b;
  let r = r_a + r_b;
  let dist_sq = delta.length_squared();

  if dist_sq >= r * r {
    return None;
  }
  if dist_sq == 0.0 {
    // exaclty stacked - no definite push direction so push along fixed axis
    return Some(vec2(r, 0.0));
  }

  let dist = dist_sq.sqrt();
  Some(delta / dist * (r - dist)) // push along line of centers
}
