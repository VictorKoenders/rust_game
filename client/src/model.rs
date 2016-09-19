use render::DisplayData;
use glium::vertex::VertexBuffer;
use glium::index::{NoIndices, PrimitiveType};
use vecmath::{Vector2, Vector3, col_mat4_mul, row_mat4_mul};
use handler::texture::{Texture, TextureData};
use game_state::Entity;
use error::GameError;
use glium::Surface;
use std::rc::Rc;

pub struct Model {
	shape: VertexBuffer<Vertex3D>,
	diffuse_texture: Rc<TextureData>,
	normal_texture: Rc<TextureData>,
}

impl Model {
	pub fn new_cube(display: &DisplayData) -> Result<Model, GameError> {
		// TODO: Make this indexed, see target.draw in render<F>
		let shape = try!(VertexBuffer::new(&display.display, &[
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
		]).map_err(GameError::from_creation_error)); // TODO: Deal with unwrap

		Ok(Model {
			shape: shape,
			diffuse_texture: Texture::get(Texture::Wall),
			normal_texture: Texture::get(Texture::WallNormal),
		})
	}

	pub fn render<F>(&self, display_data: &DisplayData, target: &mut F, entity: &Entity) -> Result<(), GameError>
		where F: Surface {
		/*let matrix = fps_view_matrix(entity.position, entity.rotation);
		let scale_matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		let matrix = col_mat4_mul(matrix, scale_matrix);
		println!("position: {:?}, rotation: {:?}", entity.position, entity.rotation);
		println!("Matrix: {:?}", matrix);*/

		// TODO: This doesn't work, figure out why it doesn't work

		// The matrix for position: [0, 0, 0], rotation: [0, 0, 0] is:
		// [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]
		// The matrix we need to get:
		// [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]


		let position_matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[entity.position[0], entity.position[1], entity.position[2], 1.0]
		];
		// For more info, see https://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
		let rotation_matrix = [
			[
				entity.rotation[1].cos() * entity.rotation[2].cos(),
				-entity.rotation[2].sin(),
				entity.rotation[1].sin(),
				0.0,
			],
			[
				entity.rotation[2].sin(),
				entity.rotation[0].cos() * entity.rotation[2].cos(),
				-entity.rotation[0].sin(),
				0.0,
			],
			[
				-entity.rotation[1].sin() * entity.rotation[2].sin(),
				entity.rotation[0].sin(),
				entity.rotation[0].cos() * entity.rotation[1].cos(),
				0.0,
			],
			[
				0.0,
				0.0,
				0.0,
				1.0,
			],
		];
		let scale_matrix = [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		];

		let matrix = col_mat4_mul(row_mat4_mul(position_matrix, rotation_matrix), scale_matrix);

		let diffuse_tex = match self.diffuse_texture.get_srgb_texture2d() {
			Some(t) => t,
			None => return Err(GameError::CouldNotGetTexture)
		};
		let normal_tex = match self.normal_texture.get_texture2d() {
			Some(t) => t,
			None => return Err(GameError::CouldNotGetTexture)
		};
		// TODO: make this indexed
		// see: http://tomaka.github.io/glium/glium/index/struct.IndexBuffer.html
		let indices = NoIndices(PrimitiveType::TriangleStrip);
		try!(target.draw(
			&self.shape,
			indices,
			&display_data.program,
			&uniform! {
				model: matrix,
				view: display_data.view,
				perspective: display_data.perspective,
				u_light: display_data.light,
				diffuse_tex: diffuse_tex,
				normal_tex: normal_tex,
			},
			&display_data.draw_parameters
		).map_err(GameError::from_draw_error));
		Ok(())
	}
}

// TODO: Move this to general render data
#[derive(Copy, Clone)]
struct Vertex3D {
	position: Vector3<f32>,
	normal: Vector3<f32>,
	tex_coords: Vector2<f32>,
}
implement_vertex!(Vertex3D, position, normal, tex_coords);
