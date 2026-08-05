use std::{
  iter::Zip,
  slice::{Iter, IterMut},
};

use glam::Vec2;

use crate::systems::ability_system::Ability;

pub const TICKS_PER_FRAME: usize = 8;
pub const WALK_FRAMES: usize = 4;

pub const MAX_ENTITIES: usize = 10000;
pub const EMPTY_ENTITY: usize = usize::MAX;

#[derive(Copy, Clone)]
pub enum Action {
  Idle,
  Run,
}

#[derive(Copy, Clone)]
pub enum Facing {
  S,
  SE,
  E,
  NE,
  N,
  NW,
  W,
  SW,
}

impl Facing {
  pub fn usize(&self) -> usize {
    *self as usize
  }
}

#[derive(Copy, Clone)]
pub enum AiType {
  Goblin,
}

#[derive(Copy, Clone)]
pub enum TextureType {
  Player,
  Goblin,
  Arrow,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Entity {
  pub id: u32,
  pub generation: u32,
}

pub struct EntityStore {
  slots_used: u32,
  free_list: Vec<u32>,
  generations: [u32; MAX_ENTITIES],
}

impl EntityStore {
  pub fn new() -> Self {
    EntityStore {
      slots_used: 0,
      free_list: Vec::new(),
      generations: [0; MAX_ENTITIES],
    }
  }

  pub fn create(&mut self) -> Entity {
    if self.slots_used as usize >= MAX_ENTITIES && self.free_list.is_empty() {
      panic!("entity limit reached, TODO: handle gracefully???");
    }
    let id;
    let generation;
    if let Some(free_id) = self.free_list.pop() {
      id = free_id;
      generation = self.generations[id as usize];
    } else {
      id = self.slots_used;
      generation = 0;
      self.slots_used += 1;
    }

    Entity { id, generation }
  }

  pub fn remove(&mut self, e: Entity) {
    if self.generations[e.id as usize] != e.generation {
      panic!("attempting to remove entity with a stale reference");
    }
    self.generations[e.id as usize] += 1;
    self.free_list.push(e.id);
  }
}

pub struct ComponentStore<T> {
  pub dense: Vec<T>,
  pub entities: Vec<Entity>,
  sparse: [usize; MAX_ENTITIES],
}

impl<T> ComponentStore<T> {
  pub fn new() -> Self {
    ComponentStore {
      dense: Vec::new(),
      entities: Vec::new(),
      sparse: [EMPTY_ENTITY; MAX_ENTITIES],
    }
  }

  pub fn iter(&self) -> Zip<Iter<'_, T>, Iter<'_, Entity>> {
    self.dense.iter().zip(self.entities.iter())
  }

  pub fn iter_mut(&mut self) -> Zip<IterMut<'_, T>, IterMut<'_, Entity>> {
    self.dense.iter_mut().zip(self.entities.iter_mut())
  }

  pub fn insert(&mut self, component: T, e: Entity) {
    let dense_index = self.dense.len();
    self.dense.push(component);
    self.entities.push(e);
    self.sparse[e.id as usize] = dense_index;
  }

  pub fn get(&self, e: Entity) -> Option<&T> {
    let dense_index = self.sparse[e.id as usize];
    if dense_index == EMPTY_ENTITY {
      return None;
    }
    let store_e = self.entities[dense_index];
    if store_e != e {
      return None;
    }
    self.dense.get(dense_index)
  }

  pub fn get_mut(&mut self, e: Entity) -> Option<&mut T> {
    let dense_index = self.sparse[e.id as usize];
    if dense_index == EMPTY_ENTITY {
      return None;
    }
    let store_e = self.entities[dense_index];
    if store_e != e {
      return None;
    }
    self.dense.get_mut(dense_index)
  }

  pub fn clear(&mut self) {
    self.dense.clear();
    self.entities.clear();
    self.sparse = [EMPTY_ENTITY; MAX_ENTITIES];
  }

