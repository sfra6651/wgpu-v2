#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  position: [f32; 3],
  uv: [f32; 2],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    }
  }
}

impl std::convert::From<((f32, f32, f32), (f32, f32))> for Vertex {
  fn from(value: ((f32, f32, f32), (f32, f32))) -> Self {
    Self {
      position: [value.0.0, value.0.1, value.0.2],
      uv: [value.1.0, value.1.1],
    }
  }
}

pub fn unit_square() -> [Vertex; 6] {
  [
    ((-0.5, -0.5, 0.0), (0.0, 0.0)).into(),
    ((0.5, -0.5, 0.0), (1.0, 0.0)).into(),
    ((0.5, 0.5, 0.0), (1.0, 1.0)).into(),
    ((-0.5, -0.5, 0.0), (0.0, 0.0)).into(),
    ((0.5, 0.5, 0.0), (1.0, 1.0)).into(),
    ((-0.5, 0.5, 0.0), (0.0, 1.0)).into(),
  ]
}
