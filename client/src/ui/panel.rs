use handler::texture::{Texture, TextureData};
use glium::index::PrimitiveType;
use glium::draw_parameters::DrawParameters;
use glium::{Surface, Frame, Program};
use glium::vertex::VertexBuffer;
use vecmath::{Matrix4,mat4_id};
use render::DisplayData;
use ui::UIElement;
use ui::Vertex2D;
use std::rc::Rc;
use glium_text::{self,TextDisplay,FontTexture};
use glium::index::IndexBuffer;

pub struct Panel {
	shape: VertexBuffer<Vertex2D>,
	indices: IndexBuffer<u8>,
	texture: Rc<TextureData>,
	program: Program,
	text_position: Matrix4<f32>,
	text: Option<TextDisplay<Rc<FontTexture>>>,
}

#[inline]
fn get_dimension(val: u32, total: u32) -> f32 {
	(val as f32) / (total as f32) * 2.0 - 1.0
}

impl Panel {
	pub fn new(display: &DisplayData) -> Panel {

		// TODO: Store this in a 2D drawing state
		let vertex_shader_src = include_str!("../../assets/shaders/ui.vert");
		let fragment_shader_src = include_str!("../../assets/shaders/ui.frag");
		let program = Program::from_source(
			&display.display,
			vertex_shader_src,
			fragment_shader_src,
			None
		).unwrap();// TODO: Deal with unwrap

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
		]).unwrap();// TODO: Deal with unwrap

		let mut result = Panel {
			shape: VertexBuffer::new(&display.display, &[]).unwrap(), // TODO: Deal with unwrap
			indices: indices,
			texture: Texture::get(Texture::PanelBackground),
			text_position: mat4_id(),
			text: None,
			program: program,
		};
		let dimensions = display.display
			.get_window().unwrap()// TODO: Deal with unwrap
			.get_inner_size_pixels().unwrap();// TODO: Deal with unwrap
		result.window_size_changed(display, dimensions.0, dimensions.1);
		result
	}
}

impl UIElement for Panel {
	fn draw(&self, target: &mut Frame, display: &DisplayData) {

		// TODO: Draw a nice border around the panel
		// TODO: Draw a nice background
		// see: http://i1057.photobucket.com/albums/t398/Duvvel/senatry/67b655d5.jpg
		target.draw(
			&self.shape,
			&self.indices,
			&self.program,
			&uniform! { tex: self.texture.get_texture2d().unwrap() }, // TODO: Deal with unwrap
			&DrawParameters::default()
		).unwrap();// TODO: Deal with unwrap

		if let Some(ref text) = self.text {
			glium_text::draw(&text, &display.text_system, target, self.text_position, (1.0, 0.0, 0.0, 1.0));
		}
	}

	fn window_size_changed(&mut self, display: &DisplayData, width: u32, height: u32) {
		// TODO: Move this logic to a handler in UI, seeing as all UI elements are a rectangle of some sort
		// TODO: including an anchor position:
		//		 - top, vertical_center, bottom
		//		 - left, horizontal_center, right

		let desired_x = 50;
		let desired_y = 50;
		let desired_width = width - 100;
		let desired_height = 200;

		let outer_left = get_dimension(desired_x, width);
		let outer_top = get_dimension(desired_y, height);
		let outer_right = get_dimension(desired_x + desired_width, width);
		let outer_bottom = get_dimension(desired_y + desired_height, height);

		let inner_left = get_dimension(desired_x + 13, width);
		let inner_top = get_dimension(desired_y + 13, height);
		let inner_right = get_dimension(desired_x + desired_width - 13, width);
		let inner_bottom = get_dimension(desired_y + desired_height - 13, height);

		self.shape = VertexBuffer::new(&display.display, &[
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
		]).unwrap();// TODO: Deal with unwrap

		self.text = Some(glium_text::TextDisplay::new(&display.text_system, display.font_texture.clone(), "Hello world!"));
		let left = get_dimension(60, width);
		let top = get_dimension(135, height);
		self.text_position = [
			[0.05, 0.00, 0.00, 0.00],
			[0.00, 0.05, 0.00, 0.00],
			[0.00, 0.00, 0.05, 0.00],
			[left, top, 0.00, 1.00]
		];
	}
}
