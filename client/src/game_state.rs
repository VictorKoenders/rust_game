use glium::glutin::{ VirtualKeyCode, ElementState, MouseButton, CursorState };
use vecmath::{ Vector2, Vector3 };
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

impl GameState {
	pub fn new() -> GameState {
		GameState {
			keyboard: KeyboardState::new(),
			mouse: MouseState::new(),
			player: None,
			entities: Vec::new(),
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