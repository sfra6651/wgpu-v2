use glam::{Mat4, Vec2};

pub fn projection_matrix(aspect: f32, view_height: f32) -> Mat4 {
  let half_h = view_height / 2.0;
  let half_w = half_h * aspect;
  glam::camera::rh::proj::directx::orthographic(-half_w, half_w, -half_h, half_h, -1.0, 1.0)
}

pub fn model_matrix(pos: glam::Vec2, scale: glam::Vec2, rotation: f32) -> Mat4 {
  Mat4::from_translation(pos.extend(0.0))
    * Mat4::from_rotation_z(rotation)
    * Mat4::from_scale(scale.extend(1.0))
}

pub fn ui_projection_matrix(screen_size: Vec2) -> Mat4 {
  // left, right, bottom, top — note bottom/top swapped so y points down
  glam::camera::rh::proj::directx::orthographic(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0)
}
