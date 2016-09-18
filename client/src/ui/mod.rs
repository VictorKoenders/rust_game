mod panel;
mod textbox;

use glium_text;
use glium::draw_parameters::DrawParameters;
use glium::{Surface, Frame, Program};
use glium::index::PrimitiveType;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use handler::texture::Texture;
use glium::glutin::{ Event, ElementState };
use render::DisplayData;
use vecmath::Vector2;

pub trait UIElement {
	fn get_initial_position(&self, parent_width: u32, parent_height: u32) -> (u32, u32);
	fn get_desired_size(&self, parent_width: u32, parent_height: u32) -> (u32, u32);
	fn draw(&self, render: &mut UIRender);
	fn update(&mut self, delta_time: f32);
	fn handle_event(&mut self, ev: &Event) -> EventResult;
	fn click(&mut self) -> EventResult;
	fn set_focus(&mut self) -> bool;

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

pub struct UIRender {
	pub height: u32,
	pub width: u32,
	pub commands: Vec<RenderCommand>
}

impl UIRender {
	pub fn new(width: u32, height: u32) -> UIRender {
		UIRender {
			width: width,
			height: height,
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
	pub fn new<T>(display: &DisplayData, inner: T, parent_x: u32, parent_y: u32, parent_width: u32, parent_height: u32) -> UIWrapper
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
		wrapper.resize(display, parent_x, parent_y, parent_width, parent_height);
		wrapper
	}
	pub fn resize(&mut self, display: &DisplayData, parent_x: u32, parent_y: u32, parent_width: u32, parent_height: u32) {
		let desired_size = self.element.get_desired_size(parent_width, parent_height);
		self.size = desired_size;

		let (width, height) = display.get_screen_dimensions();
		let desired_width = desired_size.0;
		let desired_height = desired_size.1;
		let x = parent_x + self.position.0;
		let y = parent_y + self.position.1;

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

	pub fn draw(&mut self, target: &mut Frame, display: &DisplayData, _ /*parent_width*/: u32, _ /*parent_height*/: u32){
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

					let x = self.position.0 + x;
					let y = self.position.1 + y;

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

		for child in &mut self.children {
			child.draw(target, display, self.size.0, self.size.1);
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		self.element.update(delta_time);
		for child in &mut self.children {
			child.update(delta_time);
		}
	}

	pub fn click(&mut self, x: u32, y: u32) -> EventResult {
		println!("Click {}/{} is between {}/{} and {}/{}?", x, y,
				 self.position.0, self.position.1, self.position.0 + self.size.0, self.position.1 + self.size.1);

		if x < self.position.0 || x > self.position.0 + self.size.0 { return EventResult::Unhandled; }
		if y < self.position.1 || y > self.position.1 + self.size.1 { return EventResult::Unhandled; }

		println!("Click {}/{} is between {}/{} and {}/{}?", x, y,
				 self.position.0, self.position.1, self.position.0 + self.size.0, self.position.1 + self.size.1);
		let result = self.element.click();
		match result {
			EventResult::Unhandled => {},
			x => return x
		};

		for child in self.children.iter_mut() {
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

// TODO: Move this to general render data
#[derive(Copy, Clone)]
pub struct Vertex2D {
	position: Vector2<f32>,
	tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex2D, position, tex_coords);

pub struct UI {
	pub elements: Vec<UIWrapper>,

	mouse_x: u32,
	mouse_y: u32,
	screen_height: u32
}

impl UI {
	pub fn new() -> UI {
		UI {
			elements: Vec::new(),
			mouse_x: 0,
			mouse_y: 0,
			screen_height: 0
		}
	}

	pub fn load(&mut self, display: &DisplayData, view: UIView) {
		if let UIView::Login = view {
			let size = display.get_screen_dimensions();
			let mut panel = UIWrapper::new(display, panel::Panel::new(), 0, 0, size.0, size.1);

			let mut username_textbox = textbox::Textbox::new();
			let mut password_textbox = textbox::Textbox::new();

			username_textbox.has_focus = true;
			password_textbox.is_password = true;

			let mut username_textbox = UIWrapper::new(display, username_textbox, panel.position.0, panel.position.1, panel.size.0, panel.size.1);
			let mut password_textbox = UIWrapper::new(display, password_textbox, panel.position.0, panel.position.1, panel.size.0, panel.size.1);

			username_textbox.position = (50, 100);
			password_textbox.position = (50, 50);

			username_textbox.resize(display, panel.position.0, panel.position.1, panel.size.0, panel.size.1);
			password_textbox.resize(display, panel.position.0, panel.position.1, panel.size.0, panel.size.1);

			panel.children.push(username_textbox);
			panel.children.push(password_textbox);

			self.elements.push(panel);
		}
	}

	pub fn render(&mut self, target: &mut Frame, display: &DisplayData) {
		let screen_size = display.get_screen_dimensions();
		for element in &mut self.elements {
			element.draw(target, display, screen_size.0, screen_size.1);
		}
	}

	pub fn resize(&mut self, display: &DisplayData, width: u32, height: u32){
		self.screen_height = display.get_screen_dimensions().1;
		for element in &mut self.elements {
			element.resize(display, 0, 0, width, height);
		}
	}

	pub fn handle_event(&mut self,  event: &Event) -> bool {
		if let Event::MouseMoved(x, y) = *event {
			self.mouse_x = x as u32;
			self.mouse_y = y as u32;
		}

		for element in &mut self.elements {
			let result = element.handle_event(event);
			if let EventResult::Unhandled = result {
				continue;
			}
			return true;
		}

		if let Event::MouseInput(ElementState::Pressed, _) = *event {
			for element in &mut self.elements {
				println!("screen height: {}, mouse_y: {}", self.screen_height, self.mouse_y);
				let result = element.click(self.mouse_x, self.screen_height - self.mouse_y);
				if let EventResult::Unhandled = result {
					continue;
				}
				return true;
			}
		}

		false
	}
	pub fn update(&mut self, delta_time: f32) {
		for element in &mut self.elements {
			element.update(delta_time);
		}
	}
}

pub enum UIView {
	Login,
	None
}
