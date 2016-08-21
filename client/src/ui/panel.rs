use glium::texture::{SrgbTexture2d, RawImage2d};
use glium::draw_parameters::DrawParameters;
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;
use glium::{Surface,Frame,Program};
use render::DisplayData;
use std::io::Cursor;
use ui::UIElement;
use ui::Vertex2D;
use image;

pub struct Panel {
	shape: VertexBuffer<Vertex2D>,
	texture: SrgbTexture2d,
	program: Program,
}

impl UIElement for Panel {
	fn draw(&self, target: &mut Frame) {
		target.draw(&self.shape, NoIndices(PrimitiveType::TriangleStrip), &self.program,
					&uniform! { diffuse_tex: &self.texture },
					&DrawParameters::default()).unwrap();
	}
}

impl Panel {
	fn load_image<'a>(bytes: &[u8], encoding: image::ImageFormat) -> RawImage2d<'a, u8> {
		let image = image::load(Cursor::new(bytes), encoding).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
	}

	pub fn new(display: &DisplayData) -> Panel {
		let texture = Panel::load_image(include_bytes!("../../assets/blank.png"), image::PNG);
		let texture = SrgbTexture2d::new(&display.display, texture).unwrap();

		let vertex_shader_src = r#"
			#version 140
			in vec2 position;
			void main() {
				gl_Position = vec4(position, 0.0, 1.0);
			}
		"#;

			let fragment_shader_src = r#"
			#version 140
			out vec4 color;
			void main() {
				color = vec4(1.0, 0.0, 0.0, 1.0);
			}
		"#;
		let program = Program::from_source(&display.display, vertex_shader_src, fragment_shader_src, None).unwrap();

		Panel {
			shape: VertexBuffer::new(&display.display, &[
				Vertex2D { position: [-0.1, -0.1], tex_coords: [0.0, 0.0] },
				Vertex2D { position: [ 0.1, -0.1], tex_coords: [1.0, 0.0] },
				Vertex2D { position: [-0.1,  0.1], tex_coords: [0.0, 1.0] },
				Vertex2D { position: [ 0.1,  0.1], tex_coords: [1.0, 1.0] },
			]).unwrap(),
			texture: texture,
			program: program,
		}
	}
}