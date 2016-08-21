use glium::{DisplayBuild, Program, DrawParameters, Depth};
use glium::draw_parameters::DepthTest;
use glium::backend::glutin_backend::{GlutinFacade, WinRef};
use glium::glutin::{ WindowBuilder, CursorState };
use std::f32::consts;
use game_state::GameState;
use vecmath::{ Vector3, Matrix4, vec3_dot, mat4_id };
use model::Model;

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

	current_cursor_state: Option<CursorState>,
}

impl<'a> DisplayData<'a> {
	pub fn new() -> DisplayData<'a> {
		let display = WindowBuilder::new()
			.with_depth_buffer(24)
			.build_glium().unwrap();

		let vertex_shader_src = include_str!("../assets/default.vert");
		let fragment_shader_src = include_str!("../assets/default.frag");

		let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

		let perspective = {
			let window: WinRef = display.get_window().unwrap();
			let (width, height) = window.get_inner_size_pixels().unwrap();

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

		DisplayData {
			display: display,
			program: program,
			perspective: perspective,
			light: light,
			draw_parameters: params,
			camera_position: [0.0, 0.0, -10.0],
			camera_rotation: [0.0, 0.0, 0.0],
			view: mat4_id(),
			current_cursor_state: None,
		}
	}

	pub fn get_screen_dimensions(&self) -> (u32, u32) {
		// TODO: error checking?
		self.display.get_window().unwrap().get_inner_size_pixels().unwrap()
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

    pub fn update(&mut self, game_state: &mut GameState){
	    if let Some(position) = game_state.mouse.desired_cursor_position {
			self.display.get_window().unwrap().set_cursor_position(position[0] as i32, position[1] as i32).unwrap();
		}

		match self.current_cursor_state {
			None => {
				self.display.get_window().unwrap().set_cursor_state(game_state.mouse.desired_cursor_state).unwrap();
				self.current_cursor_state = Some(game_state.mouse.desired_cursor_state);
			},
			Some(state) => {
				if state != game_state.mouse.desired_cursor_state {
					self.display.get_window().unwrap().set_cursor_state(game_state.mouse.desired_cursor_state).unwrap();
					self.current_cursor_state = Some(game_state.mouse.desired_cursor_state);
				}
			}
		};

		if let Some(ref mut player) = game_state.player {
			if let None = player.model {
				// TODO: I don't think this ever gets reached
				player.model = Some(Model::new_cube(self));
			}
		}
		for mut entity in game_state.entities.iter_mut().filter(|e| e.model.is_none()){
			// TODO: We should load the model when the entity gets created
			entity.model = Some(Model::new_cube(self));
		}

		// TODO: Make the camera follow the player
		if let Some(ref player) = game_state.player {
			self.camera_position = player.position;
			// TODO: Make the camera movable around the player
			self.camera_position[2] -= 1f32;
			self.camera_rotation = player.rotation;
		}
	    self.view = fps_view_matrix(self.camera_position, self.camera_rotation);
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
		[-vec3_dot(xaxis, position.clone()), -vec3_dot(yaxis, position.clone()), -vec3_dot(zaxis, position.clone()), 1.0]
	]
}
