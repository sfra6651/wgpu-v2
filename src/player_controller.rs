use std::cmp;

use winit::{
  event::{ElementState, KeyEvent},
  keyboard::{KeyCode, PhysicalKey},
};

use crate::{entity::Entity, world::World};

pub fn update_player(world: &mut World, event: KeyEvent) {
  let key = event.physical_key;
  let Some(player) = world.entities.iter_mut().find(|e| e.is_player) else {
    return;
  };
  match event.state {
    ElementState::Pressed => handle_pressed(player, key),
    ElementState::Released => handle_released(player, key),
  }
}

fn handle_pressed(player: &mut Entity, key: PhysicalKey) {
  match key {
    PhysicalKey::Code(KeyCode::KeyW) => player.velocity.y -= 10.0,
    PhysicalKey::Code(KeyCode::KeyA) => player.velocity.x -= 10.0,
    PhysicalKey::Code(KeyCode::KeyS) => player.velocity.y += 10.0,
    PhysicalKey::Code(KeyCode::KeyD) => player.velocity.x += 10.0,
    _ => {}
  }

  player.velocity.x = player.velocity.x.clamp(-10.0, 10.0);
  player.velocity.y = player.velocity.y.clamp(-10.0, 10.0);
}

fn handle_released(player: &mut Entity, key: PhysicalKey) {
  match key {
    PhysicalKey::Code(KeyCode::KeyW) => player.velocity.y += 10.0,
    PhysicalKey::Code(KeyCode::KeyA) => player.velocity.x += 10.0,
    PhysicalKey::Code(KeyCode::KeyS) => player.velocity.y -= 10.0,
    PhysicalKey::Code(KeyCode::KeyD) => player.velocity.x -= 10.0,
    _ => {}
  }
  player.velocity.x = player.velocity.x.clamp(-10.0, 10.0);
  player.velocity.y = player.velocity.y.clamp(-10.0, 10.0);
}
