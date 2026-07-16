use glam::Vec2;

use crate::entity::{AiType, Facing, Kind};

pub const MAX_ENTITIES: usize = 1000;

#[derive(Clone, Copy, PartialEq)]
pub struct Entity {
  id: u32,
  generation: u32,
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
    if self.slots_used as usize >= MAX_ENTITIES && self.free_list.len() == 0 {
      panic!("entity limit reached, TODO: handle gracefully???");
    }
    let id;
    let generation;
    if let Some(free_id) = self.free_list.pop() {
      id = free_id;
      self.generations[id as usize] += 1;
      generation = self.generations[id as usize];
    } else {
      id = self.slots_used;
      generation = 0;
    }

    self.slots_used += 1;
    Entity { id, generation }
  }

  pub fn remove(&mut self, e: Entity) {
    if self.generations[e.id as usize] != e.generation {
      panic!("attempting to remove entity with a stale reference");
    }
    self.free_list.push(e.id);
  }
}

#[derive(Copy, Clone)]
pub struct SparseElement {
  pub dense_index: usize,
  pub generation: u32,
}

const EMPTY_ENTITY: usize = usize::MAX;

pub struct ComponentStore<T> {
  dense: Vec<T>,
  entities: Vec<Entity>,
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

  pub fn remove(&mut self, e: Entity) {
    let dense_index = self.sparse[e.id as usize];
    if dense_index == EMPTY_ENTITY {
      return;
    }
    let dense_last_index = self.dense.len() - 1;

    if self.dense.len() <= 1 {
      self.dense.pop();
      self.entities.pop();
    } else {
      self.dense.swap(dense_index, dense_last_index);
      self.entities.swap(dense_index, dense_last_index);
      let other_e = self.entities[dense_index];
      self.sparse[other_e.id as usize] = dense_index;
    }

    self.sparse[e.id as usize] = EMPTY_ENTITY;
  }
}

pub struct EntityManager {
  entities: EntityStore,
  pub positions: ComponentStore<Vec2>,
  pub render_sizes: ComponentStore<Vec2>,
  pub hit_boxes: ComponentStore<Vec2>,
  pub dir_intents: ComponentStore<Vec2>,
  pub speeds: ComponentStore<f32>,
  pub anim_ticks: ComponentStore<usize>,
  pub facings: ComponentStore<Facing>,
  pub kinds: ComponentStore<Kind>,
  pub ai: ComponentStore<AiType>,
}

impl EntityManager {
  pub fn new() -> Self {
    Self {
      entities: EntityStore::new(),
      positions: ComponentStore::new(),
      render_sizes: ComponentStore::new(),
      hit_boxes: ComponentStore::new(),
      dir_intents: ComponentStore::new(),
      speeds: ComponentStore::new(),
      anim_ticks: ComponentStore::new(),
      facings: ComponentStore::new(),
      kinds: ComponentStore::new(),
      ai: ComponentStore::new(),
    }
  }
  pub fn create(&mut self) -> Entity {
    self.entities.create()
  }

  pub fn remove(&mut self, e: Entity) {
    let EntityManager {
      entities,
      positions,
      render_sizes,
      hit_boxes,
      dir_intents,
      speeds,
      anim_ticks,
      facings,
      kinds,
      ai,
    } = self;

    positions.remove(e);
    entities.remove(e);
    render_sizes.remove(e);
    hit_boxes.remove(e);
    dir_intents.remove(e);
    speeds.remove(e);
    anim_ticks.remove(e);
    facings.remove(e);
    kinds.remove(e);
    ai.remove(e);
  }
}

// --------------tests--------------

#[test]
fn entity_store() {
  let mut store = EntityStore::new();
  let e = store.create();
  assert_eq!(store.slots_used, 1);
  store.remove(e);
  assert_eq!(store.free_list.len(), 1);

  // now check genrations
  let b = store.create();
  assert_eq!(store.free_list.len(), 0);
  assert_eq!(store.generations[0], 1);
}

#[test]
#[should_panic]
fn entity_store_removing_state_ref() {
  let mut store = EntityStore::new();
  let e = store.create();
  store.remove(e);
  let _ = store.create();
  store.remove(e);
}

#[test]
fn component_store() {
  let mut c_store: ComponentStore<Vec2> = ComponentStore::new();
  let mut e_store = EntityStore::new();
  let e = e_store.create();

  c_store.insert((1.0, 1.0).into(), e);

  assert_eq!(c_store.dense.len(), 1);
  assert_eq!(c_store.entities.len(), 1);

  c_store.remove(e);
  assert_eq!(e.id, 0);
  assert_eq!(c_store.dense.len(), 0);
  assert_eq!(c_store.sparse[0], EMPTY_ENTITY);

  // test stale Entity reference call fails
  let b = e_store.create();
  assert_eq!(b.id, 1);
  assert!(c_store.get(e).is_none());
}

