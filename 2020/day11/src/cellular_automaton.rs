use ahash::AHasher;
use array2d::Array2D;
use itertools::*;
use std::hash::Hasher;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug)]
pub struct CellularAutomaton {
    field: Array2D<CellState>,
}

impl CellularAutomaton {
    pub fn new(rows: &[Vec<CellState>]) -> Self {
        Self {
            field: Array2D::from_rows(rows),
        }
    }

    fn width(&self) -> usize {
        self.field.num_columns()
    }

    fn height(&self) -> usize {
        self.field.num_rows()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<CellState> {
        (x.saturating_sub(1)..=x + 1)
            .cartesian_product(y.saturating_sub(1)..=y + 1)
            .filter(|xy| *xy != (x, y))
            .flat_map(|(x, y)| self.field.get(y, x))
            .cloned()
            .collect()
    }

    fn occupied_neighbors(&self, x: usize, y: usize) -> usize {
        count_occupied_cells(self.get_neighbors(x, y).iter())
    }

    fn new_state_for_old(&self, x: usize, y: usize) -> CellState {
        match self.field[(y, x)] {
            CellState::Empty => {
                if self.occupied_neighbors(x, y) == 0 {
                    CellState::Occupied
                } else {
                    CellState::Empty
                }
            }
            CellState::Occupied => {
                if self.occupied_neighbors(x, y) >= 4 {
                    CellState::Empty
                } else {
                    CellState::Occupied
                }
            }
            CellState::Floor => CellState::Floor,
        }
    }

    fn new_state_for_new(&self, x: usize, y: usize) -> CellState {
        match self.field[(y, x)] {
            CellState::Empty => {
                if self.occupied_neighbors_ex(x, y) == 0 {
                    CellState::Occupied
                } else {
                    CellState::Empty
                }
            }
            CellState::Occupied => {
                if self.occupied_neighbors_ex(x, y) >= 5 {
                    CellState::Empty
                } else {
                    CellState::Occupied
                }
            }
            CellState::Floor => CellState::Floor,
        }
    }

    pub fn tick(&mut self) {
        self.field = Array2D::from_iter_row_major(
            (0..self.height())
                .cartesian_product(0..self.width())
                .map(|(y, x)| self.new_state_for_old(x, y)),
            self.height(),
            self.width(),
        );
    }

    pub fn tick2(&mut self) {
        self.field = Array2D::from_iter_row_major(
            (0..self.height())
                .cartesian_product(0..self.width())
                .map(|(y, x)| self.new_state_for_new(x, y)),
            self.height(),
            self.width(),
        );
    }

    pub fn get_state_hash(&self) -> u64 {
        let bytes: Vec<_> = self
            .field
            .elements_column_major_iter()
            .map(|el| match *el {
                CellState::Empty => 1u8,
                CellState::Occupied => 2u8,
                CellState::Floor => 0u8,
            })
            .collect();
        let mut hasher = AHasher::default();
        hasher.write(&bytes);
        hasher.finish()
    }

    pub fn occupied_count(&self) -> usize {
        count_occupied_cells(self.field.elements_column_major_iter())
    }

    fn occupied_neighbors_ex(&self, x: usize, y: usize) -> usize {
        count_occupied_cells(self.get_neighbors_ex(x, y).iter())
    }

    fn get_neighbors_ex(&self, x: usize, y: usize) -> Vec<CellState> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|xy| *xy != (0, 0))
            .map(|(dx, dy)| self.find_first_seat((x, y), dx, dy))
            .collect()
    }

    fn find_first_seat(&self, from: (usize, usize), dx: i32, dy: i32) -> CellState {
        let (x, y) = from;
        let (mut x, mut y) = (x as i32, y as i32);
        loop {
            x += dx;
            y += dy;
            if x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32 {
                return CellState::Floor;
            }
            let cell = &self.field[(y as usize, x as usize)];
            if cell != &CellState::Floor {
                return *cell;
            }
        }
    }
}

fn count_occupied_cells<'a>(iter: impl Iterator<Item = &'a CellState>) -> usize {
    iter.filter(|s| **s == CellState::Occupied).count()
}
