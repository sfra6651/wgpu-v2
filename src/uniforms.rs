#[repr(usize)]
pub enum UniformVariant {
  Projection,
  Count,
}

pub struct Uniform {
  pub buffer: wgpu::Buffer,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub bind_group: wgpu::BindGroup,
}

pub struct UniformManager {
  pub uniforms: [Uniform; UniformVariant::COUNT],
}

impl UniformVariant {
  pub const COUNT: usize = UniformVariant::Count as usize;
  pub const ALL: [UniformVariant; Self::COUNT] = [UniformVariant::Projection];

  pub fn name(&self) -> &'static str {
    match self {
      UniformVariant::Projection => "camera",
      UniformVariant::Count => unreachable!("Count is not a real uniform"),
    }
  }

  pub fn size(&self) -> u64 {
    match self {
      UniformVariant::Projection => 64,
      UniformVariant::Count => unreachable!("Count is not a real uniform"),
    }
  }
}

impl UniformManager {
  pub fn new(device: &wgpu::Device) -> Self {
    let uniforms: [Uniform; UniformVariant::COUNT] =
      UniformVariant::ALL.map(|v| Uniform::new(device, v.name(), v.size()));

    Self { uniforms }
  }

  pub fn get(&self, variant: UniformVariant) -> &Uniform {
    &self.uniforms[variant as usize]
  }
}

impl Uniform {
  pub fn new(device: &wgpu::Device, name: &str, size: u64) -> Self {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some(&format!("{} buffer", name)),
      size: size,
      usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some(&format!("{} bing group layout", name)),
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

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some(&format!("{} bind group", name)),
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
    });

    Uniform {
      buffer,
      bind_group_layout,
      bind_group,
    }
  }
}
