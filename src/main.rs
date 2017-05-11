extern crate cgmath;
mod particle;
mod sph;
mod sph_renderer;
mod constants;
mod grid;
use sph_renderer::SPHRenderer;
fn main() {
    SPHRenderer::new().run();
}
