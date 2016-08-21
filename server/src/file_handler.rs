use shared::User;
use std::fs::OpenOptions;
use std::io::Read;
use bincode::rustc_serialize::{encode, decode};
use bincode::SizeLimit;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct UserPassword {
	pub user_id: u32,
	pub password: String,
}

#[allow(dead_code)] // TODO: Implement
pub struct FileHandler {
}

impl FileHandler {
	#[allow(dead_code)] // TODO: Implement
	pub fn load_users() -> Vec<(User, UserPassword)> {
		if let Ok(mut file) = OpenOptions::new().read(true).open("users.dat") {
			let mut data = Vec::new();
			let size = match file.read_to_end(&mut data) {
				Ok(s) => s,
				Err(_) => return Vec::new()
			};
			let users: Vec<(User, UserPassword)> = decode(&data[0..size]).unwrap();
			users
		} else {
			Vec::new()
		}
	}

	#[allow(dead_code)] // TODO: Implement
	pub fn save_users(users: &Vec<(User, UserPassword)>) {
		let data: Vec<u8> = encode(users, SizeLimit::Infinite).unwrap();
		println!("Data: {:?}", data);
	}
}