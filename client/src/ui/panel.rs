use ui::{ UIElement, UIRender, EventResult };
use handler::texture::Texture;
use glium::glutin::Event;

pub struct Panel {
}

impl Panel {
	pub fn new() -> Panel {
		Panel {
		}
	}
}

impl UIElement for Panel {
	fn get_initial_position(&self, _: u32, _: u32) -> (u32, u32) {
		(50, 50)
	}
	fn get_desired_size(&self, parent_width: u32, _: u32) -> (u32, u32) {
		(parent_width - 100, 200)
	}

	fn draw(&self, render: &mut UIRender) {
		render.set_background(Texture::PanelBackground);
		render.draw_text_at("Hello world!", 50, 90);
	}

	fn update(&mut self, _: f32) {
	}

	fn handle_event(&mut self, _: &Event) -> EventResult {
		EventResult::Unhandled
	}

	fn click(&mut self) -> EventResult {
		EventResult::Unhandled
	}
	fn set_focus(&mut self) -> bool {
		false
	}
}

