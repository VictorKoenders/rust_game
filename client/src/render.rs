use glium::{DisplayBuild, Program, DrawParameters, Depth};
use glium::draw_parameters::DepthTest;
use glium::backend::glutin_backend::{GlutinFacade, WinRef};
use glium::glutin::{ WindowBuilder, CursorState };
use std::f32::consts;
use game_state::GameState;
use vecmath::{ Vector3, Matrix4, vec3_dot, mat4_id };
use model::Model;
use std::io::Cursor;
use std::rc::Rc;
use error::GameError;
use glium_text::{TextSystem,FontTexture};

pub struct DisplayData<'a> {
	pub display: GlutinFacade,
	pub program: Program,
	pub perspective: Matrix4<f32>,

	// TODO: Move this to a seperate camera struct
	pub camera_position: Vector3<f32>,
	pub camera_rotation: Vector3<f32>,

	pub view: Matrix4<f32>,
	pub light: Vector3<f32>,

	// TODO: These draw parameters are only used in a 3D context
	// so; make that 3D context
	pub draw_parameters: DrawParameters<'a>,
	pub text_system: TextSystem,
	pub font_texture: Rc<FontTexture>,

	current_cursor_state: Option<CursorState>,
}

#[cfg(windows)]
const ARIAL_FONT: &'static [u8] = include_bytes!("C:/Windows/Fonts/ARIAL.TTF");

impl<'a> DisplayData<'a> {
	pub fn new() -> Result<DisplayData<'a>, GameError> {
		let display = try!(WindowBuilder::new()
			.with_depth_buffer(24)
			.build_glium());

		let vertex_shader_src = include_str!("../assets/shaders/default.vert");
		let fragment_shader_src = include_str!("../assets/shaders/default.frag");

		let program = try!(Program::from_source(
			&display,
			vertex_shader_src,
			fragment_shader_src,
			None
		));

		let perspective = {
			let window: WinRef = try_get!(display.get_window(), "Could not get window");
			let (width, height) = try_get!(window.get_inner_size_pixels(), "Could not get window pixel size");

			DisplayData::get_perspective(width, height)
		};

		// TODO: Do something with the light
		let light = [1.4, 0.4, 0.7f32];

		let params = DrawParameters {
			depth: Depth {
				test: DepthTest::IfLess,
				write: true,
				..Default::default()
			},
			..Default::default()
		};

		let text_system = TextSystem::new(&display);
		let font = try!(FontTexture::new(&display, Cursor::new(ARIAL_FONT), 32), "Could not create font Arial 32pt");

		Ok(DisplayData {
			display: display,
			program: program,
			perspective: perspective,
			light: light,
			draw_parameters: params,
			camera_position: [0.0, 0.0, -10.0],
			camera_rotation: [0.0, 0.0, 0.0],
			view: mat4_id(),

			text_system: text_system,
			font_texture: Rc::new(font),
			current_cursor_state: None,
		})
	}

	pub fn get_screen_dimensions(&self) -> Result<(u32, u32), GameError> {
		// TODO: Cache this data and update it when we get a resize event
		Ok(try_get!(
			try_get!(self.display.get_window(), "Could not get window")
			.get_inner_size_pixels(), "Could not get window pixel size"))
	}

	pub fn resize(&mut self, width: u32, height: u32) {
		self.perspective = DisplayData::get_perspective(width, height);
	}

	fn get_perspective(width: u32, height: u32) -> Matrix4<f32> {
		let aspect_ratio = height as f32 / width as f32;

		let fov: f32 = consts::PI / 3.0;
		let zfar = 1024.0;
		let znear = 0.1;

		let f = 1.0 / (fov / 2.0).tan();

		[
			[f * aspect_ratio, 0.0, 0.0, 0.0],
			[0.0, f, 0.0, 0.0],
			[0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
			[0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
		]
	}

	pub fn update(&mut self, game_state: &mut GameState) -> Result<(), GameError> {
		if let Some(position) = game_state.mouse.desired_cursor_position {
			let window = try_get!(self.display.get_window(), "Could not get window");
			try!(window.set_cursor_position(position[0] as i32, position[1] as i32), "Could not set cursor position");
		}

		match self.current_cursor_state {
			None => {
				let window = try_get!(self.display.get_window(), "Could not get window handle");
				try!(window.set_cursor_state(game_state.mouse.desired_cursor_state), "Could not set cursor state");
				self.current_cursor_state = Some(game_state.mouse.desired_cursor_state);
			},
			Some(state) => {
				if state != game_state.mouse.desired_cursor_state {
					let window = try_get!(self.display.get_window(), "Could not get window handle");
					try!(window.set_cursor_state(game_state.mouse.desired_cursor_state), "Could not set cursor state");
					self.current_cursor_state = Some(game_state.mouse.desired_cursor_state);
				}
			}
		};

		if let Some(ref mut player) = game_state.player {
			if let None = player.model {
				// TODO: We should load the model when the player logs in
				player.model = Some(try!(Model::new_cube(self)));
			}
		}

		for mut entity in game_state.entities.iter_mut().filter(|e| e.model.is_none()) {
			// TODO: We should load the model when the entity gets created
			println!("Creating entity model");
			entity.model = Some(try!(Model::new_cube(self)));
		}

		// TODO: Make the camera follow the player
		if let Some(ref player) = game_state.player {
			self.camera_position = player.position;
			// TODO: Make the camera movable around the player
			self.camera_position[2] -= 1f32;
			self.camera_rotation = player.rotation;
		}
		self.view = fps_view_matrix(self.camera_position, self.camera_rotation);
		Ok(())
	}
}

pub fn fps_view_matrix(position: Vector3<f32>, rotation: Vector3<f32>) -> Matrix4<f32> {
	let cos_pitch = rotation[0].cos();
	let sin_pitch = rotation[0].sin();
	let cos_yaw = rotation[1].cos();
	let sin_yaw = rotation[1].sin();

	let xaxis = [cos_yaw, 0.0, -sin_yaw];
	let yaxis = [sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
	let zaxis = [sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

	// Create a 4x4 view matrix from the right, up, forward and eye position vectors
	[
		[xaxis[0], yaxis[0], zaxis[0], 0.0],
		[xaxis[1], yaxis[1], zaxis[1], 0.0],
		[xaxis[2], yaxis[2], zaxis[2], 0.0],
		[-vec3_dot(xaxis, position), -vec3_dot(yaxis, position), -vec3_dot(zaxis, position), 1.0]
	]
}
