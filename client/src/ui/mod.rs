mod render_state;
mod traits;
mod utils;
mod wrapper;
mod elements;

pub use ui::utils::*;
pub use ui::render_state::*;
pub use ui::traits::*;
pub use ui::wrapper::*;

use glium::Frame;
use glium::glutin::{ Event, ElementState };
use render::DisplayData;
use ui::elements::{ Panel, Textbox };
use error;

pub struct UI {
	pub elements: Vec<UIWrapper>,

	mouse_x: u32,
	mouse_y: u32
}

impl UI {
	pub fn new() -> UI {
		UI {
			elements: Vec::new(),
			mouse_x: 0,
			mouse_y: 0
		}
	}

	pub fn load(&mut self, display: &DisplayData, view: UIView) -> Result<(), error::GameError> {
		if let UIView::Login = view {
			let size = try!(display.get_screen_dimensions());
			let mut panel = try!(UIWrapper::new(display, Panel::new(), &Dimension { x: 0, y: 0, width: size.0, height: size.1 }));

			let mut username_textbox = Textbox::new();
			let mut password_textbox = Textbox::new();

			username_textbox.has_focus = true;
			password_textbox.is_password = true;

			let mut username_textbox = try!(UIWrapper::new(display, username_textbox, &Dimension::from_uielement(&panel)));
			let mut password_textbox = try!(UIWrapper::new(display, password_textbox, &Dimension::from_uielement(&panel)));

			username_textbox.position = (0, 0);
			username_textbox.size = (100, 100);
			password_textbox.position = (0, 100);
			password_textbox.size = (100, 100);

			try!(username_textbox.resize(display, &Dimension::from_uielement(&panel)));
			try!(password_textbox.resize(display, &Dimension::from_uielement(&panel)));

			panel.children.push(username_textbox);
			panel.children.push(password_textbox);

			self.elements.push(panel);
		}
		Ok(())
	}

	pub fn render(&mut self, target: &mut Frame, display: &DisplayData) -> Result<(), error::GameError> {
		for element in &mut self.elements {
			try!(element.draw(target, display, 0, 0));
		}
		Ok(())
	}

	pub fn resize(&mut self, display: &DisplayData, width: u32, height: u32) -> Result<(), error::GameError>{
		for element in &mut self.elements {
			try!(element.resize(display, &Dimension { x: 0, y: 0, width: width, height: height }));
		}
		Ok(())
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
				let result = element.click(self.mouse_x,  self.mouse_y);
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
