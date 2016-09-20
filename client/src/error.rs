use std::error::Error;
use std::fmt::{ Display, Formatter, Result };

#[derive(Debug)]
pub struct GameError {
	format: String
}

impl GameError {
	pub fn create(err: String, file: &'static str, line: u32) -> GameError {
		let format = format!("[{}:{}]: {}", file, line, err);
		GameError {
			format: format
		}
	}
}

impl Error for GameError {
	fn description(&self) -> &str {
		self.format.as_str()
	}
}

impl Display for GameError {
	fn fmt(&self, formatter: &mut Formatter) -> Result {
		formatter.write_str(self.description())
	}
}

#[macro_export]
macro_rules! try_get {
	($e: expr, $description: expr) => {
		match $e {
			None => return Err(::error::GameError::create($description.to_string(), file!(), line!())),
			Some(e) => e
		}
	}
}

#[macro_export]
macro_rules! try {
	($e: expr) => {
		match $e {
			Err(e) => {
				#[allow(unused_imports)]
		 		use std::error::Error;
				return Err(::error::GameError::create(e.description().to_string(), file!(), line!()));
			},

			Ok(d) => d
		}
	};
	($e: expr, $description: expr) => {
		match $e {
			Err(_) => return Err(::error::GameError::create($description.to_string(), file!(), line!())),
			Ok(d) => d
		}
	};
}
#[macro_export]
macro_rules! throw {
	($e: expr) => {
		return Err(GameError::create($e.to_string(), file!(), line!()))
	};
}