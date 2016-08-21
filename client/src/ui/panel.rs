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

#[inline]
fn get_dimension(val: u32, total: u32) -> f32 {
	(val as f32) / (total as f32) * 2.0 - 1.0
}

impl Panel {
	fn load_image<'a>(bytes: &[u8], encoding: image::ImageFormat) -> RawImage2d<'a, u8> {
		let image = image::load(Cursor::new(bytes), encoding).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
	}

	pub fn new(display: &DisplayData) -> Panel {
		// TODO: Load the textures from a TextureManager
		let texture = Panel::load_image(include_bytes!("../../assets/tuto-14-diffuse.jpg"), image::JPEG);
		let texture = SrgbTexture2d::new(&display.display, texture).unwrap();

		// TODO: Store this in a 2D drawing state
		let vertex_shader_src = include_str!("../../assets/ui.vert");
		let fragment_shader_src = include_str!("../../assets/ui.frag");
		let program = Program::from_source(&display.display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let mut result = Panel {
			shape: VertexBuffer::new(&display.display, &[]).unwrap(),
			texture: texture,
			program: program,
		};
		let dimensions = display.display.get_window().unwrap().get_inner_size_pixels().unwrap();
		result.window_size_changed(display, dimensions.0, dimensions.1);
		result
	}
}

impl UIElement for Panel {
	fn draw(&self, target: &mut Frame) {
		// TODO: Draw a nice border around the panel
		// TODO: Draw a nice background
		// see: http://i1057.photobucket.com/albums/t398/Duvvel/senatry/67b655d5.jpg
		target.draw(
			&self.shape,
			NoIndices(PrimitiveType::TriangleStrip),
			&self.program,
			&uniform! { tex: &self.texture },
			&DrawParameters::default()
		).unwrap();
	}

	fn window_size_changed(&mut self, display: &DisplayData, width: u32, height: u32) {
		// TODO: Move this logic to a handler in UI, seeing as all UI elements are a rectangle of some sort
		// TODO: including an anchor position:
		//		 - top, vertical_center, bottom
		//		 - left, horizontal_center, right

		let desired_x = 50;
		let desired_y = 50;
		let desired_width = 200;
		let desired_height = 200;

		let left = get_dimension(desired_x, width);
		let top = get_dimension(desired_y, height);
		let right = get_dimension(desired_x + desired_width, width);
		let bottom = get_dimension(desired_y + desired_height, height);

		self.shape = VertexBuffer::new(&display.display, &[
			Vertex2D { position: [left, top], tex_coords: [0.0, 0.0] },
			Vertex2D { position: [right, top], tex_coords: [1.0, 0.0] },
			Vertex2D { position: [left, bottom], tex_coords: [0.0, 1.0] },
			Vertex2D { position: [right, bottom], tex_coords: [1.0, 1.0] },
		]).unwrap();
	}
}
