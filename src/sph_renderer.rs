extern crate piston_window;
use self::piston_window::*;
use sph::SPH;
use cgmath::*;
use constants::*;
pub struct SPHRenderer {
    window: PistonWindow,
    mouse: Mouse,
}

impl SPHRenderer {
    pub fn new() -> Self {
        let window: PistonWindow = WindowSettings::new("smoothed particle hydrodynamics",
                                                       [WINDOW_SIZE.0 as u32,
                                                        WINDOW_SIZE.1 as u32])
            .exit_on_esc(true)
            .vsync(true)
            .build()
            .unwrap();
        SPHRenderer {
            window: window,
            mouse: Mouse::new(),
        }
    }

    pub fn run(&mut self) {
        let mut sph = SPH::new();
        let mut update = false;
        let mut initial_mouse_pos = Point2::new(0.0, 0.0);
        let mut temp_wall_left = 0.0;

        while let Some(e) = self.window.next() {
            match e {
                Event::Update(_) => {
                    if update {
                        if self.mouse.is_down() {
                            initial_mouse_pos = self.mouse.position;
                            temp_wall_left = sph.wall_left;
                        }
                        if self.mouse.is_holding() {
                            let diff = self.mouse.position - initial_mouse_pos;
                            sph.wall_left = temp_wall_left + diff.x.min(2.0);
                        }
                        sph.update(0.0001);
                    }
                    self.mouse.update_hold();
                }
                Event::Render(_) => {
                    self.window.draw_2d(&e, |c, g| {
                        clear([1.0; 4], g);

                        // render all particles
                        let scale = KERNEL_RANGE * SPH_SCALE;
                        for p in sph.particles() {
                            rectangle([1.0, 0.0, 0.0, 1.0],
                                      [(p.position.x * SPH_SCALE - scale * 0.5),
                                       (p.position.y * SPH_SCALE - scale * 0.5),
                                       scale,
                                       scale],
                                      c.transform,
                                      g);
                        }
                    });
                }
                Event::Input(input) => {
                    match input {
                        Input::Release(Button::Keyboard(Key::P)) => {
                            update = !update;
                        }
                        Input::Release(Button::Mouse(MouseButton::Left)) => {
                            self.mouse.pressed = false;
                        }
                        Input::Press(Button::Mouse(MouseButton::Left)) => {
                            self.mouse.pressed = true;
                        }
                        Input::Move(Motion::MouseCursor(x, y)) => {
                            self.mouse.position = Point2::new(x / SPH_SCALE, y / SPH_SCALE);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

        }
    }
}

// // very very rough implementation
struct Mouse {
    pub position: Point2<f64>,
    pub pressed: bool,
    hold: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            position: Point2::new(0.0, 0.0),
            pressed: false,
            hold: false,
        }
    }
    pub fn update_hold(&mut self) {
        self.hold = self.pressed;
    }

    pub fn is_holding(&self) -> bool {
        self.pressed && self.hold
    }
    pub fn is_down(&self) -> bool {
        self.pressed && !self.hold
    }
}