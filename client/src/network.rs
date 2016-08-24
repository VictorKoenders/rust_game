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
	last_send_messages: Vec<(NetworkMessage, u64)>,
}

impl Network {
	pub fn new() -> Network {
		Network {
			socket: ClientSocket::create("localhost", 8080),
			last_connect_time: None,
			is_connecting: false,
			connect_receiver: None,
			last_send_messages: Vec::new(),
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
				clone.connect().unwrap_or_else(|_|());
				sender.send(clone).unwrap();// TODO: Deal with unwrap
			});
		}
	}

	fn handle_message(&mut self, message: NetworkMessage, game_state: &mut GameState){
		// TODO: Find a better solution for this
		// because this function is going to be massive if we all put it in here
		if message == NetworkMessage::Ping {
			if let Err(e) = self.socket.send(NetworkMessage::Ping) {
				println!("Socket error: {:?}", e);
				self.disconnect();
				return;
			}
		}
		if let NetworkMessage::Identify(uid) = message {
			if let Some(ref mut player) = game_state.player {
				player.id = uid;
			} else {
				game_state.player = Some(Entity {
					position: [0.0, 0.0, 0.0],
					rotation: [0.0, 0.0, 0.0],
					id: uid,
					model: None,
				});
			}
			return;
		}
		if let NetworkMessage::RemoveEntity { uid } = message{
			if let Some(index) = game_state.entities.iter().position(|x| x.id == uid) {
				game_state.entities.remove(index);
			}
		}
		if let NetworkMessage::SetPosition { uid, position, .. } = message {
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

	pub fn send_throttled(&mut self, message: NetworkMessage, delay_in_ms: u64){
		let position = self.last_send_messages.iter_mut().position(|x| x.0.is_same_type_as(&message)).unwrap_or_else(||self.last_send_messages.len());
		if position != self.last_send_messages.len() {
			let time = self.last_send_messages[position].1;
			if time > ::time::precise_time_ns() - delay_in_ms * 1_000_000u64 {
				return;
			}
			self.last_send_messages.remove(position);
		}
		self.last_send_messages.push((message.clone(), time::precise_time_ns()));
		self.send(message);
	}

	pub fn send(&mut self, message: NetworkMessage){
		if self.socket.is_connected() {
			// TODO: Handle error message?
			// TODO: Disconnect on error?
			self.socket.send(message).unwrap();
		}
	}

	pub fn update(&mut self, game_state: &mut GameState) {
		if !self.socket.is_connected() {
			self.attempt_connect();
			return;
		}
		while match self.socket.get_message() {
			Ok(Some(message)) => {
				self.handle_message(message, game_state);
				true
			},
			Ok(None) => false,
			Err(e) => {
				println!("Socket error: {:?}", e);
				self.disconnect();
				false
			}
		} {}
	}
}