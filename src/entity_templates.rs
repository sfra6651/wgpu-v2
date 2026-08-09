use glam::{Vec2, vec2};

use crate::{
  entity::{
    Action, AiType, AnimTick, Damage, DirIntent, Entity, EntityManager, Facing, FrontalCone,
    Health, HitBoxRad, LastShotAt, Layer, LifeTime, Npc, Position, Projectile, RenderSize,
    Rotation, Speed, TextureType,
  },
  systems::ability_system::{Ability, AbilityName},
  ui::{AnchorPoint, RenderPos, UiSize},
  world,
};

pub fn player(em: &mut EntityManager) -> Entity {
  let e = em.create();
  em.attach(e, Position(vec2(world::WIDTH / 2.0, world::HEIGHT / 2.0)));
  em.attach(e, RenderSize(vec2(3.0, 3.0)));
  em.attach(e, HitBoxRad(0.5));
  em.attach(e, DirIntent(Vec2::ZERO));
  em.attach(e, Speed(0.1));
  em.attach(e, AnimTick(0));
  em.attach(e, Facing::S);
  em.attach(e, TextureType::Player);
  em.attach(e, Action::Idle);
  em.attach(e, LastShotAt(0));
  em.attach(e, Layer::WorldSpace);
  em.attach(
    e,
    Ability {
      cd: 60.0,
      remaing_cd: 0.0,
      name: AbilityName::Dash,
    },
  );
  e
}

pub fn goblin(em: &mut EntityManager, pos: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, Npc);
  em.attach(e, Position(pos));
  em.attach(e, RenderSize(vec2(2.0, 2.0)));
  em.attach(e, HitBoxRad(0.25));
  em.attach(e, DirIntent(Vec2::ZERO));
  em.attach(e, Speed(0.05));
  em.attach(e, AnimTick(0));
  em.attach(e, Facing::S);
  em.attach(e, Action::Idle);
  em.attach(e, TextureType::Goblin);
  em.attach(e, AiType::Goblin);
  em.attach(e, Health(3.0));
  em.attach(e, Layer::WorldSpace);
  e
}

pub fn arrow(em: &mut EntityManager, pos: Vec2, dir: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, Projectile);
  em.attach(e, Position(pos));
  em.attach(e, RenderSize(vec2(0.5, 0.5)));
  em.attach(e, HitBoxRad(0.1));
  em.attach(e, DirIntent(dir));
  em.attach(e, Rotation(dir.y.atan2(dir.x)));
  em.attach(e, Speed(0.2));
  em.attach(e, TextureType::Arrow);
  em.attach(e, Damage(1.0));
  em.attach(e, Layer::WorldSpace);
  e
}

pub fn frontal_cone(em: &mut EntityManager, pos: Vec2, dir: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, FrontalCone);
  em.attach(e, Position(pos));
  em.attach(e, RenderSize(vec2(1.0, 1.0)));
  em.attach(e, HitBoxRad(2.0));
  em.attach(e, DirIntent(dir));
  em.attach(e, Layer::WorldSpace);
  em.attach(e, LifeTime(60.0));
  e
}

pub fn ui_box(em: &mut EntityManager, ap: AnchorPoint, size: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, ap);
  em.attach(e, UiSize(size));
  e
}
