use std::time::Instant;

use glam::{Vec2, vec2};

use crate::{
  entity::{Damage, DirIntent, Entity, EntityManager, Health, HitBoxRad, Position},
  spatial_grid::CellValue,
  world::{PosUpdate, World},
};

pub fn handle_physics_collisions(w: &mut World) {
  //player collision
  if let Some(player) = w.em.player
    && let Some(pos) = w.em.positions.get(player)
    && let Some(rad) = w.em.hit_box_rads.get(player)
  {
    for n in w.spatial_grid.near_entities(pos.0) {
      if n.e == player || w.em.projectiles.get(n.e).is_some() {
        continue;
      }
      let Some(mtv) = penetration((*pos, *rad), n) else {
        continue;
      };
      w.position_updates.push(PosUpdate {
        e: player,
        offset: mtv / 2.0,
      });
    }
  }

  //npc collisions
  for (_, &e) in w.em.npcs.iter() {
    if let Some(pos) = w.em.positions.get(e)
      && let Some(rad) = w.em.hit_box_rads.get(e)
    {
      for n in w.spatial_grid.near_entities(pos.0) {
        if n.e == e || w.em.projectiles.get(n.e).is_some() {
          continue;
        }
        let Some(mtv) = penetration((*pos, *rad), n) else {
          continue;
        };
        w.position_updates.push(PosUpdate {
          e,
          offset: mtv / 2.0,
        });
      }
    }
  }
}

pub fn handle_projectile_collisions(w: &mut World) {
  for (_, &e) in w.em.projectiles.iter() {
    if let Some(pos) = w.em.positions.get(e)
      && let Some(rad) = w.em.hit_box_rads.get(e)
      && let Some(DirIntent(dir)) = w.em.dir_intents.get(e)
    {
      for n in w.spatial_grid.near_entities(pos.0) {
        if n.e == e {
          continue;
        };
        if penetration((*pos, *rad), n).is_some() {
          let Some(Damage(dmg)) = w.em.damages.get(e) else {
            continue;
          };
          let Some(Health(hp)) = w.em.healths.get_mut(n.e) else {
            continue;
          };
          w.removals.push(e);
          w.position_updates.push(PosUpdate {
            e: n.e,
            offset: dir.normalize_or_zero() * 0.2,
          });
          *hp -= dmg;
          if *hp <= 0.0 {
            w.removals.push(n.e);
          }
        }
      }
    }
  }
}

/// Minimum translation vector that pushes `a` out of `b`, or `None` if the
/// circles don't overlap. Positions are centers; hitboxes are radius's
fn penetration(e: (Position, HitBoxRad), other: CellValue) -> Option<Vec2> {
  let (Position(pos_a), HitBoxRad(r_a)) = e;
  let Position(pos_b) = other.pos;
  let r_b = other.rad;

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
