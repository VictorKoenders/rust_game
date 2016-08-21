mod panel;

use glium::Frame;
use vecmath::Vector2;

pub use ui::panel::*;

pub trait UIElement {
	fn draw(&self, &mut Frame);
}

#[derive(Copy, Clone)]
pub struct Vertex2D {
	position: Vector2<f32>,
	tex_coords: Vector2<f32>,
}

implement_vertex!(Vertex2D, position, tex_coords);