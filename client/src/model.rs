use render::{DisplayData, fps_view_matrix};
use glium::vertex::VertexBuffer;
use glium::texture::{SrgbTexture2d, RawImage2d, Texture2d};
use std::io::Cursor;
use image;
use glium::index::{NoIndices, PrimitiveType};
use glium::Surface;
use vecmath::{ Vector2, Vector3, col_mat4_mul };

pub struct Model {
    shape: VertexBuffer<Vertex3D>,
    //matrix: Matrix4<f32>,
    diffuse_texture: SrgbTexture2d,
    normal_texture: Texture2d,

	pub id: u32,
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
	pub scale: Vector3<f32>,
}

impl Model {
    /*pub fn new(display: &DisplayData) -> Model {
        let shape = VertexBuffer::new(&display.display, &[
            Vertex { position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();

        let diffuse_texture = Model::load_image(include_bytes!("../assets/tuto-14-diffuse.jpg"), image::JPEG);
        let diffuse_texture = SrgbTexture2d::new(&display.display, diffuse_texture).unwrap();

        let normal_texture = Model::load_image(include_bytes!("../assets/tuto-14-normal.png"), image::PNG);
        let normal_texture = Texture2d::new(&display.display, normal_texture).unwrap();

        let matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];

        Model {
            shape: shape,
			matrix: matrix,
			diffuse_texture: diffuse_texture,
			normal_texture: normal_texture,
			id: 0,
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			scale: [1.0, 1.0, 1.0],
        }
    }*/

    pub fn new_cube(display: &DisplayData) -> Model {
		let shape = VertexBuffer::new(&display.display, &[
			Vertex3D { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
			Vertex3D { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
			Vertex3D { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
			Vertex3D { position: [1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
			Vertex3D { position: [1.0, -1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
			Vertex3D { position: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
			Vertex3D { position: [1.0, 1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
			Vertex3D { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
			Vertex3D { position: [-1.0, 1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
			Vertex3D { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
			Vertex3D { position: [-1.0, -1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
			Vertex3D { position: [1.0, -1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
			Vertex3D { position: [-1.0, 1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
			Vertex3D { position: [1.0, 1.0, 2.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
		]).unwrap();

		let diffuse_texture = Model::load_image(include_bytes!("../assets/tuto-14-diffuse.jpg"), image::JPEG);
		let diffuse_texture = SrgbTexture2d::new(&display.display, diffuse_texture).unwrap();

		let normal_texture = Model::load_image(include_bytes!("../assets/tuto-14-normal.png"), image::PNG);
		let normal_texture = Texture2d::new(&display.display, normal_texture).unwrap();

		//let matrix = [
		//	[1.0, 0.0, 0.0, 0.0],
		//	[0.0, 1.0, 0.0, 0.0],
		//	[0.0, 0.0, 1.0, 0.0],
		//	[0.0, 0.0, 0.0, 1.0f32]
		//];

		Model {
			shape: shape,
			//matrix: matrix,
			diffuse_texture: diffuse_texture,
			normal_texture: normal_texture,
			id: 0,
			position: [0.0, 0.0, 0.0],
			rotation: [0.0, 0.0, 0.0],
			scale: [1.0, 1.0, 1.0],
		}
	}

	fn load_image<'a>(bytes: &[u8], encoding: image::ImageFormat) -> RawImage2d<'a, u8> {
		let image = image::load(Cursor::new(bytes), encoding).unwrap().to_rgba();
		let image_dimensions = image.dimensions();
		RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
	}

	pub fn render<F>(&self, display_data: &DisplayData, target: &mut F) where F: Surface {
		let matrix = fps_view_matrix(self.position, self.rotation);

		let scale_matrix = [
			[self.scale[0], 0.0, 0.0, 0.0],
			[0.0, self.scale[1], 0.0, 0.0],
			[0.0, 0.0, self.scale[2], 0.0],
			[0.0, 0.0, 0.0, 1.0]
		];

		let matrix = col_mat4_mul(matrix, scale_matrix);

		target.draw(&self.shape, NoIndices(PrimitiveType::TriangleStrip), &display_data.program,
					&uniform! { model: matrix, view: display_data.view, perspective: display_data.perspective,
                                u_light: display_data.light, diffuse_tex: &self.diffuse_texture, normal_tex: &self.normal_texture },
					&display_data.draw_parameters).unwrap();
	}
}

#[derive(Copy, Clone)]
struct Vertex3D {
	position: Vector3<f32>,
	normal: Vector3<f32>,
	tex_coords: Vector2<f32>,
}

implement_vertex!(Vertex3D, position, normal, tex_coords);