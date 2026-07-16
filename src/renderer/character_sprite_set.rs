use crate::{
  entity::{Action, Entity, Facing},
  renderer::texture::Texture,
};

pub struct CharacterSpriteSet {
  idle: [Texture; 8],
  run: [[Texture; 4]; 8],
}

impl CharacterSpriteSet {
  pub fn resolve(&self, action: Action, facing: Facing, tick: usize) -> &Texture {
    match action {
      Action::Idle => &self.idle[facing.usize()],
      Action::Run => {
        let frame = (tick / Entity::TICKS_PER_FRAME) % Entity::WALK_FRAMES;
        &self.run[facing.usize()][frame]
      }
    }
  }

  pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, name: &str) -> Self {
    let idle_textures: [Texture; 8] = std::array::from_fn(|i| {
      let n = i + 1;
      Texture::new(
        device,
        queue,
        &format!("src/assets/{}/{}_000{}.png", name, name, n),
        &format!("{}_{}", name, n),
      )
    });

    let directions = ["s", "se", "e", "ne", "n", "nw", "w", "sw"];
    let run_textures: [[Texture; 4]; 8] = std::array::from_fn(|i| {
      let code = directions[i];
      std::array::from_fn(|j| {
        let n = j + 1;
        Texture::new(
          device,
          queue,
          &format!("src/assets/{}/run_{}_000{}.png", name, code, n),
          &format!("{}_{}_{}", name, code, n),
        )
      })
    });

    Self {
      idle: idle_textures,
      run: run_textures,
    }
  }
}
