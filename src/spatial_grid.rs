use glam::Vec2;

use crate::{
  entity::Entity,
  world::{self, World},
};

pub struct SpatialGrid {
  cell_size: f32,
  rows: usize,
  cols: usize,
  cells: Vec<Vec<usize>>,
  //needs to be cleared every iteration so we dont have to create a vec in a hot loop
  near_list: Vec<usize>,
}

impl SpatialGrid {
  pub fn new(cell_size: f32) -> Self {
    let rows = (world::HEIGHT / cell_size).ceil() as usize;
    let cols = (world::WIDTH / cell_size).ceil() as usize;
    let mut cells: Vec<Vec<usize>> = Vec::new();
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

  pub fn insert(&mut self, e: &Entity, e_idx: usize) {
    let (row, col) = self.cell_at(e.pos);
    let i = self.cell_index(row, col);
    self.cells[i].push(e_idx);
  }

  pub fn near_entities(&mut self, pos: Vec2) -> &Vec<usize> {
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
}
