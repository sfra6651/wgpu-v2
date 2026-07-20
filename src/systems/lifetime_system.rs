use crate::entity::{Entity, EntityManager, LifeTime, OnDestroy};
use crate::world::World;

pub fn handle_lifetimes(em: &mut EntityManager, removals: &mut Vec<Entity>) {
  let mut destory_list: Vec<Entity> = Vec::new();
  for (LifeTime(lt), e) in em.lifetimes.iter_mut() {
    *lt -= 1.0;
    if *lt <= 0.0 {
      removals.push(*e);
      destory_list.push(*e);
    }
  }

  for e in destory_list.iter() {
    if let Some(OnDestroy(on_d)) = em.on_destroys.get(*e) {
      on_d(em, *e);
    }
  }
  // tick down cd's
  for (ability, _) in em.abilities.iter_mut() {
    ability.remaing_cd = (ability.remaing_cd - 1.0).max(0.0);
  }
}
