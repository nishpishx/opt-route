use crate::state::SeaState;

#[derive(Clone)]
pub struct Cell {
    pub sea: SeaState,
    pub is_land: bool,
}

pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn idx(&self, r: usize, c: usize) -> usize {
        r * self.cols + c
    }

    pub fn get(&self, r: usize, c: usize) -> &Cell {
        &self.cells[self.idx(r, c)]
    }

    pub fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.rows && (c as usize) < self.cols
    }

    pub fn neighbors(&self, r: usize, c: usize) -> Vec<(usize, usize, f64)> {
        let dirs = [
            (-1, 0), (-1, 1), (0, 1), (1, 1),
            (1, 0), (1, -1), (0, -1), (-1, -1),
        ];

        let mut out = vec![];

        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if self.in_bounds(nr, nc) {
                let dist = if dr == 0 || dc == 0 { 1.0 } else { std::f64::consts::SQRT_2 };
                out.push((nr as usize, nc as usize, dist));
            }
        }

        out
    }
}