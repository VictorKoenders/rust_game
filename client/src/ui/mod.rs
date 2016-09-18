mod panel;
mod textbox;

use glium_text;
use glium::draw_parameters::DrawParameters;
use glium::{Surface, Frame, Program};
use glium::index::PrimitiveType;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use handler::texture::Texture;
use render::DisplayData;
use vecmath::Vector2;

pub trait UIElement {
	fn get_initial_position(&self, parent_width: u32, parent_height: u32) -> (u32, u32);
	fn get_desired_size(&self, parent_width: u32, parent_height: u32) -> (u32, u32);
	fn update(&mut self, delta_time: f32, children: &Vec<UIWrapper>);
	fn draw(&self, render: &mut UIRender);
}

pub enum RenderCommand {
	DrawBackground(Texture),
	DrawText { text: String, x: u32, y: u32 },
}

pub struct UIRender {
	pub commands: Vec<RenderCommand>
}

impl UIRender {
	pub fn new() -> UIRender {
		UIRender {
			commands: Vec::new()
		}
	}

	pub fn set_background(&mut self, texture: Texture) {
		self.commands.push(RenderCommand::DrawBackground(texture));
	}

	pub fn draw_text_at<T: ToString>(&mut self, text: T, x: u32, y: u32) {
		self.commands.push(RenderCommand::DrawText {
			text: text.to_string(),
			x: x,
			y: y
		});
	}
}

pub struct UIWrapper {
	pub element: Box<UIElement>,
	pub children: Vec<UIWrapper>,
	pub position: (u32, u32),
	pub size: (u32, u32),

	pub program: Program,
	pub indices: IndexBuffer<u8>,
	pub shape: Option<VertexBuffer<Vertex2D>>,
}

impl UIWrapper {
	pub fn new<T>(display: &DisplayData, inner: T, parent_width: u32, parent_height: u32) -> UIWrapper
		where T : UIElement + 'static {

		let indices = IndexBuffer::new(&display.display, PrimitiveType::TrianglesList, &[
			0, 1, 4, 1, 4, 5, // top-left
			1, 2, 5, 2, 5, 6, // top
			2, 3, 6, 3, 6, 7, // top-right

			4, 5, 8, 5, 8, 9, // left
			5, 6, 9, 6, 9, 10, // middle
			6, 7, 10, 7, 10, 11, // right

			8, 9, 12, 9, 12, 13, // bottom-left
			9, 10, 13, 10, 13, 14, // bottom
			10, 11, 14, 11, 14, 15, // bottom-right
		]).unwrap(); // TODO: Deal with unwrap

		let vertex_shader_src = include_str!("../../assets/shaders/ui.vert");
		let fragment_shader_src = include_str!("../../assets/shaders/ui.frag");
		let program = Program::from_source(
			&display.display,
			vertex_shader_src,
			fragment_shader_src,
			None
		).unwrap(); // TODO: Deal with unwrap

		let position = inner.get_initial_position(parent_width, parent_height);

