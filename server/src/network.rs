use std::string;
use std::net::TcpListener;
use std::sync::mpsc::SendError;

use shared::{ClientSocket, NetworkMessage, ClientError, NO_CONTENT_CODE};

pub struct ServerSocket {
	listener: TcpListener,
	pub clients: Vec<ClientSocket>,
}

#[derive(Debug)]
pub enum ServerError {
	CouldNotAcceptSocket,
	ClientError(ClientError),
	ThreadError,
}

impl From<ClientError> for ServerError {
	fn from(err: ClientError) -> ServerError {
		ServerError::ClientError(err)
	}
}
impl From<SendError<NetworkMessage>> for ServerError {
	fn from(_: SendError<NetworkMessage>) -> ServerError {
		ServerError::ThreadError
	}
}

impl ServerSocket {
	pub fn create<T: string::ToString>(host: T, port: i32) -> ServerSocket {
		let address = format!("{}:{}", host.to_string(), port);
		println!("Setting up socket on: {}", &address);
		let listener = TcpListener::bind(address.as_str()).unwrap();
		//        let listener = TcpListener::bind(format!("{}:{}", host.to_string(), port).as_str()).unwrap();
		listener.set_nonblocking(true).unwrap();
		ServerSocket {
			listener: listener,
			clients: Vec::new(),
		}
	}

	//pub fn send_with() {}

	pub fn broadcast(&mut self, message: NetworkMessage) {
		for client in self.clients.iter_mut() {
			client.send(message.clone()).unwrap();
		}
	}

	pub fn listen<F1, F2, F3>(&mut self,
	                          client_created_callback: F1,
	                          client_message_callback: F2,
	                          client_removed_callback: F3) -> Result<(), ServerError>
	                          where F1 : Fn(&mut ClientSocket) -> Result<(), ServerError>,
	                                F2 : Fn(&mut ClientSocket, NetworkMessage) -> Result<(), ServerError>,
	                                F3 : Fn(&mut ClientSocket) -> Result<(), ServerError> {
		match self.listener.accept() {
			Err(e) => {
				let mut no_clients_error = false;
				if let Some(os_error) = e.raw_os_error() {
					if os_error == NO_CONTENT_CODE {
						no_clients_error = true;
					}
				}
				if !no_clients_error {
					println!("{:?}", e);
					return Err(ServerError::CouldNotAcceptSocket);
				}
			},
			Ok(s) => {
				println!("Client connected: {:?}", s.1);
				let mut client = ClientSocket::from_stream(s.0);
				try!(client_created_callback(&mut client));
				self.clients.push(client);
			}
		};

		let mut remove_indexes: Vec<usize> = Vec::new();

		for i in 0..self.clients.len() {
			let ref mut client = self.clients[i];
			match client.get_message() {
				Ok(Some(message)) => {
					try!(client_message_callback(client, message));
				},
				Ok(None) => {},
				Err(ClientError::Disconnected) => {
					remove_indexes.push(i);
				},
				Err(e) => {
					println!("Unknown error: {:?}", e);
					remove_indexes.push(i);
				}
			};
		}
		remove_indexes.reverse();
		for remove_index in remove_indexes {
			try!(client_removed_callback(&mut self.clients[remove_index]));
			println!("Removing at {}", remove_index);
			self.clients.remove(remove_index);
		}
		Ok(())
	}
}