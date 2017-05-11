use cgmath::*;
use particle::Particle;
use constants;
use std::collections::HashMap;
const OFFSETS: [(i64, i64); 9] = [(0, 0), (0, 1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (1, -1),
                                  (1, 0), (1, 1)];
type Cell = Vec<usize>;

pub struct Grid {
    cells: HashMap<(i64, i64), Cell>,
}

impl Grid {
    pub fn new() -> Self {
        Grid { cells: HashMap::new() }
    }

    pub fn neighbor_cells(&self, position: &Point2<f64>) -> Vec<&Cell> {
        let mut res = Vec::new();
        let cell = position / constants::KERNEL_RANGE;
        for offset in &OFFSETS {
            if let Some(cell) = self.cells
                .get(&(cell.x as i64 + offset.0, cell.y as i64 + offset.1)) {
                res.push(cell);
            }
        }
        res
    }

    pub fn update(&mut self, particles: &Vec<Particle>) {
        self.cells.clear();
        for i in 0..particles.len() {
            let ref p = particles[i];
            let cell = p.position / constants::KERNEL_RANGE;
            self.cells.entry((cell.x as i64, cell.y as i64)).or_insert(Vec::new()).push(i);
        }
    }
}