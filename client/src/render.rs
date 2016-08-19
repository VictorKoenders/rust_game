use glium::{DisplayBuild, Program, DrawParameters, Depth};
use glium::draw_parameters::DepthTest;
use glium::backend::glutin_backend::{GlutinFacade, WinRef};
use glium::glutin::{ WindowBuilder, VirtualKeyCode, CursorState };
use std::f32::consts;
use game_state::GameState;
use vecmath::{ Vector3, Matrix4, vec3_dot, mat4_id, vec3_normalized, vec3_square_len };

const MOVE_SPEED: f32 = 5f32;
const ROTATE_SPEED: f32 = 0.005f32;

pub struct DisplayData<'a> {
    pub display: GlutinFacade,
    pub program: Program,
    pub perspective: Matrix4<f32>,

	pub camera_position: Vector3<f32>,
	pub camera_rotation: Vector3<f32>,

    pub view: Matrix4<f32>,
    pub light: Vector3<f32>,
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
        };

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

    pub fn update(&mut self, game_state: &GameState, delta_time: f32){
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

		let mut transformation = [0.0f32, 0.0f32, 0.0f32];
	    let mut rotation = [0.0f32, 0.0f32, 0.0f32];
	    if game_state.keyboard.is_pressed(VirtualKeyCode::A) {
		    transformation[0] -= 1.0f32;
	    }
	    if game_state.keyboard.is_pressed(VirtualKeyCode::D) {
		    transformation[0] += 1.0f32;
	    }
	    if game_state.keyboard.is_pressed(VirtualKeyCode::W) {
		    transformation[2] += 1.0f32;
	    }
	    if game_state.keyboard.is_pressed(VirtualKeyCode::S) {
		    transformation[2] -= 1.0f32;
	    }

	    if game_state.mouse.is_dragging {
		    rotation[0] = game_state.mouse.drag_difference[1];
		    rotation[1] = game_state.mouse.drag_difference[0];
	    }

	    self.camera_rotation[0] += rotation[0] * ROTATE_SPEED;
	    self.camera_rotation[1] += rotation[1] * ROTATE_SPEED;
	    self.camera_rotation[2] += rotation[2] * ROTATE_SPEED;

	    if vec3_square_len(transformation) != 0.0f32 {
		    transformation = vec3_normalized(transformation);
	    }

	    let rotated_transformation = {
		    let sin_angle = (-self.camera_rotation[1]).sin();
		    let cos_angle = (-self.camera_rotation[1]).cos();

		    [
			    transformation[0] * cos_angle - transformation[2] * sin_angle,
			    0.0f32,
			    transformation[0] * sin_angle + transformation[2] * cos_angle
		    ]
	    };

	    self.camera_position[0] += rotated_transformation[0] * delta_time / 1_000_000f32 * MOVE_SPEED;
	    self.camera_position[1] += rotated_transformation[1] * delta_time / 1_000_000f32 * MOVE_SPEED;
	    self.camera_position[2] += rotated_transformation[2] * delta_time / 1_000_000f32 * MOVE_SPEED;

	    self.view = fps_view_matrix(self.camera_position, self.camera_rotation);
    }
}

pub fn fps_view_matrix(position: Vector3<f32>, rotation: Vector3<f32>) -> Matrix4<f32> {
	let cos_pitch = rotation[0].cos();
	let sin_pitch = rotation[0].sin();
	let cos_yaw = rotation[1].cos();
	let sin_yaw = rotation[1].sin();

	let xaxis = [cos_yaw, 0.0, -sin_yaw];
	let yaxis = [ sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
	let zaxis = [ sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

	// Create a 4x4 view matrix from the right, up, forward and eye position vectors
	[
		[       xaxis[0],            yaxis[0],            zaxis[0],      0.0 ],
		[       xaxis[1],            yaxis[1],            zaxis[1],      0.0 ],
		[       xaxis[2],            yaxis[2],            zaxis[2],      0.0 ],
		[ -vec3_dot( xaxis, position.clone() ), -vec3_dot( yaxis, position.clone() ), -vec3_dot( zaxis, position.clone() ), 1.0 ]
	]
}