		let mut wrapper = UIWrapper {
			element: Box::new(inner),
			children: Vec::new(),
			position: position,
			size: (0, 0),
			program: program,
			indices: indices,
			shape: None
		};
		wrapper.resize(display, parent_width, parent_height);
		wrapper
	}
	pub fn resize(&mut self, display: &DisplayData, parent_width: u32, parent_height: u32) {
		let desired_size = self.element.get_desired_size(parent_width, parent_height);
		self.size = desired_size;

		let (width, height) = display.get_screen_dimensions();
		let desired_width = desired_size.0;
		let desired_height = desired_size.1;
		let x = self.position.0;
		let y = parent_height - self.position.1 - desired_height;

		const SPACING: u32 = 13;

		let outer_left = get_dimension(x, width);
		let outer_top = get_dimension(y, height);
		let outer_right = get_dimension(x + desired_width, width);
		let outer_bottom = get_dimension(y + desired_height, height);

		let inner_left = get_dimension(x + SPACING, width);
		let inner_top = get_dimension(y + SPACING, height);
		let inner_right = get_dimension(x + desired_width - SPACING, width);
		let inner_bottom = get_dimension(y + desired_height - SPACING, height);

		self.shape = Some(VertexBuffer::new(&display.display, &[
			Vertex2D { position: [outer_left, outer_top], tex_coords: [0.0, 0.0] },
			Vertex2D { position: [inner_left, outer_top], tex_coords: [0.0, 0.1] },
			Vertex2D { position: [inner_right, outer_top], tex_coords: [0.0, 0.9] },
			Vertex2D { position: [outer_right, outer_top], tex_coords: [0.0, 1.0] },
			Vertex2D { position: [outer_left, inner_top], tex_coords: [0.1, 0.0] },
			Vertex2D { position: [inner_left, inner_top], tex_coords: [0.1, 0.1] },
			Vertex2D { position: [inner_right, inner_top], tex_coords: [0.1, 0.9] },
			Vertex2D { position: [outer_right, inner_top], tex_coords: [0.1, 1.0] },
			Vertex2D { position: [outer_left, inner_bottom], tex_coords: [0.9, 0.0] },
			Vertex2D { position: [inner_left, inner_bottom], tex_coords: [0.9, 0.1] },
			Vertex2D { position: [inner_right, inner_bottom], tex_coords: [0.9, 0.9] },
			Vertex2D { position: [outer_right, inner_bottom], tex_coords: [0.9, 1.0] },
			Vertex2D { position: [outer_left, outer_bottom], tex_coords: [1.0, 0.0] },
			Vertex2D { position: [inner_left, outer_bottom], tex_coords: [1.0, 0.1] },
			Vertex2D { position: [inner_right, outer_bottom], tex_coords: [1.0, 0.9] },
			Vertex2D { position: [outer_right, outer_bottom], tex_coords: [1.0, 1.0] },
		]).unwrap()); // TODO: Deal with unwrap
	}

	pub fn draw(&mut self, target: &mut Frame, display: &DisplayData, _ /*parent_width*/: u32, parent_height: u32){
		let mut render = UIRender::new();
		self.element.draw(&mut render);

		for command in render.commands {
			match command {
				RenderCommand::DrawBackground(texture) => {
					if let Some(ref shape) = self.shape {
						target.draw(
							shape,
							&self.indices,
							&self.program,
							&uniform! { tex: Texture::get(texture).get_texture2d().unwrap() }, // TODO: Deal with unwrap
							&DrawParameters::default()
						).unwrap(); // TODO: Deal with unwrap
					}
				},
				RenderCommand::DrawText { text, x, y } => {
					// TODO: Cache text surfaces
					let screen_size = display.get_screen_dimensions();
					let text = glium_text::TextDisplay::new(&display.text_system, display.font_texture.clone(), &text);

					let x = self.position.0 + x;
					let y = parent_height - self.position.1 -  y;

					let left = get_dimension(x, screen_size.0);
					let top = get_dimension(y, screen_size.1);
					let text_position = [
						[0.05, 0.00, 0.00, 0.00],
						[0.00, 0.05, 0.00, 0.00],
						[0.00, 0.00, 0.05, 0.00],
						[left, top, 0.00, 1.00]
					];

					glium_text::draw(&text, &display.text_system, target, text_position, (1.0, 0.0, 0.0, 1.0));
				}
			}
		}
	}
}

#[inline]
fn get_dimension(val: u32, total: u32) -> f32 {
	(val as f32) / (total as f32) * 2.0 - 1.0
}

// TODO: Move this to general render data
#[derive(Copy, Clone)]
pub struct Vertex2D {
	position: Vector2<f32>,
	tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex2D, position, tex_coords);

pub struct UI {
	pub elements: Vec<UIWrapper>
}

impl UI {
	pub fn new() -> UI {
		UI {
			elements: Vec::new()
		}
	}

	pub fn load(&mut self, display: &DisplayData, view: UIView) {
		if let UIView::Login = view {
			let size = display.get_screen_dimensions();
			self.elements.push(UIWrapper::new(display, panel::Panel::new(), size.0, size.1));
		}
	}

	pub fn render(&mut self, target: &mut Frame, display: &DisplayData) {
		let screen_size = display.get_screen_dimensions();
		for element in &mut self.elements {
			element.draw(target, display, screen_size.0, screen_size.1);
		}
	}

	pub fn resize(&mut self, display: &DisplayData, width: u32, height: u32){
		for element in &mut self.elements {
			element.resize(display, width, height);
		}
	}
}

pub enum UIView {
	Login,
	None
}
