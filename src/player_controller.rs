use std::collections::HashSet;

use glam::Vec2;
use winit::{
  event::{ElementState, KeyEvent},
  keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct InputState {
  held: HashSet<KeyCode>,
}

impl InputState {
  pub fn handle_key_event(&mut self, event: &KeyEvent) {
    let PhysicalKey::Code(code) = event.physical_key else {
      return;
    };
    match event.state {
      ElementState::Pressed => self.held.insert(code),
      ElementState::Released => self.held.remove(&code),
    };
  }

  pub fn is_down(&self, code: KeyCode) -> bool {
    self.held.contains(&code)
  }
}

pub fn player_dir_intent(input: &InputState) -> Vec2 {
  let mut dir = Vec2::ZERO;
  if input.is_down(KeyCode::KeyW) {
    dir.y += 1.0
  }
  if input.is_down(KeyCode::KeyS) {
    dir.y -= 1.0
  }
  if input.is_down(KeyCode::KeyA) {
    dir.x -= 1.0
  }
  if input.is_down(KeyCode::KeyD) {
    dir.x += 1.0
  }
  dir.normalize_or_zero() // diagonal is length 1, not √2
}
