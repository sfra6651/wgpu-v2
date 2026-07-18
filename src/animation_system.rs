use glam::Vec2;

use crate::entity::{Action, AnimTick, DirIntent, EntityManager, Speed};

pub fn update_animations(em: &mut EntityManager) {
  for (&DirIntent(dir), &e) in em.dir_intents.iter() {
    let Some(Speed(speed)) = em.speeds.get(e) else {
      continue;
    };
    let vel = dir * speed;
    let Some(AnimTick(tick)) = em.anim_ticks.get_mut(e) else {
      continue;
    };
    let Some(action) = em.actions.get_mut(e) else {
      continue;
    };
    if vel != Vec2::ZERO {
      *tick += 1;
    } else {
      *tick = 0;
    }
    *action = if vel != Vec2::ZERO {
      Action::Run
    } else {
      Action::Idle
    };
  }
}