  pub fn remove(&mut self, e: Entity) {
    let dense_index = self.sparse[e.id as usize];
    if dense_index == EMPTY_ENTITY {
      return;
    }
    if self.entities[dense_index] != e {
      return;
    }
    self.dense.swap_remove(dense_index);
    self.entities.swap_remove(dense_index);
    // the moved element (if any) needs its sparse index repaired
    if dense_index < self.entities.len() {
      let other_e = self.entities[dense_index];
      self.sparse[other_e.id as usize] = dense_index;
    }
    self.sparse[e.id as usize] = EMPTY_ENTITY;
  }
}

// this *generates* struct EntityManager, new(), create(), remove(), and all the Component impls
macro_rules! components {
  ($($field:ident: $comp:ty),+ $(,)?) => {
    pub struct EntityManager {
      entities: EntityStore,
      pub player: Option<Entity>,
      $(pub $field: ComponentStore<$comp>,)+
    }

    impl EntityManager {
      pub fn new() -> Self {
        Self {
          entities: EntityStore::new(),
          player: None,
          $($field: ComponentStore::new(),)+
        }
      }

      pub fn create(&mut self) -> Entity {
        self.entities.create()
      }

      pub fn remove(&mut self, e: Entity) {
        self.entities.remove(e);
        $(self.$field.remove(e);)+
      }
    }

    $(
      impl Component for $comp {
        fn store(em: &EntityManager) -> &ComponentStore<Self> { &em.$field }
        fn store_mut(em: &mut EntityManager) -> &mut ComponentStore<Self> { &mut em.$field }
      }
    )+
  };
}

// this is not the source of truth for enitites, these are only wrappers for basic types so that we can attatch the Component Traid to them,
// without this we could not distinquist a Position Vec2 from a RenderSize Vec2 at the type level in the store
#[derive(Clone, Copy)]
pub struct Position(pub Vec2);

#[derive(Clone, Copy)]
pub struct RenderSize(pub Vec2);

#[derive(Clone, Copy)]
pub struct HitBoxRad(pub f32);

#[derive(Clone, Copy)]
pub struct DirIntent(pub Vec2);

#[derive(Clone, Copy)]
pub struct Rotation(pub f32);

#[derive(Clone, Copy)]
pub struct Speed(pub f32);

#[derive(Clone, Copy)]
pub struct SpeedOveride(pub Option<f32>);

#[derive(Clone, Copy)]
pub struct AnimTick(pub usize);

#[derive(Clone, Copy)]
pub struct LastShotAt(pub usize);

#[derive(Clone, Copy)]
pub struct Damage(pub f32);

#[derive(Clone, Copy)]
pub struct Health(pub f32);

#[derive(Clone, Copy)]
pub struct LifeTime(pub f32);

#[derive(Clone, Copy)]
pub struct Parent(pub Entity);

#[derive(Clone, Copy)]
pub struct OnDestroy(pub fn(&mut EntityManager, Entity));

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Layer {
  Floor = 0,
  WorldSpace = 2,
}

// Entity type tags
#[derive(Clone, Copy)]
pub struct Npc;

#[derive(Clone, Copy)]
pub struct Projectile;

#[derive(Clone, Copy)]
pub struct FrontalCone;

pub trait Component: Sized {
  fn store(em: &EntityManager) -> &ComponentStore<Self>;
  fn store_mut(em: &mut EntityManager) -> &mut ComponentStore<Self>;
}

// this is the SOURCE OF TRUTH for components as every component here gets the Component trait implemented
// thanks to the macro expansion
components! {
  positions: Position,
  render_sizes: RenderSize,
  hit_box_rads: HitBoxRad,
  dir_intents: DirIntent,
  rotations: Rotation,
  speeds: Speed,
  speed_overides: SpeedOveride,
  anim_ticks: AnimTick,
  facings: Facing,
  ai: AiType,
  last_shot_at: LastShotAt,
  actions: Action,
  damages: Damage,
  healths: Health,
  layers: Layer,
  parents: Parent,
  lifetimes: LifeTime,
  abilities: Ability,
  on_destroys: OnDestroy,

  //entity tags
  npcs: Npc,
  projectiles: Projectile,
  frontal_cones: FrontalCone,

  //render specifics
  texture_types: TextureType,
}

impl EntityManager {
  pub fn attach<C: Component>(&mut self, e: Entity, component: C) {
    C::store_mut(self).insert(component, e);
  }
}
