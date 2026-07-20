use glam::Vec2;

use crate::{
  entity::{Entity, EntityManager, Position},
  world::{self},
};

pub struct SpatialGrid {
  cell_size: f32,
  rows: usize,
  cols: usize,
  cells: Vec<Vec<Entity>>,
  //needs to be cleared every call of near entities. so we dont have to create a vec in a hot loop
  near_list: Vec<Entity>,
}

impl SpatialGrid {
  pub fn new(cell_size: f32) -> Self {
    let rows = (world::HEIGHT / cell_size).ceil() as usize;
    let cols = (world::WIDTH / cell_size).ceil() as usize;
    let mut cells: Vec<Vec<Entity>> = Vec::new();
    for _ in 0..rows * cols {
      cells.push(Vec::new());
    }

    Self {
      cell_size,
      rows,
      cols,
      cells,
      near_list: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    for cell in self.cells.iter_mut() {
      cell.clear();
    }
  }

  /// World position -> clamped (row, col) cell coordinates.
  fn cell_at(&self, pos: Vec2) -> (usize, usize) {
    let row = ((pos.y / self.cell_size) as usize).min(self.rows - 1);
    let col = ((pos.x / self.cell_size) as usize).min(self.cols - 1);
    (row, col)
  }

  fn cell_index(&self, row: usize, col: usize) -> usize {
    row * self.cols + col
  }

  pub fn insert(&mut self, em: &EntityManager, e: Entity) {
    let Some(&Position(pos)) = em.positions.get(e) else {
      return;
    };
    let (row, col) = self.cell_at(pos);
    let i = self.cell_index(row, col);
    self.cells[i].push(e);
  }

  pub fn near_entities(&mut self, pos: Vec2) -> &Vec<Entity> {
    //clear the near_list, its only valid for one call
    self.near_list.clear();

    let (row, col) = self.cell_at(pos);

    let rows = row.saturating_sub(1)..=(row + 1).min(self.rows - 1);
    let cols = col.saturating_sub(1)..=(col + 1).min(self.cols - 1);

    for r in rows {
      for c in cols.clone() {
        let i = self.cell_index(r, c);
        self.near_list.extend_from_slice(&self.cells[i]);
      }
    }

    &self.near_list
  }

  pub fn find_nearest(
    &mut self,
    em: &EntityManager,
    e: Entity,
    search_radius: f32,
  ) -> Option<Entity> {
    let search_cells: usize = (search_radius / self.cell_size).ceil() as usize;

    self.near_list.clear();

    let &Position(pos) = em.positions.get(e)?;

    let (row, col) = self.cell_at(pos);

    for i in 0..search_cells {
      let mut found = false;
      let rows = row.saturating_sub(i)..=(row + i).min(self.rows - 1);
      let cols = col.saturating_sub(i)..=(col + i).min(self.cols - 1);
      for r in rows {
        for c in cols.clone() {
          let cell_index = self.cell_index(r, c);
          if self.cells[cell_index].is_empty() || self.cells[cell_index].contains(&e) {
            continue;
          }
          self.near_list.extend_from_slice(&self.cells[cell_index]);
          found = true;
        }
      }
      if found {
        break;
      }
    }

    if self.near_list.is_empty() {
      return None;
    };

    let mut closest = Entity::MAX;
    let mut closest_distance = f32::MAX;
    for e_c in self.near_list.iter() {
      if *e_c == e {
        continue;
      }
      let &Position(pos_n) = em.positions.get(*e_c)?;
      let dist = pos.distance(pos_n);
      if dist < closest_distance {
        closest_distance = dist;
        closest = *e_c;
      }
    }
    if closest == Entity::MAX {
      return None;
    }
    Some(closest)
  }
}
