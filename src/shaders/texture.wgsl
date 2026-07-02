@group(0) @binding(0)
var<uniform> projection: mat4x4<f32>;

@group(1) @binding(0)
var<storage, read> models: array<mat4x4<f32>>;

@group(2) @binding(0)
var tex: texture_2d<f32>;
@group(2) @binding(1)
var samp: sampler;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput, @builtin(instance_index) instance: u32) -> VertexOutput {
  var out: VertexOutput;
  out.clip_position = projection * models[instance] * vec4<f32>(in.position, 1.0);
  out.uv = in.uv;
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(tex, samp, in.uv);
}
