use ui::{ UIElement, UIRender, UIWrapper };
use handler::texture::Texture;

pub struct Panel {
}

impl Panel {
	pub fn new() -> Panel {
		Panel {
		}
	}
}

impl UIElement for Panel {
	fn get_initial_position(&self, _: u32, parent_height: u32) -> (u32, u32) {
		(50, parent_height - 250)
	}
	fn get_desired_size(&self, parent_width: u32, _: u32) -> (u32, u32) {
		(parent_width - 100, 200)
	}

	fn update(&mut self, _: f32, _: &Vec<UIWrapper>) {
		// TODO: Implement events
		unimplemented!()
	}

	fn draw(&self, render: &mut UIRender) {
		render.set_background(Texture::PanelBackground);
		render.draw_text_at("Hello world!", 50, 90);
	}
}

