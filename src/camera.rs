use crate::utils::projection_matrix;
use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
  pub pos: Vec2, // world point the camera is centered on
  pub velocity: Vec2,
  pub zoom: f32,        // > 1.0 = zoomed in
  pub view_height: f32, // world units visible vertically at zoom 1.0
}

impl Camera {
  pub fn new(world_size: Vec2) -> Self {
    Self {
      pos: world_size / 2.0,
      velocity: (0.0, 0.0).into(),
      zoom: 1.0,
      view_height: 20.0, // show ~20 units tall
    }
  }

  /// world space → camera space.
  pub fn view_matrix(&self) -> Mat4 {
    Mat4::from_scale(Vec3::new(self.zoom, self.zoom, 1.0))
      * Mat4::from_translation((-self.pos).extend(0.0))
  }

  pub fn view_projection(&self, aspect: f32) -> Mat4 {
    projection_matrix(aspect, self.view_height) * self.view_matrix()
  }
}
