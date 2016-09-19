use glium::SwapBuffersError;
use std::error::Error;
use glium::DrawError;
use glium::vertex::BufferCreationError;
use std::fmt::{ Display, Formatter, Result };

#[derive(Debug)]
pub enum GameError {
	SwapBuffersError(SwapBuffersError),
	CreationError(BufferCreationError),
	DrawError(DrawError),
	NoWindow,
	StringError(String),
	CouldNotGetTexture,
}

impl GameError {
	pub fn from_swap_buffers_error(err: SwapBuffersError) -> GameError {
		GameError::SwapBuffersError(err)
	}
	pub fn from_creation_error(err: BufferCreationError) -> GameError {
		GameError::CreationError(err)
	}
	pub fn from_draw_error(err: DrawError) -> GameError {
		GameError::DrawError(err)
	}
	pub fn from_string(err: String) -> GameError {
		GameError::StringError(err)
	}
}

impl Error for GameError {
	fn description(&self) -> &str {
		match *self {
			GameError::SwapBuffersError(ref e) => e.description(),
			GameError::CreationError(ref e) => e.description(),
			GameError::DrawError(ref e) => e.description(),
			GameError::NoWindow => "No window present",
			GameError::StringError(ref e) => e,
			GameError::CouldNotGetTexture => "Could not get texture",
		}
	}
}

impl Display for GameError {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		formatter.write_str(self.description())
	}
}