@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(1) @binding(0)
var<storage, read> models: array<mat4x4<f32>>;

struct VertexInput {
  @location(0) position: vec3<f32>
}


@vertex
fn vs_main(in: VertexInput, @builtin(instance_index) instance: u32) -> @builtin(position) vec4<f32> {
  return projection * models[instance] * vec4<f32>(in.position, 1.0);
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
  let r = 1.0;
  let g = 0.5;
  let b = 0.0;
  return vec4<f32>(r, g, b, 1.0);
}
