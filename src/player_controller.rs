use winit::{
  event::{ElementState, KeyEvent},
  keyboard::{KeyCode, PhysicalKey},
};

use crate::{entity::Entity, world::World};

const PLAYER_SPEED: f32 = 0.1;
pub fn update_player(world: &mut World, event: &KeyEvent) {
  let key = event.physical_key;
  let Some(player) = world.entities.iter_mut().find(|e| e.is_player) else {
    return;
  };
  match event.state {
    ElementState::Pressed => handle_pressed(player, key),
    ElementState::Released => handle_released(player, key),
  }

  world.camera.velocity = player.velocity;
}

fn handle_pressed(player: &mut Entity, key: PhysicalKey) {
  match key {
    PhysicalKey::Code(KeyCode::KeyW) => player.velocity.y += PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyA) => player.velocity.x -= PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyS) => player.velocity.y -= PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyD) => player.velocity.x += PLAYER_SPEED,
    _ => {}
  }

  player.velocity.x = player.velocity.x.clamp(-PLAYER_SPEED, PLAYER_SPEED);
  player.velocity.y = player.velocity.y.clamp(-PLAYER_SPEED, PLAYER_SPEED);

  // set the dir, only set on press, should store the characters current direction - on key release direction does not change
  player.dir = player.velocity.normalize_or_zero();
}

fn handle_released(player: &mut Entity, key: PhysicalKey) {
  match key {
    PhysicalKey::Code(KeyCode::KeyW) => player.velocity.y -= PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyA) => player.velocity.x += PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyS) => player.velocity.y += PLAYER_SPEED,
    PhysicalKey::Code(KeyCode::KeyD) => player.velocity.x -= PLAYER_SPEED,
    _ => {}
  }
  player.velocity.x = player.velocity.x.clamp(-PLAYER_SPEED, PLAYER_SPEED);
  player.velocity.y = player.velocity.y.clamp(-PLAYER_SPEED, PLAYER_SPEED);
}
