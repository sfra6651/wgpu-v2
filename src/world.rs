use glam::{Vec2, vec2};

use crate::{
  ai_system, animation_system,
  camera::Camera,
  collision_system::{self, Correction},
  entity::{DirIntent, Entity, EntityManager, LastShotAt, Position, Speed},
  entity_templates::{arrow, goblin, player},
  movement_system::{self},
  player_controller::{self, InputState},
  spatial_grid::SpatialGrid,
};

pub const WIDTH: f32 = 30.0;
pub const HEIGHT: f32 = 30.0;

pub struct World {
  pub em: EntityManager,
  pub player: Entity,
  pub spatial_grid: SpatialGrid,
  pub camera: Camera,
  pub size: Vec2,
  pub position_corrections: Vec<Correction>,
}

impl World {
  pub fn new() -> Self {
    let mut em = EntityManager::new();
    let player = player(&mut em);
    let _ = goblin(&mut em, vec2(15.0, 20.0));
    let _ = goblin(&mut em, vec2(10.0, 10.0));
    let _ = goblin(&mut em, vec2(20.0, 20.0));

    Self {
      em,
      player,
      camera: Camera::new((WIDTH, HEIGHT).into()),
      size: (WIDTH, HEIGHT).into(),
      spatial_grid: SpatialGrid::new(2.0),
      position_corrections: Vec::new(),
    }
  }

  pub fn update(&mut self, input: &InputState) {
    ai_system::update_ai(&mut self.em, self.player);
    // update plater sate
    if let Some(DirIntent(dir)) = self.em.dir_intents.get_mut(self.player) {
      *dir = player_controller::player_intent(input);
    };
    movement_system::update_positions(&mut self.em);
    //// re-build after positions update
    self.re_build_spatial_grid();
    animation_system::update_animations(&mut self.em);
    collision_system::handle_collisions(self);

    self.shoot_arrows();

    // update camera
    let Some(&DirIntent(dir_intent)) = self.em.dir_intents.get(self.player) else {
      return;
    };
    let Some(&Speed(speed)) = self.em.speeds.get(self.player) else {
      return;
    };
    self.camera.velocity = dir_intent * speed;
    if let Some(&Position(pos)) = self.em.positions.get(self.player) {
      self.camera.pos = pos;
    }
  }

  fn shoot_arrows(&mut self) {
    let Some(&LastShotAt(last_shot_at)) = self.em.last_shot_at.get(self.player) else {
      return;
    };

    let mut shot = false;
    if last_shot_at >= 120 {
      shot = true;
      let Some(Position(pos)) = self.em.positions.get_copy(self.player) else {
        return;
      };
      arrow(&mut self.em, pos);
    }

    if let Some(LastShotAt(last_shot_at)) = self.em.last_shot_at.get_mut(self.player) {
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
