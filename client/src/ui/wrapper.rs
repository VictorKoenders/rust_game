use glium::{ IndexBuffer, VertexBuffer, Surface, Frame, Program };
use ui::utils::{ Dimension, EventResult, RenderCommand, Vertex2D };
use glium::draw_parameters::DrawParameters;
use glium::index::PrimitiveType;
use ui::render_state::UIRender;
use handler::texture::Texture;
use ui::traits::UIElement;
use glium::glutin::Event;
use render::DisplayData;
use glium_text;

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
	pub fn new<T>(display: &DisplayData, inner: T, parent_dimensions: &Dimension) -> UIWrapper
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

		let position = inner.get_initial_position(parent_dimensions);

		let mut wrapper = UIWrapper {
			element: Box::new(inner),
			children: Vec::new(),
			position: position,
			size: (0, 0),
			program: program,
			indices: indices,
			shape: None
		};
		wrapper.resize(display, parent_dimensions);
		wrapper
	}
	pub fn resize(&mut self, display: &DisplayData, parent_dimensions: &Dimension) {
		let desired_size = self.element.get_desired_size(parent_dimensions);
		self.size = desired_size;

		let (width, height) = display.get_screen_dimensions();
		let desired_width = desired_size.0;
		let desired_height = desired_size.1;
		let x = parent_dimensions.x + self.position.0;
		let y = height - (parent_dimensions.y + self.position.1) - desired_height;

		println!("Drawing background from {}/{} to {}/{}", x, y, desired_width, desired_height);
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

	pub fn draw(&mut self, target: &mut Frame, display: &DisplayData, parent_x: u32, parent_y: u32){
		let mut render = UIRender::new(self.size.0, self.size.1);
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
					// TODO: Cache this data as this wil mostly be the same between frames
					let screen_size = display.get_screen_dimensions();
					let text = glium_text::TextDisplay::new(&display.text_system, display.font_texture.clone(), &text);

					let x = parent_x + self.position.0 + x;
					let y = screen_size.1 - (parent_y + self.position.1 + y) - 20;

					let left = get_dimension(x, screen_size.0);
					let top = get_dimension(y, screen_size.1);
					let text_position = [
						[0.03, 0.00, 0.00, 0.00],
						[0.00, 0.03, 0.00, 0.00],
						[0.00, 0.00, 0.03, 0.00],
						[left, top, 0.00, 1.00]
					];

					glium_text::draw(&text, &display.text_system, target, text_position, (1.0, 0.0, 0.0, 1.0));
				}
			}
		}

		for child in &mut self.children {
			child.draw(target, display, parent_x + self.position.0, parent_y + self.position.1);
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		self.element.update(delta_time);
		for child in &mut self.children {
			child.update(delta_time);
		}
	}

	pub fn click(&mut self, x: u32, y: u32) -> EventResult {
		if x < self.position.0 || x > self.position.0 + self.size.0 { return EventResult::Unhandled; }
		if y < self.position.1 || y > self.position.1 + self.size.1 { return EventResult::Unhandled; }

		let result = self.element.click();
		match result {
			EventResult::Unhandled => {},
			x => return x
		};

		for child in &mut self.children {
			match child.click(x - self.position.0, y - self.position.1) {
				EventResult::Unhandled => continue,
				x => return x
			}
		}
		EventResult::Unhandled
	}

	pub fn handle_event(&mut self, event: &Event) -> EventResult {
		let result = self.element.handle_event(event);

		match result {
			EventResult::Unhandled => {
				for i in 0..self.children.len() {
					match self.children[i].handle_event(event) {
						EventResult::Unhandled => continue,
						EventResult::Handled => return EventResult::Handled,
						EventResult::SelectNext => {
							if i < self.children.len() - 1 {
								self.children[i+1].element.set_focus();
							}
							return EventResult::Handled;
						}
					}
				}
				EventResult::Unhandled
			},
			x => x
		}
	}
}

#[inline]
fn get_dimension(val: u32, total: u32) -> f32 {
	(val as f32) / (total as f32) * 2.0 - 1.0
}
