use std::time::Instant;

use glam::Vec2;
use winit::keyboard::KeyCode;

use crate::{
  camera::Camera,
  entity::{DirIntent, Entity, EntityManager, Health, LastShotAt, Position, Speed},
  entity_templates::{arrow, player},
  player_controller::{self, InputState},
  spatial_grid::SpatialGrid,
  systems::{
    ability_system, ai_system, animation_system,
    collision_system::{self},
    enemy_spawn_system::spawn_enemies,
    lifetime_system::handle_lifetimes,
    movement_system,
  },
};

pub const WIDTH: f32 = 30.0;
pub const HEIGHT: f32 = 30.0;

pub struct PosUpdate {
  pub e: Entity,
  pub offset: Vec2,
}

pub struct World {
  pub em: Box<EntityManager>,
  pub spatial_grid: SpatialGrid,
  pub camera: Camera,
  pub size: Vec2,
  pub position_updates: Vec<PosUpdate>,
  pub removals: Vec<Entity>,
  pub last_spawn: Instant,
}

impl World {
  pub fn new() -> Self {
    let mut em = Box::new(EntityManager::new());

    let player = player(&mut em);
    em.player = Some(player);

    Self {
      em,
      camera: Camera::new((WIDTH, HEIGHT).into()),
      size: (WIDTH, HEIGHT).into(),
      spatial_grid: SpatialGrid::new(1.0),
      last_spawn: Instant::now(),
      position_updates: Vec::new(),
      removals: Vec::new(),
    }
  }

  pub fn update(&mut self, input: &InputState) {
    self.removals.clear();

    handle_lifetimes(&mut self.em, &mut self.removals);

    ai_system::update_ai(&mut self.em);
    self.update_player(input);

    let t = Instant::now();
    movement_system::update_positions(&mut self.em);

    //// re-build after positions update
    self.re_build_spatial_grid();

    animation_system::update_animations(&mut self.em);

    collision_system::handle_physics_collisions(self);
    collision_system::handle_projectile_collisions(self);

    self.apply_pos_corrections();
    self.position_updates.clear();

    self.update_camera();

    self.shoot_arrows();

    let elapsed = self.last_spawn.elapsed().as_secs_f32();
    if elapsed >= 1.0 {
      spawn_enemies(&mut self.em, &self.camera);
      self.last_spawn = Instant::now();
    }

    // clear removals and entity updates for the next frame
    self.handle_removals();
  }

  fn apply_pos_corrections(&mut self) {
    for u in self.position_updates.iter() {
      if let Some(Position(pos)) = self.em.positions.get_mut(u.e) {
        *pos += u.offset;
      }
    }
  }

  fn handle_removals(&mut self) {
    // the same entity can be queued more than once in a frame
    // (e.g. one arrow overlapping two goblins), so skip duplicates
    for i in 0..self.removals.len() {
      if self.removals[..i].contains(&self.removals[i]) {
        continue;
      }
      self.em.remove(self.removals[i]);
    }
  }

  pub fn update_player(&mut self, input: &InputState) {
    let Some(player) = self.em.player else {
      return;
    };
    if let Some(DirIntent(dir)) = self.em.dir_intents.get_mut(player) {
      *dir = player_controller::player_dir_intent(input);

      if input.is_down(KeyCode::Space) {
        ability_system::begin_dash(&mut self.em, player);
      }
    };
  }

  fn update_camera(&mut self) {
    let Some(player) = self.em.player else {
      return;
    };
    // update camera
    let Some(&DirIntent(dir_intent)) = self.em.dir_intents.get(player) else {
      return;
    };
    let Some(&Speed(speed)) = self.em.speeds.get(player) else {
      return;
    };
    self.camera.velocity = dir_intent * speed;
    if let Some(&Position(pos)) = self.em.positions.get(player) {
      self.camera.pos = pos;
    }
  }

  fn shoot_arrows(&mut self) {
    let Some(player) = self.em.player else {
      return;
    };
    let Some(&LastShotAt(last_shot_at)) = self.em.last_shot_at.get(player) else {
      return;
    };

    let mut shot = false;
    if last_shot_at >= 60 {
      shot = true;
      if let Some(closest) = self.spatial_grid.find_nearest(&self.em, player, 10.0) {
        let Some(&Position(pos)) = self.em.positions.get(player) else {
          return;
        };

        let dir = (closest.pos.0 - pos).normalize_or_zero();
        if dir != Vec2::ZERO {
          arrow(&mut self.em, pos, dir);
        }
      }
    }

    if let Some(LastShotAt(last_shot_at)) = self.em.last_shot_at.get_mut(player) {
      if shot {
        *last_shot_at = 0;
      } else {
        *last_shot_at += 1;
      }
    }
  }

  fn re_build_spatial_grid(&mut self) {
    self.spatial_grid.clear();
    for (_, &e) in self.em.positions.iter() {
      self.spatial_grid.insert(&self.em, e);
    }
  }
}
