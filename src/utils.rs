use glam::Mat4;

pub fn projection_matrix(width: f32, height: f32) -> Mat4 {
  glam::camera::rh::proj::directx::orthographic(0.0, width, height, 0.0, -1.0, 1.0)
}

pub fn model_matrix(pos: glam::Vec2, scale: glam::Vec2) -> Mat4 {
  Mat4::from_translation(pos.extend(0.0)) * Mat4::from_scale(scale.extend(1.0))
}
