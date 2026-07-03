use winit::{
  event::{ElementState, KeyEvent},
  keyboard::{KeyCode, PhysicalKey},
};

use crate::{camera::Camera, entity::Entity, world::World};

const CAMERA_SPEED: f32 = 1.0;

pub fn update_camera(world: &mut World, event: &KeyEvent) {
  let key = event.physical_key;
  match event.state {
    ElementState::Pressed => handle_pressed(&mut world.camera, key),
    ElementState::Released => {}
  }
}

fn handle_pressed(camera: &mut Camera, key: PhysicalKey) {
  match key {
    PhysicalKey::Code(KeyCode::ArrowUp) => camera.pos.y += CAMERA_SPEED,
    PhysicalKey::Code(KeyCode::ArrowLeft) => camera.pos.x -= CAMERA_SPEED,
    PhysicalKey::Code(KeyCode::ArrowDown) => camera.pos.y -= CAMERA_SPEED,
    PhysicalKey::Code(KeyCode::ArrowRight) => camera.pos.x += CAMERA_SPEED,
    _ => {}
  }
}
