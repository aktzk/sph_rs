use cgmath::*;

pub struct Particle {
    pub position: Point2<f64>,
    pub velocity: Vector2<f64>,
    pub force: Vector2<f64>,
    pub mass: f64,
    pub density: f64,
    pub pressure: f64,
}

impl Particle {
    pub fn new(position: Point2<f64>, mass: f64) -> Self {
        Particle {
            position: position,
            velocity: Vector2::new(0.0, 0.0),
            force: Vector2::new(0.0, 0.0),
            mass: mass,
            density: 0.0,
            pressure: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64, wall_left: f64) {
        self.velocity += self.force / self.density * dt;
        self.position += self.velocity * dt;
        self.force = Vector2::new(0.0, 0.0);
        self.reflect(wall_left);
    }

    fn reflect(&mut self, wall_left:f64) {
        use constants::*;
        if self.position.x < wall_left {
            self.damp_reflect_horizontal(wall_left);
        }
        if self.position.x > SPH_SIZE.0 {
            self.damp_reflect_horizontal(SPH_SIZE.0);
        }
        if self.position.y < 0.0 {
            self.damp_reflect_vertical(0.0);
        }
        if self.position.y > SPH_SIZE.1 {
            self.damp_reflect_vertical(SPH_SIZE.1);
        }
    }

    fn damp_reflect_horizontal(&mut self, barrier: f64) {
        let damp = 0.75;
        if self.velocity.x == 0.0 {
            return;
        }
        let bounce = (self.position.x - barrier) / self.velocity.x;
        self.position = self.position  +  self.velocity * (damp - 1.0) * bounce;
        self.position.x = 2.0 * barrier - self.position.x;
        self.velocity.x = -1.0 * self.velocity.x;
        //damp
        self.velocity = self.velocity * damp;
    }
    fn damp_reflect_vertical(&mut self, barrier: f64) {
        let damp = 0.75;
        if self.velocity.y == 0.0 {
            return;
        }
        let bounce = (self.position.y - barrier) / self.velocity.y;
        self.position =self.position + self.velocity * (damp - 1.0) * bounce;
        self.position.y = 2.0 * barrier - self.position.y;
        self.velocity.y = -1.0 * self.velocity.y;
        //damp
        self.velocity = self.velocity * damp;
    }
}