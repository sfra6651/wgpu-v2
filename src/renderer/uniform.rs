#[repr(usize)]
pub enum UniformVariant {
  WorldSpaceProjection,
  ScreenSpaceProjection,
  Count,
}

pub struct Uniform {
  pub buffer: wgpu::Buffer,
  pub bind_group: wgpu::BindGroup,
}

pub struct UniformManager {
  pub uniforms: [Uniform; UniformVariant::COUNT],
  pub bind_group_layout: wgpu::BindGroupLayout,
}

impl UniformVariant {
  pub const COUNT: usize = UniformVariant::Count as usize;
  pub const ALL: [UniformVariant; Self::COUNT] = [
    UniformVariant::WorldSpaceProjection,
    UniformVariant::ScreenSpaceProjection,
  ];

  pub fn name(&self) -> &'static str {
    match self {
      UniformVariant::WorldSpaceProjection => "camera",
      UniformVariant::ScreenSpaceProjection => "ui",
      UniformVariant::Count => unreachable!("Count is not a real uniform"),
    }
  }

  pub fn size(&self) -> u64 {
    match self {
      UniformVariant::WorldSpaceProjection => 64,
      UniformVariant::ScreenSpaceProjection => 64,
      UniformVariant::Count => unreachable!("Count is not a real uniform"),
    }
  }
}

impl UniformManager {
  pub fn new(device: &wgpu::Device) -> Self {
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("uniform bind group layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let uniforms: [Uniform; UniformVariant::COUNT] =
      UniformVariant::ALL.map(|v| Uniform::new(device, &bind_group_layout, v.name(), v.size()));

    Self {
      uniforms,
      bind_group_layout,
    }
  }

  pub fn get(&self, variant: UniformVariant) -> &Uniform {
    &self.uniforms[variant as usize]
  }
}

impl Uniform {
  pub fn new(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    name: &str,
    size: u64,
  ) -> Self {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some(&format!("{} buffer", name)),
      size,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some(&format!("{} bind group", name)),
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
    });

    Uniform { buffer, bind_group }
  }
}
