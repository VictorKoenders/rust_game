use glium::glutin::{ VirtualKeyCode, ElementState, MouseButton, CursorState };
use vecmath::{ Vector2, Vector3, vec3_normalized, vec3_square_len };
use model::Model;

pub struct GameState {
	pub keyboard: KeyboardState,
	pub mouse: MouseState,
	pub player: Option<Entity>,
	pub entities: Vec<Entity>,
}

pub struct Entity {
	pub id: u32,
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
	pub model: Option<Model>,
}

const MOVE_SPEED: f32 = 5f32;
const ROTATE_SPEED: f32 = 0.005f32;

impl GameState {
	pub fn new() -> GameState {
		GameState {
			keyboard: KeyboardState::new(),
			mouse: MouseState::new(),
			player: Some(Entity { id: 0, position: [0.0, 0.0, -10.0], rotation: [0.0, 0.0, 0.0], model: None }),
			entities: Vec::new(),
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		if let Some(ref mut player) = self.player {

			let mut transformation = [0.0f32, 0.0f32, 0.0f32];
			let mut rotation = [0.0f32, 0.0f32, 0.0f32];
			if self.keyboard.is_pressed(VirtualKeyCode::A) {
				transformation[0] -= 1.0f32;
			}
			if self.keyboard.is_pressed(VirtualKeyCode::D) {
				transformation[0] += 1.0f32;
			}
			if self.keyboard.is_pressed(VirtualKeyCode::W) {
				transformation[2] += 1.0f32;
			}
			if self.keyboard.is_pressed(VirtualKeyCode::S) {
				transformation[2] -= 1.0f32;
			}
			if self.mouse.is_dragging {
				rotation[0] = self.mouse.drag_difference[1];
				rotation[1] = self.mouse.drag_difference[0];
			}

			player.rotation[0] += rotation[0] * ROTATE_SPEED;
			player.rotation[1] += rotation[1] * ROTATE_SPEED;
			player.rotation[2] += rotation[2] * ROTATE_SPEED;

			if vec3_square_len(transformation) != 0.0f32 {
				transformation = vec3_normalized(transformation);
			}

			let rotated_transformation = {
				let sin_angle = (-player.rotation[1]).sin();
				let cos_angle = (-player.rotation[1]).cos();

				[
					transformation[0] * cos_angle - transformation[2] * sin_angle,
					0.0f32,
					transformation[0] * sin_angle + transformation[2] * cos_angle
				]
			};

			player.position[0] += rotated_transformation[0] * delta_time / 1_000_000f32 * MOVE_SPEED;
			player.position[1] += rotated_transformation[1] * delta_time / 1_000_000f32 * MOVE_SPEED;
			player.position[2] += rotated_transformation[2] * delta_time / 1_000_000f32 * MOVE_SPEED;
		}
	}
}

pub struct KeyboardState {
	keys: Vec<VirtualKeyCode>,
}

impl KeyboardState {
	pub fn new() -> KeyboardState {
		KeyboardState {
			keys: Vec::new()
		}
	}
	pub fn is_pressed(&self, code: VirtualKeyCode) -> bool {
		self.keys.contains(&code)
	}

	fn add_key(&mut self, code: VirtualKeyCode){
		if !self.is_pressed(code) {
			self.keys.push(code);
		}
	}

	fn remove_key(&mut self, code: VirtualKeyCode){
		if let Some(index) = self.keys.iter().position(|x| *x == code) {
			self.keys.remove(index);
		}
	}

	pub fn update(&mut self, code: VirtualKeyCode, state: ElementState){
		match state {
			ElementState::Pressed => self.add_key(code),
			ElementState::Released => self.remove_key(code)
		};
	}
}

pub struct MouseState {
	pub drag_difference: Vector2<f32>,
	pub is_dragging: bool,
	last_mouse_position: Vector2<f32>,
	pub desired_cursor_position: Option<Vector2<f32>>,
	pub desired_cursor_state: CursorState,
}

impl MouseState {
	pub fn new() -> MouseState {
		MouseState {
			drag_difference: [0.0f32, 0.0f32],
			is_dragging: false,
			last_mouse_position: [0.0f32, 0.0f32],
			desired_cursor_position: None,
			desired_cursor_state: CursorState::Normal,
		}
	}

	pub fn reset(&mut self){
		self.drag_difference = [0f32, 0f32];
		self.desired_cursor_position = None;
	}

	pub fn mouse_moved(&mut self, x: i32, y: i32) {
		if self.is_dragging {
			self.drag_difference = [
				x as f32 - self.last_mouse_position[0],
				y as f32 - self.last_mouse_position[1]
			];
			self.desired_cursor_position = Some(self.last_mouse_position);
		} else {
			self.last_mouse_position = [x as f32, y as f32];
		}
	}

	pub fn mouse_button(&mut self, button: MouseButton, state: ElementState) {
		match state {
			ElementState::Pressed => if let MouseButton::Right = button {
				self.is_dragging = true;
				self.desired_cursor_state = CursorState::Hide;
			},
			ElementState::Released => if let MouseButton::Right = button {
				self.is_dragging = false;
				self.desired_cursor_state = CursorState::Normal;
			}
		};
	}
}