use glium::glutin::{ ElementState, Event, VirtualKeyCode };
use ui::utils::{ Dimension, EventResult };
use ui::render_state::UIRender;
use handler::texture::Texture;
use ui::traits::UIElement;

const CURSOR_TOGGLE_DELAY: f32 = 300_000f32;

pub struct Textbox {
	pub text: String,
	pub is_password: bool,
	pub has_focus: bool,

	show_cursor: bool,
	cursor_time: f32,
}

impl Textbox {
	pub fn new() -> Textbox {
		Textbox {
			text: String::new(),
			is_password: false,
			has_focus: false,

			show_cursor: true,
			cursor_time: 0f32
		}
	}
}

impl UIElement for Textbox {
	fn get_initial_position(&self, _: &Dimension) -> (u32, u32){
		(50, 50)
	}
	fn get_desired_size(&self, _: &Dimension) -> (u32, u32){
		(200, 30)
	}

	fn set_focus(&mut self) -> bool {
		self.has_focus = true;
		true
	}

	fn draw(&self, render: &mut UIRender) {
		let mut text_to_draw = self.text.clone();
		if self.has_focus && self.show_cursor { text_to_draw.push('|'); }

		render.set_background(Texture::PanelBackground);
		render.draw_text_at(text_to_draw, 10, 0);
	}

	fn update(&mut self, delta_time: f32) {
		self.cursor_time += delta_time;
		if self.cursor_time > CURSOR_TOGGLE_DELAY {
			self.cursor_time -= CURSOR_TOGGLE_DELAY;
			self.show_cursor = !self.show_cursor;
		}
	}

	fn click(&mut self) -> EventResult {
		self.set_focus();
		EventResult::Handled
	}

	fn handle_event(&mut self, ev: &Event) -> EventResult {
		if self.has_focus {
			if let Event::ReceivedCharacter(char) = *ev {
				if char as u8 != 8 {
					self.text.push(char);
				}
				return EventResult::Handled;
			}
			if let Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Back)) = *ev {
				self.text.pop();
				return EventResult::Handled;
			}
			if let Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Escape)) = *ev {
				self.has_focus = false;
				return EventResult::Handled;
			}
			if let Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Tab)) = *ev {
				self.has_focus = false;
				return EventResult::SelectNext;
			}

			// Capture all other keyboard inputs so the game doesn't react
			if let Event::KeyboardInput(_, _, _) = *ev {
				return EventResult::Handled;
			}

			if let Event::MouseInput(ElementState::Pressed, _) = *ev {
				self.has_focus = false;
			}
		}

		EventResult::Unhandled
	}
}
