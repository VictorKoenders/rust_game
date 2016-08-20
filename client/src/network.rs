use std::sync::mpsc::{ Receiver, channel };
use shared::{ ClientSocket, NetworkMessage };
use time;
use std::thread;
use game_state::{ GameState, Entity };

pub struct Network {
	socket: ClientSocket,
	last_connect_time: Option<f64>,
	is_connecting: bool,
	connect_receiver: Option<Receiver<ClientSocket>>,
}

impl Network {
	pub fn new() -> Network {
		Network {
			socket: ClientSocket::create("localhost", 8080),
			last_connect_time: None,
			is_connecting: false,
			connect_receiver: None
		}
	}

	fn disconnect(&mut self){
		self.socket.disconnect();
		self.is_connecting = false;
		self.connect_receiver = None;

		self.last_connect_time = Some(time::precise_time_s());
	}

	fn attempt_connect(&mut self) {
		if self.is_connecting {
			if let Some(ref receiver) = self.connect_receiver {
				if let Ok(socket) = receiver.try_recv() {
					self.is_connecting = false;
					self.socket = socket;
					self.last_connect_time = Some(time::precise_time_s());
				}
			}
			return;
		}
		let should_connect = match self.last_connect_time {
			None => true,
			Some(t) => time::precise_time_s() - t > 1f64
		};
		if should_connect {
			self.is_connecting = true;
			let (sender, receiver) = channel();
			self.connect_receiver = Some(receiver);
			let mut clone = self.socket.clone();
			thread::spawn(move ||{
				clone.connect().unwrap();
				sender.send(clone).unwrap();
			});
		}
	}

	fn handle_message(&mut self, message: NetworkMessage, game_state: &mut GameState){
		if message == NetworkMessage::Ping {
			if let Err(e) = self.socket.send(NetworkMessage::Ping) {
				println!("Socket error: {:?}", e);
				self.disconnect();
				return;
			}
		}
		if let NetworkMessage::Identify(uid) = message {
			game_state.player = Some(Entity {
				position: [0.0, 0.0, 0.0],
				rotation: [0.0, 0.0, 0.0],
				id: uid,
				model: None,
			});
			return;
		}
		if let NetworkMessage::RemoveEntity { uid } = message{
			if let Some(index) = game_state.entities.iter().position(|x| x.id == uid) {
				game_state.entities.remove(index);
			}
		}
		if let NetworkMessage::SetPosition { uid, position, rotation: _ } = message {
			if let Some(ref player) = game_state.player {
				if player.id == uid {
					return;
				}
			}
			let mut found = false;
			{
				let item = game_state.entities.iter_mut().find(|c| c.id == uid);
				if let Some(c) = item {
					c.position = position;
					found = true;
				}
			}
			if !found {
				let e = Entity {
					position: position,
					rotation: [0.0, 0.0, 0.0],
					id: uid,
					model: None,
				};
				game_state.entities.push(e);
			}
		}
	}

	pub fn send_throttled(&mut self, message: NetworkMessage, delay_in_ms: u32){
		// TODO: Implement
		self.send(message);
	}

	pub fn send(&mut self, message: NetworkMessage){
		if self.socket.is_connected() {
			self.socket.send(message).unwrap();
		}
	}

	pub fn update(&mut self, game_state: &mut GameState) {
		if !self.socket.is_connected() {
			self.attempt_connect();
			return;
		}
		match self.socket.get_message() {
			Ok(Some(message)) => self.handle_message(message, game_state),
			Ok(None) => {},
			Err(e) => {
				println!("Socket error: {:?}", e);
				self.disconnect();
			}
		}
	}
}