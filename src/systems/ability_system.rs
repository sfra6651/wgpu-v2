use crate::entity::{Entity, EntityManager, LifeTime, OnDestroy, Parent, SpeedOveride};

pub const DASH_SPEED: f32 = 0.5;

#[derive(Clone, Copy, Debug)]
pub enum AbilityName {
  Dash,
}

#[derive(Clone, Copy, Debug)]
pub struct Ability {
  pub name: AbilityName,
  pub cd: f32,
  pub remaing_cd: f32,
}

pub fn begin_dash(em: &mut EntityManager, e: Entity) {
  let Some(ability) = em.abilities.get_mut(e) else {
    eprintln!("entity {:?}, has no abitility {:?}", e, AbilityName::Dash);
    return;
  };

  if ability.remaing_cd > 0.0 {
    return;
  }

  ability.remaing_cd = ability.cd;

  let dash = em.create();
  em.attach(dash, Parent(e));
  em.attach(dash, LifeTime(8.0));
  em.attach(dash, OnDestroy(dash_ends));
  // if we already have a speed overide - overide otherwise attatch it and set it(first use case)
  if let Some(SpeedOveride(spd_ovr_opt)) = em.speed_overides.get_mut(e) {
    *spd_ovr_opt = Some(DASH_SPEED);
  } else {
    em.attach(e, SpeedOveride(Some(DASH_SPEED)));
    if let Some(SpeedOveride(spd_ovr_opt)) = em.speed_overides.get_mut(e) {
      *spd_ovr_opt = Some(DASH_SPEED);
    };
  };
}

pub fn dash_ends(em: &mut EntityManager, dash_e: Entity) {
  let Some(Parent(p)) = em.parents.get(dash_e) else {
    return;
  };
  if let Some(SpeedOveride(spd_ovr_opt)) = em.speed_overides.get_mut(*p) {
    *spd_ovr_opt = None;
  };
}
