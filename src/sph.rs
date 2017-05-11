use particle::Particle;
use cgmath::*;
use constants;
use grid;
pub struct SPH {
    particles: Vec<Particle>,
    neighborhoods: Vec<Vec<usize>>,
    grid: grid::Grid,
    pub wall_left: f64,
}

impl SPH {
    pub fn new() -> Self {
        let mut particles = Vec::new();
        let left = (constants::SPH_SIZE.0 - 1.0) * 0.5;
        let top = 0.1;
        let mut grid = grid::Grid::new();
        for i in 0..constants::ONE_SIDE_PARTICLE_COUNT {
            for j in 0..constants::ONE_SIDE_PARTICLE_COUNT {
                let pos = Point2::new(left, top) + Vector2::new(i as f64 * constants::PARTICLE_SPACING, j as f64 * constants::PARTICLE_SPACING);
                let p = Particle::new(pos, constants::MASS);
                particles.push(p);
            }
        }
        grid.update(&particles);
        SPH {
            particles: particles,
            neighborhoods: Vec::new(),
            grid: grid,
            wall_left: 0.0,
        }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn update(&mut self, dt: f64) {
        self.find_neighbors();
        self.calculate_density();
        self.calculate_pressure();
        self.calculate_force();
        self.move_particles(dt);

        self.grid.update(&self.particles);
    }

    fn find_neighbors(&mut self) {
        self.neighborhoods.clear();
        let max_dist_2 = constants::KERNEL_RANGE * constants::KERNEL_RANGE;
        for p in &self.particles {
            let mut neighbors = Vec::new();
            let neighbor_cells = self.grid.neighbor_cells(&p.position);
            for cell in neighbor_cells {
                for index in cell {
                    let n = p.position - self.particles[*index].position;
                    let dist_2 = n.magnitude2();
                    if dist_2 <= max_dist_2 {
                        neighbors.push(*index);
                    }
                }
            }
            self.neighborhoods.push(neighbors);
        }
    }

    fn calculate_density(&mut self) {
        for i in 0..self.particles.len() {
            let ref neighbors = self.neighborhoods[i];
            let mut density_sum = 0.0;
            for n in 0..neighbors.len() {
                let j = neighbors[n];
                let x = self.particles[i].position - self.particles[j].position;
                density_sum += self.particles[j].mass * kernel(&x, constants::KERNEL_RANGE);
            }
            self.particles[i].density = density_sum;
        }
    }

    fn calculate_pressure(&mut self) {
        for p in &mut self.particles {
            p.pressure = (constants::STIFFNESS * (p.density - constants::REFERENCE_DENSITY)).max(0.0);
        }
    }

    fn calculate_force(&mut self) {
        for i in 0..self.particles.len() {
            let mut f_pressure = Vector2::new(0.0, 0.0);
            let mut f_viscosity = Vector2::new(0.0, 0.0);
            let ref neighbors = self.neighborhoods[i];
            for j in neighbors {
                let j = *j;
                let x = self.particles[i].position - self.particles[j].position;
                let inv_density = 1.0 / self.particles[j].density;
                f_pressure += self.particles[j].mass *
                              (self.particles[i].pressure + self.particles[j].pressure) * 0.5 * inv_density *
                              grad_kernel(&x, constants::KERNEL_RANGE);
                f_viscosity += self.particles[j].mass *
                                (self.particles[j].velocity - self.particles[i].velocity) * inv_density *
                                laplace_kernel(&x, constants::KERNEL_RANGE);
            }
            
            f_pressure *= -1.0;
            f_viscosity *= constants::VISCOSITY;
            let f_gravity = self.particles[i].density * Vector2::new(0.0, constants::GRAVITY);
  
            self.particles[i].force = f_pressure + f_viscosity + f_gravity;
        }
    }

    fn move_particles(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.update(dt, self.wall_left);
        }
    }

}

// 実装においては、x.magnitude() < h が保証されている
fn kernel(x: &Vector2<f64>, h: f64) -> f64 {
    use std::f64;
    let r2 = x.magnitude2();
    let h2 = h * h;
    if r2 < 0.0 {
        return 0.0;
    }

    315.0 * f64::consts::FRAC_1_PI / (64.0  * h.powi(9)) * (h2 - r2).powi(3)
}

fn grad_kernel(x: &Vector2<f64>, h:f64)-> Vector2<f64> {
    use std::f64;
    let r = x.magnitude();
    if r == 0.0 {
        return Vector2::new(0.0,0.0);
    }
    let t1 = -45.0 * f64::consts::FRAC_1_PI / h.powi(6);
    let t2 = x / r;
    let t3 = (h-r) * (h-r);
    t1 * t2 * t3
}

fn laplace_kernel(x: &Vector2<f64>, h: f64) -> f64 {
    use std::f64;
    let r = x.magnitude();
    45.0 * f64::consts::FRAC_1_PI / h.powi(6) * (h-r)
}