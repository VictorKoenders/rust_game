mod panel;

use glium::Frame;
use vecmath::Vector2;
use render::DisplayData;
use glium::vertex::VertexBuffer;

// TODO: implement the following types:
pub use ui::panel::*; // panel
// label
// input (text + password)
// button
// image

// TODO: click/hover events
// TODO: Nested elements
pub trait UIElement {
	fn draw(&self, &mut Frame, display: &DisplayData);
	fn window_size_changed(&mut self, display: &DisplayData, width: u32, height: u32);
}

pub struct UIWrapper {
	pub element: Box<UIElement>,
	shape: VertexBuffer<Vertex2D>,
}

// TODO: Move this to general render data
#[derive(Copy, Clone)]
pub struct Vertex2D {
	position: Vector2<f32>,
	tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex2D, position, tex_coords);

pub struct UI {
	pub elements: Vec<Box<UIElement>>,
}

impl UI {
	pub fn new(display: &DisplayData) -> UI {
		let mut elements: Vec<Box<UIElement>> = Vec::new();
		elements.push(Box::new(Panel::new(display)));

		UI {
			elements: elements,
		}
	}

	// TODO: We only need the DisplayData for the VertexBuffer::new() call
	// Since the display data is always the same, make a wrapper class for this
	// So we don't have to constantly pass this along
	pub fn resize(&mut self, display: &DisplayData, width: u32, height: u32) {
		for element in &mut self.elements {
			element.window_size_changed(display, width, height);
		}
	}

	pub fn render(&self, target: &mut Frame, display: &DisplayData) {
		for element in &self.elements {
			element.draw(target, display);
		}
	}
}
