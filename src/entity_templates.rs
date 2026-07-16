use glam::{Vec2, vec2};

use crate::{
  entity::{Action, AiType, Facing, Kind},
  entity_manager::{
    AnimTick, DirIntent, Entity, EntityManager, HitBox, LastShotAt, Position, RenderSize, Speed,
  },
  world,
};

pub fn player(em: &mut EntityManager) -> Entity {
  let e = em.create();
  em.attach(e, Position(vec2(world::WIDTH / 2.0, world::HEIGHT / 2.0)));
  em.attach(e, RenderSize(vec2(3.0, 3.0)));
  em.attach(e, HitBox(vec2(1.0, 1.5)));
  em.attach(e, DirIntent(Vec2::ZERO));
  em.attach(e, Speed(0.1));
  em.attach(e, AnimTick(0));
  em.attach(e, Facing::S);
  em.attach(e, Action::Idle);
  em.attach(e, Kind::Player);
  em.attach(e, LastShotAt(0));
  e
}

pub fn goblin(em: &mut EntityManager, pos: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, Position(pos));
  em.attach(e, RenderSize(vec2(2.0, 2.0)));
  em.attach(e, HitBox(vec2(0.5, 0.5)));
  em.attach(e, DirIntent(Vec2::ZERO));
  em.attach(e, Speed(0.05));
  em.attach(e, AnimTick(0));
  em.attach(e, Facing::S);
  em.attach(e, Action::Idle);
  em.attach(e, Kind::Enemy);
  em.attach(e, AiType::Goblin);
  e
}

pub fn arrow(em: &mut EntityManager, pos: Vec2) -> Entity {
  let e = em.create();
  em.attach(e, Position(pos));
  em.attach(e, RenderSize(vec2(0.5, 0.5)));
  em.attach(e, HitBox(vec2(0.5, 0.2)));
  em.attach(e, DirIntent(vec2(1.0, 0.0)));
  em.attach(e, Speed(0.1));
  em.attach(e, Kind::Projectile);
  e
}
