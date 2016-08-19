extern crate vecmath;
extern crate bincode;
extern crate rustc_serialize;
extern crate byteorder;

use std::net::{TcpListener, TcpStream};
use vecmath::Vector3;
use bincode::SizeLimit;
use bincode::rustc_serialize::{ encode, decode };
use std::io::{ Read, Write };
use byteorder::ByteOrder;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub enum NetworkMessage {
	Connect,
	Ping,
	Login { username: String, password: String },
	LoginResponse(Option<User>),

	Disconnect,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
pub struct User {
	pub position: Vector3<f32>,
	pub rotation: Vector3<f32>,
}

pub struct ClientSocket {
	stream: TcpStream,
	buffer: Vec<u8>,
	buff: [u8;1024],
}

impl ClientSocket {
	pub fn connect<T>(host: T, port: i32) -> Option<ClientSocket> where T : std::string::ToString{
		let stream = match TcpStream::connect(format!("{}:{}", host.to_string(), port).as_str()) {
			Err(e) => {
				println!("Could not connect to server! {:?}", e);
				return None;
			},
			Ok(s) => s
		};
		if let Err(_) = stream.set_nonblocking(true) {
			println!("Could not set stream to non-blocking mode");
			return None;
		}
		Some(ClientSocket::from_stream(stream))
	}

	pub fn from_stream(stream: TcpStream) -> ClientSocket {
		ClientSocket {
			stream: stream,
			buffer: Vec::new(),
			buff: [0;1024],
		}
	}
	pub fn get_message(&mut self) -> Option<NetworkMessage> {
		match self.stream.read(&mut self.buff) {
			Ok(size) => {
				println!("size: {}", size);
				if size == 0 {
					return Some(NetworkMessage::Disconnect);
				}
				self.buffer.extend_from_slice(&self.buff[0..size]);
			},
			Err(e) => {
				if let Some(os_error) = e.raw_os_error() {
					if os_error == 10035 {
						return None;
					}
				}
				println!("{:?}", e);
				return Some(NetworkMessage::Disconnect);
			}
		}
		if self.buffer.len() < 4 {
			return None;
		}
		let len = byteorder::BigEndian::read_u32(&self.buffer.as_slice()) as usize;
		println!("len: {}, buffer len: {}", len, self.buffer.len());
		if len + 4 <= self.buffer.len(){
			let message: Vec<u8> = self.buffer.drain(0..4+len).skip(4).collect();
			let decoded: NetworkMessage = decode(&message).unwrap();
			return Some(decoded);
		}
		None
	}
	pub fn send(&mut self, message: NetworkMessage){
		let bytes = encode(&message, SizeLimit::Infinite).unwrap();
		let mut len_bytes: [u8;4] = [0;4];
		byteorder::BigEndian::write_u32(&mut len_bytes, bytes.len() as u32);

		println!("Writing {:?}{:?}", len_bytes, bytes);
		self.stream.write(&len_bytes).unwrap();
		self.stream.write(&bytes).unwrap();
	}
}

pub struct ServerSocket {
	listener: TcpListener,
	pub clients: Vec<ClientSocket>,
}
impl ServerSocket {
	pub fn create<T>(host: T, port: i32) -> ServerSocket where T : std::string::ToString {
		let listener = TcpListener::bind(format!("{}:{}", host.to_string(), port).as_str()).unwrap();
		listener.set_nonblocking(true).unwrap();
		ServerSocket {
			listener: listener,
			clients: Vec::new(),
		}
	}

	pub fn listen<'a>(&mut self) -> Vec<NetworkMessage> {
		match self.listener.accept() {
			Err(e) => {
				let mut no_clients_error = false;
				if let Some(os_error) = e.raw_os_error() {
					if os_error == 10035 {
						no_clients_error = true;
					}
				}
				if !no_clients_error {
					println!("{:?}", e);
					return Vec::new();
				}
			},
			Ok(s) => {
				println!("Client connected: {:?}", s.1);
				self.clients.push(ClientSocket::from_stream(s.0));
			}
		};

		let mut remove_indexes = Vec::new();
		let mut messages = Vec::new();

		for i in 0..self.clients.len() {
			let ref mut client = self.clients[i];
			match client.get_message() {
				Some(message) => {
					println!("Message: {:?}", message);
					if message == NetworkMessage::Disconnect {
						remove_indexes.push(i);
						continue;
					}

					messages.push(message);
				},
				None => {}
			};
		}
		remove_indexes.reverse();
		for remove_index in &remove_indexes {
			println!("Removing at {}", remove_index);
			self.clients.remove(*remove_index);
		}

		return messages;
	}
}