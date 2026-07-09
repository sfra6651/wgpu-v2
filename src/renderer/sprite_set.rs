use crate::{
  entity::{Action, Entity, Facing},
  renderer::texture::Texture,
};

pub struct SpriteSet {
  idle: [Texture; 8],
  run: [[Texture; 4]; 8],
}

impl SpriteSet {
  pub fn resolve(&self, action: Action, facing: Facing, tick: usize) -> &Texture {
    match action {
      Action::Idle => &self.idle[facing.to_i()],
      Action::Run => {
        let frame = (tick / Entity::TICKS_PER_FRAME) % Entity::WALK_FRAMES;
        &self.run[facing.to_i()][frame]
      }
    }
  }

  pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
    let idle_textures: [Texture; 8] = std::array::from_fn(|i| {
      let n = i + 1;
      Texture::new(
        device,
        queue,
        &format!("src/assets/goblin/goblin_000{}.png", n),
        &format!("goblin_{}", n),
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
          &format!("src/assets/goblin/run_{}_000{}.png", code, n),
          &format!("goblin_{}_{}", code, n),
        )
      })
    });

    Self {
      idle: idle_textures,
      run: run_textures,
    }
  }
}
