use handler::texture::Texture;
use ui::wrapper::UIWrapper;
use vecmath::Vector2;

pub struct Dimension {
	pub x: u32,
	pub y: u32,
	pub width: u32,
	pub height: u32,
}

impl Dimension {
	pub fn from_uielement(elem: &UIWrapper) -> Dimension {
		Dimension {
			x: elem.position.0,
			y: elem.position.1,
			width: elem.size.0,
			height: elem.size.1
		}
	}
}

pub enum RenderCommand {
	DrawBackground(Texture),
	DrawText { text: String, x: u32, y: u32 },
}

pub enum EventResult {
	Unhandled,
	Handled,
	SelectNext
}

// TODO: Move this to general render data
#[derive(Copy, Clone)]
pub struct Vertex2D {
	pub position: Vector2<f32>,
	pub tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex2D, position, tex_coords);

pub enum UIView {
	Login,
	None
}
