use handler::texture::Texture;
use ui::utils::RenderCommand;

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
