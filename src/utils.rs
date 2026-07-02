use glam::Mat4;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];

  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}

impl std::convert::From<(f32, f32, f32)> for Vertex {
  fn from(value: (f32, f32, f32)) -> Self {
    Self {
      position: [value.0, value.1, value.2],
    }
  }
}

pub fn unit_square() -> [Vertex; 6] {
  [
    (-0.5, -0.5, 0.0).into(),
    (0.5, -0.5, 0.0).into(),
    (0.5, 0.5, 0.0).into(),
    (-0.5, -0.5, 0.0).into(),
    (0.5, 0.5, 0.0).into(),
    (-0.5, 0.5, 0.0).into(),
  ]
}

pub fn pos_to_trangle(x: f32, y: f32, size: f32) -> [Vertex; 3] {
  [
    (x - size, y - size, 0.0).into(),
    (x + size, y - size, 0.0).into(),
    (x, y + size, 0.0).into(),
  ]
}

pub fn projection_matrix(width: f32, height: f32) -> Mat4 {
  glam::camera::rh::proj::directx::orthographic(0.0, width, height, 0.0, -1.0, 1.0)
}

pub fn model_matrix(pos: glam::Vec2, scale: glam::Vec2) -> Mat4 {
  Mat4::from_translation(pos.extend(0.0)) * Mat4::from_scale(scale.extend(1.0))
}
