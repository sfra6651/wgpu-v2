use glam::Vec2;

use crate::{
  entity::{Entity, EntityManager, Position},
  world::{self},
};

#[derive(Copy, Clone)]
pub struct CellValue {
  pub e: Entity,
  pub pos: Position,
  pub rad: f32,
}

pub struct SpatialGrid {
  cell_size: f32,
  rows: usize,
  cols: usize,
  cells: Vec<Vec<CellValue>>,
  //needs to be cleared every call of near entities. so we dont have to create a vec in a hot loop
}

impl SpatialGrid {
  pub fn new(cell_size: f32) -> Self {
    let rows = (world::HEIGHT / cell_size).ceil() as usize;
    let cols = (world::WIDTH / cell_size).ceil() as usize;
    let mut cells: Vec<Vec<CellValue>> = Vec::new();
    for _ in 0..rows * cols {
      cells.push(Vec::new());
    }

    Self {
      cell_size,
      rows,
      cols,
      cells,
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
    let Some(&pos) = em.positions.get(e) else {
      return;
    };
    let Some(rad) = em.hit_box_rads.get(e) else {
      return;
    };
    let (row, col) = self.cell_at(pos.0);
    let i = self.cell_index(row, col);
    self.cells[i].push(CellValue { e, pos, rad: rad.0 });
  }

  pub fn near_entities(&self, pos: Vec2) -> impl Iterator<Item = CellValue> + '_ {
    let (row, col) = self.cell_at(pos);
    let r0 = row.saturating_sub(1);
    let r1 = (row + 1).min(self.rows - 1);
    let c0 = col.saturating_sub(1);
    let c1 = (col + 1).min(self.cols - 1);

    (r0..=r1)
      .flat_map(move |r| {
        // cells in one row are contiguous, so take the whole span as one slice
        let base = r * self.cols;
        &self.cells[base + c0..=base + c1]
      })
      .flat_map(|cell| cell.iter().copied())
  }

  pub fn find_nearest(
    &self,
    em: &EntityManager,
    e: Entity,
    search_radius: f32,
  ) -> Option<CellValue> {
    let search_cells = (search_radius / self.cell_size).ceil() as usize;
    let &Position(pos) = em.positions.get(e)?;
    let (row, col) = self.cell_at(pos);

    for i in 0..search_cells {
      let mut closest = None;
      let mut closest_dist = f32::MAX;

      for r in row.saturating_sub(i)..=(row + i).min(self.rows - 1) {
        for c in col.saturating_sub(i)..=(col + i).min(self.cols - 1) {
          for &cell_val in &self.cells[self.cell_index(r, c)] {
            if cell_val.e == e {
              continue;
            }
            let dist = pos.distance_squared(cell_val.pos.0);
            if dist < closest_dist {
              closest_dist = dist;
              closest = Some(cell_val);
            }
          }
        }
      }

      if closest.is_some() {
        return closest;
      }
    }
    None
  }
}
