use glam::Vec2;

use crate::entity::{DirIntent, Entity, EntityManager, Position, Speed};

pub fn update_ai(em: &mut EntityManager) {
  use crate::entity::AiType::*;

  let Some(player) = em.player else {
    return;
  };
  let Some(Position(player_pos)) = em.positions.get(player) else {
    return;
  };

  for (ai, &e) in em.ai.iter() {
    let Some(Position(pos)) = em.positions.get(e) else {
      continue;
    };
    let Some(DirIntent(dir)) = em.dir_intents.get_mut(e) else {
      continue;
    };
    match ai {
      Goblin => {
        let speed = em.speeds.get(e).map_or(0.0, |&Speed(s)| s);
        let to_player = *player_pos - *pos;
        // Stop within one step of the player so we don't overshoot and
        // flip direction every frame.
        *dir = if to_player.length() > speed {
          to_player.normalize_or_zero()
        } else {
          Vec2::ZERO
        }
      }
      None => {}
    }
  }
}
