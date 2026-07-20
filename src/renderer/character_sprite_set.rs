use crate::{
  entity::{self, Action, AnimTick, Entity, EntityManager, Facing},
  renderer::texture::Texture,
};

pub struct CharacterSpriteSet {
  idle: [Texture; 8],
  run: [[Texture; 4]; 8],
}

impl CharacterSpriteSet {
  pub fn resolve(&self, em: &EntityManager, e: Entity) -> Option<&Texture> {
    let &facing = em.facings.get(e)?;
    let &action = em.actions.get(e)?;
    let &AnimTick(anim_tick) = em.anim_ticks.get(e)?;

    match action {
      Action::Idle => Some(&self.idle[facing.usize()]),
      Action::Run => {
        let frame = (anim_tick / entity::TICKS_PER_FRAME) % entity::WALK_FRAMES;
        Some(&self.run[facing.usize()][frame])
      }
      _ => self.idle.first(),
    }
  }

  pub fn resolve_raw(&self, action: Action, facing: Facing, tick: usize) -> &Texture {
    match action {
      Action::Idle => &self.idle[facing.usize()],
      Action::Run => {
        let frame = (tick / entity::TICKS_PER_FRAME) % entity::WALK_FRAMES;
        &self.run[facing.usize()][frame]
      }
      _ => &self.idle[0],
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
