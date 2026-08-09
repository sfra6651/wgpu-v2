use glam::{Mat4, vec2};

use crate::{
  renderer::{DrawEntity, UiElement},
  utils::{model_matrix, ui_projection_matrix},
};

pub struct UiTransfroms {
  pub buffer: wgpu::Buffer,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub bind_group: wgpu::BindGroup,
  pub count: u64,
}

const MAX_UI_TRANSFORMS: u64 = 100;

impl UiTransfroms {
  pub fn new(device: &wgpu::Device) -> Self {
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("model transforms buffer"),
      size: size_of::<Mat4>() as u64 * MAX_UI_TRANSFORMS,
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

    UiTransfroms {
      buffer,
      bind_group_layout,
      bind_group,
      count: 0,
    }
  }
  pub fn write_transforms(&mut self, ui_elements: &[UiElement], queue: &wgpu::Queue) {
    let mut instances: Vec<Mat4> = Vec::new();

    for draw_e in ui_elements.iter() {
      instances.push(model_matrix(draw_e.pos, draw_e.size.0, 0.0));
    }

    let data: Vec<[f32; 16]> = instances.iter().map(|m| m.to_cols_array()).collect();

    queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));

    self.count = instances.len() as u64;
  }
}
