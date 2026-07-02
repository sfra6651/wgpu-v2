use glam::Mat4;
use rand::RngExt;

use crate::{utils::model_matrix, world::Entity};

pub struct ModelTransforms {
  pub buffer: wgpu::Buffer,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub bind_group: wgpu::BindGroup,
  pub count: u64,
}

const MAX_MODEL_TRANSFORMS: u64 = 10000;

impl ModelTransforms {
  pub fn new(device: &wgpu::Device) -> Self {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("model transforms buffer"),
      size: size_of::<Mat4>() as u64 * MAX_MODEL_TRANSFORMS,
      usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("model transforms bind group layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Storage { read_only: true },
          has_dynamic_offset: false,
          min_binding_size: None,
        },
        count: None,
      }],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("model transforms bind group"),
      layout: &bind_group_layout,
      entries: &[wgpu::BindGroupEntry {
        binding: 0,
        resource: buffer.as_entire_binding(),
      }],
    });

    ModelTransforms {
      buffer,
      bind_group_layout,
      bind_group,
      count: 0,
    }
  }
  pub fn write_transforms(&mut self, queue: &wgpu::Queue, entities: &Vec<Entity>) {
    let mut instances: Vec<Mat4> = Vec::new();

    for entity in entities {
      instances.push(model_matrix(entity.pos, entity.size));
    }

    let data: Vec<[f32; 16]> = instances.iter().map(|m| m.to_cols_array()).collect();

    queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));

    self.count = instances.len() as u64;
  }
}
