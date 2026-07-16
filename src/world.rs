use glam::Vec2;

use crate::{
  camera::Camera,
  entity::{Action, Entity, Facing, Kind, PlayerState},
  entity_manager::EntityManager,
  player_controller::{self, InputState},
  spatial_grid::SpatialGrid,
};

pub const WIDTH: f32 = 30.0;
pub const HEIGHT: f32 = 30.0;

pub enum CollisionResponse {
  Block,
  Push,
  Trigger,
  Ignore,
}

fn collision_response(a: &Kind, b: &Kind) -> CollisionResponse {
  use Kind::*;
  match (a, b) {
    (Player, Enemy) | (Enemy, Player) => CollisionResponse::Push,
    (Enemy, Enemy) => CollisionResponse::Push,
    (Projectile, Enemy) | (Enemy, Projectile) => CollisionResponse::Trigger,
    _ => CollisionResponse::Ignore,
  }
}

pub struct Correction {
  index: usize,
  correction: Vec2,
}

impl Correction {
  pub fn new(index: usize, correction: Vec2) -> Self {
    Correction { index, correction }
  }
}

pub struct World {
  pub entities: Vec<Entity>,
  pub entity_manager: EntityManager,
  pub player_state: PlayerState,
  pub spatial_grid: SpatialGrid,
  pub camera: Camera,
  pub size: Vec2,
  position_corrections: Vec<Correction>,
}

impl World {
  pub fn new() -> Self {
    let entities: Vec<Entity> = vec![
      Entity::player(),
      Entity::goblin((15.0, 25.0).into()),
      Entity::goblin((10.0, 10.0).into()),
      Entity::goblin((20.0, 20.0).into()),
    ];

    let spatial_grid = SpatialGrid::new(3.0);

    let player_state = PlayerState { last_shot_at: 0 };

    Self {
      entities,
      player_state,
      camera: Camera::new((WIDTH, HEIGHT).into()),
      size: (WIDTH, HEIGHT).into(),
      spatial_grid,
      position_corrections: Vec::new(),
      entity_manager: EntityManager::new(),
    }
  }

  pub fn update(&mut self, input: &InputState) {
    self.update_ai();
    self.update_player_state(input);
    self.update_positions();
    // re-build after positions update
    self.re_build_spatial_grid();
    self.update_animations();
    self.handle_collisions();

    self.shoot_arrows();

    // update camera
    let player = self.entities.first().unwrap();
    self.camera.velocity = player.dir_intent * player.speed;
    self.camera.pos = player.pos;
  }

  fn handle_collisions(&mut self) {
    // clear corrections
    self.position_corrections.clear();
    // step 1: build collections
    for i in 0..self.entities.len() {
      let e = &self.entities[i];
      let near = self.spatial_grid.near_entities(e.pos);
      for &n in near.iter() {
        if n == i {
          continue;
        }
        let other = &self.entities[n];
        let Some(mtv) = e.penetration(other) else {
          continue;
        };

        match collision_response(&e.kind, &other.kind) {
          // full push-out; `other` is treated as immovable
          CollisionResponse::Block => self.position_corrections.push(Correction::new(i, mtv)),
          // each pair is visited from both sides, so each entity
          // takes half the separation
          CollisionResponse::Push => self
            .position_corrections
            .push(Correction::new(i, mtv / 2.0)),
          CollisionResponse::Ignore => {}
          CollisionResponse::Trigger => {}
        }
      }
    }
    //step 2: resolve corrections
    for e in self.position_corrections.iter() {
      self.entities[e.index].pos += e.correction;
    }
  }

  fn shoot_arrows(&mut self) {
    let Some(player) = self
      .entities
      .iter_mut()
      .find(|e| matches!(e.kind, Kind::Player))
    else {
      return;
    };

    self.player_state.last_shot_at += 1;
    let pos = player.pos;

    if self.player_state.last_shot_at.is_multiple_of(120) {
      self.player_state.last_shot_at = 0;
      self.entities.push(Entity::arrow(pos));
    }
  }

  fn re_build_spatial_grid(&mut self) {
    self.spatial_grid.clear();
    for (i, e) in self.entities.iter().enumerate() {
      self.spatial_grid.insert(e, i);
    }
  }

  fn update_player_state(&mut self, input: &InputState) {
    if let Some(player) = self
      .entities
      .iter_mut()
      .find(|e| matches!(e.kind, Kind::Player))
    {
      player.dir_intent = player_controller::player_intent(input);
    }
  }

  fn update_ai(&mut self) {
    use crate::entity::AiType::*;
    let player_pos = self
      .entities
      .iter()
      .find(|e| matches!(e.kind, Kind::Player))
      .unwrap()
      .pos;

    for entity in self.entities.iter_mut() {
      match entity.ai {
        Goblin => {
          if entity.pos.x > player_pos.x {
            entity.dir_intent.x = -1.0
          }
          if entity.pos.x < player_pos.x {
            entity.dir_intent.x = 1.0
          }
          if entity.pos.y > player_pos.y {
            entity.dir_intent.y = -1.0
          }
          if entity.pos.y < player_pos.y {
            entity.dir_intent.y = 1.0
          }
        }
        None => {}
      }
    }
  }

  fn update_positions(&mut self) {
    for i in 0..self.entities.len() {
      let dir = self.entities[i].dir_intent.normalize_or_zero();
      let speed = self.entities[i].speed;

      let vel = dir * speed;
      self.entities[i].pos += vel;
      // update facing, if velocity is (0,0) keep old facing
      if dir != Vec2::ZERO {
        self.update_facing(i);
      }
    }
  }

  fn update_animations(&mut self) {
    for i in 0..self.entities.len() {
      let vel = self.entities[i].dir_intent * self.entities[i].speed;
      if vel != Vec2::ZERO {
        self.entities[i].anim_tick += 1;
      } else {
        self.entities[i].anim_tick = 0;
      }
      self.entities[i].action = if vel != Vec2::ZERO {
        Action::Run
      } else {
        Action::Idle
      };
    }
  }

  fn update_facing(&mut self, i: usize) {
    use Facing::*;
    let e = &mut self.entities[i];
    let angle = e.dir_intent.y.atan2(e.dir_intent.x);
    let octant = (angle / std::f32::consts::FRAC_PI_4).round() as i32;
    match octant.rem_euclid(8) {
      0 => e.facing = E,
      1 => e.facing = NE,
      2 => e.facing = N,
      3 => e.facing = NW,
      4 => e.facing = W,
      5 => e.facing = SW,
      6 => e.facing = S,
      7 => e.facing = SE,
      _ => unreachable!(),
    }
  }
}
