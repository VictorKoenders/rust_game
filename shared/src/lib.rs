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
			Err(_) => { println!("Could not connect to server!"); return None; },
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
				self.buffer.extend_from_slice(&self.buff[0..size]);
			},
			Err(e) => {
				println!("{:?}", e);
				return None;
			}
		}
		let len = byteorder::BigEndian::read_u32(&self.buffer.as_slice()) as usize;
		if len + 4 >= self.buffer.len(){
			let message: Vec<u8> = self.buffer.drain(0..4+len).skip(4).collect();
			let decoded: NetworkMessage = decode(&message).unwrap();
			return Some(decoded);
		}
		None
	}
}

pub struct ServerSocket {
	listener: TcpListener,
	clients: Vec<ClientSocket>,
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

	pub fn listen<'a>(&mut self) -> Vec<(&'a ClientSocket, NetworkMessage)> {
		match self.listener.accept() {
			Err(e) => {
				println!("{:?}", e);
				return None;
			},
			Ok(s) => {
				self.clients.push(ClientSocket::from_stream(s));
			}
		};

		let mut remove_indexes = Vec::new();
		let mut messages = Vec::new();

		for i in 0..self.clients.len() {
			let mut client = self.clients[i];
			match client.get_message() {
				Some(message) => {
					if message == NetworkMessage::Disconnect {
						remove_indexes.push(i);
						continue;
					}

					messages.push((&client, message));
				},
				None => {}
			};
		}
		remove_indexes.reverse();
		for remove_index in &remove_indexes {
			self.clients.remove_at(remove_indexes);
		}

		return messages;
	}
}