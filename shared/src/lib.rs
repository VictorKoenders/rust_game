extern crate bincode;
extern crate byteorder;
extern crate rustc_serialize;
extern crate vecmath;

use bincode::SizeLimit;
use bincode::rustc_serialize::{ encode, decode };

use byteorder::ByteOrder;

use std::string;
use std::net::{TcpListener, TcpStream};
use std::io::{ Read, Write };

use vecmath::Vector3;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug, Clone)]
pub enum NetworkMessage {
	None,
	//Connect,
	Ping,
	PingResult(u32),
	Identify(u32),
	RemoveEntity { uid: u32 },
	SetPosition { uid: u32, position: Vector3<f32>, rotation: Vector3<f32> },
	//Login { username: String, password: String },
	//LoginResponse(Option<User>),
}

#[cfg(windows)]
pub static NO_CONTENT_CODE: i32 = 10035;

#[cfg(unix)]
pub static NO_CONTENT_CODE: i32 = 35;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct User {
	pub id: u32,
	pub name: String,
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
}

pub struct ClientSocket {
	stream: Option<TcpStream>,
	host: String,
	port: u16,
	buffer: Vec<u8>,
	buff: [u8;1024],
	pub id: u32,
	pub last_ping_time: f64
}

#[derive(Debug)]
pub enum ClientError {
	CouldNotConnect,
	Disconnected,
}

static mut LAST_ID: u32 = 0;
impl ClientSocket {
	pub fn create<T : string::ToString>(host: T, port: u16) -> ClientSocket {
		unsafe { LAST_ID += 1 };
		ClientSocket {
			stream: None,
			host: host.to_string(),
			port: port,
			buffer: Vec::new(),
			buff: [0;1024],
			id: unsafe { LAST_ID },
			last_ping_time: 0f64,
		}
	}

	pub fn from_stream(stream: TcpStream) -> ClientSocket {
		unsafe { LAST_ID += 1 };
		ClientSocket {
			stream: Some(stream),
			host: String::new(),
			port: 0,
			buffer: Vec::new(),
			buff: [0;1024],
			id:  unsafe { LAST_ID },
			last_ping_time: 0f64,
		}
	}
	pub fn is_connected(&self) -> bool {
		match self.stream { None => false, Some(_) => true }
	}
	pub fn connect(&mut self) -> Result<(), ClientError> {
		let stream = match TcpStream::connect(format!("{}:{}", self.host, self.port).as_str()) {
			Err(e) => {
				println!("Could not connect to server! {:?}", e);
				return Err(ClientError::CouldNotConnect);
			},
			Ok(s) => s
		};
		if let Err(_) = stream.set_nonblocking(true) {
			println!("Could not set stream to non-blocking mode");
			return Err(ClientError::CouldNotConnect);
		}
		self.stream = Some(stream);
		Ok(())
	}

	pub fn disconnect(&mut self){
		self.stream = None;
	}
	pub fn get_message(&mut self) -> Result<Option<NetworkMessage>, ClientError> {
		let mut stream = match self.stream {
			None => return Err(ClientError::Disconnected),
			Some(ref s) => s
		};

		match stream.read(&mut self.buff) {
			Ok(size) => {
				if size == 0 {
					return Err(ClientError::Disconnected);
				}
				self.buffer.extend_from_slice(&self.buff[0..size]);
			},
			Err(e) => {
				if let Some(os_error) = e.raw_os_error() {
					if os_error == NO_CONTENT_CODE {
						return Ok(None);
					}
				}
				println!("{:?}", e);
				return Err(ClientError::Disconnected);
			}
		}
		if self.buffer.len() < 4 {
			return Ok(None);
		}
		let len = byteorder::BigEndian::read_u32(&self.buffer.as_slice()) as usize;
		if len + 4 <= self.buffer.len(){
			let message: Vec<u8> = self.buffer.drain(0..4+len).skip(4).collect();
			let decoded: NetworkMessage = decode(&message).unwrap();
			return Ok(Some(decoded));
		}
		Ok(None)
	}
	pub fn send(&mut self, message: NetworkMessage) -> Result<(), ClientError>{
		let mut stream = match self.stream {
			Some(ref s) => s,
			None => return Err(ClientError::Disconnected)
		};
		let bytes = encode(&message, SizeLimit::Infinite).unwrap();
		let mut len_bytes: [u8;4] = [0;4];
		byteorder::BigEndian::write_u32(&mut len_bytes, bytes.len() as u32);

		if stream.write(&len_bytes).is_err() { return Err(ClientError::Disconnected); }
		if stream.write(&bytes).is_err() { return Err(ClientError::Disconnected); }
		Ok(())
	}
}